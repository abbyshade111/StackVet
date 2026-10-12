//! The account changes an app does not offer, confirmed by its owner in `sv review` (ADR-105), and
//! what the report then makes of V7.5.1.

use super::tests::{HEAD, Scratch, with_app};
use super::*;

const USERS: &str = "\n[stack.run]\nstart = \"python app.py\"\n[stack.run.users]\n\
                     not-offered = [\"phone\", \"recovery\"]\n";

fn manifest(s: &Scratch) -> sv_manifest::Manifest {
    sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap()
}

fn credit(rule: &str) -> sv_check::Verified {
    sv_check::Verified::new(rule, &["V7.5.1"], format!("{rule} refused")).in_part()
}

/// The email change and the two-factor change credited in part, as a run would leave them, with
/// phone and recovery listed as not offered.
fn after_a_run() -> Vec<sv_check::Verified> {
    vec![
        credit("probe.email-change-without-password"),
        credit("probe.two-factor-change-without-password"),
    ]
}

#[test]
fn the_owner_confirms_what_is_not_offered_and_v7_5_1_is_then_met_whole() {
    let s = Scratch::new("not-offered");
    with_app(&s, &format!("{HEAD}{USERS}"));
    // Before the owner's word: the list counts for nothing, and is said.
    let mut verified = after_a_run();
    let mut gaps = Vec::new();
    crate::assemble::account_changes_whole(&manifest(&s), &s.checker(), &mut verified, &mut gaps);
    assert!(verified.iter().all(|v| v.in_part), "{verified:?}");
    assert_eq!(gaps.len(), 1, "{gaps:?}");
    assert_eq!(gaps[0].reason, sv_report::GapReason::PersonOnly);
    assert_eq!(gaps[0].requirements, ["V7.5.1"]);
    assert!(
        gaps[0].why.contains("`phone`, `recovery`"),
        "{}",
        gaps[0].why
    );

    // The level's answers come first and are left as they are (Enter); then someone who is not the
    // owner is refused, and the owner confirms.
    let (result, out) = s.run("\nai-tool\nowner\n");
    result.unwrap();
    assert!(
        out.contains("nobody can change these in your app, because it has none: a phone number; a way to recover the account"),
        "{out}"
    );
    assert!(
        out.contains("Only the app's owner knows what it lets people change"),
        "{out}"
    );
    assert!(out.contains("Confirmed as your answer."), "{out}");
    let m = manifest(&s);
    let entry = m.account_changes_review.as_ref().expect("written");
    assert_eq!(entry.not_offered, ["phone", "recovery"]);
    assert_eq!(entry.by, "owner");
    assert!(entry.seal.is_some());
    assert!(counts(&m, &Waiting::NotOffered, &s.checker()));

    // Now whole, saying which part rests on the owner's word, and nothing is left to say.
    let mut verified = after_a_run();
    let mut gaps = Vec::new();
    crate::assemble::account_changes_whole(&m, &s.checker(), &mut verified, &mut gaps);
    assert!(verified.iter().all(|v| !v.in_part), "{verified:?}");
    assert!(
        verified[0].scope.ends_with(
            "; with phone and recovery not offered, on the owner's word through `sv review`"
        ),
        "{}",
        verified[0].scope
    );
    assert!(gaps.is_empty(), "{gaps:?}");

    // A second run has nothing of it to ask.
    let (result, out) = s.run("");
    result.unwrap();
    assert!(!out.contains("nobody can change these"), "{out}");
}

#[test]
fn a_list_changed_after_it_was_confirmed_is_unconfirmed_again() {
    let s = Scratch::new("not-offered-changed");
    with_app(&s, &format!("{HEAD}{USERS}"));
    let (result, _) = s.run("\nowner\n");
    result.unwrap();
    // The tool takes `recovery` off the list without the owner.
    let path = s.app().join("stackvet.toml");
    let text = std::fs::read_to_string(&path).unwrap();
    let changed = text.replacen(
        "not-offered = [\"phone\", \"recovery\"]",
        "not-offered = [\"phone\"]",
        1,
    );
    assert_ne!(changed, text, "the setup: the list was found to change");
    std::fs::write(&path, changed).unwrap();
    let m = manifest(&s);
    assert!(!counts(&m, &Waiting::NotOffered, &s.checker()));
    let mut verified = after_a_run();
    let mut gaps = Vec::new();
    crate::assemble::account_changes_whole(&m, &s.checker(), &mut verified, &mut gaps);
    assert!(verified.iter().all(|v| v.in_part), "{verified:?}");
    assert_eq!(gaps.len(), 1, "{gaps:?}");
}

#[test]
fn every_change_refused_is_whole_without_a_list_and_no_email_credit_is_never_whole() {
    let s = Scratch::new("not-offered-none");
    with_app(
        &s,
        &format!("{HEAD}\n[stack.run]\nstart = \"python app.py\"\n[stack.run.users]\n"),
    );
    let m = manifest(&s);
    let all = |email: bool| {
        let mut v: Vec<sv_check::Verified> = ["phone", "two-factor", "recovery"]
            .iter()
            .map(|k| credit(&format!("probe.{k}-change-without-password")))
            .collect();
        if email {
            v.push(credit("probe.email-change-without-password"));
        }
        v
    };
    let mut verified = all(true);
    let mut gaps = Vec::new();
    crate::assemble::account_changes_whole(&m, &s.checker(), &mut verified, &mut gaps);
    assert!(verified.iter().all(|v| !v.in_part), "{verified:?}");
    assert!(verified.iter().all(|v| !v.scope.contains("owner's word")));
    assert!(gaps.is_empty());

    // Without the email change refused, nothing is whole.
    let mut verified = all(false);
    crate::assemble::account_changes_whole(&m, &s.checker(), &mut verified, &mut gaps);
    assert!(verified.iter().all(|v| v.in_part), "{verified:?}");
}

#[test]
fn a_review_written_into_the_file_by_hand_counts_for_nothing() {
    let s = Scratch::new("not-offered-forged");
    // What an AI coding tool could write itself: the entry, with a seal it made up.
    with_app(
        &s,
        &format!(
            "{HEAD}{USERS}\n[account-changes-review]\nnot-offered = [\"phone\", \"recovery\"]\n\
             by = \"owner\"\non = \"2026-10-12\"\nseal = \"ssh-ed25519 made-up\"\n"
        ),
    );
    let m = manifest(&s);
    assert!(
        m.account_changes_review.is_some(),
        "the setup: the entry was read"
    );
    assert!(!counts(&m, &Waiting::NotOffered, &s.checker()));
    let mut verified = after_a_run();
    let mut gaps = Vec::new();
    crate::assemble::account_changes_whole(&m, &s.checker(), &mut verified, &mut gaps);
    assert!(verified.iter().all(|v| v.in_part), "{verified:?}");
    assert_eq!(gaps.len(), 1, "{gaps:?}");
}
