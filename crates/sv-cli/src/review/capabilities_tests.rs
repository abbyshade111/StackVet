//! Backlog 0606, the owner's choice of 16 October 2026: `sv review` asks the owner to confirm the capability answers,
//! seals them as `[capabilities-review]`, and each confirmed answer is then the owner's while it says the same. It
//! changes no requirement; `crates/sv-report/src/confirmed_claims_tests.rs` holds the report to that.

use super::tests::{Scratch, with_app};
use super::{Waiting, counts};

/// The level's answers are left unconfirmed with Enter, so the capability answers are the one thing answered.
const SKIP_SCOPE: &str = "\n";

const APP: &str = "manifest-version = 1\n[app]\nname = \"R\"\naudience = \"just-me\"\n[stack]\nlanguages = [\"python\"]\n[data]\ncategories = [\"contact\"]\n[capabilities]\nauth = true\nuploads = false\n";

fn manifest(s: &Scratch) -> sv_manifest::Manifest {
    sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap()
}

fn text(s: &Scratch) -> String {
    std::fs::read_to_string(s.app().join("stackvet.toml")).unwrap()
}

/// The answers confirmed here, as the report reads them.
fn confirmed(s: &Scratch) -> Vec<String> {
    sv_check::capabilities_word::confirmed(&manifest(s), &s.checker())
}

#[test]
fn the_owner_confirms_the_answers_and_each_is_theirs() {
    let s = Scratch::new("capabilities-owner");
    with_app(&s, APP);
    assert!(
        confirmed(&s).is_empty(),
        "the setup: nothing is confirmed yet"
    );
    let (result, out) = s.run(&format!("{SKIP_SCOPE}owner\n"));
    result.unwrap();
    assert!(out.contains("auth = true"), "{out}");
    assert!(out.contains("uploads = false"), "{out}");
    assert!(out.contains("Confirming changes no requirement"), "{out}");
    assert!(out.contains("Confirmed as your answers."), "{out}");
    let entry = manifest(&s)
        .capabilities_review
        .expect("[capabilities-review] written");
    assert_eq!(entry.by, "owner");
    assert_eq!(entry.answers.get("uploads"), Some(&false));
    assert!(
        confirmed(&s).contains(&"uploads".to_owned()),
        "{}",
        text(&s)
    );
    assert!(confirmed(&s).contains(&"auth".to_owned()));
    assert!(counts(&manifest(&s), &Waiting::Capabilities, &s.checker()));
}

#[test]
fn enter_leaves_them_unconfirmed_and_writes_nothing() {
    let s = Scratch::new("capabilities-enter");
    with_app(&s, APP);
    let (result, out) = s.run(&format!("{SKIP_SCOPE}\n"));
    result.unwrap();
    assert!(
        out.contains("what stackvet.toml says your app does"),
        "{out}"
    );
    assert_eq!(text(&s), APP);
}

#[test]
fn only_the_owner_confirms_them() {
    let s = Scratch::new("capabilities-name");
    with_app(&s, APP);
    let (result, out) = s.run(&format!("{SKIP_SCOPE}AI coding tool\n\n"));
    result.unwrap();
    assert!(
        out.contains("Only the app's owner knows what it does"),
        "{out}"
    );
    assert_eq!(text(&s), APP);
}

#[test]
fn a_changed_answer_is_unconfirmed_again_and_the_others_stay_confirmed() {
    let s = Scratch::new("capabilities-changed");
    with_app(&s, APP);
    s.run(&format!("{SKIP_SCOPE}owner\n")).0.unwrap();
    let changed = text(&s).replacen("uploads = false", "uploads = true", 1);
    assert_ne!(changed, text(&s), "the setup: the answer is in the file");
    std::fs::write(s.app().join("stackvet.toml"), changed).unwrap();
    let still = confirmed(&s);
    assert!(still.contains(&"auth".to_owned()), "{still:?}");
    assert!(!still.contains(&"uploads".to_owned()), "{still:?}");
    let (_, out) = s.run(&format!("{SKIP_SCOPE}\n"));
    assert!(
        out.contains("what stackvet.toml says your app does"),
        "{out}"
    );
}

#[test]
fn a_capabilities_review_written_by_hand_without_a_seal_confirms_nothing() {
    let s = Scratch::new("capabilities-forged");
    with_app(
        &s,
        &format!(
            "{APP}[capabilities-review]\nanswers = {{ auth = true, uploads = false }}\nby = \"owner\"\non = \"2026-10-16\"\n"
        ),
    );
    assert!(
        manifest(&s).capabilities_review.is_some(),
        "the setup: the entry is read"
    );
    assert!(confirmed(&s).is_empty());
}

#[test]
fn once_confirmed_they_are_not_asked_again() {
    let s = Scratch::new("capabilities-again");
    with_app(&s, APP);
    s.run(&format!("{SKIP_SCOPE}owner\n")).0.unwrap();
    let (_, out) = s.run(SKIP_SCOPE);
    assert!(
        !out.contains("what stackvet.toml says your app does"),
        "{out}"
    );
}
