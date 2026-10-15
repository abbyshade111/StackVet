//! The record of the build loop (ADR-076): one line for each call an AI coding tool makes to `sv`'s
//! MCP server for an app, written in the app's hidden `.stackvet` folder, and read back when a report
//! is written so the report can say how the app was built with `sv`, or that nothing shows it was.
//!
//! Until 14 October 2026 it was kept in the report folder, and an AI coding tool that cleared that
//! folder before a report took the record with it: the report then said nothing showed `sv` was used,
//! after dozens of calls (backlog 0404). It is kept apart from the report now, and a record found in
//! the old place is moved to the new one when the next line is written.
//!
//! Each line holds the time, the tool's name, and, for a call that checked the app, how many findings
//! and requirements it came to; never an argument, a path, or a finding's text. The record is not
//! sealed: it grows between reports, and each report carries what it read from it, under the report's
//! own seal. It is evidence about how the app was built, never about the app.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;
use sv_report::{BuildLoop, LoopCounts};

/// The largest record written to or read: tens of thousands of calls. Beyond it, nothing more is
/// written, and a report reads the first part and says the record is full (`BuildLoop::full`).
pub const MAX_BYTES: u64 = 4 * 1024 * 1024;

/// One call, as a line of the record.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Line {
    pub time: String,
    pub tool: String,
    pub counts: Option<LoopCounts>,
    /// How the call ended (ADR-084): `ok`, `failed`, `timed-out`, `crashed` (a fault in `sv`), or
    /// `refused` (a tool `sv` does not have). Never the error's words, which can
    /// quote the app. `None` in a line written before it.
    pub outcome: Option<String>,
    /// The AI coding tool, as it named itself when it connected, cut short (`client_name`).
    pub client: Option<String>,
    /// The `sv` that answered.
    pub sv: Option<String>,
    /// A line that says only that the record was turned off at this time, so the gap after it is
    /// seen as one (ADR-084); `tool` is empty.
    pub off: bool,
    /// At a check, its findings' fingerprints (hashes, never text), sorted, at most
    /// `MAX_FINGERPRINTS` (ADR-084, decision 6). `None` in a line that is not a check, or was written
    /// before it.
    pub fingerprints: Option<Vec<String>>,
    /// The check had more findings than `MAX_FINGERPRINTS`, so `fingerprints` holds only some, and
    /// no comparison is made from it.
    pub fingerprints_cut: bool,
    /// At a check, for each fingerprint of a finding on a line of code: the rule that found it and a key for its file
    /// (`sv_check::review::file_key`, a hash of the path, never the path), so a later report can look again at the line
    /// of a finding that went (backlog 0401). Empty in a line written before it.
    pub placed: BTreeMap<String, (String, String)>,
    /// Names `sv` itself defines that the call asked about (ADR-084, decision 3): a feature or
    /// topic from the tool's own list, a requirement id. Never a word the AI tool chose.
    pub asked: Vec<String>,
    /// What `sv` handed over since the last line (ADR-084, decision 5): `instructions` (given when
    /// the AI tool connected), `prompt:<id>` (a prompt fetched), `report:<file>` (a report file read).
    pub handed: Vec<String>,
    /// The size in bytes of each thing `handed` names that has one: a report file read, a prompt
    /// fetched (backlog 0227, decided 11 October 2026, ADR-084). Keyed by the same name as `handed`;
    /// empty for a line written before sizes were kept, and then the dashboard says so.
    pub handed_sizes: BTreeMap<String, u64>,
    /// The call came at the folder the server serves before it held a `stackvet.toml` (backlog 0586): the AI tool's
    /// first calls, `stackvet_status` and the spec, are written down too, and say so.
    pub no_manifest: bool,
}

/// The most names a line keeps of what a call asked, and of what was handed over.
pub const MAX_NAMES: usize = 20;

/// Whether `s` is a name as `sv` defines them: `sign-in`, `V1.2.4`, `read-my-report`, `report.html`.
pub fn is_name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
}

/// Whether `s` is something `sv` hands over, as a line keeps it.
fn is_handed(s: &str) -> bool {
    s == "instructions"
        || s.strip_prefix("prompt:").is_some_and(is_name)
        || s.strip_prefix("report:").is_some_and(is_name)
}

