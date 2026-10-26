//! A report on somebody else's project, fetched from its address, for a reader weighing the risk of using it (backlog
//! 0670, part 2; ADR-154).
//!
//! Everything else in a report is written to the person who built the app: fix this, answer that, write a test. The
//! reader of this one did not write the project and cannot do any of it. They want to know what was found, what was
//! not looked at, and what rests on answers only the author could give, so they can decide. So the report names the
//! project, its address, and the commit at its top; its short version counts risks to weigh rather than things to
//! fix; and it never reads "nothing found", or the author's own answers, as the project being safe.

use crate::Report;
use serde::{Deserialize, Serialize};

/// Whose answers the project's `stackvet.toml` holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Answers {
    /// The project carried none, and `sv` wrote one with nothing answered.
    Nobody,
    /// The project carried its own: its author's word, not the reader's.
    Author,
    /// The `stackvet.toml` has been changed since the fetch: whoever changed it answered, not the author.
    Changed,
}

/// Which project a report is of, when it was fetched from its address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchedProject {
    /// The address, as `sv` writes it back: `https://github.com/OWNER/PROJECT`, with `/tree/BRANCH` when one was named.
    pub address: String,
    /// The commit checked, from the archive; `None` when the archive did not say.
    pub commit: Option<String>,
    pub answers: Answers,
}

/// The lines at the top of every page of a report on a fetched project: whose project it is, which commit, how to
/// read what follows, and whose answers there are. `None` for an app of the person's own.
pub fn banner(report: &Report) -> Option<Vec<String>> {
    let f = report.fetched.as_ref()?;
    let commit = match &f.commit {
        Some(c) => format!("at commit {c}"),
        None => "at a commit its archive did not name".to_owned(),
    };
    let mut lines = vec![
        format!(
            "This is somebody else's project: {} {commit}, fetched from its address and checked without running any \
             of it. It may have changed since.",
            f.address
        ),
        "It is checked to help you decide whether to use it. Where this report says to fix something, read it as a \
         risk to weigh: fixing it is the author's to do. Nothing found is not the same as safe."
            .to_owned(),
    ];
    lines.push(match f.answers {
        Answers::Nobody => "The project carried no stackvet.toml, so nobody has answered its questions: you cannot \
                            answer for somebody else's project, and what rests on them is left open, not passed."
            .to_owned(),
        Answers::Author => "The project carries its own stackvet.toml. Its answers are its author's word about the \
                            project, not yours, and sv does not check them."
            .to_owned(),
        Answers::Changed => "Its stackvet.toml has been changed since it was fetched, so its answers are whoever \
                             changed it, not the project's author."
            .to_owned(),
    });
    Some(lines)
}

/// The banner as a Markdown page opens with it, each line in bold; nothing for an app of the person's own.
pub fn markdown(report: &Report) -> String {
    banner(report)
        .unwrap_or_default()
        .iter()
        .map(|line| format!("**{}**\n\n", crate::markdown::inert(line)))
        .collect()
}

/// The short version's first sentence for a fetched project: the risks found, then what was not looked at and what
/// rests on the author, never a word that reads as safe.
pub fn headline(report: &Report) -> String {
    let c = &report.counts;
    let notes = report.findings.iter().filter(|f| crate::is_note(f)).count();
    let found = report.findings.len() - notes;
    let listed_apart: Vec<&sv_check::Finding> = report
        .findings
        .iter()
        .filter(|f| !crate::is_note(f) && f.apart())
        .collect();
    let apart = listed_apart.len();
    let own = found - apart;
    let named = crate::apart_named(&listed_apart);
    let were = |n: usize| {
        if n == 1 {
            "risk to weigh was"
        } else {
            "risks to weigh were"
        }
    };
    let risks = match (own, apart) {
        (0, 0) => "Nothing here found a problem in this project. That does not make it safe to use"
            .to_owned(),
        (0, m) => format!(
            "{m} {} found, {named}, none in the project's own code",
            were(m)
        ),
        (n, 0) => format!("{n} {} found in the project's own code", were(n)),
        (n, m) => format!(
            "{n} {} found in the project's own code, and {m} more {named}",
            were(n)
        ),
    };
    let person = crate::bluf::only_a_person_can(report);
    format!(
        "{risks}: {} of the {} requirements that apply had nothing look at them, {person} of those rest on what only \
         its author could say, and {} more could not be placed without the author's answers.",
        c.not_verified, c.applicable, c.not_assessed
    )
}

/// What the reader can do with the report, in order: none of it is the owner's work of fixing, answering, or testing.
pub fn next_steps(report: &Report) -> Vec<crate::bluf::NextStep> {
    use crate::bluf::NextStep;
    let mut steps = Vec::new();
    let c = &report.counts;
    let found = report
        .findings
        .iter()
        .filter(|f| !crate::is_note(f))
        .count();
    if found > 0 {
        steps.push(NextStep {
            what: format!(
                "Weigh the {found} risk{} found: how serious each is, and whether it matters for how you would use the \
                 project.",
                if found == 1 { "" } else { "s" }
            ),
            where_to_look: "each is in security.md, with why it matters".to_owned(),
        });
    }
    let person = crate::bluf::only_a_person_can(report);
    if person + c.not_assessed > 0 {
        steps.push(NextStep {
            what: format!(
                "Know what is left open: {person} requirement{} rest{} on what only the author could say, and {} \
                 could not be placed without the author's answers. None of them passed.",
                if person == 1 { "" } else { "s" },
                if person == 1 { "s" } else { "" },
                c.not_assessed
            ),
            where_to_look: "your-decisions.md and \"Requirements nobody has placed\" list them; the project's own \
                            documentation, or its author, may answer some"
                .to_owned(),
        });
    }
    if report
        .run_status
        .as_ref()
        .is_none_or(|s| !matches!(s, crate::RunStatus::Started { .. }))
    {
        steps.push(NextStep {
            what: "Know that the project was never started: nothing that needs it running was checked.".to_owned(),
            where_to_look: "not-examined.md says what that leaves".to_owned(),
        });
    }
    steps
}

#[cfg(test)]
mod tests;
