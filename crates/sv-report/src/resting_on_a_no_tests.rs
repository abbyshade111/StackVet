//! The count of requirements that do not apply only because stackvet.toml answers `false` (backlog 0600).

use super::*;

fn excluded(id: &str, condition: &str, rests_on: &'static str) -> ExcludedRequirement {
    ExcludedRequirement {
        id: id.into(),
        description: String::new(),
        chapter: String::new(),
        reason: String::new(),
        condition: condition.into(),
        rests_on,
    }
}

fn claim(name: &str, claimed: Option<bool>, found_in_code: Option<bool>) -> ClaimLine {
    ClaimLine {
        name: name.into(),
        claimed,
        found_in_code,
        state: String::new(),
        note: String::new(),
        by: claimed.map(|_| "ai-tool"),
    }
}

#[test]
fn each_requirement_resting_on_a_no_is_counted_and_each_answer_named_once() {
    let ex = [
        excluded("V5.1.1", "uploads", "claim"),
        excluded("V5.2.1", "uploads", "claim"),
        excluded("V12.1.1", "payments", "claim"),
        // On the code's word, not the file's: not counted.
        excluded("V17.1.1", "webrtc", "derived"),
    ];
    let claims = [
        claim("uploads", Some(false), None),
        claim("payments", Some(false), Some(false)),
        claim("webrtc", None, Some(false)),
    ];
    let line = resting_on_a_no(&ex, &claims).expect("three rest on a no");
    assert!(
        line.starts_with(
            "3 requirements do not apply only because stackvet.toml says no to `payments` and `uploads`."
        ),
        "{line}"
    );
    assert!(line.contains("`sv` did not check those answers"), "{line}");
    assert!(line.contains("your AI coding tool's word"), "{line}");
}

#[test]
fn one_requirement_reads_in_the_singular() {
    let line = resting_on_a_no(
        &[excluded("V5.1.1", "uploads", "claim")],
        &[claim("uploads", Some(false), None)],
    )
    .expect("one rests on a no");
    assert!(
        line.starts_with("1 requirement does not apply only because stackvet.toml says no to `uploads`. `sv` did not check that answer"),
        "{line}"
    );
}

#[test]
fn an_exclusion_on_a_yes_or_on_the_code_is_not_counted() {
    // `no-auth` is ruled out by `auth = true`: the file's word, but not a no, and no claim of its own.
    let ex = [
        excluded("V6.1.1", "no-auth", "claim"),
        excluded("V17.1.1", "webrtc", "derived"),
    ];
    let claims = [
        claim("auth", Some(true), None),
        claim("webrtc", Some(false), None),
    ];
    assert_eq!(resting_on_a_no(&ex, &claims), None);
    assert_eq!(resting_on_a_no(&[], &claims), None);
}

#[test]
fn who_gave_an_answer_is_shown_as_the_tools_unless_nobody_gave_one() {
    assert_eq!(
        claim("uploads", Some(false), None).who_said(),
        "not recorded, so read as your AI coding tool's"
    );
    assert_eq!(
        claim("uploads", Some(true), None).who_said(),
        "not recorded, so read as your AI coding tool's"
    );
    assert_eq!(claim("webrtc", None, Some(true)).who_said(), "nobody");
}