/// The most findings' fingerprints one line keeps: about 17 KB, so the record's 4 MB still holds
/// hundreds of checks of the largest apps.
pub const MAX_FINGERPRINTS: usize = 1000;

/// A check's findings' fingerprints as a line keeps them: sorted, each once, empty ones left out,
/// at most `MAX_FINGERPRINTS`; and whether some were left out for that limit.
pub fn fingerprints_of<'a>(fingerprints: impl IntoIterator<Item = &'a str>) -> (Vec<String>, bool) {
    let mut all: Vec<String> = fingerprints
        .into_iter()
        .filter(|f| !f.is_empty())
        .map(str::to_owned)
        .collect();
    all.sort();
    all.dedup();
    let cut = all.len() > MAX_FINGERPRINTS;
    all.truncate(MAX_FINGERPRINTS);
    (all, cut)
}

/// For each of a check's findings on a line of code whose fingerprint `kept` holds: the rule that found it and a key
/// for its file, as a line keeps them (backlog 0401).
pub fn placed_of(
    findings: &[sv_check::Finding],
    kept: &[String],
) -> BTreeMap<String, (String, String)> {
    findings
        .iter()
        .filter(|f| {
            f.fingerprint.starts_with(sv_check::review::FINGERPRINT_V2)
                && f.location.is_file()
                && is_rule(&f.rule_id)
                && kept.binary_search(&f.fingerprint).is_ok()
        })
        .map(|f| {
            (
                f.fingerprint.clone(),
                (
                    f.rule_id.clone(),
                    sv_check::review::file_key(&f.location.file),
                ),
            )
        })
        .collect()
}

/// Whether `s` is a rule's id as `sv` and the outside tools it runs name one (`config.debug-mode`, `bandit.B104`,
/// `semgrep.python.lang.security.audit.x`): no spaces, and not too long.
fn is_rule(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 160
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.:/".contains(c))
}

