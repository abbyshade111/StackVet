//! What the record of the build loop shows in a report (ADR-076): the counts, the lines each page says, and the caveat
//! that goes with them. Moved out of `lib.rs` on 21 October 2026 (backlog 0324), unchanged; the types and lines are
//! still named from the crate's root.

use super::*;

/// What the record of the build loop shows, read when the report is written into its folder
/// (ADR-076). Evidence about how the app was built, never about the app: nothing here credits a
/// requirement or changes a count.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BuildLoop {
    /// The record is turned off in `stackvet.toml`, so nothing was written down.
    pub off: bool,
    /// Calls to `sv`'s MCP server written down for this app, of any tool.
    pub calls: usize,
    /// Those that ran a check of the app, and so carry counts.
    pub checks: usize,
    /// When the first and the last call were made, in UTC, as written down.
    pub first: Option<String>,
    pub last: Option<String>,
    /// The counts at the first and the last check.
    pub first_counts: Option<LoopCounts>,
    pub last_counts: Option<LoopCounts>,
    /// Lines of the record that could not be read, said rather than skipped quietly.
    pub unreadable: usize,
    /// The record reached its size limit, so calls after it were not written down and "the last
    /// check" is the last one written. Left out of a report when false, so a report written before
    /// it reads and seals the same.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub full: bool,
    /// Of the calls, those that did not end with an answer (ADR-084): failed, ran out of time,
    /// stopped on a fault in `sv`, or refused. Each left out of a report when 0, so a report written
    /// before it reads and seals the same, as `full` is.
    #[serde(skip_serializing_if = "is_zero")]
    pub failed: usize,
    #[serde(skip_serializing_if = "is_zero")]
    pub timed_out: usize,
    #[serde(skip_serializing_if = "is_zero")]
    pub crashed: usize,
    #[serde(skip_serializing_if = "is_zero")]
    pub refused: usize,
    /// The AI coding tools that called, each as it named itself, and the `sv`s that answered.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clients: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub svs: Vec<String>,
    /// Lines of the record whose hash is chained to the line before, and checks out (backlog 0239). Left out of a
    /// report when 0, so a report written before the chain reads and seals the same.
    #[serde(skip_serializing_if = "is_zero")]
    pub chained: usize,
    /// The line where the chain first stops checking out, counting the record's lines from 1: a line was changed,
    /// removed, or added after it was written (backlog 0239). Left out when the chain holds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_broken_at: Option<usize>,
    /// The last chain that checked out, the record's head, which a report keeps so a later rewrite of the whole record
    /// shows when two reports are compared (backlog 0239).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_head: Option<String>,
    /// How many times the record was turned off while the app was built, each a gap in it.
    #[serde(skip_serializing_if = "is_zero")]
    pub turned_off: usize,
    /// Calls this process could not write into the record, so the counts are short by them.
    #[serde(skip_serializing_if = "is_zero")]
    pub unwritten: usize,
    /// The findings' fingerprints at the first and the last check, when their lines kept them all;
    /// read to make `findings_moved`, never written into the report.
    #[serde(skip)]
    pub first_fingerprints: Option<Vec<String>>,
    #[serde(skip)]
    pub last_fingerprints: Option<Vec<String>>,
    /// The first check's findings placed by rule and file key (backlog 0401); read to make `went_same`, never written
    /// into the report.
    #[serde(skip)]
    pub first_placed: BTreeMap<String, (String, String)>,
    /// Of the findings no longer found between the first check and the last, those whose line still reads as it did
    /// (backlog 0401). Left out when there are none, so a report written before it reads and seals the same.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub went_same: Vec<WentSame>,
    /// How many of the findings that went were looked for again in the app's code, and how many were past the limit
    /// and not. Left out when 0.
    #[serde(skip_serializing_if = "is_zero")]
    pub looked_again: usize,
    #[serde(skip_serializing_if = "is_zero")]
    pub not_looked_again: usize,
    /// The names `sv` defines that the AI tool asked about, each once, in the order first asked
    /// (ADR-084, decision 3), and what `sv` handed over (decision 5).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub asked: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub handed: Vec<String>,
    /// The size in bytes of each of `handed` that has one (backlog 0227, ADR-084). Left out when none has a
    /// size, so a report written before sizes were kept reads and seals the same.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub handed_sizes: BTreeMap<String, u64>,
    /// Between the first check and the last, which findings went and came (ADR-084, decision 6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_moved: Option<FindingsMoved>,
    /// With no call recorded, the signs that `sv` was used with this app all the same, in plain words (backlog 0404):
    /// the record may have been deleted, and the report says so rather than that nothing shows `sv` was used. Left out
    /// when there are none, so a report written before it reads and seals the same.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub signs_of_use: Vec<String>,
}

