//! What the report makes of the app's own tests when the run ran them (backlog 0324). No test in sv-cli held this before
//! `running_app` handed it to `own_tests`: with the function returning nothing, every test passed. Each test here fails
//! with its arm's work taken out.

use super::*;

/// An app folder with one test that names a requirement, the way `sv init` asks for it.
fn app(name: &str) -> (std::path::PathBuf, sv_scan::files::Listing) {
    let dir = std::env::temp_dir().join(format!("sv-own-tests-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("tests")).unwrap();
    std::fs::write(
        dir.join("tests/test_search.py"),
        "def test_V1_2_1_search_uses_bound_parameters():\n    assert True\n",
    )
    .unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    (dir, listing)
}

fn result(exit_code: i32, stopped: bool) -> sv_run::TestResult {
    sv_run::TestResult {
        exit_code,
        output: if exit_code == 0 {
            "1 passed".into()
        } else {
            "1 failed".into()
        },
        report: None,
        report_note: None,
        stopped_after: stopped.then(|| std::time::Duration::from_secs(600)),
    }
}

#[test]
fn a_suite_that_passed_credits_the_requirement_its_test_names() {
    let loaded = Loaded::load().unwrap();
    let (dir, listing) = app("passed");
    // The setup: the walk found the test file.
    assert!(
        listing
            .files
            .iter()
            .any(|e| e.relative == "tests/test_search.py"),
        "{:?}",
        listing.files
    );
    let (mut findings, mut gaps) = (Vec::new(), Vec::new());
    let (examined, output, verified) = own_tests(
        &Some(result(0, false)),
        &listing,
        &loaded.frameworks,
        loaded.secret_rules,
        &mut findings,
        &mut gaps,
    );
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        verified
            .iter()
            .any(|v| v.requirement_ids.iter().any(|id| id == "V1.2.1")),
        "{verified:?}"
    );
    assert!(
        output.is_none(),
        "a suite that passed has no failing output"
    );
    assert!(gaps.is_empty(), "{gaps:?}");
    assert_ne!(
        examined,
        sv_report::Examined::not_run("tests.", "stubbed"),
        "the tests were read"
    );
}

#[test]
fn a_suite_that_failed_credits_nothing_and_keeps_its_output() {
    let loaded = Loaded::load().unwrap();
    let (dir, listing) = app("failed");
    let (mut findings, mut gaps) = (Vec::new(), Vec::new());
    let (_, output, verified) = own_tests(
        &Some(result(1, false)),
        &listing,
        &loaded.frameworks,
        loaded.secret_rules,
        &mut findings,
        &mut gaps,
    );
    std::fs::remove_dir_all(&dir).ok();
    assert!(verified.is_empty(), "{verified:?}");
    assert!(
        output.is_some(),
        "the failing run's output is kept for the report"
    );
}

#[test]
fn a_suite_stopped_for_taking_too_long_credits_nothing_and_says_so() {
    let loaded = Loaded::load().unwrap();
    let (dir, listing) = app("stopped");
    let (mut findings, mut gaps) = (Vec::new(), Vec::new());
    let (examined, _, verified) = own_tests(
        &Some(result(0, true)),
        &listing,
        &loaded.frameworks,
        loaded.secret_rules,
        &mut findings,
        &mut gaps,
    );
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        verified.is_empty(),
        "a suite cut short credits nothing, whatever it printed: {verified:?}"
    );
    assert_eq!(gaps.len(), 1, "{gaps:?}");
    assert_eq!(
        gaps[0].what,
        "anything the app's own tests would have shown"
    );
    assert_eq!(
        examined,
        sv_report::Examined::not_run(
            "tests.",
            "the app's own tests were stopped before they finished"
        )
    );
}

#[test]
fn no_test_command_is_said_as_such() {
    let loaded = Loaded::load().unwrap();
    let (dir, listing) = app("none");
    let (mut findings, mut gaps) = (Vec::new(), Vec::new());
    let (examined, output, verified) = own_tests(
        &None,
        &listing,
        &loaded.frameworks,
        loaded.secret_rules,
        &mut findings,
        &mut gaps,
    );
    std::fs::remove_dir_all(&dir).ok();
    assert!(verified.is_empty() && output.is_none());
    assert_eq!(
        examined,
        sv_report::Examined::not_run("tests.", "stackvet.toml declares no test command")
    );
}
