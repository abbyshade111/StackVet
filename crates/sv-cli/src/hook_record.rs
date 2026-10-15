//! The hook record (backlog 0566, ADR-122): a line for each command the AI coding tool ran and each file it wrote, kept
//! by a hook the person installed, outside the app's folder, and read only when the person asks for it for one report.
//!
//! `sv connect claude --hook` prints the hook; the person adds it to Claude Code's settings. The hook runs
//! `sv hook-record --app DIR` after each of the tool's commands and file writes and when it ends a turn, and hands it
//! the fields Claude Code's hooks page documents (`hook_event_name`, `session_id`, `tool_name`, `tool_input`). A line
//! holds the time by `sv`'s clock, the event and tool, the session, and the command, cut short and with any key cut as
//! the secrets scan cuts one, or the path of the file written. Never a file's contents, a command's output, the person's
//! prompts, or the tool's words.
//!
//! Each line is chained to the one before it, as the build-loop record's are (ADR-076): the tool runs as the person
//! and could rewrite the record, and the chain, with its end kept under each report's seal, shows a change made after a
//! report in the next one.

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use sv_frameworks::paths::Canonical;

/// The tools whose use is recorded: those that run a command or write a file, as Claude Code's hooks page lists them.
pub const TOOLS: [&str; 4] = ["Bash", "PowerShell", "Edit", "Write"];

/// The events recorded: a tool that finished, one that failed (a command that exits with an error, such as failing
/// tests), and a turn's end.
pub const EVENTS: [&str; 3] = ["PostToolUse", "PostToolUseFailure", "Stop"];

/// The most of a command a line keeps.
const COMMAND_CHARS: usize = 300;

/// The most the hook's input is read of, and the most a record is let grow to; past it, nothing more is written, and
/// `sv hook-record` says so.
const MAX_INPUT: u64 = 1024 * 1024;
pub const MAX_BYTES: u64 = 16 * 1024 * 1024;

/// The chain a record starts from.
const GENESIS: &str = "sv hook record, before its first line";

/// Where the records are kept: `$XDG_STATE_HOME/stackvet/hook-records`, or `~/.local/state/stackvet/hook-records`.
pub fn folder() -> Option<PathBuf> {
    std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .filter(|p| p.is_absolute())
                .map(|home| home.join(".local/state"))
        })
        .map(|state| {
            state
                .join(sv_frameworks::names::CONFIG_DIR)
                .join("hook-records")
        })
}

/// The record of the app at `app` (a full path): one file per app, named by a hash of its path, so the name tells
/// nothing of the app.
pub fn record_of(app: &Path) -> Option<PathBuf> {
    let key = crate::bundle::sha256(app.display().to_string().as_bytes());
    folder().map(|f| f.join(format!("{}.jsonl", &key[..16])))
}

/// The line the hook's `input` makes, for the app at `app`, at `at`, or `None` for an event or tool not recorded.
pub fn line_of(
    input: &Value,
    app: &Path,
    at: &str,
    rules: &sv_check::secrets::SecretRules,
) -> Option<Value> {
    let event = input["hook_event_name"].as_str()?;
    let mut line = json!({ "at": at, "event": event });
    if let Some(session) = input["session_id"].as_str() {
        line["session"] = json!(sv_check::text::cut_with_count(session, 80));
    }
    match event {
        "Stop" => {
            // What the tool said it had done, by kind, never its words (backlog 0565, ADR-122, Later).
            let claims = claims_in(input["last_assistant_message"].as_str().unwrap_or_default());
            if !claims.is_empty() {
                line["claims"] = json!(claims);
            }
        }
        "PostToolUse" | "PostToolUseFailure" => {
            let tool = input["tool_name"].as_str().filter(|t| TOOLS.contains(t))?;
            line["tool"] = json!(tool);
            if matches!(tool, "Bash" | "PowerShell") {
                let command = input["tool_input"]["command"].as_str()?;
                let (redacted, _) = sv_check::secrets::redact_text(rules, command);
                let one_line: String = redacted
                    .chars()
                    .map(|c| if c.is_control() { ' ' } else { c })
                    .collect();
                line["command"] = json!(sv_check::text::cut_with_count(
                    one_line.trim(),
                    COMMAND_CHARS
                ));
            } else {
                let file = input["tool_input"]["file_path"].as_str()?;
                let file = Path::new(file);
                let shown = file
                    .strip_prefix(app)
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| file.display().to_string());
                line["file"] = json!(shown);
            }
        }
        _ => return None,
    }
    Some(line)
}

