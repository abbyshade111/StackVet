//! What only a person can check, and how to check it.
//!
//! On a real app, 98 of the applicable requirements can be settled by nobody but the owner. They sat
//! in the report as *not verified*, indistinguishable from the ones nothing had got around to, with
//! no hint of what doing something about them would even involve.
//!
//! The instructions come from three places, because two of them already existed:
//!
//! - **The security notes** (`security-notes.json`) — 19 requirements that ask for a written
//!   decision. The question is the instruction: write the thing down.
//! - **The design questions** (`design-questions.json`) — 18 that ask how the app is built. The
//!   question is the instruction: answer it, and say where in the code.
//! - **`human-checks.json`** — the 20 ASVS requirements left over, each with a line saying what to
//!   go and look at and what would count as the wrong answer.
//!
//! One requirement never appears twice: `crates/sv-check/tests/human_checks.rs` refuses an entry in
//! `human-checks.json` that another catalog already asks about, because the two would give different
//! advice about the same thing and the reader has no way to tell which is meant.
//!
//! # It credits nothing
//!
//! Every requirement here is still *not verified*, and stays that way until the owner does something
//! that produces evidence — writes the notes section, answers the design question, or writes a test.
//! The instruction sits beside the requirement; it is not a substitute for having followed it. This
//! is the one thing the section could get wrong, and `nothing_here_credits_a_requirement` holds it.

use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct HumanCheck {
    pub id: String,
    pub title: String,
    /// What to go and do, in the words of somebody who is not a programmer.
    pub how: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HumanChecks {
    pub checks: Vec<HumanCheck>,
}

impl HumanChecks {
    pub fn load(path: &Path) -> Result<HumanChecks> {
        let checks: HumanChecks = sv_frameworks::data::read_json(path)?;
        for c in &checks.checks {
            if c.how.trim().is_empty() {
                anyhow::bail!("{}: the check for {} says nothing", path.display(), c.id);
            }
        }
        Ok(checks)
    }
}

/// One line of the checklist: a requirement, and what doing something about it involves.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub how: String,
    /// The question in plain words, shown first, with `how` under it as the standard's words (backlog
    /// 0427). Absent for a `human-checks.json` entry, whose `how` is already written for a person who
    /// is not a programmer.
    pub plain: Option<String>,
    /// Where to go and look, when the question alone does not say. Absent for a `human-checks.json`
    /// entry, whose `how` is already the instruction.
    pub where_to_look: Option<String>,
    /// Where the instruction came from, so the reader knows what kind of answer is wanted.
    pub route: Route,
    /// For a design question, what its `where` should name. Absent for the other two routes.
    pub where_means: Option<String>,
    /// Lines of the app's code the question is about, found by a locator (backlog 0487). Where to
    /// look, never evidence: it changes nothing about the requirement's state. Absent when no locator
    /// speaks to the question, or one does and found nothing.
    pub rests_on: Option<RestsOn>,
    /// The packages of the app's the question is about, found by name in the bill of materials (backlog 0490). Where
    /// to look, never evidence, as `rests_on` is. Absent when no package locator speaks to the question, or one does
    /// and none of its packages is in the list.
    pub in_the_packages: Option<InThePackages>,
    /// Why nothing `sv` does can check the requirement, in plain words, for a question whose catalog says (backlog
    /// 0490). Absent for the rest.
    pub why_not_checked: Option<String>,
    /// The app's routes and what is written on each, for the authorization questions (backlog 0493). Where to look,
    /// never evidence, like `rests_on`; never in `report.json`, which holds none of the app's code.
    #[serde(skip)]
    pub routes: Option<crate::route_guards::RouteTable>,
    /// The ways into the app the code shows, for the sign-in pathway questions (backlog 0491). Where to look, never
    /// evidence, as `routes` is; never in `report.json`.
    #[serde(skip)]
    pub pathways: Option<crate::sign_in_paths::Pathways>,
    /// The fields the app's schema shows whose names look like personal data, for the data-classification questions
    /// (backlog 0497). Where to look, never evidence; never in `report.json`.
    #[serde(skip)]
    pub personal_fields: Option<crate::personal_fields::PersonalFields>,
    /// A first draft of the app's cryptographic inventory, for the key-management questions (backlog 0494). Where to
    /// look, never evidence; never in `report.json`.
    #[serde(skip)]
    pub crypto: Option<crate::crypto_inventory::CryptoInventory>,
    /// The app's own token as a signed-in run saw it, for the audience question (backlog 0500, V9.2.4): where it travels
    /// and the services its `aud` field names. Where to look, never evidence; never in `report.json`.
    #[serde(skip)]
    pub token_audience: Option<crate::personal_fields::TokenFields>,
    /// For a hand check whose requirement other checks measure parts of, those parts with this report's verdict on each
    /// (backlog 0610), so the owner reads them in one place. They answer the parts, never the question.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub measured_by: Vec<MeasuredBy>,
}

