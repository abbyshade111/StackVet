//! Backlog 0639: a test run that stopped on writing into the app's read-only folder is said as that, not as tests
//! failing. The owner's Cursor build wrote `--junitxml=junit.xml` beside its code; its 27 tests pass.

use crate::{RunStatus, test_output_intro};

fn output(text: &str, exit_code: i32) -> sv_check::suite::FailingOutput {
    sv_check::suite::FailingOutput {
        exit_code,
        text: text.to_owned(),
        lines_kept: text.lines().count(),
        lines_total: text.lines().count(),
        redacted: 0,
        stopped_after: None,
    }
}

#[test]
fn a_read_only_write_is_not_said_as_tests_failing() {
    let said = test_output_intro(&output(
        "INTERNALERROR> OSError: [Errno 30] Read-only file system: '/app/junit.xml'\n",
        3,
    ));
    assert!(
        said.contains("stopped when `sv` ran them (exit 3) on writing into the app's folder"),
        "{said}"
    );
    assert!(
        said.contains("whether they pass is not known from this run"),
        "{said}"
    );
    assert!(said.contains("--junitxml=/sv-reports/junit.xml"), "{said}");
    assert!(!said.contains("tests failed"), "{said}");
}

#[test]
fn a_real_failure_is_still_said_as_one() {
    let said = test_output_intro(&output(
        "FAILED tests/test_app.py::test_login - assert 401 == 200\n",
        1,
    ));
    assert!(
        said.starts_with("The app's own tests failed when `sv` ran them (exit 1)."),
        "{said}"
    );
    assert!(RunStatus::wrote_into_read_only(
        "OSError: [Errno 30] Read-only file system: '/app/x'"
    ));
    assert!(!RunStatus::wrote_into_read_only("1 failed, 26 passed"));
}