/// Whether `s` is a file's key as `sv_check::review::file_key` makes one.
fn is_file_key(s: &str) -> bool {
    s.len() == 16 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Whether `s` is a fingerprint as `sv` writes one (`882dedc677bff5a5`, `v2-61eb1668992f3990`):
/// letters, digits, and hyphens, and not too long to be one. A finding's words have spaces.
fn is_fingerprint(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// The outcomes a line may name, and nothing else.
pub const OUTCOMES: [&str; 5] = ["ok", "failed", "timed-out", "crashed", "refused"];

/// What an AI coding tool calls itself, as it is kept: letters, digits, spaces, and `.-_+/()`
/// only, each part at most 40 characters. The name is the tool's own, sent when it connects; it is
/// kept short and plain so nothing it sends can run on into the record or the report.
pub fn client_name(name: &str, version: &str) -> Option<String> {
    let clean = |s: &str| -> String {
        s.chars()
            .filter(|c| c.is_ascii_alphanumeric() || " .-_+/()".contains(*c))
            .take(40)
            .collect::<String>()
            .trim()
            .to_owned()
    };
    let (name, version) = (clean(name), clean(version));
    match (name.is_empty(), version.is_empty()) {
        (true, _) => None,
        (false, true) => Some(name),
        (false, false) => Some(format!("{name} {version}")),
    }
}

impl Line {
    /// The line as the record writes it: one JSON object.
    pub fn to_json(&self) -> String {
        if self.off {
            return json!({ "time": self.time, "off": true }).to_string();
        }
        let mut line = json!({ "time": self.time, "tool": self.tool });
        for (key, value) in [
            ("outcome", &self.outcome),
            ("client", &self.client),
            ("sv", &self.sv),
        ] {
            if let Some(value) = value {
                line[key] = json!(value);
            }
        }
        if let Some(fingerprints) = &self.fingerprints {
            line["fingerprints"] = json!(fingerprints);
        }
        if self.fingerprints_cut {
            line["fingerprints_cut"] = json!(true);
        }
        if !self.placed.is_empty() {
            line["placed"] = json!(self.placed);
        }
        if !self.asked.is_empty() {
            line["asked"] = json!(self.asked);
        }
        if !self.handed.is_empty() {
            line["handed"] = json!(self.handed);
        }
        if !self.handed_sizes.is_empty() {
            line["sizes"] = json!(self.handed_sizes);
        }
        if self.no_manifest {
            line["no_manifest"] = json!(true);
        }
        if let Some(c) = &self.counts {
            line["counts"] = json!({
                "findings": c.findings,
                "checked": c.checked,
                "needs_attention": c.needs_attention,
                "not_assessed": c.not_assessed,
            });
        }
        line.to_string()
    }

    /// A line of the record read back; `None` for one that is not a line `sv` writes.
    pub fn parse(raw: &str) -> Option<Line> {
        let v: Value = serde_json::from_str(raw).ok()?;
        let text = |key: &str| v.get(key)?.as_str().map(str::to_owned);
        if v.get("off") == Some(&Value::Bool(true)) {
            return Some(Line {
                time: text("time")?,
                off: true,
                ..Line::default()
            });
        }
        // Fingerprints that are not hex are not ones `sv` wrote, nor is a list past the limit.
        let fingerprints = match v.get("fingerprints") {
            None => None,
            Some(list) => {
                let list = list.as_array()?;
                if list.len() > MAX_FINGERPRINTS {
                    return None;
                }
                Some(
                    list.iter()
                        .map(|f| f.as_str().filter(|f| is_fingerprint(f)).map(str::to_owned))
                        .collect::<Option<Vec<String>>>()?,
                )
            }
        };
        // A fingerprint placed that the line does not list, a rule that is not a name, or a key that is not sixteen hex
        // characters makes the line one `sv` did not write.
        let placed = match v.get("placed") {
            None => BTreeMap::new(),
            Some(map) => {
                let map = map.as_object()?;
                let listed = fingerprints.as_ref()?;
                if map.len() > MAX_FINGERPRINTS {
                    return None;
                }
                map.iter()
                    .map(|(fp, pair)| {
                        let pair = pair.as_array().filter(|p| p.len() == 2)?;
                        let rule = pair[0].as_str().filter(|r| is_rule(r))?;
                        let key = pair[1].as_str().filter(|k| is_file_key(k))?;
                        listed
                            .contains(fp)
                            .then(|| (fp.clone(), (rule.to_owned(), key.to_owned())))
                    })
                    .collect::<Option<BTreeMap<_, _>>>()?
            }
        };
        // Names that are not ones `sv` writes make the line one it did not write.
        let names = |key: &str, ok: fn(&str) -> bool| -> Option<Vec<String>> {
            match v.get(key) {
                None => Some(Vec::new()),
                Some(list) => {
                    let list = list.as_array()?;
                    if list.len() > MAX_NAMES {
                        return None;
                    }
                    list.iter()
                        .map(|n| n.as_str().filter(|n| ok(n)).map(str::to_owned))
                        .collect()
                }
            }
        };
        let asked = names("asked", is_name)?;
        let handed = names("handed", is_handed)?;
        // A size is kept only for a name the line hands over, and not past the limit: anything else
        // makes the line one `sv` did not write.
        let handed_sizes = match v.get("sizes") {
            None => BTreeMap::new(),
            Some(sizes) => {
                let sizes = sizes.as_object()?;
                if sizes.len() > MAX_NAMES {
                    return None;
                }
                sizes
                    .iter()
                    .map(|(name, size)| {
                        if !is_handed(name) || !handed.contains(name) {
                            return None;
                        }
                        Some((name.clone(), size.as_u64()?))
                    })
                    .collect::<Option<BTreeMap<String, u64>>>()?
            }
        };
        // An outcome `sv` does not write makes the line one it did not write.
        let outcome = match v.get("outcome") {
            None => None,
            Some(o) => Some(o.as_str().filter(|o| OUTCOMES.contains(o))?.to_owned()),
        };
        let counts = match v.get("counts") {
            None => None,
            Some(c) => {
                let n = |key: &str| c.get(key)?.as_u64().and_then(|n| usize::try_from(n).ok());
                Some(LoopCounts {
                    findings: n("findings")?,
                    checked: n("checked")?,
                    needs_attention: n("needs_attention")?,
                    not_assessed: n("not_assessed")?,
                })
            }
        };
        Some(Line {
            time: text("time")?,
            tool: text("tool")?,
            counts,
            outcome,
            client: text("client"),
            sv: text("sv"),
            off: false,
            fingerprints,
            fingerprints_cut: v.get("fingerprints_cut") == Some(&Value::Bool(true)),
            placed,
            asked,
            handed,
            handed_sizes,
            no_manifest: v.get("no_manifest") == Some(&Value::Bool(true)),
        })
    }
}

/// Whether the app's `stackvet.toml` turns the record off. A manifest that cannot be read leaves it
/// on: the check says what is wrong with the file, and the record is no reason to say it twice.
pub fn off(app_dir: &Path) -> bool {
    sv_manifest::locate(app_dir)
        .ok()
        .flatten()
        .and_then(|located| sv_manifest::Manifest::load(&located.path).ok())
        .is_some_and(|m| m.app.build_loop_record == Some(false))
}

/// Writes down one call for the app at `app_dir`, unless its manifest turns the record off. The
/// report folder is made if it is not there, one level at a time and never through a link, and the
/// record is opened without following one, so a link planted in the app cannot send the line
/// anywhere else. Any failure is the caller's to ignore: the record never stops a tool's answer.
pub fn record(app_dir: &Path, line: &Line) -> Result<()> {
    if off(app_dir) && !line.off {
        return Ok(());
    }
    let written = append(app_dir, line);
    if written.is_err() {
        *unwritten()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .entry(key(app_dir))
            .or_default() += 1;
    }
    written
}

/// The calls this process could not write into each app's record, by app: what the next report
/// written by this process says (ADR-084). A report written by another process, `sv report` at a
/// terminal, cannot know of them, and says nothing.
fn unwritten() -> &'static std::sync::Mutex<std::collections::HashMap<std::path::PathBuf, usize>> {
    static UNWRITTEN: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<std::path::PathBuf, usize>>,
    > = std::sync::OnceLock::new();
    UNWRITTEN.get_or_init(Default::default)
}

/// The app's folder as `unwritten` keeps it: its real place, so two ways of naming it are one.
fn key(app_dir: &Path) -> std::path::PathBuf {
    sv_frameworks::paths::canonical(app_dir).unwrap_or_else(|_| app_dir.to_path_buf())
}

/// The record in the report folder, where it was kept before 14 October 2026 (backlog 0404).
fn old_place(app_dir: &Path) -> std::path::PathBuf {
    sv_scan::ecosystems::default_report_dir_in(app_dir).join(sv_scan::ecosystems::BUILD_LOOP_RECORD)
}

fn append(app_dir: &Path, line: &Line) -> Result<()> {
    let (folder, _) = crate::mcp::create_below(app_dir, Path::new(sv_scan::ecosystems::STATE_DIR))?;
    let path = folder.join(sv_scan::ecosystems::BUILD_LOOP_RECORD);
    crate::refuse_link(
        &path,
        "Remove the link; sv writes its record only as a plain file.",
    )?;
    // A record kept in the report folder before the move is `sv`'s own: moved, whole, so its chain carries on.
    let old = old_place(app_dir);
    if std::fs::symlink_metadata(&path).is_err()
        && std::fs::symlink_metadata(&old).is_ok_and(|m| m.is_file())
    {
        std::fs::rename(&old, &path)
            .with_context(|| format!("moving {} to {}", old.display(), path.display()))?;
    }
    // A pipe here is opened to write and waits for a reader, on the server's one thread for answering
    // (backlog 0282). Only a plain file is written.
    if let Ok(meta) = std::fs::symlink_metadata(&path)
        && !meta.is_file()
    {
        anyhow::bail!(
            "{} is not an ordinary file (it is a pipe, a device, a socket, or a folder), so sv does not \
             write its record there. Remove it; sv writes its record only as a plain file.",
            path.display()
        );
    }
    if std::fs::metadata(&path).is_ok_and(|m| m.len() >= MAX_BYTES) {
        return Ok(());
    }
    let mut options = std::fs::OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    let mut file = options
        .open(&path)
        .with_context(|| format!("opening {}", path.display()))?;
    // Each line is chained to the one before it (backlog 0239): its hash covers the previous line's chain and this
    // line's own text without the chain, so an edit to a line, or a line removed or added, breaks the chain after it.
    let previous = std::fs::read(&path)
        .ok()
        .and_then(|bytes| last_chain(&bytes))
        .unwrap_or_else(|| GENESIS.to_owned());
    let mut value: Value = serde_json::from_str(&line.to_json()).context("the line as JSON")?;
    value["chain"] = json!(chain_of(&previous, &value.to_string()));
    let mut text = value.to_string();
    text.push('\n');
    file.write_all(text.as_bytes())
        .with_context(|| format!("writing {}", path.display()))
}

/// The chain a record starts from, before its first chained line (backlog 0239).
const GENESIS: &str = "sv build-loop record, before its first chained line";

/// The hash that chains a line to the one before it: over the previous line's chain, and this line's own text without
/// its chain (backlog 0239).
fn chain_of(previous: &str, body: &str) -> String {
    crate::bundle::sha256(format!("{previous}\n{body}").as_bytes())
}

/// The chain of the last line of `bytes` that carries one, or `None` when no line does.
fn last_chain(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .lines()
        .rev()
        .filter_map(|raw| serde_json::from_str::<Value>(raw).ok())
        .find_map(|value| value.get("chain")?.as_str().map(str::to_owned))
}

/// What checking a record's chain found (backlog 0239): how many chained lines check out, the line where the chain
/// first stops checking out (counting the record's lines from 1), and the last chain that checked out, its head.
/// Lines written before the chain carry none, and are not counted.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ChainCheck {
    pub chained: usize,
    pub broken_at: Option<usize>,
    pub head: Option<String>,
}