/// One requirement whose checks measure part of a hand check's requirement, with this report's verdict on it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct MeasuredBy {
    pub id: String,
    /// What the part is, in plain words.
    pub what: String,
    /// This report's verdict, as the report says it ("needs attention", "not verified").
    pub status: String,
}

/// What the parts are, and are not, shown above them.
pub const MEASURED_SAY: &str = "Checks that measure parts of this, with what this report found for each. They are \
     here so the parts can be read in one place; they do not answer the question, which asks whether every control is \
     as the document says.";

/// Each hand check whose requirement others' checks measure parts of, with those parts (backlog 0610).
pub const MEASURED_PARTS: &[(&str, &[(&str, &str)])] = &[
    (
        "V14.2.4",
        &[
            ("V14.2.1", "sensitive data kept out of web addresses"),
            ("V14.3.2", "private pages not kept in a browser's cache"),
            (
                "V16.2.5",
                "sensitive data kept out of the logs, or masked in them",
            ),
            (
                "V14.3.1",
                "what the browser kept for a signed-in person, cleared when they sign out",
            ),
        ],
    ),
    // The written defenses against guessed and reused passwords (backlog 0609, from 0491): the signed-in run tries
    // more wrong passwords in a row than stackvet.toml's `failed-sign-ins` allows, and judges whether the app pushes
    // back (`probe.failed-sign-ins-unlimited`, V6.3.1).
    (
        "V6.1.1",
        &[(
            "V6.3.1",
            "wrong passwords in a row, pushed back on at the number stackvet.toml states",
        )],
    ),
    // The availability defenses the running checks try (backlog 0498): creating records without limit, and a file
    // larger than the app can handle.
    (
        "V15.2.2",
        &[
            (
                "V2.4.1",
                "how fast one person can make the app do work, limited",
            ),
            ("V5.2.1", "a file too large for the app to handle, refused"),
        ],
    ),
];

/// What the standard's own words are called where a question shows them under its plain words.
pub const STANDARDS_WORDS: &str = "In the standard's words:";

impl Item {
    /// The question as a person reads it: the plain sentence, then the standard's own words after
    /// [`STANDARDS_WORDS`] (backlog 0427). Just the standard's words where there is no plain sentence.
    pub fn asked(&self) -> String {
        match &self.plain {
            Some(plain) => format!("{plain} {STANDARDS_WORDS} {}", self.how),
            None => self.how.clone(),
        }
    }
}

/// What a question shows of the app's code, and the sentence that says what it is.
pub const POINTERS_SAY: &str = "These are where to look, not evidence: StackVet found them by their names or their shape (a 403 sent, a query by owner) and did not read what they do.";

/// The most places one question shows; the rest are counted.
pub const MOST_POINTERS: usize = 3;

/// The places in the app's code a question is about.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RestsOn {
    /// What the lines are, in plain words: the locator's own `finds`.
    pub finds: String,
    pub places: Vec<Pointer>,
    /// Places found and not shown.
    pub more: usize,
}

/// The sentence beside the packages a question shows (backlog 0490).
pub const PACKAGES_SAY: &str = "This is where to look, not evidence: StackVet found it by its name in the list of the app's packages and did not read how it is set up.";

/// What the bill of materials shows of a question: the packages a package locator names that the app has.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct InThePackages {
    /// What the packages are, in plain words: the package locator's own `finds`.
    pub finds: String,
    /// Each package as a person reads it: `gunicorn 22.0.0`, saying when the version is only the one asked for.
    pub packages: Vec<String>,
    /// What else the report already says about these packages: the package locator's own `says`.
    pub says: String,
}

