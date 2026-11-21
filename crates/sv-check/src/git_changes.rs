//! What each commit since a given one changed, for the report's section on what the AI coding tool did that is worth a
//! look (backlog 0630, ADR-131): the files, and the lines added and removed, of the commits whose message names an AI
//! coding tool in a trailer (`trailer_tool`, ADR-092). Git runs through `git`, with the overrides of ADR-032, and the
//! diffs are of committed blobs only, with no external diff, no textconv, and no colour, so nothing in the repository
//! runs.

use super::{LOG_OVERRIDES, git, trailer_tool};
use std::path::Path;

/// The most lines kept, added and removed together, of one commit's diff, and the most characters of each.
const LINES_KEPT: usize = 4000;
const LINE_CHARS: usize = 200;

/// A commit read for the section.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Commit {
    pub hash: String,
    pub subject: String,
    /// The AI coding tool its trailer names, when one does. Only such a commit's files and lines are read.
    pub tool: Option<String>,
    /// Each file it changed: git's status letter (`A`, `M`, `D`, …) and the path.
    pub files: Vec<(char, String)>,
    /// Lines it added and removed, each with its file, cut to `LINE_CHARS`.
    pub added: Vec<(String, String)>,
    pub removed: Vec<(String, String)>,
}

/// The commits read: those since `from` on HEAD's own line when `from` is in HEAD's history (`true`), else the last
/// `at_most` (`false`), newest first, merges left out. `None` when git could not say, or the app has no commits.
pub fn since(app_dir: &Path, from: Option<&str>, at_most: usize) -> Option<(Vec<Commit>, bool)> {
    let head = super::head_commit(app_dir)?;
    let from = from.filter(|f| {
        f.len() >= 7
            && f.bytes().all(|b| b.is_ascii_hexdigit())
            && git(app_dir)
                .args(["merge-base", "--is-ancestor", f, &head])
                .output()
                .is_ok_and(|o| o.status.success())
    });
    let range = match from {
        Some(f) => format!("{f}..{head}"),
        None => head.clone(),
    };
    let limit = format!("--max-count={at_most}");
    let mut log = git(app_dir);
    log.args(LOG_OVERRIDES)
        .args(["log", "--no-merges", "--no-show-signature", "--format=%H"]);
    if from.is_none() {
        log.arg(&limit);
    }
    let out = log.args([range.as_str(), "--"]).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let hashes: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::trim)
        .filter(|h| matches!(h.len(), 40 | 64) && h.bytes().all(|b| b.is_ascii_hexdigit()))
        .map(str::to_owned)
        .collect();
    let commits = hashes
        .into_iter()
        .filter_map(|hash| read(app_dir, hash))
        .collect();
    Some((commits, from.is_some()))
}

/// One commit: its message, and when a trailer names a tool, what it changed.
fn read(app_dir: &Path, hash: String) -> Option<Commit> {
    let hash_for_show = hash.clone();
    let show = |args: &[&str]| {
        git(app_dir)
            .args(LOG_OVERRIDES)
            .args([
                "show",
                "--no-show-signature",
                "--no-ext-diff",
                "--no-textconv",
                "--no-color",
            ])
            .args(args)
            .arg(hash_for_show.as_str())
            .arg("--")
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
    };
    let message = show(&["-s", "--format=%B"])?;
    let subject = message
        .split("\n\n")
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let mut commit = Commit {
        tool: trailer_tool(&message),
        hash,
        subject,
        ..Default::default()
    };
    if commit.tool.is_some() {
        let diff = show(&[
            "--format=",
            "--raw",
            "--patch",
            "--unified=0",
            "--no-renames",
        ])?;
        parse(&diff, &mut commit);
    }
    Some(commit)
}