/// The hash that chains a line to the one before it, over the previous line's chain and this line's own text.
fn chain_of(previous: &str, body: &str) -> String {
    crate::bundle::sha256(format!("{previous}\n{body}").as_bytes())
}

/// `sv hook-record --app DIR`: one line for what the hook hands over on standard input. An event or tool not recorded
/// writes nothing. Never blocks the tool: a failure is said on standard error and the exit status is 1, which Claude
/// Code shows as a hook error and carries on.
pub fn command(args: &[String]) -> Result<()> {
    let mut app = None;
    let mut words = args.iter();
    while let Some(word) = words.next() {
        match word.as_str() {
            "--app" => {
                app = Some(PathBuf::from(
                    words.next().context("--app needs the app's folder")?,
                ))
            }
            other => bail!("unknown option: {other}"),
        }
    }
    let app =
        app.context("--app needs the app's folder, as `sv connect claude --hook` prints it")?;
    let app = app
        .canonical()
        .with_context(|| format!("{} is not a folder sv can open", app.display()))?;
    let mut input = String::new();
    std::io::stdin()
        .take(MAX_INPUT)
        .read_to_string(&mut input)
        .context("reading what the hook handed over")?;
    let input: Value =
        serde_json::from_str(&input).context("what the hook handed over is not JSON")?;
    let rules = sv_check::secrets::SecretRules::shared()
        .map_err(|why| anyhow::anyhow!("loading the secret rules: {why}"))?;
    let Some(line) = line_of(&input, &app, &now(), rules) else {
        return Ok(());
    };
    let path = record_of(&app).context(
        "this computer has no folder for sv's records (neither HOME nor XDG_STATE_HOME is set)",
    )?;
    append(&path, line)
}

fn now() -> String {
    crate::bundle::utc_time(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    )
}

/// Appends `line` to the record at `path`, chained to the last line, under a lock, so two hooks at once cannot both
/// chain to the same line.
pub fn append(path: &Path, mut line: Value) -> Result<()> {
    let folder = path.parent().context("the record has no folder")?;
    std::fs::create_dir_all(folder).with_context(|| format!("making {}", folder.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(folder, std::fs::Permissions::from_mode(0o700));
    }
    crate::refuse_link(
        path,
        "Remove the link; sv writes its record only as a plain file.",
    )?;
    if let Ok(meta) = std::fs::symlink_metadata(path)
        && !meta.is_file()
    {
        bail!(
            "{} is not an ordinary file, so sv does not write its record there. Remove it.",
            path.display()
        );
    }
    let mut options = std::fs::OpenOptions::new();
    options.create(true).read(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW).mode(0o600);
    }
    let mut file = options
        .open(path)
        .with_context(|| format!("opening {}", path.display()))?;
    file.lock()
        .with_context(|| format!("locking {}", path.display()))?;
    if file.metadata().is_ok_and(|m| m.len() >= MAX_BYTES) {
        bail!(
            "{} has reached {} MB, so nothing more is recorded; move it aside to start a new one",
            path.display(),
            MAX_BYTES / (1024 * 1024)
        );
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .with_context(|| format!("reading {}", path.display()))?;
    let previous = String::from_utf8_lossy(&bytes)
        .lines()
        .rev()
        .filter_map(|raw| serde_json::from_str::<Value>(raw).ok())
        .find_map(|v| v.get("chain")?.as_str().map(str::to_owned))
        .unwrap_or_else(|| GENESIS.to_owned());
    line["chain"] = json!(chain_of(&previous, &line.to_string()));
    let mut text = line.to_string();
    text.push('\n');
    file.write_all(text.as_bytes())
        .with_context(|| format!("writing {}", path.display()))
}

/// The kinds of claim `said` makes that the record can be read against (backlog 0565): today one, that the app's tests
/// pass. A sentence claims it when it names tests and passing and says nothing of failing, of not passing, or of
/// passing only some day or on some condition.
pub fn claims_in(said: &str) -> Vec<&'static str> {
    let claims_tests_pass = said
        .split(['.', '!', '?', '\n', ';'])
        .map(str::to_lowercase)
        .any(|sentence| {
            let words: Vec<&str> = sentence
                .split(|c: char| !c.is_alphanumeric() && c != '\'')
                .filter(|w| !w.is_empty())
                .collect();
            let tests = words.iter().any(|w| w.starts_with("test"));
            let pass = words
                .iter()
                .any(|w| matches!(*w, "pass" | "passes" | "passed" | "passing" | "green"));
            let against = words.iter().any(|w| {
                w.starts_with("fail")
                    || matches!(
                        *w,
                        "not"
                            | "no"
                            | "don't"
                            | "doesn't"
                            | "didn't"
                            | "won't"
                            | "can't"
                            | "couldn't"
                            | "never"
                            // An intention or a condition is not a claim that they do.
                            | "will"
                            | "i'll"
                            | "should"
                            | "would"
                            | "could"
                            | "might"
                            | "need"
                            | "needs"
                            | "make"
                            | "once"
                            | "until"
                            | "if"
                            | "try"
                            | "unless"
                    )
            });
            tests && pass && !against
        });
    if claims_tests_pass {
        vec![TESTS_PASS]
    } else {
        Vec::new()
    }
}

