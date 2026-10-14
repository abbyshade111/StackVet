//! Backlog 0426: each row left *not verified* says what could settle it. A kind of run that did not happen is named
//! with how to run it; one that happened says it settled nothing, rather than inviting the same run again; a row no
//! check reaches is the app's tests' or the person's.

use crate::next_step_tests::empty_report;
use crate::{Reach, RequirementLine, RunStatus, Status, could_settle, could_settle_note};
use std::collections::{BTreeMap, BTreeSet};

fn row(id: &str, status: Status) -> RequirementLine {
    RequirementLine {
        id: id.into(),
        description: String::new(),
        chapter: String::new(),
        level: 1,
        status,
        findings: Vec::new(),
        information: Vec::new(),
        checked_by: Vec::new(),
        tested_by: Vec::new(),
        supported_by: Vec::new(),
        documented_by: Vec::new(),
        attested_by: Vec::new(),
        by_hand: Vec::new(),
        withheld_by: Vec::new(),
        whose_word: None,
    }
}

fn reach() -> Reach {
    let kinds = |k: &[&str]| k.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    Reach {
        requirements: BTreeMap::from([
            ("V3.4.1".to_owned(), kinds(&["running"])),
            ("V6.3.1".to_owned(), kinds(&["signed-in"])),
            ("V1.2.4".to_owned(), kinds(&["static", "tools"])),
        ]),
        tools_languages: BTreeMap::new(),
    }
}

fn started(signed_in: bool) -> RunStatus {
    RunStatus::Started {
        image: "app".into(),
        asked: 3,
        answered: 3,
        signed_in,
        tests: "not-declared".into(),
    }
}

#[test]
fn a_run_that_did_not_happen_is_named_with_how_to_run_it() {
    let mut r = empty_report(Vec::new());
    r.requirements = vec![
        row("V3.4.1", Status::NotVerified),
        row("V1.2.4", Status::NotVerified),
        row("V8.3.1", Status::NotVerified),
        row("V2.1.1", Status::Checked),
    ];
    r.run_status = Some(RunStatus::NotAsked {
        why: "not asked".into(),
    });
    let said = could_settle(&r, &reach(), false, &BTreeSet::new());
    assert!(
        said["V3.4.1"].contains("`sv report --run`"),
        "{}",
        said["V3.4.1"]
    );
    assert!(
        said["V1.2.4"].contains("an outside tool could"),
        "{}",
        said["V1.2.4"]
    );
    assert!(
        said["V8.3.1"].starts_with("only you can"),
        "{}",
        said["V8.3.1"]
    );
    assert!(
        !said.contains_key("V2.1.1"),
        "a checked row was given a phrase"
    );
}

#[test]
fn a_run_that_happened_and_settled_nothing_says_so_rather_than_asking_for_it_again() {
    let mut r = empty_report(Vec::new());
    r.requirements = vec![
        row("V3.4.1", Status::NotVerified),
        row("V6.3.1", Status::NotVerified),
        row("V1.2.4", Status::NotVerified),
    ];
    r.run_status = Some(started(false));
    let said = could_settle(&r, &reach(), true, &BTreeSet::new());
    assert_eq!(
        said["V3.4.1"],
        "the app ran, and no check of it settled this"
    );
    assert!(
        said["V6.3.1"].contains("signed in as a test user"),
        "{}",
        said["V6.3.1"]
    );
    assert_eq!(
        said["V1.2.4"],
        "the outside tools ran, and none settled this"
    );
    for phrase in said.values() {
        assert!(
            !phrase.contains("could settle it (`sv report --run`)"),
            "{phrase}"
        );
    }
}

#[test]
fn a_row_a_test_could_settle_says_so_and_the_note_follows_the_status() {
    let mut r = empty_report(Vec::new());
    r.requirements = vec![row("V8.3.1", Status::NotVerified)];
    r.tests_to_write = vec![crate::TestToWrite {
        id: "V8.3.1".into(),
        level: 1,
        description: String::new(),
    }];
    r.could_settle = could_settle(&r, &reach(), true, &BTreeSet::new());
    assert!(
        r.could_settle["V8.3.1"].contains("a test of the app's could"),
        "{}",
        r.could_settle["V8.3.1"]
    );
    let note = could_settle_note(&r, &r.requirements[0]);
    assert!(
        note.starts_with(" \u{2014} a test of the app's could"),
        "{note}"
    );
    // Shown in both renderings of the row.
    assert!(crate::html::page(&r).contains("a test of the app&#39;s could"));
    assert!(crate::markdown::compliance(&r).contains("a test of the app's could"));
}