/// Checks each chained line in order against the one before it. After the first that does not check out, nothing more
/// is vouched for.
pub fn check_chain(bytes: &[u8]) -> ChainCheck {
    let mut check = ChainCheck::default();
    let mut previous = GENESIS.to_owned();
    for (number, raw) in String::from_utf8_lossy(bytes).lines().enumerate() {
        if check.broken_at.is_some() || raw.trim().is_empty() {
            continue;
        }
        let Ok(mut value) = serde_json::from_str::<Value>(raw) else {
            continue;
        };
        let Some(stored) = value
            .get("chain")
            .and_then(Value::as_str)
            .map(str::to_owned)
        else {
            continue;
        };
        if let Some(object) = value.as_object_mut() {
            object.remove("chain");
        }
        if chain_of(&previous, &value.to_string()) == stored {
            check.chained += 1;
            previous = stored.clone();
            check.head = Some(stored);
        } else {
            check.broken_at = Some(number + 1);
        }
    }
    check
}

/// What the record for the app at `app_dir` shows, for a report about to be written.
pub fn read(app_dir: &Path) -> BuildLoop {
    let unwritten = unwritten()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&key(app_dir))
        .copied()
        .unwrap_or(0);
    let mut summary = read_record(app_dir);
    summary.unwritten = unwritten;
    if summary.calls == 0 && !summary.off {
        summary.signs_of_use = signs_of_use(app_dir);
    }
    summary
}

