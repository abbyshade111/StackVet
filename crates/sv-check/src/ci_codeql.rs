//! A CodeQL results file from the owner's own CI, read when the owner gives it (ADR-113; backlog
//! 0274).
//!
//! `sv` runs CodeQL itself for JavaScript, Python, and Ruby (ADR-078). A file the owner's CI wrote
//! reaches the same rules from a run `sv` did not make, so it is believed only for the code it was
//! made from: each run in it names the commit it read (`versionControlProvenance`), and a run is
//! read only when that is the commit checked out in the app's folder, with no change to a tracked
//! file since. Any other run is said in the report and read for nothing: not its findings, which
//! may be of code since changed, and not its silence. `sv` opens no connection for this; the file
//! is one the owner downloaded.

use crate::Verified;
use crate::adapters::{Adapter, Adapters, clean_run_evidence, loaded_rules, looked_away};
use crate::finding::Finding;
use std::path::Path;

/// What one file gave.
#[derive(Debug, Default)]
pub struct CiResults {
    pub findings: Vec<Finding>,
    pub verified: Vec<Verified>,
    /// For each run not read, or read and not counted as clean, what kind of reason and why, in words
    /// the owner can act on.
    pub not_used: Vec<(Unused, String)>,
}

/// What kind of reason a run was not used, for the report's gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unused {
    /// The file could not be read as SARIF.
    Unreadable,
    /// A run `sv` has no map for: another tool, or CodeQL for a language not mapped.
    NoMap,
    /// A run of other code than the app's: another commit, no commit named, or changes since.
    OtherCode,
    /// Read, and it did not look at all of the app, so finding nothing is not a clean result.
    LookedAway,
}

/// The app's commit, as git says it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppCommit {
    /// Checked out, with no change to a tracked file.
    Clean(String),
    /// Checked out, with changes to tracked files not committed.
    Changed(String),
    /// Not a git checkout, or git could not say.
    Unknown,
}

impl AppCommit {
    /// The app in `app_dir`, as git says it is.
    pub fn of(app_dir: &Path) -> Self {
        match (
            crate::git::head_commit(app_dir),
            crate::git::has_uncommitted_changes(app_dir),
        ) {
            (Some(commit), Some(false)) => Self::Clean(commit),
            (Some(commit), Some(true)) => Self::Changed(commit),
            _ => Self::Unknown,
        }
    }
}

/// Reads `text`, the file at `named` (as the owner gave it), against the app.
pub fn read(
    adapters: &Adapters,
    text: &str,
    named: &str,
    app: &AppCommit,
    app_dir: &Path,
    app_languages: &[String],
) -> CiResults {
    let mut out = CiResults::default();
    let document: serde_json::Value =
        match serde_json::from_str(text) {
            Ok(document) => document,
            Err(e) => {
                out.not_used.push((Unused::Unreadable, format!(
                "{named} could not be read as a SARIF file ({e}), so nothing in it was used."
            )));
                return out;
            }
        };
    let Some(runs) = document["runs"].as_array() else {
        out.not_used.push((
            Unused::Unreadable,
            format!("{named} has no `runs`, so it is not a SARIF file and nothing in it was used."),
        ));
        return out;
    };
    for run in runs {
        let tool = run["tool"]["driver"]["name"].as_str().unwrap_or("");
        if !tool.eq_ignore_ascii_case("CodeQL") {
            out.not_used.push((Unused::NoMap, format!(
                "{named} holds a run of {}, not of CodeQL; only CodeQL's results are read from a \
                 file of your CI's.",
                if tool.is_empty() {
                    "an unnamed tool"
                } else {
                    tool
                }
            )));
            continue;
        }
        // One run at a time, as a document of its own, so a file with a run for each language is
        // read against each language's map.
        let one = serde_json::json!({ "version": document["version"], "runs": [run] }).to_string();
        let loaded = loaded_rules(&one);
        let Some(adapter) = adapter_for(adapters, &loaded) else {
            out.not_used.push((Unused::NoMap, format!(
                "{named} holds a CodeQL run whose rules (such as {}) are for a language `sv` has \
                 no map of CodeQL's rules for, so it credits nothing and its results are not shown.",
                loaded.iter().next().map_or("none named", String::as_str)
            )));
            continue;
        };
        let commit = run["versionControlProvenance"]
            .as_array()
            .into_iter()
            .flatten()
            .find_map(|p| p["revisionId"].as_str())
            .map(str::to_lowercase);
        let short = |c: &str| c.chars().take(12).collect::<String>();
        let why_not = match (&commit, app) {
            (None, _) => Some(
                "names no commit (`versionControlProvenance`), so which code it read cannot be told"
                    .to_owned(),
            ),
            (Some(_), AppCommit::Unknown) => Some(
                "is of a commit, and the app's folder is not a git checkout (or git could not say \
                 which commit it is), so the two cannot be matched"
                    .to_owned(),
            ),
            (Some(theirs), AppCommit::Clean(ours) | AppCommit::Changed(ours)) if theirs != ours => {
                Some(format!(
                    "is of commit {}, and the app is at commit {}",
                    short(theirs),
                    short(ours)
                ))
            }
            (Some(_), AppCommit::Changed(ours)) => Some(format!(
                "is of commit {}, the one checked out, but files git tracks have changed since, so \
                 the code is not exactly that commit",
                short(ours)
            )),
            (Some(_), AppCommit::Clean(_)) => None,
        };
        if let Some(why) = why_not {
            let results = run["results"].as_array().map_or(0, Vec::len);
            out.not_used.push((Unused::OtherCode, format!(
                "{named}'s {} run {why}. Nothing in it was used: neither its {results} result{} nor \
                 its finding nothing. Run CodeQL in CI on the commit you check, and give that file.",
                adapter.name,
                if results == 1 { "" } else { "s" }
            )));
            continue;
        }
        let Some(commit) = commit else { continue };
        let findings =
            crate::adapters::parse_sarif_relative_to(adapter, &one, app_dir).unwrap_or_default();
        let away = looked_away(adapter, &one, app_dir);
        let from = format!(
            "{}, from your own CI (the results file {named}, of commit {})",
            adapter.name,
            short(&commit)
        );
        if findings.is_empty() && away.is_empty() {
            let ids = clean_run_evidence(adapter, &loaded, app_languages, &[]);
            let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
            if !ids.is_empty() {
                out.verified.push(Verified::new(
                    &format!("adapter.{}", adapter.id),
                    &ids,
                    format!("{from}, over the {} in this app", adapter.subject()),
                ));
            }
        } else if findings.is_empty() {
            out.not_used.push((Unused::LookedAway, format!(
                "{from} found nothing, but it did not look at all of this app, so finding nothing is \
                 not counted as a clean result: {}.",
                away.join("; ")
            )));
        }
        out.findings.extend(findings);
    }
    out
}

/// The CodeQL adapter whose map names a rule the run loaded: CodeQL's rule ids begin with their
/// language (`js/`, `py/`, `rb/`), so at most one does.
fn adapter_for<'a>(
    adapters: &'a Adapters,
    loaded: &std::collections::BTreeSet<String>,
) -> Option<&'a Adapter> {
    adapters
        .all()
        .iter()
        .filter(|a| a.id.starts_with("codeql-"))
        .find(|a| loaded.iter().any(|rule| a.rules.contains_key(rule)))
}

#[cfg(test)]
mod read_tests;