/// A finding no longer found whose line still reads in the app as it did when it was found (backlog 0401): its rule,
/// where the line is now, and how it reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct WentSame {
    pub rule: String,
    pub file: String,
    pub line: usize,
    #[serde(flatten)]
    pub how: sv_check::review::StillReads,
}

/// Between the first check of the build loop and the last: findings no longer found, those of them
/// a person set aside as false alarms, and findings that were new. Counts only.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct FindingsMoved {
    pub no_longer_found: usize,
    pub set_aside: usize,
    pub new: usize,
}

fn is_zero(n: &usize) -> bool {
    *n == 0
}

/// What the AI tool asked about and was handed, in a sentence or two (ADR-084, decisions 3 and 5);
/// empty when the record holds neither. Only what happened is said: a record from before this
/// was kept cannot show what was not.
fn handed_said(b: &BuildLoop) -> String {
    let mut out = String::new();
    if !b.asked.is_empty() {
        const SHOWN: usize = 12;
        let mut named: Vec<String> = b.asked.iter().take(SHOWN).cloned().collect();
        if b.asked.len() > SHOWN {
            named.push(format!("{} more", b.asked.len() - SHOWN));
        }
        out.push_str(&format!(" It asked sv about {}.", list_and(&named)));
    }
    let with = |prefix: &str| -> Vec<String> {
        b.handed
            .iter()
            .filter_map(|h| h.strip_prefix(prefix).map(str::to_owned))
            .collect()
    };
    let mut given = Vec::new();
    if b.handed.iter().any(|h| h == "instructions") {
        given.push("sv's instructions when it connected".to_owned());
    }
    let prompts = with("prompt:");
    if !prompts.is_empty() {
        given.push(format!(
            "the prompt{} {}",
            if prompts.len() == 1 { "" } else { "s" },
            list_and(&prompts)
        ));
    }
    let reports = with("report:");
    if !reports.is_empty() {
        given.push(format!(
            "the report file{} {} to read",
            if reports.len() == 1 { "" } else { "s" },
            list_and(&reports)
        ));
    }
    if !given.is_empty() {
        out.push_str(&format!(" sv gave it {}.", list_and(&given)));
    }
    out
}

/// What the record of the build loop leaves out, said after what it shows (ADR-084): the times it
/// was turned off, and the calls that could not be written into it.
fn loop_gaps(b: &BuildLoop) -> String {
    let mut out = String::new();
    match b.turned_off {
        0 => {}
        1 => out.push_str(
            " The record was turned off once while the app was built (`build-loop-record = false` \
             in stackvet.toml), so the calls made while it was off are not written down.",
        ),
        n => out.push_str(&format!(
            " The record was turned off {n} times while the app was built (`build-loop-record = \
             false` in stackvet.toml), so the calls made while it was off are not written down."
        )),
    }
    match b.unwritten {
        0 => {}
        1 => out.push_str(
            " One call could not be written into the record, so the counts here are short by one.",
        ),
        n => out.push_str(&format!(
            " {n} calls could not be written into the record, so the counts here are short by them."
        )),
    }
    out
}

/// The counts one check came to, as the record keeps them: no finding's text, only how many.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LoopCounts {
    pub findings: usize,
    pub checked: usize,
    pub needs_attention: usize,
    pub not_assessed: usize,
}

impl LoopCounts {
    /// The counts of a report, as the record keeps them.
    pub fn of(report: &Report) -> Self {
        Self {
            findings: report.findings.len(),
            checked: report.counts.checked,
            needs_attention: report.counts.needs_attention,
            not_assessed: report.counts.not_assessed,
        }
    }

    fn describe(&self) -> String {
        format!(
            "{} finding{}, {} requirement{} checked, {} needing attention, {} not assessed",
            self.findings,
            if self.findings == 1 { "" } else { "s" },
            self.checked,
            if self.checked == 1 { "" } else { "s" },
            self.needs_attention,
            self.not_assessed
        )
    }
}