/// With no call recorded, what shows `sv` was used with the app all the same (backlog 0404, the owner's answer of
/// 14 October 2026), in words for the report: the record may have been deleted. Each is something only `sv`'s own
/// instructions or tools write; none is proof of a call to the MCP server, and the report says only that the record
/// may have been deleted.
pub fn signs_of_use(app_dir: &Path) -> Vec<String> {
    let mut signs = Vec::new();
    let manifest = sv_manifest::locate(app_dir)
        .ok()
        .flatten()
        .and_then(|located| read_small(&located.path));
    if manifest.is_some_and(|text| {
        text.lines()
            .any(|l| l.replace(' ', "").contains("by=\"ai-tool\""))
    }) {
        signs.push(
            "stackvet.toml holds answers marked as the AI coding tool's (`by = \"ai-tool\"`), as sv \
             asks the tool to mark them"
                .to_owned(),
        );
    }
    if read_small(&app_dir.join("security-notes.md")).is_some_and(|text| {
        text.lines()
            .any(|l| l.trim_start().starts_with("Written by:"))
    }) {
        signs.push(
            "security-notes.md has sections with a `Written by:` line, the form sv gives them"
                .to_owned(),
        );
    }
    if sv_scan::ecosystems::report_marker_in(&sv_scan::ecosystems::default_report_dir_in(app_dir))
        .and_then(|marker| read_small(&marker))
        .is_some_and(|text| {
            text.lines()
                .any(|l| l.starts_with(crate::report_seal::SEAL_LINE))
        })
    {
        signs.push("the report folder holds an earlier report that sv sealed".to_owned());
    }
    signs
}

