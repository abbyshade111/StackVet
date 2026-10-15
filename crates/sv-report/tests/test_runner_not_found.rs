//! Backlog 0532: when the app's tests stop with exit 127, the shell's code for a program it could not find, the
//! report says the test runner is most likely not installed where the app runs and points to `sv preflight`, rather
//! than only "the tests failed", which the AI coding tool in the trial of 14 October 2026 read as the app's fault.

fn ended(exit_code: i32) -> String {
    sv_report::test_output_intro(&sv_check::suite::FailingOutput {
        exit_code,
        text: "sh: 1: pytest: not found".to_owned(),
        lines_kept: 1,
        lines_total: 1,
        redacted: 0,
        stopped_after: None,
    })
}

#[test]
fn exit_127_names_the_missing_runner_and_the_preflight() {
    let intro = ended(127);
    assert!(intro.contains("could not start"), "{intro}");
    assert!(intro.contains("test runner is not installed"), "{intro}");
    assert!(intro.contains("`sv preflight`"), "{intro}");
    assert!(!intro.contains("failed when"), "{intro}");
}

#[test]
fn any_other_exit_still_says_the_tests_failed() {
    // The control: the new words are for 127 alone.
    let intro = ended(1);
    assert!(
        intro.contains("The app's own tests failed when `sv` ran them (exit 1)"),
        "{intro}"
    );
    assert!(!intro.contains("`sv preflight`"), "{intro}");
}
