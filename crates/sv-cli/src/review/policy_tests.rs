//! Backlog 0498, the owner's choice of 16 October 2026: `sv review` asks the owner to confirm `[policy]
//! fix-within-days`, seals it as `[policy-review]`, and V15.1.1 is then documented by the owner; unconfirmed, or changed
//! after, it is the AI coding tool's word.

use super::tests::{Scratch, with_app};
use super::{Waiting, counts};
use sv_check::verified::Tier;

/// The level's answers are left unconfirmed with Enter, so the time frames are the one thing answered.
const SKIP_SCOPE: &str = "\n";

const APP: &str = "manifest-version = 1\n[app]\nname = \"R\"\naudience = \"just-me\"\n[stack]\nlanguages = [\"python\"]\n[data]\ncategories = [\"contact\"]\n[policy]\nfix-within-days = { critical = 7, high = 30 }\n";

fn manifest(s: &Scratch) -> sv_manifest::Manifest {
    sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap()
}

fn text(s: &Scratch) -> String {
    std::fs::read_to_string(s.app().join("stackvet.toml")).unwrap()
}

/// The tier V15.1.1 rests on from the time frames, as the report gives it.
fn tier(s: &Scratch) -> Option<Tier> {
    sv_check::policy_word::fix_within_days(&manifest(s), &s.checker()).map(|v| {
        assert_eq!(v.requirement_ids, ["V15.1.1"]);
        v.tier
    })
}

#[test]
fn the_owner_confirms_the_time_frames_and_v15_1_1_is_documented() {
    let s = Scratch::new("policy-owner");
    with_app(&s, APP);
    assert_eq!(
        tier(&s),
        Some(Tier::Stated),
        "the setup: unconfirmed is the AI tool's word"
    );
    let (result, out) = s.run(&format!("{SKIP_SCOPE}owner\n"));
    result.unwrap();
    assert!(
        out.contains("critical within 7 days, high within 30 days"),
        "{out}"
    );
    assert!(out.contains("Confirmed as your answer."), "{out}");
    let entry = manifest(&s).policy_review.expect("[policy-review] written");
    assert_eq!(entry.by, "owner");
    assert_eq!(entry.fix_within_days.critical, Some(7));
    assert_eq!(entry.fix_within_days.medium, None);
    assert!(counts(&manifest(&s), &Waiting::Policy, &s.checker()));
    assert_eq!(tier(&s), Some(Tier::Documented), "{}", text(&s));
}

#[test]
fn enter_leaves_them_unconfirmed_and_writes_nothing() {
    let s = Scratch::new("policy-enter");
    with_app(&s, APP);
    let (result, out) = s.run(&format!("{SKIP_SCOPE}\n"));
    result.unwrap();
    assert!(out.contains("how soon a known vulnerability"), "{out}");
    assert_eq!(text(&s), APP);
    assert_eq!(tier(&s), Some(Tier::Stated));
}

#[test]
fn only_the_owner_confirms_them() {
    let s = Scratch::new("policy-name");
    with_app(&s, APP);
    let (result, out) = s.run(&format!("{SKIP_SCOPE}AI coding tool\n\n"));
    result.unwrap();
    assert!(
        out.contains("Only the app's owner can say how soon"),
        "{out}"
    );
    assert_eq!(text(&s), APP);
}

#[test]
fn once_confirmed_it_is_not_asked_again_and_a_change_undoes_it() {
    let s = Scratch::new("policy-again");
    with_app(&s, APP);
    s.run(&format!("{SKIP_SCOPE}owner\n")).0.unwrap();
    let (_, out) = s.run(SKIP_SCOPE);
    assert!(!out.contains("how soon a known vulnerability"), "{out}");
    let changed = text(&s).replacen("critical = 7", "critical = 14", 1);
    assert_ne!(
        changed,
        text(&s),
        "the setup: the time frame is in the file"
    );
    std::fs::write(s.app().join("stackvet.toml"), changed).unwrap();
    assert_eq!(
        tier(&s),
        Some(Tier::Stated),
        "still documented after the change"
    );
    let (_, out) = s.run(&format!("{SKIP_SCOPE}\n"));
    assert!(out.contains("how soon a known vulnerability"), "{out}");
}

#[test]
fn no_time_frames_give_nothing_and_ask_nothing() {
    let s = Scratch::new("policy-none");
    let app = APP.replace(
        "[policy]\nfix-within-days = { critical = 7, high = 30 }\n",
        "",
    );
    with_app(&s, &app);
    assert_eq!(tier(&s), None);
    let (_, out) = s.run(SKIP_SCOPE);
    assert!(!out.contains("how soon a known vulnerability"), "{out}");
}

#[test]
fn a_policy_review_written_by_hand_without_a_seal_is_not_the_owners_word() {
    let s = Scratch::new("policy-forged");
    with_app(
        &s,
        &format!(
            "{APP}[policy-review]\nfix-within-days = {{ critical = 7, high = 30 }}\nby = \"owner\"\non = \"2026-10-16\"\n"
        ),
    );
    assert!(
        manifest(&s).policy_review.is_some(),
        "the setup: the entry is read"
    );
    assert_eq!(tier(&s), Some(Tier::Stated));
}
