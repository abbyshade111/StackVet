//! Backlog 0372: a requirement with a plain-language line shows it beside the standard's own words, never in place
//! of them, and the report says how many rows have one.

use crate::next_step_tests::empty_report;
use crate::{RequirementLine, Status, plain_note};

fn row(id: &str) -> RequirementLine {
    RequirementLine {
        id: id.into(),
        description: "Verify that the formal words are kept.".into(),
        chapter: "V1 Encoding and Sanitization".into(),
        level: 1,
        status: Status::NotVerified,
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

#[test]
fn the_plain_line_stands_beside_the_standard_s_words_and_the_rows_without_one_are_counted() {
    let mut r = empty_report(Vec::new());
    r.requirements = vec![row("V1.2.4"), row("V1.2.5")];
    r.plain.insert(
        "V1.2.4".into(),
        "Database queries keep the data apart.".into(),
    );
    let html = crate::html::page(&r);
    assert!(html.contains("Database queries keep the data apart."));
    assert!(
        html.contains("In its own words: Verify that the formal words are kept."),
        "the standard's words were dropped"
    );
    let md = crate::markdown::compliance(&r);
    assert!(
        md.contains(" | 1 | Database queries keep the data apart. |"),
        "{md}"
    );
    assert!(md.contains("Verify that the formal words are kept."));
    let note = plain_note(&r).expect("no note for a report where some rows have no plain line");
    assert!(
        note.starts_with("1 of these 2 requirements have a line in plain words"),
        "{note}"
    );
    assert!(html.contains(&note) || html.contains("1 of these 2 requirements"));
}