/// One place in the app's code.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Pointer {
    pub file: String,
    pub line: usize,
    /// The line, with the one before and the one after, redacted (`Secret::redact` through
    /// `secrets::redact_text_in`) and cut to a length a person reads. Never in `report.json`, which
    /// holds no code of the app's (`Report::code_lines` says the same of a finding's lines).
    #[serde(skip)]
    pub lines: Vec<(usize, String)>,
}

/// What kind of thing would settle this requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Route {
    /// Write it down in security-notes.md.
    WriteItDown,
    /// Answer it in the [design] section of stackvet.toml.
    AnswerInTheManifest,
    /// Go and look at the running app, or at how it is deployed.
    GoAndLook,
}

impl Route {
    pub fn what_to_do(self) -> &'static str {
        match self {
            Route::WriteItDown => {
                "tell your AI coding tool your answer, and it writes it in security-notes.md (at a \
                 terminal, `sv notes` makes the file)"
            }
            Route::AnswerInTheManifest => {
                "tell your AI coding tool your answer, and it records it in the `[design]` section of \
                 stackvet.toml"
            }
            Route::GoAndLook => "check it by hand; nothing here can",
        }
    }
}

/// The checklist for one app, in the order the work should be done.
///
/// `applicable` decides membership, so a requirement this app is not being assessed against never
/// appears — the instruction would be busywork. Requirements that already have evidence are the
/// caller's to exclude.
pub fn checklist(
    notes: &crate::notes::Catalog,
    design: &crate::design::Questions,
    human: &HumanChecks,
    wanted: &BTreeSet<String>,
) -> Vec<Item> {
    let mut out: Vec<Item> = Vec::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for section in &notes.sections {
        if wanted.contains(&section.id) && seen.insert(&section.id) {
            out.push(Item {
                id: section.id.clone(),
                title: section.title.clone(),
                how: section.asks.clone(),
                plain: section.plain.clone(),
                where_to_look: section.how_to_find_out.clone(),
                route: Route::WriteItDown,
                where_means: None,
                rests_on: None,
                in_the_packages: None,
                why_not_checked: None,
                routes: None,
                pathways: None,
                personal_fields: None,
                crypto: None,
                token_audience: None,
                measured_by: Vec::new(),
            });
        }
    }
    for question in &design.questions {
        if wanted.contains(&question.id) && seen.insert(&question.id) {
            out.push(Item {
                id: question.id.clone(),
                title: question.title.clone(),
                how: question.asks.clone(),
                plain: question.plain.clone(),
                where_to_look: question.how_to_find_out.clone(),
                route: Route::AnswerInTheManifest,
                where_means: Some(question.where_means.clone()),
                rests_on: None,
                in_the_packages: None,
                why_not_checked: question.why_not_checked.clone(),
                routes: None,
                pathways: None,
                personal_fields: None,
                crypto: None,
                token_audience: None,
                measured_by: Vec::new(),
            });
        }
    }
    for check in &human.checks {
        if wanted.contains(&check.id) && seen.insert(&check.id) {
            out.push(Item {
                id: check.id.clone(),
                title: check.title.clone(),
                how: check.how.clone(),
                plain: None,
                where_to_look: None,
                route: Route::GoAndLook,
                where_means: None,
                rests_on: None,
                in_the_packages: None,
                why_not_checked: None,
                routes: None,
                pathways: None,
                personal_fields: None,
                crypto: None,
                token_audience: None,
                measured_by: Vec::new(),
            });
        }
    }
    out
}