/// A small text file's contents, when it is a plain file: never through a link, and never one larger than a person
/// writes by hand.
fn read_small(path: &Path) -> Option<String> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    if !meta.is_file() || meta.len() > 1_000_000 {
        return None;
    }
    std::fs::read_to_string(path).ok()
}

fn read_record(app_dir: &Path) -> BuildLoop {
    if off(app_dir) {
        return BuildLoop {
            off: true,
            ..BuildLoop::default()
        };
    }
    // The record where it is kept, or, before the next line moves it, where it was kept (backlog 0404).
    let new = sv_scan::ecosystems::build_loop_record_in(app_dir);
    let path = if std::fs::symlink_metadata(&new).is_err() {
        old_place(app_dir)
    } else {
        new
    };
    match std::fs::symlink_metadata(&path) {
        Ok(meta) if meta.is_file() => {
            let bytes = read_at_most(&path);
            let mut summary = summarize(&bytes);
            summary.full = meta.len() >= MAX_BYTES;
            let chain = check_chain(&bytes);
            summary.chained = chain.chained;
            summary.chain_broken_at = chain.broken_at;
            summary.chain_head = chain.head;
            summary
        }
        // A link, a folder, or nothing at all: no record sv wrote.
        _ => BuildLoop::default(),
    }
}

/// Between the first check and the last, how many findings were no longer found, how many of those
/// a person set aside as false alarms in the report about to be written (`set_aside`, the
/// fingerprints of its false alarms), and how many were new (ADR-084, decision 6). Left unsaid
/// when there was one check, or either check's line did not keep every fingerprint.
pub fn settle(summary: &mut BuildLoop, set_aside: &[String]) {
    let (Some(first), Some(last)) = (&summary.first_fingerprints, &summary.last_fingerprints)
    else {
        return;
    };
    if summary.checks < 2 {
        return;
    }
    let gone: Vec<&String> = first.iter().filter(|f| !last.contains(f)).collect();
    let set = gone.iter().filter(|f| set_aside.contains(f)).count();
    summary.findings_moved = Some(sv_report::FindingsMoved {
        no_longer_found: gone.len() - set,
        set_aside: set,
        new: last.iter().filter(|f| !first.contains(f)).count(),
    });
}

/// The most findings that went a report looks for again in the app's code (backlog 0401): each costs its file's
/// length squared at worst.
const MOST_LOOKED_AGAIN: usize = 50;

/// Of the findings no longer found between the first check and the last, and not set aside as false alarms
/// (`set_aside`), those whose line still reads in the app as it did when the first check found it: as written, or but
/// for a comment that tells a scanner to look away (backlog 0401). A fix changes the line, so these went some other
/// way, and the report names them. Only a finding the first check placed (`Line::placed`) can be looked for, and at
/// most `MOST_LOOKED_AGAIN`; a line rewritten to do the same thing is not found, and the report says so.
pub fn look_again(summary: &mut BuildLoop, app_dir: &Path, set_aside: &[String]) {
    let (Some(first), Some(last)) = (&summary.first_fingerprints, &summary.last_fingerprints)
    else {
        return;
    };
    if summary.checks < 2 {
        return;
    }
    let gone: Vec<(String, String, String)> = first
        .iter()
        .filter(|f| !last.contains(f) && !set_aside.contains(f))
        .filter_map(|f| {
            let (rule, key) = summary.first_placed.get(f)?;
            Some((f.clone(), rule.clone(), key.clone()))
        })
        .collect();
    if gone.is_empty() {
        return;
    }
    let last: BTreeSet<String> = last.iter().cloned().collect();
    let listing = sv_scan::files::Listing::of(app_dir);
    let by_key: BTreeMap<String, String> = listing
        .app_files()
        .map(|e| (sv_check::review::file_key(&e.relative), e.relative.clone()))
        .collect();
    summary.looked_again = gone.len().min(MOST_LOOKED_AGAIN);
    summary.not_looked_again = gone.len() - summary.looked_again;
    for (fingerprint, rule, key) in gone.iter().take(MOST_LOOKED_AGAIN) {
        let Some(file) = by_key.get(key) else {
            continue;
        };
        let Some((line, how)) = sv_check::review::still_reads(app_dir, rule, file, fingerprint)
        else {
            continue;
        };
        // Still found on that line, under the fingerprint its words have now: it did not go, so it is not counted
        // among those looked for either.
        if sv_check::review::fingerprint_now(app_dir, rule, file, line)
            .is_some_and(|now| last.contains(&now))
        {
            summary.looked_again -= 1;
            continue;
        }
        summary.went_same.push(sv_report::WentSame {
            rule: rule.clone(),
            file: file.clone(),
            line,
            how,
        });
    }
}

