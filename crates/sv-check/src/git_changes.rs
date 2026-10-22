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
                    commit.files.push((status, path.to_owned()));
                }
            }
        } else if line.starts_with("diff ") {
            (new, old, header) = (None, None, true);
        } else if header {
            if let Some(path) = line.strip_prefix("+++ ") {
                new = path.strip_prefix("b/").map(str::to_owned);
            } else if let Some(path) = line.strip_prefix("--- ") {
                old = path.strip_prefix("a/").map(str::to_owned);
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

fn cut(text: &str) -> String {
    text.chars().take(LINE_CHARS).collect()
}

#[cfg(test)]
#[path = "git_changes_tests.rs"]
mod tests;
