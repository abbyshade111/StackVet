//! What the hook record showed, for a report written with the person's consent (`sv report --hook-record`, backlog
//! 0566, ADR-122): a section of its own, apart from every count, that credits nothing.
//!
//! The record is kept by a hook the person installed in their AI coding tool, which appends a line for each command the
//! tool ran and each file it wrote. It is the one view of the build from outside the tool's own account of it, and the
//! tool runs as the person and could change it: the lines are chained, and the report keeps where the chain ended
//! (`tip`), so a change made after one report shows in the next one, and a change made before it does not. The report
//! says so, and never quotes more of the record than the last test command, cut short and redacted when written.

use serde::Serialize;

/// What the hook record held, as a report keeps it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct HookSeen {
    /// Why nothing was read, when nothing was: no record for this app on this computer, or one that could not be read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_read: Option<String>,
    /// Commands the tool ran.
    pub commands: usize,
    /// Files the tool wrote.
    pub writes: usize,
    /// Turns the tool ended.
    pub turns: usize,
    /// The first and last line's time, by `sv`'s clock when the hook ran.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    /// Commands among them that run an app's tests, and the last of them: when, and the command, cut short and with any
    /// key cut out when the hook wrote it.
    pub tests: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_test: Option<(String, String)>,
    /// Whether that last test command failed: Claude Code hands a command that exits with an error to the hook as a
    /// failure.
    pub last_test_failed: bool,
    /// Files written after the last test command (or every one, when none ran).
    pub written_after_last_test: usize,
    /// Turns the tool ended saying the app's tests pass (backlog 0565), and those where the record shows otherwise. Only
    /// the kind of claim is kept, never the tool's words.
    pub claims: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mismatches: Vec<Said>,
    /// Turns the tool ended saying it had dealt with one of `CLAIM_KINDS` (backlog 0607), by kind. Only the kind is
    /// kept, never the tool's words.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub said: Vec<SaidKind>,
    /// Lines whose chain checks out, and the line where it first does not, counting from 1.
    pub chained: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broken_at: Option<usize>,
    /// Where the chain ended: its line count and last chain, kept under the report's seal for the next report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<Tip>,
    /// What became of the last sealed report's tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earlier: Option<Earlier>,
}

/// A turn the tool ended saying the app's tests pass, where its own session's record shows otherwise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Said {
    pub at: String,
    pub what: Mismatch,
}

/// What the record showed against a claim that the app's tests pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mismatch {
    /// No test command ran in that session before it.
    NoTestRan,
    /// The last test command before it, at `at`, failed.
    LastTestFailed { at: String },
    /// The last test command before it, at `at`, finished without an error, and `files` files were written after it.
    WrittenSince { at: String, files: usize },
}

/// How many turns the tool ended saying it had dealt with one kind of thing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SaidKind {
    pub kind: String,
    pub turns: usize,
}

/// A kind of claim about security the hook record keeps (backlog 0607): its name in the record, what it is about in
/// plain words, and the requirements it speaks to, each read from the requirement's own text. Beside each, the report
/// shows what it found for those requirements. It credits nothing, and the tool's words are never kept.
pub struct ClaimKind {
    pub kind: &'static str,
    pub topic: &'static str,
    pub requirements: &'static [&'static str],
}

pub const CLAIM_KINDS: &[ClaimKind] = &[
    ClaimKind {
        kind: "rate-limiting",
        topic: "limiting how often something can be tried (rate limiting, lockout)",
        requirements: &["V2.4.1", "V6.3.1"],
    },
    ClaimKind {
        kind: "cross-site-scripting",
        topic: "cross-site scripting (escaping what the page shows)",
        requirements: &["V1.2.1", "V3.2.2"],
    },
    ClaimKind {
        kind: "forgery-token",
        topic: "forged requests from other sites (an anti-forgery token)",
        requirements: &["V3.5.1"],
    },
    ClaimKind {
        kind: "password-storage",
        topic: "how passwords are stored (hashing them)",
        requirements: &["V11.4.2"],
    },
    ClaimKind {
        kind: "sql-injection",
        topic: "SQL injection (queries built safely)",
        requirements: &["V1.2.4"],
    },
    ClaimKind {
        kind: "command-injection",
        topic: "command injection (shell commands built safely)",
        requirements: &["V1.2.5"],
    },
    ClaimKind {
        kind: "access-control",
        topic: "who may do what (access control)",
        requirements: &["V8.2.1"],
    },
    ClaimKind {
        kind: "secrets",
        topic: "keeping secrets and keys out of the code",
        requirements: &["V13.3.1"],
    },
];

/// Where a record's chain ended.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct Tip {
    pub lines: usize,
    pub chain: String,
}

/// The last sealed report's tip, read against the record now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Earlier {
    /// The record still passes through it: nothing before it was changed or cut since.
    Kept { lines: usize },
    /// It does not: lines up to it were changed or removed since that report.
    NotKept { lines: usize },
}

/// What the section says first, whatever the record held.
const WHAT_IT_IS: &str = "A hook you installed in your AI coding tool recorded each command the tool ran and each file \
     it wrote, on this computer, and you asked for it to be read for this report. It credits nothing and counts toward \
     nothing. The tool runs as you and could have changed the record: its lines are chained and this report keeps \
     where the chain ended, so a change made after this report shows in the next one, and a change made before it does \
     not.";

