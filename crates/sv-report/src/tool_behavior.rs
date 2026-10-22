//! What the AI coding tool did that is worth a look (backlog 0630, ADR-131): a section of its own, read from git, the
//! hook record when the person consented to it for this run, and the last report this folder held. It decides nothing:
//! no requirement's status changes because of it, it counts toward no total, it is never a finding, and the verdict
//! reads the same without it.
//!
//! Each line says what was seen, when, and where (a commit, or the time of a line in the hook record), and never why: a
//! force-push or a removed test can be right. What it could not look for, it says.

use serde::Serialize;

/// The section's heading, by which `pages::information` places it.
pub const HEADING: &str = "What the AI coding tool did that is worth a look";

/// What the section says at its top.
pub const WHAT_IT_IS: &str = "Information only: nothing here changes a requirement's status, counts toward anything, or \
     is a finding. Each line is something the AI coding tool did that a person may want to look at, with when and where \
     it was seen. Many of them are right to do; a line says what happened, not that it was wrong.";

/// What was read, and what was seen.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ToolBehavior {
    /// The commit the app was at when this report was made, kept for the next report to compare from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_commit: Option<String>,
    /// The commits read, as a phrase ("the 3 commits since the last report").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_read: Option<String>,
    /// Commits in that range made by an AI coding tool, by a trailer naming one, and those with no such trailer.
    pub tool_commits: usize,
    pub other_commits: usize,
    /// What was seen, oldest first within each kind.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seen: Vec<Seen>,
    /// Lines seen past the most listed, counted and not listed.
    #[serde(skip_serializing_if = "is_zero")]
    pub more: usize,
    /// What could not be looked for, each the end of a sentence.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_seen: Vec<String>,
}

/// One thing seen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Seen {
    pub kind: Kind,
    /// What was seen, as a sentence.
    pub what: String,
    /// Where: a commit (its first eight characters and subject), or "the hook record, <time>".
    #[serde(rename = "where")]
    pub place: String,
}

/// The kinds, in the order the section lists them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// A finding of the last report that is not in this one.
    FindingGone,
    /// `stackvet.toml` changed by the tool.
    AnswersChanged,
    /// A test skipped, removed, or its expected value changed with the code it tests.
    TestsTouched,
    /// A check's settings or a CI workflow changed, or a marker added that tells a check to look away.
    GuardEdited,
    /// A command that destroys work or steps around a guard.
    Command,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::FindingGone => "A finding went away",
            Kind::AnswersChanged => "Answers changed",
            Kind::TestsTouched => "Tests skipped or removed",
            Kind::GuardEdited => "A guard edited",
            Kind::Command => "A command worth a look",
        }
    }
}

/// The section's sentences, in plain text: what it is, what was read, then what could not be looked for. The lines
/// seen are a table, which each format draws itself.
pub fn sentences(t: &ToolBehavior) -> Vec<String> {
    let mut out = vec![WHAT_IT_IS.to_owned()];
    if let Some(read) = &t.commits_read {
        let tools = match t.tool_commits {
            1 => "1 was made by an AI coding tool".to_owned(),
            n => format!("{n} were made by an AI coding tool"),
        };
        out.push(format!(
            "Read: {read}, of which {tools}, by a trailer in its message that names one; the {} that name none are not \
             read as the tool's.",
            t.other_commits
        ));
    }
    if t.seen.is_empty() {
        out.push("Nothing was seen that is worth a look.".to_owned());
    }
    if t.seen.iter().any(|s| s.kind == Kind::FindingGone) {
        out.push(
            "A finding goes away for good reasons too: the code was fixed, or this run did not run the check that \
             found it (the options a report was written with are in its run record). The line says where the file \
             changed, so a person can look at the change itself."
                .to_owned(),
        );
    }
    if t.more > 0 {
        out.push(format!(
            "{} more {} seen and not listed here.",
            t.more,
            if t.more == 1 {
                "line was"
            } else {
                "lines were"
            }
        ));
    }
    for gap in &t.not_seen {
        out.push(format!("Not looked for: {gap}."));
    }
    out
}

fn is_zero(n: &usize) -> bool {
    *n == 0
}
