//! The comments in the app's code that tell a scanner to look away (backlog 0400; ADR-110, Later, 14 October 2026).
//! An AI coding tool asked to make a finding go away may add `# nosec` to the line rather than fix it: found in the
//! owner's trial in Antigravity on 13 October 2026. Where a scanner `sv` runs can be made to look anyway, it is
//! (`data/adapters.json`, `adapters::suppressed_by`), so the finding stays in the report; this lists the comments
//! themselves, so a person sees that something tried to hide one. The owner's choice, 14 October 2026: each on a line
//! of its own, in a section of their own, counting against no requirement and crediting nothing.
//!
//! Read: Bandit's and gosec's `nosec`, gosec's `gosec:disable`, Semgrep's `nosemgrep`, CodeQL's `lgtm` and
//! `codeql[...]`, each in a comment, and the warnings listed in Brakeman's `config/brakeman.ignore`. A comment is
//! known by what comes before the word on its line (`#`, `//`, `/*`, `<!--`, `--`, or a line that starts with `*`),
//! not by the language's grammar, so the same words inside a string after one of those would be listed too.

use crate::secrets::SecretRules;
use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;
use sv_scan::files::Listing;

/// What the app's code tells scanners to skip.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct LookAway {
    /// One for each comment, or each warning a scanner's own ignore file lists, in file and line order.
    pub comments: Vec<Comment>,
    /// Files not read for them, each with why.
    pub not_read: Vec<String>,
}

/// One place the code tells a scanner to skip something.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Comment {
    /// The file, relative to the app's folder: for an ignore file's entry, the file the warning it hides is in.
    pub file: String,
    pub line: usize,
    /// The scanner that reads it.
    pub scanner: &'static str,
    /// What it says, from the word that tells the scanner on, any key in it cut as the secrets scan shows one.
    pub says: String,
}

/// The files whose comments are read: the languages the scanners `sv` runs read, and the templates and settings
/// Semgrep reads too.
const CODE: &[&str] = &[
    "py", "pyi", "go", "rb", "erb", "rake", "js", "jsx", "mjs", "cjs", "ts", "tsx", "vue",
    "svelte", "java", "kt", "php", "cs", "rs", "swift", "scala", "c", "cc", "cpp", "h", "hpp",
    "sh", "html", "yml", "yaml", "tf",
];

/// Reads every file of the app's own code for comments that tell a scanner to skip a line.
pub fn read(listing: &Listing, rules: &SecretRules) -> LookAway {
    let mut out = LookAway::default();
    for entry in listing.app_files() {
        let relative = entry.relative.as_str();
        let brakeman =
            relative == "config/brakeman.ignore" || relative.ends_with("/config/brakeman.ignore");
        let extension = relative
            .rsplit_once('.')
            .map(|(_, e)| e.to_ascii_lowercase());
        if !brakeman && !extension.as_deref().is_some_and(|e| CODE.contains(&e)) {
            continue;
        }
        let text = match entry.read_text() {
            Ok(text) => text,
            Err(why) => {
                out.not_read
                    .push(format!("`{relative}` ({})", why.explain()));
                continue;
            }
        };
        if brakeman {
            brakeman_ignored(relative, &text, rules, &mut out);
            continue;
        }
        // A copy of another project's library is that library's code, and its comments are its authors' (`bundled`).
        if let Some(library) = crate::bundled::library_in(&text) {
            out.not_read.push(format!(
                "`{relative}` (a copy of {library}, not the app's own code)"
            ));
            continue;
        }
        for (i, line) in text.lines().enumerate() {
            if let Some((scanner, says)) = on_line(line, extension.as_deref().unwrap_or("")) {
                out.comments.push(Comment {
                    file: relative.to_owned(),
                    line: i + 1,
                    scanner,
                    says: crate::ai_tool::quote(rules, says),
                });
            }
        }
    }
    out.comments
        .sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    out
}

/// The scanner a comment on `line` speaks to, and what it says from its word on; `None` when the line has none.
pub(crate) fn on_line<'a>(line: &'a str, extension: &str) -> Option<(&'static str, &'a str)> {
    static WORD: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)\b(?:(?:nosemgrep|gosec:disable|nosec|lgtm)\b|codeql\[)")
            .expect("a fixed pattern")
    });
    let found = WORD
        .find_iter(line)
        .find(|m| in_comment(&line[..m.start()]))?;
    let word = found.as_str().to_ascii_lowercase();
    let scanner = if word == "nosemgrep" {
        "Semgrep"
    } else if word == "gosec:disable" {
        "gosec"
    } else if word == "nosec" {
        match extension {
            "py" | "pyi" => "Bandit",
            "go" => "gosec",
            _ => "Bandit or gosec",
        }
    } else {
        "CodeQL"
    };
    // To where the comment closes, when it closes on the line.
    let rest = &line[found.start()..];
    let end = ["*/", "-->"]
        .iter()
        .filter_map(|close| rest.find(close))
        .min()
        .unwrap_or(rest.len());
    let says = rest[..end].trim_end();
    Some((scanner, says))
}

/// `line` without the comment that tells a scanner to look away, from where that comment opens: what the line read
/// before the comment was added to it (backlog 0401). `None` when the line has no such comment, or nothing but it.
pub(crate) fn without_comment(line: &str, extension: &str) -> Option<String> {
    let (_, says) = on_line(line, extension)?;
    // `says` is a slice of `line`, so where it starts is where the word that tells the scanner is.
    let word_at = says.as_ptr() as usize - line.as_ptr() as usize;
    let before = &line[..word_at];
    let opens = ["<!--", "/*", "//", "#", "--"]
        .iter()
        .filter_map(|o| before.rfind(o))
        .max()?;
    let kept = line[..opens].trim_end();
    (!kept.trim().is_empty()).then(|| kept.to_owned())
}

/// Whether what comes before a word on its line opens a comment.
fn in_comment(before: &str) -> bool {
    let start = before.trim_start();
    before.contains('#')
        || before.contains("//")
        || before.contains("/*")
        || before.contains("<!--")
        || start.starts_with('*')
        || start.starts_with("--")
}

/// Each warning `config/brakeman.ignore` tells Brakeman to skip, where the warning is.
fn brakeman_ignored(relative: &str, text: &str, rules: &SecretRules, out: &mut LookAway) {
    let Some(ignored) = serde_json::from_str::<serde_json::Value>(text)
        .ok()
        .and_then(|v| v["ignored_warnings"].as_array().cloned())
    else {
        out.not_read.push(format!(
            "`{relative}` (not the list of ignored warnings Brakeman writes)"
        ));
        return;
    };
    for warning in ignored {
        let kind = warning["warning_type"].as_str().unwrap_or("a warning");
        let note = warning["note"].as_str().filter(|n| !n.trim().is_empty());
        let says = match note {
            Some(note) => format!("{kind}, listed in {relative}: {note}"),
            None => format!("{kind}, listed in {relative}"),
        };
        out.comments.push(Comment {
            file: warning["file"].as_str().unwrap_or(relative).to_owned(),
            line: warning["line"].as_u64().map_or(0, |l| l as usize),
            scanner: "Brakeman",
            says: crate::ai_tool::quote(rules, &says),
        });
    }
}

#[cfg(test)]
#[path = "look_away_tests.rs"]
mod tests;