/// The record's first `MAX_BYTES`, as bytes, since one byte that is not UTF-8 must cost one line
/// and not the whole record. Cut at the limit, the line it cuts through is left out, not counted
/// as unreadable: `sv` wrote it whole.
fn read_at_most(path: &Path) -> Vec<u8> {
    use std::io::Read;
    let mut bytes = Vec::new();
    if let Ok(file) = std::fs::File::open(path) {
        let _ = file.take(MAX_BYTES).read_to_end(&mut bytes);
    }
    if bytes.len() as u64 >= MAX_BYTES {
        let whole = bytes
            .iter()
            .rposition(|&b| b == b'\n')
            .map_or(0, |at| at + 1);
        bytes.truncate(whole);
    }
    bytes
}

/// What a record's bytes show: every line read, in order; one that is not UTF-8 or does not read
/// as a line `sv` writes is counted, not skipped quietly (ADR-076).
pub fn summarize(bytes: &[u8]) -> BuildLoop {
    let mut summary = BuildLoop::default();
    for raw in bytes.split(|&b| b == b'\n') {
        let Ok(raw) = std::str::from_utf8(raw) else {
            summary.unreadable += 1;
            continue;
        };
        if raw.trim().is_empty() {
            continue;
        }
        let Some(line) = Line::parse(raw) else {
            summary.unreadable += 1;
            continue;
        };
        if line.off {
            summary.turned_off += 1;
            continue;
        }
        summary.calls += 1;
        match line.outcome.as_deref() {
            Some("failed") => summary.failed += 1,
            Some("timed-out") => summary.timed_out += 1,
            Some("crashed") => summary.crashed += 1,
            Some("refused") => summary.refused += 1,
            _ => {}
        }
        for (name, size) in &line.handed_sizes {
            summary.handed_sizes.entry(name.clone()).or_insert(*size);
        }
        for (seen, names) in [
            (&mut summary.asked, &line.asked),
            (&mut summary.handed, &line.handed),
        ] {
            for name in names {
                if !seen.contains(name) {
                    seen.push(name.clone());
                }
            }
        }
        for (seen, named) in [
            (&mut summary.clients, &line.client),
            (&mut summary.svs, &line.sv),
        ] {
            if let Some(named) = named
                && !seen.contains(named)
            {
                seen.push(named.clone());
            }
        }
        if summary.first.is_none() {
            summary.first = Some(line.time.clone());
        }
        summary.last = Some(line.time);
        if line.counts.is_some() {
            // What the first and the last check found, when the line kept all of it.
            let kept = line.fingerprints.clone().filter(|_| !line.fingerprints_cut);
            if summary.checks == 0 {
                summary.first_fingerprints = kept.clone();
                summary.first_placed = line.placed.clone();
            }
            summary.last_fingerprints = kept;
        }
        if let Some(counts) = line.counts {
            summary.checks += 1;
            if summary.first_counts.is_none() {
                summary.first_counts = Some(counts);
            }
            summary.last_counts = Some(counts);
        }
    }
    summary
}

#[cfg(test)]
mod fingerprint_tests;
#[cfg(test)]
mod limit_tests;
#[cfg(test)]
mod names_tests;
#[cfg(test)]
mod outcome_tests;
#[cfg(test)]
mod sizes_tests;
#[cfg(test)]
mod tests;

// Whether the chain of hashes holds, and where it breaks (backlog 0239).
#[cfg(test)]
mod chain_tail_tests;
#[cfg(test)]
mod chain_tests;

// Where each finding was, and the ones that went while their line still reads as it did (backlog 0401).
#[cfg(test)]
mod placed_tests;

// The record kept apart from the report folder, and what a report says when it is gone (backlog 0404).
#[cfg(test)]
mod kept_apart_tests;
