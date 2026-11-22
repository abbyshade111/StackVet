//! What the records show, gathered under the design's six groups (ADR-159; backlog 0715, part 3), so a report reads
//! as the design's table does: a claim against the record, gaming a check, outside the envelope, tampering with
//! oversight, speaking as the owner, and trying again after a no.
//!
//! Nothing here reads a record: each line comes from what `build_loop`, `hook_record`, `tool_behavior`, and `held`
//! already read, with one reading added, of a tool's commits for an entry written in the owner's name. A group no
//! record can show says so, and why: an empty group is never left looking like a clean one.

use crate::held::{self, Held};
use sv_check::git::changes::Commit;
use sv_report::BuildLoop;
use sv_report::hook::{HookSeen, Mismatch};
use sv_report::tool_behavior::{Kind, ToolBehavior};

/// The six groups, in the design's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    Claim,
    Gaming,
    Outside,
    Oversight,
    AsOwner,
    AgainAfterNo,
}

impl Group {
    pub const ALL: [Group; 6] = [
        Group::Claim,
        Group::Gaming,
        Group::Outside,
        Group::Oversight,
        Group::AsOwner,
        Group::AgainAfterNo,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Group::Claim => "What the tool said, against the record",
            Group::Gaming => "A check made to pass without a fix",
            Group::Outside => "Outside what the task file allows",
            Group::Oversight => "Changes to what watches the tool",
            Group::AsOwner => "Written in your name",
            Group::AgainAfterNo => "Trying again after a no",
        }
    }
}

/// How a claim stands against the record (ADR-159, decision 4). A claim that could not be checked is never a line:
/// it is said under what the group could not show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Verdict {
    /// The record shows otherwise.
    Contradicted,
    /// The record does not show it, and does not show otherwise.
    NotSupported,
}

impl Verdict {
    pub fn label(self) -> &'static str {
        match self {
            Verdict::Contradicted => "contradicted by the record",
            Verdict::NotSupported => "not supported by the record",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Line {
    pub what: String,
    /// Where it was seen: a commit, a time in the hook record, a line of a record.
    pub place: String,
    /// Which record it came from, in a few words.
    pub from: &'static str,
    /// For a claim, how it stands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<Verdict>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Gathered {
    pub group: Group,
    pub lines: Vec<Line>,
    /// What of this group the records read could not show, each a sentence.
    pub not_shown: Vec<String>,
}

const FROM_GIT: &str = "git's history";
const FROM_HOOK: &str = "the hook record";
const FROM_LOOP: &str = "the build-loop record";
const FROM_PROXY: &str = "the pass-through's record";

/// Everything a run of `av report` read, as `gather` takes it.
pub struct Read<'r> {
    pub build_loop: &'r BuildLoop,
    /// The hook record, when the person asked for it.
    pub hook: Option<&'r HookSeen>,
    pub behavior: &'r ToolBehavior,
    /// What was held to the task file, when there is one that could be read.
    pub held: Option<&'r Held>,
    /// The commits read, when git could say.
    pub commits: Option<&'r [Commit]>,
    /// How each record that was read stands against the owner's checkpoints.
    pub checkpoints: &'r [(crate::checkpoint::Kind, crate::checkpoint::Standing)],
    /// How the build-loop record stands against the notes kept outside its folder.
    pub noted: &'r crate::outside_note::Standing,
    /// The pass-through's record, when the person asked for it.
    pub proxy: Option<&'r crate::proxy_seen::Seen>,
}

/// The pass-through's record, when it was asked for and could be read.
fn proxy_read<'r>(read: &Read<'r>) -> Option<&'r crate::proxy_seen::Seen> {
    read.proxy.filter(|p| p.not_read.is_none())
}

fn times(n: usize) -> String {
    match n {
        1 => "once".to_owned(),
        n => format!("{n} times"),
    }
}

