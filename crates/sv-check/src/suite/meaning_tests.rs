//! A test named in other words than its requirement's is not reported when the two say the same
//! thing (backlog 0387). The second loop trial read ten of these findings by hand at random, and
//! three were sound tests: the inactivity timeout and function-level access below.

use super::*;

fn reported(text: &str, (id, requirement): (&str, &str)) -> bool {
    let tests = vec![NamedTest {
        requirement_ids: vec![id.into()],
        text: text.into(),
        file: "tests/test_app.py".into(),
        line: 1,
    }];
    let describe = |_: &str| Some(requirement.to_owned());
    let (verified, findings) = credit(&tests, SuiteOutcome::Passed, &describe);
    // The setup: the test was read and credited, so the quiet below means something.
    assert_eq!(verified.len(), 1, "{text}");
    !findings.is_empty()
}

const V7_3_1: (&str, &str) = (
    "V7.3.1",
    "Verify that there is an inactivity timeout such that re-authentication is enforced \
                      according to risk analysis and documented security decisions.",
);
const V8_2_1: (&str, &str) = (
    "V8.2.1",
    "Verify that the application ensures that function-level access is restricted to \
                      consumers with explicit permissions.",
);

#[test]
fn the_trials_sound_tests_are_not_reported() {
    assert!(!reported(
        "def test_V7_3_1_session_unused_past_the_limit_is_refused():",
        V7_3_1
    ));
    assert!(!reported(
        "def test_V8_2_1_admin_page_refused_to_ordinary_user():",
        V8_2_1
    ));
}

#[test]
fn a_test_about_something_else_is_still_reported() {
    // The control: no shared word and no shared meaning, as before.
    assert!(reported("def test_V7_3_1_password_is_hashed():", V7_3_1));
    assert!(reported("def test_V8_2_1_homepage_renders():", V8_2_1));
}

#[test]
fn the_meanings_do_not_loosen_the_word_comparison_the_guards_read() {
    // `shares_no_words` stays strict: the citation and threat guards compare with it.
    assert!(shares_no_words(
        "def test_V7_3_1_session_unused_past_the_limit_is_refused():",
        V7_3_1.1
    ));
}
