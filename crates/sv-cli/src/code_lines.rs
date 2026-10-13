//! The lines of the app's code each code finding is on, with the line before and the line after, for the report's
//! "The code" under the finding (backlog 0378, part 2). A finding said which file and line; a person had to open the
//! file to see what it meant.
//!
//! Read once, when the report is put together, from the app's own folder only: a path that climbs out of it or starts
//! at the root is not read, and neither is a file too large to be code someone wrote. Each finding's lines are redacted
//! together, as one piece of text, so a key that runs over two lines is cut as the secrets scan would cut it.

use std::collections::BTreeMap;
use std::path::{Component, Path};
use sv_check::secrets::{SecretRules, redact_text_in};

/// The most findings whose lines are kept: a report is read by a person.
pub const MOST_FINDINGS: usize = 300;

/// The most characters of one line kept.
pub const LINE_CHARS: usize = 200;

/// A file larger than this is not read for its lines.
const MOST_BYTES: u64 = 2_000_000;

/// The lines each code finding is on, keyed by its file and line, with the lines around it, redacted.
pub fn read(
    app_dir: &Path,
    findings: &[sv_check::Finding],
    rules: &SecretRules,
) -> BTreeMap<(String, usize), Vec<(usize, String)>> {
    let mut out = BTreeMap::new();
    let mut texts: BTreeMap<String, Option<Vec<String>>> = BTreeMap::new();
    for f in findings {
        if out.len() == MOST_FINDINGS {
            break;
        }
        let (file, line) = (&f.location.file, f.location.line);
        if line == 0 || out.contains_key(&(file.clone(), line)) || !inside(file) {
            continue;
        }
        let lines = texts
            .entry(file.clone())
            .or_insert_with(|| file_lines(app_dir, &app_dir.join(file)));
        let Some(lines) = lines else { continue };
        if line > lines.len() {
            continue;
        }
        let first = line.saturating_sub(1).max(1);
        let last = (line + 1).min(lines.len());
        let window: Vec<&str> = (first..=last).map(|n| lines[n - 1].as_str()).collect();
        let (redacted, _) = redact_text_in(rules, file, &window.join("\n"));
        let kept: Vec<(usize, String)> = (first..)
            .zip(redacted.split('\n'))
            .map(|(n, text)| (n, bounded(text)))
            .collect();
        out.insert((file.clone(), line), kept);
    }
    out
}

/// Whether `file` names a place inside the app's folder: relative, and never climbing out of it.
fn inside(file: &str) -> bool {
    let path = Path::new(file);
    !file.is_empty()
        && path
            .components()
            .all(|c| matches!(c, Component::Normal(_) | Component::CurDir))
}

/// The file's lines, when it is a file of a size a person writes and reads as text, and its real place is inside
/// the app's folder: a link that points out of it is not followed.
fn file_lines(app_dir: &Path, path: &Path) -> Option<Vec<String>> {
    use sv_frameworks::paths::Canonical;
    let root = app_dir.canonical().ok()?;
    if !path.canonical().ok()?.starts_with(&root) {
        return None;
    }
    let meta = std::fs::symlink_metadata(path).ok()?;
    if !meta.is_file() || meta.len() > MOST_BYTES {
        return None;
    }
    let text = std::fs::read_to_string(path).ok()?;
    Some(text.lines().map(str::to_owned).collect())
}

/// A line, cut at `LINE_CHARS` characters with how many more there were said.
fn bounded(text: &str) -> String {
    let n = text.chars().count();
    if n <= LINE_CHARS {
        return text.to_owned();
    }
    let head: String = text.chars().take(LINE_CHARS).collect();
    format!("{head}… ({} more characters)", n - LINE_CHARS)
}

#[cfg(test)]
mod code_lines_tests;