/// The requirements in `wanted` that no catalog says anything about.
///
/// Counted rather than listed by the report: on a real app these are the Secure by Design and AISVS
/// controls, which are design review from standards that are themselves checklists, and 58 more rows
/// of "read the standard" is the wall this work exists to remove.
pub fn without_instructions(
    notes: &crate::notes::Catalog,
    design: &crate::design::Questions,
    human: &HumanChecks,
    wanted: &BTreeSet<String>,
) -> Vec<String> {
    let known: BTreeSet<&str> = notes
        .sections
        .iter()
        .map(|s| s.id.as_str())
        .chain(design.questions.iter().map(|q| q.id.as_str()))
        .chain(human.checks.iter().map(|c| c.id.as_str()))
        .collect();
    wanted
        .iter()
        .filter(|id| !known.contains(id.as_str()))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn notes() -> crate::notes::Catalog {
        crate::notes::Catalog {
            file: "security-notes.md".into(),
            sections: vec![crate::notes::Section {
                id: "V6.1.1".into(),
                title: "How sign-in is protected".into(),
                asks: "How the app defends against guessing.".into(),
                plain: None,
                facts: Vec::new(),
                how_to_find_out: None,
                heading: None,
                not_covered: None,
            }],
            elsewhere: Vec::new(),
        }
    }

    fn design() -> crate::design::Questions {
        crate::design::Questions {
            questions: vec![crate::design::Question {
                id: "V8.3.1".into(),
                title: "Authorization on the server".into(),
                asks: "Is authorization enforced on the server?".into(),
                plain: None,
                where_means: "the file where it happens".into(),
                how_to_find_out: None,
                why_not_checked: None,
            }],
        }
    }

    fn human() -> HumanChecks {
        HumanChecks {
            checks: vec![HumanCheck {
                id: "V12.2.2".into(),
                title: "The certificate is one browsers trust".into(),
                how: "Click the padlock.".into(),
            }],
        }
    }

    fn want(ids: &[&str]) -> BTreeSet<String> {
        ids.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn every_source_contributes_and_each_says_what_kind_of_answer_it_wants() {
        let list = checklist(
            &notes(),
            &design(),
            &human(),
            &want(&["V6.1.1", "V8.3.1", "V12.2.2"]),
        );
        let routes: Vec<Route> = list.iter().map(|i| i.route).collect();
        assert_eq!(
            routes,
            [
                Route::WriteItDown,
                Route::AnswerInTheManifest,
                Route::GoAndLook
            ]
        );
        for item in &list {
            assert!(!item.how.trim().is_empty(), "{} says nothing", item.id);
        }
    }

    #[test]
    fn a_where_to_look_line_reaches_the_row() {
        // The data guard in tests/human_checks.rs says the catalogs carry a `howToFindOut`. It says
        // nothing about whether that text ever reaches the reader, and dropping it on the way into
        // the row was caught by nothing: a guard on the input is not a guard on the output.
        let mut n = notes();
        n.sections[0].how_to_find_out = Some("Look in your sign-in code.".to_owned());
        let list = checklist(&n, &design(), &human(), &want(&["V6.1.1"]));
        assert_eq!(
            list[0].where_to_look.as_deref(),
            Some("Look in your sign-in code."),
            "the line is in the catalog and not in the row"
        );

        let mut d = design();
        d.questions[0].how_to_find_out =
            Some("Find the code that runs on every request.".to_owned());
        let list = checklist(&notes(), &d, &human(), &want(&["V8.3.1"]));
        assert_eq!(
            list[0].where_to_look.as_deref(),
            Some("Find the code that runs on every request.")
        );
    }

    #[test]
    fn a_requirement_that_does_not_apply_is_not_on_the_list() {
        // The instruction would be busywork, and a checklist that asks for work nobody needs is one
        // people stop reading.
        let list = checklist(&notes(), &design(), &human(), &want(&["V6.1.1"]));
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "V6.1.1");
    }

    #[test]
    fn a_requirement_in_two_catalogs_appears_once() {
        // They would give different advice about the same thing, and the reader cannot tell which
        // is meant. The data guard refuses this; this is the belt to its braces.
        let mut h = human();
        h.checks.push(HumanCheck {
            id: "V6.1.1".into(),
            title: "Also this".into(),
            how: "Do something else.".into(),
        });
        let list = checklist(&notes(), &design(), &h, &want(&["V6.1.1"]));
        assert_eq!(list.len(), 1, "{list:?}");
        assert_eq!(
            list[0].route,
            Route::WriteItDown,
            "the notes come first, so their wording wins"
        );
    }

    #[test]
    fn what_no_catalog_covers_is_reported_rather_than_dropped() {
        // On a real app this is the 58 Secure by Design and AISVS controls. Silently omitting them
        // would make the checklist look complete when it covers 40 of 98.
        let rest = without_instructions(
            &notes(),
            &design(),
            &human(),
            &want(&["V6.1.1", "SBD-AC-01", "C1.1.1"]),
        );
        assert_eq!(rest, ["C1.1.1".to_owned(), "SBD-AC-01".to_owned()]);
    }

    #[test]
    fn the_list_is_empty_when_there_is_nothing_to_do() {
        assert!(checklist(&notes(), &design(), &human(), &want(&[])).is_empty());
        assert!(without_instructions(&notes(), &design(), &human(), &want(&[])).is_empty());
    }
}