/// The six groups, each with what was seen and what could not be.
pub fn gather(read: &Read) -> Vec<Gathered> {
    Group::ALL
        .into_iter()
        .map(|group| {
            let (lines, not_shown) = match group {
                Group::Claim => claims(read),
                Group::Gaming => gaming(read),
                Group::Outside => outside(read),
                Group::Oversight => oversight(read),
                Group::AsOwner => as_owner(read),
                Group::AgainAfterNo => again_after_no(read),
            };
            Gathered {
                group,
                lines,
                not_shown,
            }
        })
        .collect()
}

fn claims(read: &Read) -> (Vec<Line>, Vec<String>) {
    let mut not_shown = vec![
        "Any claim but that the app's tests pass: av reads no transcript, and the hook record keeps only that one \
         kind of claim against what was run."
            .to_owned(),
    ];
    let Some(hook) = read.hook else {
        not_shown.insert(
            0,
            "Whether the tool said the tests pass when they had not: only the hook record shows it, and it is read \
             only when you ask, with --hook-record."
                .to_owned(),
        );
        return (Vec::new(), not_shown);
    };
    let lines = hook
        .mismatches
        .iter()
        .map(|said| {
            let (verdict, shows) = match &said.what {
                Mismatch::NoTestRan => (
                    Verdict::Contradicted,
                    "no test command ran in that session before it".to_owned(),
                ),
                Mismatch::LastTestFailed { at } => (
                    Verdict::Contradicted,
                    format!("the last test command before it, at {at}, failed"),
                ),
                Mismatch::WrittenSince { at, files } => (
                    Verdict::NotSupported,
                    format!(
                        "the last test command before it, at {at}, finished without an error, and {} written \
                         after it",
                        match files {
                            1 => "1 file was".to_owned(),
                            n => format!("{n} files were"),
                        }
                    ),
                ),
            };
            Line {
                what: format!("the tool ended a turn saying the app's tests pass; {shows}"),
                place: said.at.clone(),
                from: FROM_HOOK,
                verdict: Some(verdict),
            }
        })
        .collect();
    (lines, not_shown)
}

fn gaming(read: &Read) -> (Vec<Line>, Vec<String>) {
    let mut lines: Vec<Line> = read
        .build_loop
        .went_same
        .iter()
        .map(|w| Line {
            what: format!(
                "a finding of {} is no longer found, and its line reads as it did",
                w.rule
            ),
            place: format!("{}:{}", w.file, w.line),
            from: FROM_LOOP,
            verdict: None,
        })
        .collect();
    lines.extend(
        read.behavior
            .seen
            .iter()
            .filter(|s| {
                matches!(
                    s.kind,
                    Kind::FindingGone | Kind::TestsTouched | Kind::GuardEdited
                )
            })
            .map(|s| Line {
                what: s.what.clone(),
                place: s.place.clone(),
                from: FROM_GIT,
                verdict: None,
            }),
    );
    let mut not_shown = Vec::new();
    if read.build_loop.findings_moved.is_none() {
        not_shown.push(
            "Which findings went between the tool's checks: the build-loop record needs two checks that kept every \
             finding's fingerprint."
                .to_owned(),
        );
    }
    if read.commits.is_none() {
        not_shown.push(
            "A test deleted or skipped, an expectation changed, a comment telling a check to look away: git could \
             not say what the tool's commits changed."
                .to_owned(),
        );
    }
    not_shown.push(
        "A line rewritten to do the same thing another way: nothing here reads what a change means.".to_owned(),
    );
    (lines, not_shown)
}

fn from_of(witness: held::Witness) -> &'static str {
    match witness {
        held::Witness::Git => FROM_GIT,
        held::Witness::Hook => FROM_HOOK,
    }
}

