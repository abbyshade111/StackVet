//! What a report says it could not make of the owner's word (backlog 0324): the design decisions file, the design
//! questions, and the checks done by hand. No test held these three gaps before `the_owners_word` handed them to
//! functions of their own; each test here fails with its function's body taken out.

use super::*;

#[test]
fn a_decisions_section_whose_writer_is_not_said_is_a_gap_naming_it() {
    let catalog = sv_check::notes::Catalog::load(&crate::decisions_path())
        .expect("the decisions catalog loads");
    let decisions = sv_check::notes::Evidence {
        unreadable: vec!["Threat model".to_owned()],
        ..Default::default()
    };
    let mut gaps = Vec::new();
    decisions_gaps(&decisions, &catalog, &mut gaps);
    assert_eq!(gaps.len(), 1, "{gaps:?}");
    assert_eq!(
        gaps[0].what,
        format!("who wrote 1 section of {}", catalog.file)
    );
    assert!(gaps[0].why.contains("Threat model"), "{}", gaps[0].why);
    assert!(matches!(gaps[0].reason, sv_report::GapReason::CouldNotRead));
    // Nothing unreadable, nothing said.
    let mut none = Vec::new();
    decisions_gaps(&sv_check::notes::Evidence::default(), &catalog, &mut none);
    assert!(none.is_empty());
}

#[test]
fn a_design_answer_that_cannot_be_read_and_questions_left_unanswered_are_each_a_gap() {
    let design = sv_check::design::Outcome {
        unreadable: vec![
            "server-side-validation".to_owned(),
            "service-auth".to_owned(),
        ],
        unanswered: vec!["secrets-store".to_owned()],
        ..Default::default()
    };
    let mut gaps = Vec::new();
    design_gaps(&design, &mut gaps);
    assert_eq!(gaps.len(), 2, "{gaps:?}");
    assert_eq!(gaps[0].what, "your answer to 2 design questions");
    assert!(
        gaps[0].why.contains("server-side-validation, service-auth"),
        "{}",
        gaps[0].why
    );
    assert!(matches!(gaps[0].reason, sv_report::GapReason::CouldNotRead));
    assert_eq!(gaps[1].what, "1 question about how this app is built");
    assert!(gaps[1].why.contains("secrets-store"), "{}", gaps[1].why);
    assert!(matches!(gaps[1].reason, sv_report::GapReason::PersonOnly));
    let mut none = Vec::new();
    design_gaps(&sv_check::design::Outcome::default(), &mut none);
    assert!(none.is_empty());
}

#[test]
fn a_hand_check_that_cannot_be_read_or_is_out_of_date_is_each_a_gap() {
    let hand = sv_check::hand::Outcome {
        unreadable: vec![("V12.1.1".to_owned(), "a date that is not one".to_owned())],
        out_of_date: vec![("V12.2.1".to_owned(), "2025-01-02".to_owned())],
        ..Default::default()
    };
    let mut gaps = Vec::new();
    hand_gaps(&hand, &mut gaps);
    assert_eq!(gaps.len(), 2, "{gaps:?}");
    assert!(
        gaps[0].what.starts_with("1 check made by hand"),
        "{}",
        gaps[0].what
    );
    assert!(gaps[0].why.contains("V12.1.1"), "{}", gaps[0].why);
    assert_eq!(
        gaps[1].what,
        format!(
            "1 check made by hand more than {} days ago",
            sv_check::hand::CURRENT_FOR_DAYS
        )
    );
    assert!(
        gaps[1].why.contains("V12.2.1, checked on 2025-01-02"),
        "{}",
        gaps[1].why
    );
    assert!(matches!(gaps[1].reason, sv_report::GapReason::PersonOnly));
    let mut none = Vec::new();
    hand_gaps(&sv_check::hand::Outcome::default(), &mut none);
    assert!(none.is_empty());
}
