//! Backlog 0641, ADR-133: before the entries, `sv review` asks who reviewed the code for AC.4.1, writes the answer
//! beside the owner's check by hand, and seals it with the rest when the entry is recorded.

use super::tests::{Scratch, with_app};

const APP: &str = "manifest-version = 1\n[app]\nname = \"R\"\naudience = \"just-me\"\n[stack]\nlanguages = \
[\"python\"]\n[data]\ncategories = [\"contact\"]\n\n[checked-by-hand.'AC.4.1']\nresult = \"done\"\non = \
\"2026-10-22\"\nby = \"owner\"\nhow = \"I've looked at the files and code cited for each of the checks.\"\n";

fn reviewer(s: &Scratch) -> Option<String> {
    let m = sv_manifest::Manifest::load(&s.app().join("stackvet.toml")).unwrap();
    m.checked_by_hand["AC.4.1"].reviewer.clone()
}

#[test]
fn the_owner_is_asked_who_reviewed_and_the_answer_is_written_and_shown() {
    let s = Scratch::new("ac41-else");
    with_app(&s, APP);
    assert_eq!(reviewer(&s), None, "the setup: nothing answered yet");
    let (result, out) = s.run("yes\nowner\n");
    result.unwrap();
    assert!(
        out.contains(
            "Did someone other than the person who asked the AI tool for the code review it?"
        ),
        "{out}"
    );
    assert_eq!(reviewer(&s).as_deref(), Some("someone-else"));
    assert!(
        out.contains("reviewed by someone other than the person who asked for the code"),
        "{out}"
    );
    // Asked once: a second run has nothing to ask about it.
    let (_, again) = s.run("");
    assert!(!again.contains("Did someone other than"), "{again}");
}

#[test]
fn the_requester_s_own_review_is_written_as_that() {
    let s = Scratch::new("ac41-self");
    with_app(&s, APP);
    let (result, out) = s.run("no\nowner\n");
    result.unwrap();
    assert_eq!(reviewer(&s).as_deref(), Some("the-requester"));
    assert!(
        out.contains("AC.4.1 stays not verified until someone else reviews it"),
        "{out}"
    );
}

#[test]
fn only_yes_or_no_is_taken() {
    let s = Scratch::new("ac41-other");
    with_app(&s, APP);
    let (result, out) = s.run("maybe\nyes\n");
    result.unwrap();
    assert!(out.contains("Type `yes` or `no`."), "{out}");
    assert_eq!(reviewer(&s).as_deref(), Some("someone-else"));
}

#[test]
fn the_answer_is_sealed_with_the_entry_and_a_change_to_it_breaks_the_seal() {
    let s = Scratch::new("ac41-sealed");
    with_app(&s, APP);
    s.run("yes\nowner\n").0.unwrap();
    let path = s.app().join("stackvet.toml");
    let holds = |m: &sv_manifest::Manifest| {
        let h = &m.checked_by_hand["AC.4.1"];
        matches!(
            sv_check::seal::owner_recorded(
                &s.checker(),
                h.seal.as_deref(),
                &sv_check::seal::hand_check_fields("AC.4.1", h),
            ),
            Ok(sv_check::seal::Sealed::Signed { .. })
        )
    };
    let m = sv_manifest::Manifest::load(&path).unwrap();
    assert!(
        holds(&m),
        "the setup: the entry was recorded as the owner's"
    );
    // The AI coding tool turning the answer around afterwards is caught.
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("reviewer = \"someone-else\""), "{text}");
    std::fs::write(
        &path,
        text.replace(
            "reviewer = \"someone-else\"",
            "reviewer = \"the-requester\"",
        ),
    )
    .unwrap();
    assert!(
        !holds(&sv_manifest::Manifest::load(&path).unwrap()),
        "a changed reviewer still sealed"
    );
}