/// The findings that went between the first check of the build loop and the last while their lines still read as
/// they did, named one by one, for the top of the report (backlog 0401): a fix changes the line, so each went some
/// other way, and the person is told to look. `None` when none did, or none was looked for. The file names are the
/// app's, in backticks; each renderer makes them safe as it makes the rest of the app's text.
pub fn went_same_line(report: &Report) -> Option<String> {
    let b = report.build_loop.as_ref()?;
    if b.looked_again == 0 {
        return None;
    }
    let none_past_limit = if b.not_looked_again > 0 {
        format!(
            " {} more that went were not looked for, past the limit of {}.",
            b.not_looked_again, b.looked_again
        )
    } else {
        String::new()
    };
    if b.went_same.is_empty() {
        return Some(format!(
            "Of the findings no longer found between the first check and the last, {} {} looked for again in the \
             app's code, and none still sits on a line that reads as it did when it was found.{none_past_limit} A \
             line rewritten to do the same thing is not caught by this.",
            b.looked_again,
            if b.looked_again == 1 { "was" } else { "were" },
        ));
    }
    let items: Vec<String> = b
        .went_same
        .iter()
        .map(|w| {
            let how = match &w.how {
                sv_check::review::StillReads::LookAway { scanner } => {
                    format!("a comment there now tells {scanner} to look away")
                }
                sv_check::review::StillReads::Unchanged => {
                    "the line is unchanged, so the check stopped finding it for another reason, which the record \
                     cannot tell"
                        .to_owned()
                }
            };
            format!(
                "{} at {} line {} ({how})",
                w.rule,
                crate::markdown::code(&w.file),
                w.line
            )
        })
        .collect();
    let n = b.went_same.len();
    Some(format!(
        "Look at {}: {n} finding{} no longer found at the last check while the line {} on still reads as it did, \
         so changing the code did not make {} go: {}. Ask whoever changed it what was done, and why. A line rewritten \
         to do the same thing is not caught by this, only one that reads the same.{none_past_limit}",
        if n == 1 { "this" } else { "these" },
        if n == 1 { " was" } else { "s were" },
        if n == 1 { "it was" } else { "each was" },
        if n == 1 { "it" } else { "them" },
        items.join("; "),
    ))
}

/// One paragraph saying what the record of the build loop shows, for the top of the report. `None`
/// for a report that was not written into a report folder.
pub fn build_loop_line(report: &Report) -> Option<String> {
    report.build_loop.as_ref().map(build_loop_said)
}

/// When the record's calls were made, for a person to read: "on 2026-10-22, from 22:03 to 22:56 UTC" when the first
/// and the last fall on one day (backlog 0635), and the record's own times when they do not, or do not read as times.
fn span_said(first: Option<&str>, last: Option<&str>) -> String {
    // `2026-10-22T22:03:54Z`: the date, and the hour and minute.
    let parts = |s: &str| -> Option<(String, String)> {
        let (date, time) = s.split_once('T')?;
        let clock = time.get(..5)?;
        let ok = date.len() == 10 && time.ends_with('Z') && clock.as_bytes().get(2) == Some(&b':');
        ok.then(|| (date.to_owned(), clock.to_owned()))
    };
    match (first, last) {
        (Some(first), Some(last)) if first != last => match (parts(first), parts(last)) {
            (Some((d1, t1)), Some((d2, t2))) if d1 == d2 => {
                format!(", on {d1}, from {t1} to {t2} UTC")
            }
            _ => format!(", from {first} to {last}"),
        },
        (Some(first), _) => match parts(first) {
            Some((d, t)) => format!(", on {d} at {t} UTC"),
            None => format!(", at {first}"),
        },
        _ => String::new(),
    }
}

/// What `build_loop_line` says of one record, in one paragraph.
pub fn build_loop_said(b: &BuildLoop) -> String {
    let (lead, details) = build_loop_parts(b);
    lead + &details
}

/// `build_loop_line` in two parts, for the top of the report (backlog 0635): what a person needs first, and the
/// details a report shows folded away. Together they say what `build_loop_said` says. `None` for a report that was
/// not written into a report folder.
pub fn build_loop_lines(report: &Report) -> Option<(String, String)> {
    report.build_loop.as_ref().map(|b| {
        let (lead, details) = build_loop_parts(b);
        (lead, details.trim_start().to_owned())
    })
}