fn held_lines(read: &Read, group: held::Group) -> Vec<Line> {
    read.held
        .map(|h| {
            h.seen
                .iter()
                .filter(|s| s.group == group)
                .map(|s| Line {
                    what: s.what.clone(),
                    place: s.place.clone(),
                    from: from_of(s.witness),
                    verdict: None,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn outside(read: &Read) -> (Vec<Line>, Vec<String>) {
    match read.held {
        Some(h) => {
            let mut lines = held_lines(read, held::Group::OutsideEnvelope);
            let mut not_shown = h.not_held.clone();
            match (read.proxy, proxy_read(read)) {
                (None, _) => not_shown.push(
                    "The tools the tool called through av proxy: the pass-through's record is read only when you \
                     ask, with --proxy-record."
                        .to_owned(),
                ),
                (Some(p), None) => not_shown.push(format!(
                    "The tools the tool called through av proxy: {}.",
                    p.not_read.as_deref().unwrap_or("the record was not read")
                )),
                (Some(_), Some(p)) => match &p.outside {
                    None => not_shown.push(
                        "The tools the tool called through av proxy: the task file does not say which it may call \
                         ([may] call is left out)."
                            .to_owned(),
                    ),
                    Some(outside) => {
                        for t in outside {
                            lines.push(Line {
                                what: format!(
                                    "{} was called {}, and [may] call does not name it",
                                    t.name,
                                    times(t.calls)
                                ),
                                place: "the calls made through av proxy".to_owned(),
                                from: FROM_PROXY,
                                verdict: None,
                            });
                        }
                        not_shown.push(
                            "A tool called some other way than through av proxy: only a tool server with av proxy \
                             in front of it is seen, and the AI coding tool's own tools (its shell, its file \
                             editor) never are."
                                .to_owned(),
                        );
                    }
                },
            }
            (lines, not_shown)
        }
        None => (
            Vec::new(),
            vec!["Anything: there is no task file (agentvet.toml) that could be read, so there is nothing to be \
                  outside of."
                .to_owned()],
        ),
    }
}

fn oversight(read: &Read) -> (Vec<Line>, Vec<String>) {
    let mut lines = held_lines(read, held::Group::Oversight);
    let b = read.build_loop;
    if let Some(at) = b.chain_broken_at {
        lines.push(Line {
            what: "the record's chain stops checking out: a line was changed, removed, or added after it was written"
                .to_owned(),
            place: format!("line {at} of the record"),
            from: FROM_LOOP,
            verdict: None,
        });
    }
    if b.turned_off > 0 {
        lines.push(Line {
            what: format!(
                "the record was turned off {}, each a gap in it",
                match b.turned_off {
                    1 => "once".to_owned(),
                    n => format!("{n} times"),
                }
            ),
            place: "the record's own lines".to_owned(),
            from: FROM_LOOP,
            verdict: None,
        });
    }
    if let Some(at) = read.hook.and_then(|h| h.broken_at) {
        lines.push(Line {
            what: "the record's chain stops checking out: a line was changed, removed, or added after it was written"
                .to_owned(),
            place: format!("line {at} of the record"),
            from: FROM_HOOK,
            verdict: None,
        });
    }
    if let crate::outside_note::Standing::NotKept { at, lines: noted } = read.noted {
        lines.push(Line {
            what: format!(
                "the build-loop record no longer passes through what sv noted outside the folder at {at}, when it \
                 held {}: lines up to there were changed or cut since",
                match noted {
                    1 => "1 line".to_owned(),
                    n => format!("{n} lines"),
                }
            ),
            place: "the record's own lines, against the note outside the folder".to_owned(),
            from: "the note outside the folder",
            verdict: None,
        });
    }
    if let Some(at) = proxy_read(read).and_then(|p| p.broken_at) {
        lines.push(Line {
            what: "the record's chain stops checking out: a line was changed, removed, or added after it was written"
                .to_owned(),
            place: format!("line {at} of the record"),
            from: FROM_PROXY,
            verdict: None,
        });
    }
    for (kind, standing) in read.checkpoints {
        if let crate::checkpoint::Standing::NotKept { at, lines: signed } = standing {
            lines_changed(&mut lines, *kind, at, *signed);
        }
    }
    let mut not_shown = Vec::new();
    if read.held.is_none() {
        not_shown.push(
            "A guarded file changed: there is no task file (agentvet.toml) that could be read, and the guards are \
             held through it."
                .to_owned(),
        );
    }
    if read.hook.is_none() {
        not_shown.push(
            "Whether the hook record was changed after it was written: it is read only when you ask, with \
             --hook-record."
                .to_owned(),
        );
    }
    if read.proxy.is_none() {
        not_shown.push(
            "Whether the pass-through's record was changed after it was written: it is read only when you ask, \
             with --proxy-record."
                .to_owned(),
        );
    }
    not_shown.push(
        "A record replaced whole since your last checkpoint, or before you ever made one, and a hook taken out so \
         that nothing was written: a chain shows a change to what is there, and only a checkpoint (`av checkpoint`) \
         shows a record that was replaced."
            .to_owned(),
    );
    (lines, not_shown)
}

/// A record that no longer passes through a checkpoint the owner signed.
fn lines_changed(lines: &mut Vec<Line>, kind: crate::checkpoint::Kind, at: &str, signed: usize) {
    lines.push(Line {
        what: format!(
            "{} no longer passes through your checkpoint of {at}: of the {} you signed for, some were changed or \
             cut since",
            kind.said(),
            match signed {
                1 => "1 line".to_owned(),
                n => format!("{n} lines"),
            }
        ),
        place: "the record's own lines, against your checkpoint".to_owned(),
        from: "your checkpoint",
        verdict: None,
    });
}

/// The files an owner's word is recorded in: `stackvet.toml` (and its old name), and the notes files.
fn holds_the_owner_s_word(path: &str) -> Option<bool> {
    let name = path.rsplit(['/', '\\']).next().unwrap_or(path);
    match name {
        "stackvet.toml" | "securevibe.toml" => Some(true),
        "security-notes.md" | "design-decisions.md" => Some(false),
        _ => None,
    }
}

/// What a line added to one of those files says in the owner's name, when it does: an entry given as the owner's, a
/// notes answer marked as the owner's, or a seal, which only `sv review` at the owner's own terminal writes.
fn in_the_owner_s_name(toml: bool, line: &str) -> Option<&'static str> {
    let text: String = line.split_whitespace().collect::<Vec<_>>().join(" ");
    let lower = text.to_ascii_lowercase();
    if toml {
        if lower.starts_with('#') {
            return None;
        }
        let squeezed: String = lower.chars().filter(|c| !c.is_whitespace()).collect();
        // `by` as a whole key: at the start of the line, or after the `{` or `,` of a table written on one line.
        let given_as_owner = ["by=\"owner\"", "by='owner'"].iter().any(|key| {
            squeezed
                .match_indices(key)
                .any(|(at, _)| at == 0 || matches!(squeezed.as_bytes()[at - 1], b'{' | b','))
        });
        if given_as_owner {
            return Some("an entry was given as yours (`by = \"owner\"`)");
        }
        if squeezed.starts_with("seal=") {
            return Some("a seal was written, which only `sv review` at your own terminal makes");
        }
    } else {
        if lower.starts_with("written by: owner") {
            return Some("an answer was marked as yours (`Written by: owner`)");
        }
        if lower.starts_with("sealed by sv review:") {
            return Some("a seal was written, which only `sv review` at your own terminal makes");
        }
    }
    None
}

fn as_owner(read: &Read) -> (Vec<Line>, Vec<String>) {
    let mut not_shown = vec![
        "An entry written in your name and not committed, or committed without naming a tool: only a commit whose \
         message names a tool is read as the tool's."
            .to_owned(),
        "Whether a seal is one `sv review` made: av reads that a tool's commit added one, and does not check it; \
         `sv report` does."
            .to_owned(),
    ];
    let Some(commits) = read.commits else {
        not_shown.insert(
            0,
            "Anything: git could not say what the tool's commits changed.".to_owned(),
        );
        return (Vec::new(), not_shown);
    };
    let mut lines = Vec::new();
    for c in commits.iter().rev().filter(|c| c.tool.is_some()) {
        for (file, line) in &c.added {
            let Some(toml) = holds_the_owner_s_word(file) else {
                continue;
            };
            if let Some(what) = in_the_owner_s_name(toml, line) {
                let subject: String = c.subject.chars().take(60).collect();
                lines.push(Line {
                    what: format!("in {file}, {what}"),
                    place: format!(
                        "commit {} \"{subject}\", by {}",
                        c.hash.chars().take(8).collect::<String>(),
                        c.tool.as_deref().unwrap_or("a tool not named")
                    ),
                    from: FROM_GIT,
                    verdict: None,
                });
            }
        }
    }
    (lines, not_shown)
}

/// A no you gave at the tool's own prompt is never seen: said every time, whatever was read.
pub const NO_AT_THE_PROMPT: &str = "A no you gave at your AI coding tool's own permission prompt: a call you refuse \
                                    there is never sent to the tool server, so no record read here holds a line \
                                    for it, and none can show the tool trying another way after it. (Seen in a \
                                    trial on 21 November 2026; it needs the tool's own hooks, which av does not \
                                    read for this yet.)";

fn again_after_no(read: &Read) -> (Vec<Line>, Vec<String>) {
    let mut lines = Vec::new();
    let mut not_shown = vec![NO_AT_THE_PROMPT.to_owned()];
    match (read.proxy, proxy_read(read)) {
        (None, _) => not_shown.push(
            "A call a tool server turned down and the tool made again: the pass-through's record (av proxy) shows \
             those, and it is read only when you ask, with --proxy-record."
                .to_owned(),
        ),
        (Some(p), None) => not_shown.push(format!(
            "A call a tool server turned down and the tool made again: {}.",
            p.not_read.as_deref().unwrap_or("the record was not read")
        )),
        (Some(_), Some(p)) => {
            for again in &p.again {
                // In the session that was told no, and in a later one, are two things: a later session may know
                // nothing of the no (backlog 0721, part 1).
                let same = match again.times {
                    0 => String::new(),
                    n => format!(
                        " and called again with the same arguments {} in the same session; the last was answered {}",
                        times(n),
                        again.last_answer
                    ),
                };
                let later = match again.later {
                    0 => String::new(),
                    n => format!(
                        "{} the same call was made {} in a later session, which may not have known of the no",
                        if same.is_empty() { " and" } else { ". Also," },
                        times(n)
                    ),
                };
                lines.push(Line {
                    what: format!(
                        "{} was turned down by the server ({}){same}{later}. It was given: {}",
                        again.tool, again.answer, again.summary
                    ),
                    place: format!("first turned down at {}", again.at),
                    from: FROM_PROXY,
                    verdict: None,
                });
            }
            if p.runs.iter().any(|r| r.started.is_none()) {
                not_shown.push(
                    "Which session each of the record's older calls came from: av proxy began marking its starts \
                     on 22 November 2026, and calls written before that are read as one session, so a call made \
                     again there may have been made in another."
                        .to_owned(),
                );
            }
            if p.again_more > 0 {
                not_shown.push(format!("{} more such calls, not listed.", p.again_more));
            }
            not_shown.push(
                "Why a server turned a call down: a server that will not do a thing and a tool that ran and failed \
                 (a test that did not pass, say) can answer the same way, so both are listed, and making a failed \
                 call again is often right. A call turned down and then made with different arguments, or to \
                 another tool, is not matched to its no."
                    .to_owned(),
            );
        }
    }
    if read.build_loop.refused > 0 {
        not_shown.push(format!(
            "For what it is worth, sv itself refused {} for a tool it does not have; that is not a no from you.",
            match read.build_loop.refused {
                1 => "1 call".to_owned(),
                n => format!("{n} calls"),
            }
        ));
    }
    (lines, not_shown)
}

#[cfg(test)]
#[path = "groups_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "groups_proxy_tests.rs"]
mod proxy_tests;