/// One sentence of the section: `sv`'s words, with the place of the one piece of the record's own text it shows, the
/// last test command, marked by `COMMAND`, for each renderer to show as code in its own way.
pub const COMMAND: &str = "\u{0}command\u{0}";

/// The section's sentences, in order, plain text with `sv`'s own code spans, and the command `COMMAND` stands for.
pub fn sentences<'a>(
    seen: &'a HookSeen,
    requirements: &[crate::RequirementLine],
) -> (Vec<String>, Option<&'a str>) {
    let lines = said(seen, requirements);
    (lines, seen.last_test.as_ref().map(|(_, c)| c.as_str()))
}

fn said(seen: &HookSeen, requirements: &[crate::RequirementLine]) -> Vec<String> {
    let mut out = vec![WHAT_IT_IS.to_owned()];
    if let Some(why) = &seen.not_read {
        out.push(format!(
            "Nothing was read: {why}. `sv connect claude --hook` prints the hook to install."
        ));
        return out;
    }
    let span = match (&seen.first, &seen.last) {
        (Some(first), Some(last)) => format!(" from {first} to {last}"),
        _ => String::new(),
    };
    out.push(format!(
        "It saw {}, {}, and {}{span}.",
        count(seen.commands, "command", "commands"),
        count(seen.writes, "file write", "file writes"),
        count(seen.turns, "turn ended", "turns ended")
    ));
    out.push(match &seen.last_test {
        Some((at, _)) => format!(
            "{} of the commands ran tests; the last at {at}: {COMMAND}, which {}. {} after it.",
            seen.tests,
            if seen.last_test_failed {
                "failed"
            } else {
                "finished without an error"
            },
            match seen.written_after_last_test {
                0 => "No file was written".to_owned(),
                1 => "1 file was written".to_owned(),
                n => format!("{n} files were written"),
            }
        ),
        None => "None of the commands ran the app's tests.".to_owned(),
    });
    if seen.claims > 0 {
        out.push(format!(
            "The tool ended {} saying the app's tests pass (only that is kept of what it said, never its words); \
             {}.",
            count(seen.claims, "turn", "turns"),
            match seen.mismatches.len() {
                0 => "in each, the last test command in that session had finished without an error, with no file \
                      written since"
                    .to_owned(),
                n if n == seen.claims => "in each, the record shows otherwise".to_owned(),
                n => format!("in {n} of them the record shows otherwise"),
            }
        ));
        for said in &seen.mismatches {
            out.push(format!(
                "At {}, the tool said the tests pass: {}.",
                said.at,
                match &said.what {
                    Mismatch::NoTestRan => "no test command had run in that session".to_owned(),
                    Mismatch::LastTestFailed { at } =>
                        format!("the last test command before it, at {at}, failed"),
                    Mismatch::WrittenSince { at, files } => format!(
                        "the tests last ran at {at}, and {} written after",
                        match files {
                            1 => "1 file was".to_owned(),
                            n => format!("{n} files were"),
                        }
                    ),
                }
            ));
        }
    }
    // The other kinds of claim, each beside what this report shows for the requirements it speaks to (backlog 0607),
    // in `sv`'s words: what the code and the checks show, never a word the tool could rewrite to change it.
    for said in &seen.said {
        let Some(kind) = CLAIM_KINDS.iter().find(|k| k.kind == said.kind) else {
            continue;
        };
        let shows = kind
            .requirements
            .iter()
            .map(|id| match requirements.iter().find(|r| r.id == *id) {
                Some(r) => format!("{id}, {}", r.status.label()),
                None => format!("{id}, not one this app is held to"),
            })
            .collect::<Vec<_>>()
            .join("; ");
        out.push(format!(
            "The tool ended {} saying it had dealt with {} (only that is kept of what it said, never its words). For \
             the requirements that speaks to, this report shows: {shows}. That is what the code and the checks show, \
             whatever the tool said.",
            count(said.turns, "turn", "turns"),
            kind.topic
        ));
    }
    out.push(match seen.broken_at {
        None => format!("The chain holds over all {} lines.", seen.chained),
        Some(at) => format!(
            "The chain {}breaks at line {at}: a line there was changed, added, or removed after it was written, and \
             nothing from there on is vouched for.",
            match seen.chained {
                0 => String::new(),
                1 => "holds over the first line and ".to_owned(),
                n => format!("holds over the first {n} lines and "),
            }
        ),
    });
    if let Some(earlier) = &seen.earlier {
        out.push(match earlier {
            Earlier::Kept { lines } => format!(
                "The last report's record ended at line {lines}, and the record still passes through it."
            ),
            Earlier::NotKept { lines } => format!(
                "The last report's record ended at line {lines}, and the record no longer passes through it: lines up \
                 to there were changed or removed since that report."
            ),
        });
    }
    out
}

fn count(n: usize, one: &str, many: &str) -> String {
    match n {
        1 => format!("1 {one}"),
        n => format!("{n} {many}"),
    }
}

/// The section's heading.
pub const HEADING: &str = "What the hook saw your AI coding tool do";