/// The record's paragraph as what a person needs first (how many checks, what they came to, what moved between
/// them, and anything that changes what those numbers mean: calls that went unanswered or unwritten, a broken chain,
/// a full record) and the details: what it asked and was handed, the names the two sides gave, a chain that holds,
/// and where the record is kept.
fn build_loop_parts(b: &BuildLoop) -> (String, String) {
    let caveat = " This is about how the app was built, not about the app: it credits nothing and \
                  changes no count. sv keeps the record in the app's .stackvet folder, where the AI \
                  coding tool can write too: these numbers are what the record said when this report was \
                  written, sealed with the report, so a change to them afterwards is caught, and a \
                  change to the record before then is not.";
    let unreadable = match b.unreadable {
        0 => String::new(),
        1 => " One line of the record could not be read, and is left out.".to_owned(),
        n => format!(" {n} lines of the record could not be read, and are left out."),
    };
    if b.off {
        return (
            "The record of the build loop is turned off in stackvet.toml (`build-loop-record = \
             false`), so this report cannot say whether sv was used while the app was built."
                .to_owned(),
            String::new(),
        );
    }
    let gaps = loop_gaps(b);
    if b.calls == 0 && !b.signs_of_use.is_empty() {
        let text = format!(
            "No record of the build was found; it may have been deleted. No call from an AI coding \
             tool to sv's MCP server is recorded for this app, but other signs show sv was used with \
             it: {}. The record is kept in the app's .stackvet folder (build-loop.jsonl) and goes if \
             that folder is deleted. That is not a finding, and it does not say how the app was \
             built.{unreadable}{gaps}",
            list_and(&b.signs_of_use)
        );
        return (text, String::new());
    }
    if b.calls == 0 {
        let text = format!(
            "Nothing shows that sv was used while this app was built: no call from an AI coding \
             tool to sv's MCP server is recorded for it. That is not a finding. The app may have \
             been checked at a terminal, or built before sv kept this record.{unreadable}{gaps}"
        );
        return (text, String::new());
    }
    let span = span_said(b.first.as_deref(), b.last.as_deref());
    let mut text = format!(
        "While this app was built, its AI coding tool asked sv {} time{} through sv's MCP server, \
         {} of them a check of the app{span}.",
        b.calls,
        if b.calls == 1 { "" } else { "s" },
        b.checks
    );
    match (&b.first_counts, &b.last_counts) {
        (Some(first), Some(last)) if b.checks > 1 => text.push_str(&format!(
            " The first check came to {}; the last, {}.",
            first.describe(),
            last.describe()
        )),
        (_, Some(last)) => text.push_str(&format!(" The check came to {}.", last.describe())),
        _ => {}
    }
    let unanswered: Vec<String> = [
        (b.failed, "failed"),
        (b.timed_out, "ran out of time"),
        (b.crashed, "stopped on a fault in sv"),
        (
            b.refused,
            if b.refused == 1 {
                "was refused as a tool sv does not have"
            } else {
                "were refused as tools sv does not have"
            },
        ),
    ]
    .iter()
    .filter(|(n, _)| *n > 0)
    .map(|(n, said)| format!("{n} {said}"))
    .collect();
    if !unanswered.is_empty() {
        text.push_str(&format!(" Of those calls, {}.", list_and(&unanswered)));
    }
    if let Some(m) = &b.findings_moved {
        text.push_str(&format!(
            " Between the first check and the last, {} finding{} no longer found (fixed, or no \
             longer reached by the check: the record cannot tell which), {} {} set aside by a person \
             as a false alarm, and {} {} new.",
            m.no_longer_found,
            if m.no_longer_found == 1 { " was" } else { "s were" },
            m.set_aside,
            if m.set_aside == 1 { "was" } else { "were" },
            m.new,
            if m.new == 1 { "was" } else { "were" },
        ));
    }
    let mut details = handed_said(b);
    if !b.clients.is_empty() {
        details.push_str(&format!(
            " The AI coding tool named itself {} when it connected (its own word, not checked).",
            list_and(&b.clients)
        ));
    }
    if !b.svs.is_empty() {
        details.push_str(&format!(" It was answered by sv {}.", list_and(&b.svs)));
    }
    text.push_str(&unreadable);
    text.push_str(&gaps);
    // The chain of hashes over the record's lines (backlog 0239): whether it holds, and where it breaks if not. A
    // chain that holds says nothing of lines cut from the end, and the sentence says so (backlog 0310).
    if let Some(at) = b.chain_broken_at {
        text.push_str(&format!(
            " The record's chain of hashes breaks at its line {at}: a line was changed, removed, or added there \
             after it was written, so that line and the lines after it are not vouched for."
        ));
    } else if let (true, Some(head)) = (b.chained > 0, &b.chain_head) {
        let short: String = head.chars().take(12).collect();
        details.push_str(&format!(
            " Each of the record's {} chained lines checks out against the one before it, ending at the hash {short}. \
             Lines cut from the end, or a rewrite of the whole record, would leave a valid chain all the same, \
             so compare that hash with an earlier report's: if the record has only grown since, its chain still \
             passes through the hash that report showed.",
            b.chained
        ));
    }
    if b.full {
        text.push_str(
            " The record reached its size limit, so later calls were not written down: the last \
             check here is the last one written, not necessarily the last one made.",
        );
    }
    details.push_str(caveat);
    (text, details)
}

#[cfg(test)]
#[path = "build_loop_span_tests.rs"]
mod span_tests;
