//! Backlog 0542: a security-notes.md written by hand under headings of its own, before any of the file's questions,
//! as the Cursor trial of 14 October 2026 wrote it, has those headings named, so the report can say why the owner's
//! real decisions count for nothing; a file `sv notes` wrote names none.

use super::*;

fn catalog() -> Catalog {
    Catalog {
        file: "security-notes.md".into(),
        sections: vec![Section {
            id: "V7.3.1".into(),
            title: "How long a session lasts".into(),
            asks: "How long a signed-in session lasts before it ends.".into(),
            plain: None,
            facts: vec![],
            how_to_find_out: None,
            heading: None,
            not_covered: None,
        }],
        ..Default::default()
    }
}

/// The trial's file, in its shape: a title, then the owner's choices under headings of the tool's own.
const BY_HAND: &str = "# Security notes\n\n## Session length\n\nSessions end after 30 minutes idle.\n\n\
## Guessing limit\n\nFive wrong passwords lock the account for 15 minutes.\n\n### Who may do what\n\nOnly admins \
see every booking.\n";

#[test]
fn headings_before_any_question_are_named_and_count_for_nothing() {
    let answers = read_answers(&catalog(), BY_HAND);
    assert_eq!(
        answers.outside,
        ["Session length", "Guessing limit", "Who may do what"],
        "the title line is `sv`'s own and is not one of them"
    );
    assert!(
        answers.answered().is_empty(),
        "nothing under them is an answer"
    );
    let evidence = evidence(
        &catalog(),
        &answers,
        "security-notes.md",
        &crate::seal::Checker::no_key(),
    );
    assert_eq!(evidence.outside, answers.outside);
    assert!(evidence.documented.is_empty() && evidence.stated.is_empty());
}

#[test]
fn a_file_sv_notes_wrote_names_no_heading_outside() {
    // The control: the template's own headings are questions or `sv`'s, never one of the person's.
    let applicable: BTreeSet<String> = ["V7.3.1".to_owned()].into();
    let template =
        write_template(&catalog(), &applicable, &Facts::default(), None, &|_| None).unwrap();
    assert!(
        template.contains("V7.3.1"),
        "the template asks the question: {template}"
    );
    let answers = read_answers(&catalog(), &template);
    assert!(answers.outside.is_empty(), "{:?}", answers.outside);
}

#[test]
fn a_heading_after_a_question_is_the_other_list_not_this_one() {
    let text = "# Security notes\n\n## V7.3.1 — How long a session lasts\n\nWritten by: owner\n\nThirty minutes idle, \
eight hours at most.\n\n## My own notes\n\nTo do: ask about backups.\n";
    let answers = read_answers(&catalog(), text);
    assert!(answers.outside.is_empty(), "{:?}", answers.outside);
    assert_eq!(answers.not_read.len(), 1, "{:?}", answers.not_read);
}
