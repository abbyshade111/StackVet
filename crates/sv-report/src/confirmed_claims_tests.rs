//! Backlog 0606, the owner's choice of 16 October 2026: a capability answer the owner confirmed through `sv review` is
//! labeled theirs, and changes nothing else: the count of requirements resting on a "no" is the same.

use super::*;

fn excluded(id: &str, condition: &str) -> ExcludedRequirement {
    ExcludedRequirement {
        id: id.into(),
        description: String::new(),
        chapter: String::new(),
        reason: String::new(),
        condition: condition.into(),
        rests_on: "claim",
    }
}

fn claim(name: &str, claimed: Option<bool>) -> ClaimLine {
    ClaimLine {
        name: name.into(),
        claimed,
        found_in_code: None,
        state: String::new(),
        note: String::new(),
        by: claimed.map(|_| "ai-tool"),
    }
}

#[test]
fn a_confirmed_answer_is_labeled_the_owners_and_a_silence_stays_nobodys() {
    let mut claims = [
        claim("uploads", Some(false)),
        claim("payments", Some(false)),
        claim("webrtc", None),
    ];
    mark_confirmed(&mut claims, &["uploads".to_owned(), "webrtc".to_owned()]);
    assert!(claims[0].confirmed_by_owner());
    assert_eq!(claims[0].who_said(), "yours, confirmed through `sv review`");
    assert!(!claims[1].confirmed_by_owner());
    assert_eq!(
        claims[1].who_said(),
        "not recorded, so read as your AI coding tool's"
    );
    assert_eq!(claims[2].who_said(), "nobody");
}

#[test]
fn the_count_resting_on_a_no_is_the_same_and_names_the_confirmed_apart() {
    let ex = [
        excluded("V5.1.1", "uploads"),
        excluded("V5.2.1", "uploads"),
        excluded("V12.1.1", "payments"),
    ];
    let mut claims = [
        claim("uploads", Some(false)),
        claim("payments", Some(false)),
    ];
    let before = resting_on_a_no(&ex, &claims).expect("three rest on a no");
    mark_confirmed(&mut claims, &["uploads".to_owned()]);
    let after = resting_on_a_no(&ex, &claims).expect("still three");
    let opening = "3 requirements do not apply only because stackvet.toml says no to";
    assert!(
        before.starts_with(opening) && after.starts_with(opening),
        "{after}"
    );
    assert!(
        after.contains("you confirmed `uploads` through `sv review`"),
        "{after}"
    );
    assert!(
        after.contains(
            "does not record who gave `payments`, so it is read as your AI coding tool's word"
        ),
        "{after}"
    );
    // All confirmed: nothing is said to be the tool's.
    mark_confirmed(&mut claims, &["uploads".to_owned(), "payments".to_owned()]);
    let all = resting_on_a_no(&ex, &claims).expect("still three");
    assert!(all.starts_with(opening), "{all}");
    assert!(!all.contains("AI coding tool"), "{all}");
}
