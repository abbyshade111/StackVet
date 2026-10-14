//! A security notes section the AI coding tool wrote down is made the owner's own in `sv review` with
//! `mine`, and `owner` still only confirms it as the tool's word (backlog 0425; ADR-026, Later, 14
//! October 2026).

use super::tests::{HEAD, Scratch, with_app};

const NOTES: &str = "# Security notes\n\n## V8.1.1 — Who may do what\n\nWritten by: AI coding tool\n\n\
                     Administrators may open every page; everyone else only their own.\n\n\
                     ## V6.1.1 — Sign-in\n\nWritten by: AI coding tool\n\n\
                     Five failed sign-ins in fifteen minutes lock the account for an hour.\n";

fn read_notes(s: &Scratch) -> (String, sv_check::notes::Answers) {
    let text = std::fs::read_to_string(s.app().join("security-notes.md")).unwrap();
    let catalog = sv_check::notes::Catalog::load(&crate::notes_path()).unwrap();
    let answers = sv_check::notes::read_answers(&catalog, &text);
    (text, answers)
}

#[test]
fn mine_makes_the_tools_section_the_owners_own_and_owner_still_only_confirms() {
    let s = Scratch::new("mine");
    with_app(&s, HEAD);
    std::fs::write(s.app().join("security-notes.md"), NOTES).unwrap();
    // The setup: both sections are the tool's word, neither recorded nor confirmed.
    let (_, before) = read_notes(&s);
    for id in ["V8.1.1", "V6.1.1"] {
        assert!(before.recorded(id, &s.checker()).is_err(), "{id}");
        assert!(before.confirmed(id, &s.checker()).is_err(), "{id}");
    }

    // V8.1.1 is the owner's own decision; V6.1.1 is confirmed by the owner as the tool's word.
    let (result, out) = s.run("mine\nowner\n");
    result.unwrap();
    assert!(
        out.contains("Type `mine` if this is your own decision"),
        "{out}"
    );
    assert!(
        out.contains("Recorded as your own answer, marked `Written by: owner`."),
        "{out}"
    );
    assert!(out.contains("Recorded as confirmed by owner."), "{out}");

    let (after, answers) = read_notes(&s);
    // `mine`: the owner's own, and not a confirmation of the tool's word.
    assert!(answers.recorded("V8.1.1", &s.checker()).is_ok(), "{after}");
    assert!(answers.confirmed("V8.1.1", &s.checker()).is_err());
    assert_eq!(
        answers.writer("V8.1.1"),
        Some(sv_check::notes::Writer::Owner)
    );
    // `owner`: the tool's word, confirmed, never the owner's own.
    assert!(answers.confirmed("V6.1.1", &s.checker()).is_ok(), "{after}");
    assert!(answers.recorded("V6.1.1", &s.checker()).is_err());
    assert_eq!(
        answers.writer("V6.1.1"),
        Some(sv_check::notes::Writer::AiTool)
    );
    // Only V8.1.1's `Written by:` line changed, and a seal went under each section.
    let changed: Vec<&str> = after
        .lines()
        .filter(|l| !NOTES.lines().any(|n| n == *l))
        .collect();
    assert_eq!(changed.len(), 3, "{after}");
    assert!(changed.contains(&"Written by: owner"), "{after}");
    assert_eq!(
        after.matches(sv_check::notes::SEALED_BY).count(),
        2,
        "{after}"
    );
    assert_eq!(after.matches("Written by: AI coding tool").count(), 1);

    // The answer is the owner's, so the seal covers it: changed afterwards, it no longer counts.
    let edited = after.replace("everyone else only their own", "everyone");
    std::fs::write(s.app().join("security-notes.md"), &edited).unwrap();
    let (_, answers) = read_notes(&s);
    assert!(answers.recorded("V8.1.1", &s.checker()).is_err());
}