/// The files and lines of `git show --raw --patch` text, into `commit`. An added line belongs to the file its `+++ b/`
/// header names, a removed one to its `--- a/` header's, so a deleted file's lines are kept under its own name.
/// Every name is read as git holds it, not as it writes it (`unquoted`, `header_name`).
fn parse(diff: &str, commit: &mut Commit) {
    let (mut new, mut old): (Option<String>, Option<String>) = (None, None);
    // Between a `diff` line and its first `@@`, where `---` and `+++` are headers rather than lines of the file.
    let mut header = false;
    for line in diff.lines() {
        if let Some(raw) = line.strip_prefix(':') {
            // ":100644 100644 <old> <new> M\tpath"
            if let Some((fields, path)) = raw.split_once('\t') {
                let status = fields
                    .split_whitespace()
                    .last()
                    .and_then(|s| s.chars().next());
                if let Some(status) = status {
                    commit.files.push((status, unquoted(path)));
                }
            }
        } else if line.starts_with("diff ") {
            (new, old, header) = (None, None, true);
        } else if header {
            if let Some(path) = line.strip_prefix("+++ ") {
                new = header_name(path, "b/");
            } else if let Some(path) = line.strip_prefix("--- ") {
                old = header_name(path, "a/");
            } else if line.starts_with("@@") {
                header = false;
            }
        } else if line.starts_with("@@") || commit.added.len() + commit.removed.len() >= LINES_KEPT
        {
            continue;
        } else if let (Some(f), Some(text)) = (&new, line.strip_prefix('+')) {
            commit.added.push((f.clone(), cut(text)));
        } else if let (Some(f), Some(text)) = (&old, line.strip_prefix('-')) {
            commit.removed.push((f.clone(), cut(text)));
        }
    }
}

/// A file's name as git holds it, from how `git show` writes it (backlog 0713). Git writes a name that holds a letter
/// outside ASCII, a quote, a backslash, or a control character in quotes, each such byte as a backslash and a letter
/// (`\t`, `\"`, `\\`) or three octal digits (`\303\251`); any other name it writes as it is. A name in quotes that
/// does not read as git's quoting is left as written, never guessed at.
fn unquoted(written: &str) -> String {
    let Some(inner) = written
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .filter(|_| written.len() >= 2)
    else {
        return written.to_owned();
    };
    let mut bytes = Vec::with_capacity(inner.len());
    let mut rest = inner.bytes();
    while let Some(b) = rest.next() {
        if b == b'"' {
            // A quote inside the quotes is always written with a backslash, so this is not git's.
            return written.to_owned();
        }
        if b != b'\\' {
            bytes.push(b);
            continue;
        }
        let byte = match rest.next() {
            Some(b'a') => 0x07,
            Some(b'b') => 0x08,
            Some(b't') => b'\t',
            Some(b'n') => b'\n',
            Some(b'v') => 0x0b,
            Some(b'f') => 0x0c,
            Some(b'r') => b'\r',
            Some(b'"') => b'"',
            Some(b'\\') => b'\\',
            Some(first @ b'0'..=b'3') => {
                let (Some(second @ b'0'..=b'7'), Some(third @ b'0'..=b'7')) =
                    (rest.next(), rest.next())
                else {
                    return written.to_owned();
                };
                (first - b'0') * 64 + (second - b'0') * 8 + (third - b'0')
            }
            _ => return written.to_owned(),
        };
        bytes.push(byte);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// The file a patch's `--- ` or `+++ ` header names, without the `a/` or `b/` git puts before it: `None` for
/// `/dev/null`, which stands for no file. Git ends a name it did not put in quotes with a tab when the name holds a
/// space, so the tab is not part of the name.
fn header_name(written: &str, side: &str) -> Option<String> {
    let name = if written.starts_with('"') {
        unquoted(written)
    } else {
        written.strip_suffix('\t').unwrap_or(written).to_owned()
    };
    name.strip_prefix(side).map(str::to_owned)
}

fn cut(text: &str) -> String {
    text.chars().take(LINE_CHARS).collect()
}

#[cfg(test)]
#[path = "git_changes_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "git_changes_names_tests.rs"]
mod names_tests;
