//! The loop trials' measures added for the second loop trial (ADR-109): the `sv` tools counted under both of the
//! MCP server's names, the outside tools' findings from the second check the builder never sees, and the build-loop
//! record (`docs/prompts/loop-pilot/loop_measures.py`). The script is run on the trials' builds by hand; this holds
//! it to its self-test, which has a written-out build for each. Not on Windows: the scripts are the trials' own
//! tools, run on Mac and Linux and not part of `sv`, and the second trial's scorer failed its self-test on every
//! Windows run of `main` from its first (#1584) while passing on both; the owner asked on 12 October 2026 that it
//! leave the Windows build, as the coordination board's test did (ADR-061, Later).
#![cfg(not(windows))]

use std::process::Command;

#[test]
fn the_loop_measures_pass_their_self_test() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = Command::new("python3")
        .arg(root.join("docs/prompts/loop-pilot/loop_measures.py"))
        .arg("--self-test")
        .output()
        .expect("python3 is needed to test the loop trials' measures");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    // The setup: the self-test ran to its end, not merely started.
    assert!(said.contains("loop_measures self-test passed"), "{said}");
}

/// One of the second loop trial's scripts, run with `--self-test`, must pass and say it reached its end.
fn self_test(script: &str, said_at_end: &str) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = Command::new("python3")
        .arg(root.join(script))
        .arg("--self-test")
        .output()
        .expect("python3 is needed to test the loop trials' scripts");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    assert!(said.contains(said_at_end), "{said}");
}

#[test]
fn the_second_loop_trials_scorer_passes_its_self_test() {
    self_test(
        "docs/prompts/loop-2/score_loop2.py",
        "score_loop2 self-test passed",
    );
}

#[test]
fn the_scripted_review_answers_only_the_not_offered_question_as_the_owner() {
    self_test(
        "docs/prompts/loop-2/scripted_review.py",
        "scripted_review self-test passed",
    );
}
