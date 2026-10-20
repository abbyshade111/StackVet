//! The answers given as the owner's, recorded together (backlog 0618): `owner` records every one, numbers leave some
//! out and they stay unrecorded, Enter goes one at a time, and a name is not the owner's word.

use super::super::tests::{HEAD, Scratch, with_app};
use super::*;

/// A design answer, a check by hand, and a notes section, each given as the owner's, and one design answer that is
/// the tool's own, which is never offered as the owner's.
fn three_of_the_owners(name: &str) -> Scratch {
    let s = Scratch::new(name);
    with_app(
        &s,
        &format!(
            "{HEAD}\n[design]\n\"V8.3.1\" = {{ answer = \"yes\", where = \"app.py\", by = \"owner\" }}\n\
             \"V2.2.2\" = {{ answer = \"yes\", by = \"ai-tool\" }}\n\n\
             [checked-by-hand.'V12.2.2']\nresult = \"done\"\non = \"2026-10-01\"\nby = \"owner\"\n\
             how = \"Opened the live site; the padlock shows a trusted certificate.\"\n"
        ),
    );
    std::fs::write(
        s.app().join("security-notes.md"),
        "# Security notes\n\n## V6.1.1 — Sign-in\n\n> How is sign-in protected?\n\nWritten by: owner\n\n\
         Five failed sign-ins in fifteen minutes lock the account for an hour.\n",
    )
    .unwrap();
    s
}

fn recorded(s: &Scratch) -> [bool; 3] {
    let m = sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap();
    let notes = std::fs::read_to_string(s.app().join("security-notes.md")).unwrap();
    let catalog = sv_check::notes::Catalog::load(&crate::notes_path()).unwrap();
    [
        counts(&m, &Waiting::DesignAnswer("V8.3.1".into()), &s.checker()),
        counts(&m, &Waiting::HandAnswer("V12.2.2".into()), &s.checker()),
        sv_check::notes::read_answers(&catalog, &notes)
            .recorded("V6.1.1", &s.checker())
            .is_ok(),
    ]
}

#[test]
fn owner_records_every_answer_given_as_the_owners_at_once() {
    let s = three_of_the_owners("together-all");
    // The setup: nothing counts before.
    assert_eq!(recorded(&s), [false; 3]);
    let (result, out) = s.run("owner\n");
    result.unwrap();
    // All three listed, numbered, under the warning each had alone; the tool's own answer is not among them.
    for line in [
        "  1. ",
        "  2. ",
        "  3. ",
        "requirement V8.3.1",
        "requirement V12.2.2",
        "requirement V6.1.1",
    ] {
        assert!(out.contains(line), "{line:?} missing:\n{out}");
    }
    assert!(!out.contains("V2.2.2"), "{out}");
    assert!(
        out.contains("can write `by = \"owner\"` as well as you can"),
        "{out}"
    );
    assert!(out.contains("Type `owner` to record all 3"), "{out}");
    assert!(out.contains("Recorded 3 as your own answers."), "{out}");
    assert!(out.contains("Recorded 3 of 3"), "{out}");
    // Asked once, not three times.
    assert!(!out.contains("[1 of"), "{out}");
    assert_eq!(recorded(&s), [true; 3]);
    let m = sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap();
    assert!(m.design["V2.2.2"].seal.is_none());
    // The second time, nothing of the owner's waits.
    let (result, out) = s.run("");
    result.unwrap();
    assert!(!out.contains("requirement V8.3.1"), "{out}");
}

#[test]
fn a_number_leaves_that_answer_out_and_it_stays_unrecorded() {
    let s = three_of_the_owners("together-some");
    let (result, out) = s.run("2\nowner\n");
    result.unwrap();
    // The list is shown again without the one left out.
    let again = out.rsplit("To leave some out").nth(1).unwrap_or_default();
    assert!(out.matches("  2. ").count() == 1, "{out}");
    assert!(again.contains("  1. ") && again.contains("  3. "), "{out}");
    assert!(out.contains("Type `owner` to record all 2"), "{out}");
    assert!(out.contains("Recorded 2 of 3"), "{out}");
    assert_eq!(recorded(&s), [true, false, true]);
    // Left out, it waits for the next review, alone, and is asked about as before.
    let (result, out) = s.run("owner\n");
    result.unwrap();
    assert!(out.contains("requirement V12.2.2"), "{out}");
    assert!(out.contains("[1 of 1]"), "{out}");
    assert_eq!(recorded(&s), [true; 3]);
}

#[test]
fn leaving_every_one_out_records_nothing() {
    let s = three_of_the_owners("together-none");
    let (result, out) = s.run("1, 2 3\n");
    result.unwrap();
    assert!(out.contains("Every one is left as it is."), "{out}");
    assert!(out.contains("Recorded 0 of 3"), "{out}");
    assert_eq!(recorded(&s), [false; 3]);
}

#[test]
fn enter_goes_through_them_one_at_a_time_as_before() {
    let s = three_of_the_owners("together-one-by-one");
    let (result, out) = s.run("\nowner\n\nowner\n");
    result.unwrap();
    assert!(
        out.contains("[1 of 3]") && out.contains("[3 of 3]"),
        "{out}"
    );
    assert!(out.contains("Recorded 2 of 3"), "{out}");
    assert_eq!(recorded(&s), [true, false, true]);
}

#[test]
fn a_name_is_not_the_owners_word_and_records_nothing() {
    let s = three_of_the_owners("together-name");
    // A name, a number not in the list, and words: each refused, and the input then ends.
    let (result, out) = s.run("Sam Lee\n7\nyes please\n");
    result.unwrap();
    assert_eq!(
        out.matches("Only the app's owner gives these answers")
            .count(),
        3,
        "{out}"
    );
    assert_eq!(recorded(&s), [false; 3]);
}

#[test]
fn one_answer_alone_is_asked_about_as_before() {
    let s = Scratch::new("together-one");
    with_app(
        &s,
        &format!(
            "{HEAD}\n[design]\n\"V8.3.1\" = {{ answer = \"yes\", where = \"app.py\", by = \"owner\" }}\n"
        ),
    );
    let (result, out) = s.run("owner\n");
    result.unwrap();
    assert!(!out.contains("answers given as yours"), "{out}");
    assert!(out.contains("[1 of 1]"), "{out}");
    assert!(out.contains("Recorded 1 of 1"), "{out}");
}

#[test]
fn numbers_are_read_only_when_every_one_is_still_in_the_list() {
    let left = [0, 1, 4];
    assert_eq!(numbers("2", &left), Some(vec![1]));
    assert_eq!(numbers("1 5", &left), Some(vec![0, 4]));
    assert_eq!(numbers("1, 5", &left), Some(vec![0, 4]));
    assert_eq!(numbers("3", &left), None, "3 was left out already");
    assert_eq!(numbers("0", &left), None, "the list starts at 1");
    assert_eq!(numbers("2 and 5", &left), None);
    assert_eq!(numbers(" , ", &left), None);
    assert_eq!(numbers("owner", &left), None);
}