/// The claim that the app's tests pass.
pub const TESTS_PASS: &str = "tests-pass";

/// Whether a command runs an app's tests: the test runners of the languages `sv` reads, by their own command words.
pub fn runs_tests(command: &str) -> bool {
    const RUNNERS: [&str; 16] = [
        "pytest",
        "py.test",
        "unittest",
        "npm test",
        "npm run test",
        "npm t",
        "yarn test",
        "pnpm test",
        "jest",
        "vitest",
        "cargo test",
        "go test",
        "rspec",
        "phpunit",
        "dotnet test",
        "manage.py test",
    ];
    let words: Vec<&str> = command.split_whitespace().collect();
    let joined = format!(" {} ", words.join(" "));
    RUNNERS.iter().any(|runner| {
        joined.contains(&format!(" {runner} "))
            || words.iter().any(|w| w.rsplit('/').next() == Some(runner))
    })
}

/// What the record at `path` holds, for a report, read against the last sealed report's tip, when there was one.
pub fn read(path: &Path, earlier: Option<&sv_report::hook::Tip>) -> sv_report::hook::HookSeen {
    use sv_report::hook::{Earlier, HookSeen, Mismatch, Tip};
    let mut seen = HookSeen::default();
    if !std::fs::symlink_metadata(path).is_ok_and(|m| m.is_file()) {
        seen.not_read = Some(
            "there is no hook record for this app on this computer: the hook has not been installed, or has not yet \
             run for this folder"
                .to_owned(),
        );
        return seen;
    }
    let mut bytes = Vec::new();
    if let Err(e) =
        std::fs::File::open(path).and_then(|f| f.take(MAX_BYTES + 1).read_to_end(&mut bytes))
    {
        seen.not_read = Some(format!("the hook record could not be read ({e})"));
        return seen;
    }
    let mut previous = GENESIS.to_owned();
    let mut chains: Vec<String> = Vec::new();
    // Each session's last test command, when, and whether it failed, and the files written after it: a claim that the
    // tests pass is read against its own session's commands (backlog 0565).
    let mut sessions: std::collections::HashMap<String, Session> = std::collections::HashMap::new();
    let mut lines = 0;
    for raw in String::from_utf8_lossy(&bytes).lines() {
        if raw.trim().is_empty() {
            continue;
        }
        lines += 1;
        let Ok(mut value) = serde_json::from_str::<Value>(raw) else {
            seen.broken_at.get_or_insert(lines);
            continue;
        };
        if seen.broken_at.is_none() {
            let stored = value
                .get("chain")
                .and_then(Value::as_str)
                .map(str::to_owned);
            if let Some(object) = value.as_object_mut() {
                object.remove("chain");
            }
            match stored {
                Some(stored) if chain_of(&previous, &value.to_string()) == stored => {
                    seen.chained += 1;
                    previous = stored.clone();
                    chains.push(stored);
                }
                _ => seen.broken_at = Some(lines),
            }
        }
        let at = value["at"].as_str().map(str::to_owned);
        if seen.first.is_none() {
            seen.first = at.clone();
        }
        if at.is_some() {
            seen.last = at.clone();
        }
        let session = value["session"].as_str().unwrap_or_default().to_owned();
        match (value["event"].as_str(), value["tool"].as_str()) {
            (Some("Stop"), _) => {
                seen.turns += 1;
                let claims = value["claims"].as_array().cloned().unwrap_or_default();
                if claims.iter().any(|c| c == TESTS_PASS) {
                    seen.claims += 1;
                    let at = at.clone().unwrap_or_default();
                    let mismatch = match sessions.get(&session) {
                        None
                        | Some(Session {
                            last_test: None, ..
                        }) => Some(Mismatch::NoTestRan),
                        Some(Session {
                            last_test: Some((when, true)),
                            ..
                        }) => Some(Mismatch::LastTestFailed { at: when.clone() }),
                        Some(Session {
                            last_test: Some((when, false)),
                            written_after,
                        }) if *written_after > 0 => Some(Mismatch::WrittenSince {
                            at: when.clone(),
                            files: *written_after,
                        }),
                        Some(_) => None,
                    };
                    if let Some(what) = mismatch {
                        seen.mismatches.push(sv_report::hook::Said { at, what });
                    }
                }
            }
            (Some(event @ ("PostToolUse" | "PostToolUseFailure")), Some("Bash" | "PowerShell")) => {
                seen.commands += 1;
                let command = value["command"].as_str().unwrap_or_default();
                if runs_tests(command) {
                    seen.tests += 1;
                    let failed = event == "PostToolUseFailure";
                    sessions.insert(
                        session,
                        Session {
                            last_test: Some((at.clone().unwrap_or_default(), failed)),
                            written_after: 0,
                        },
                    );
                    seen.last_test = Some((at.unwrap_or_default(), command.to_owned()));
                    seen.last_test_failed = failed;
                    seen.written_after_last_test = 0;
                }
            }
            (Some("PostToolUse" | "PostToolUseFailure"), Some(_)) => {
                seen.writes += 1;
                seen.written_after_last_test += 1;
                if let Some(s) = sessions.get_mut(&session) {
                    s.written_after += 1;
                }
            }
            _ => {}
        }
    }
    seen.tip = chains.last().map(|chain| Tip {
        lines: chains.len(),
        chain: chain.clone(),
    });
    seen.earlier = earlier.map(|tip| {
        if chains.get(tip.lines.wrapping_sub(1)) == Some(&tip.chain) {
            Earlier::Kept { lines: tip.lines }
        } else {
            Earlier::NotKept { lines: tip.lines }
        }
    });
    seen
}

/// A session's last test command, as `read` follows it.
#[derive(Default)]
struct Session {
    last_test: Option<(String, bool)>,
    written_after: usize,
}

/// What the record of the app at `app_dir` holds, for a report the person asked to read it for.
pub fn read_for(
    app_dir: &Path,
    earlier: Option<&sv_report::hook::Tip>,
) -> sv_report::hook::HookSeen {
    match app_dir.canonical().ok().and_then(|app| record_of(&app)) {
        Some(path) => read(&path, earlier),
        None => sv_report::hook::HookSeen {
            not_read: Some(
                "this computer has no folder for sv's records (neither HOME nor XDG_STATE_HOME is set)".to_owned(),
            ),
            ..Default::default()
        },
    }
}

/// The tip the last report in `folder` kept, when `sv` can show it wrote that report and it read a record.
pub fn earlier_tip(folder: &Path) -> Option<sv_report::hook::Tip> {
    let bytes = crate::report_seal::proven_report(folder).ok()?;
    let report: Value = serde_json::from_slice(&bytes).ok()?;
    serde_json::from_value(report["hook_record"]["tip"].clone()).ok()
}
