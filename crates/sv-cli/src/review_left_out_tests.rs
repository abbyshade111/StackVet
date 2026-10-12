//! A folder left out of the checks, confirmed by its owner in `sv review` (ADR-099).

use super::tests::{HEAD, Scratch, with_app};
use super::*;

const WHY: &str = "Recorded answers from the payment provider, kept to replay in tests.";

fn entry(why: &str) -> String {
    format!("\n[[left-out]]\nfolder = \"fixtures\"\nwhy = \"{why}\"\nby = \"ai-tool\"\n")
}

fn manifest(s: &Scratch) -> sv_manifest::Manifest {
    sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap()
}

#[test]
fn the_owner_confirms_a_folder_and_then_it_is_left_out() {
    let s = Scratch::new("left-out");
    with_app(&s, &format!("{HEAD}{}", entry(WHY)));
    // Before: it waits, as the report reads it.
    let before = crate::left_out::LeftOut::of(&manifest(&s), &s.checker());
    assert!(before.entries[0].confirmed.is_err(), "{before:?}");

    // Someone who is not the owner is refused, then the owner confirms.
    let (result, out) = s.run("ai-tool\nowner\n");
    result.unwrap();
    assert!(
        out.contains("leaves the folder `fixtures` out of the checks"),
        "{out}"
    );
    assert!(out.contains(WHY), "the reason is shown: {out}");
    assert!(
        out.contains("Only the app's owner can leave part of it out"),
        "{out}"
    );
    assert!(out.contains("Confirmed: the folder is left out."), "{out}");

    let m = manifest(&s);
    assert_eq!(m.left_out[0].folder, "fixtures");
    assert_eq!(m.left_out[0].why, WHY);
    assert_eq!(m.left_out[0].by.as_deref(), Some("owner"));
    assert!(counts(&m, &Waiting::LeftOut(0), &s.checker()));
    let after = crate::left_out::LeftOut::of(&m, &s.checker());
    let by = after.entries[0].confirmed.as_ref().expect("it counts");
    assert!(by.starts_with("the owner on "), "{by}");

    // A second run has nothing of it to ask.
    let (result, out) = s.run("");
    result.unwrap();
    assert!(!out.contains("leaves the folder"), "{out}");
}

#[test]
fn a_reason_too_short_to_tell_a_reader_why_is_not_offered() {
    let s = Scratch::new("left-out-short");
    with_app(&s, &format!("{HEAD}{}", entry("noise")));
    let (result, out) = s.run("owner\n");
    result.unwrap();
    assert!(
        out.contains("The reason is shorter than 40 characters"),
        "{out}"
    );
    // Not asked, so the `owner` typed goes to the next question (the level's answers), never to
    // this entry: it is left as the AI coding tool wrote it.
    assert!(!out.contains("Confirmed: the folder is left out."), "{out}");
    let m = manifest(&s);
    assert_eq!(m.left_out[0].by.as_deref(), Some("ai-tool"));
    assert!(m.left_out[0].seal.is_none());
    assert!(!counts(&manifest(&s), &Waiting::LeftOut(0), &s.checker()));
}

#[test]
fn enter_leaves_it_unconfirmed_and_a_seal_written_by_hand_does_not_count() {
    let s = Scratch::new("left-out-enter");
    with_app(&s, &format!("{HEAD}{}", entry(WHY)));
    let (result, out) = s.run("\n");
    result.unwrap();
    assert!(out.contains("Left unconfirmed."), "{out}");
    assert!(!counts(&manifest(&s), &Waiting::LeftOut(0), &s.checker()));

    // The AI coding tool writes the owner's name and a seal of its own making.
    let forged = format!(
        "{HEAD}\n[[left-out]]\nfolder = \"fixtures\"\nwhy = \"{WHY}\"\nby = \"owner\"\n\
         on = \"2026-10-12\"\nseal = \"v1:0000:{}\"\n",
        "ab".repeat(32)
    );
    std::fs::write(s.app().join("stackvet.toml"), forged).unwrap();
    let left = crate::left_out::LeftOut::of(&manifest(&s), &s.checker());
    let why = left.entries[0]
        .confirmed
        .as_ref()
        .expect_err("a forged seal does not count");
    assert!(why.contains("does not count here"), "{why}");
}
