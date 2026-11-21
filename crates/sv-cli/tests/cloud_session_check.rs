//! `tools/cloud_session_check.py` (backlog 0710; ADR-158, Later, 21 November 2026): a session's pull request opened by
//! the owner's own account that changes a code-owned file fails the required "Decision records" check. Its self-test
//! reads the committed CODEOWNERS whole; this also holds the workflow to running it.

use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn the_cloud_session_check_passes_its_own_test() {
    let out = Command::new("python3")
        .arg("-I")
        .arg(root().join("tools/cloud_session_check.py"))
        .arg("--self-test")
        .output()
        .expect("python3 runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    assert!(said.contains("cloud_session_check self-test: ok"), "{said}");
}

#[test]
fn the_required_check_runs_it_on_every_pull_request() {
    let workflow =
        std::fs::read_to_string(root().join(".github/workflows/decision-records.yml")).unwrap();
    assert!(
        workflow.contains("name: Decision records"),
        "the setup: this is the required check's workflow"
    );
    for line in [
        "python3 -I tools/cloud_session_check.py --self-test",
        "python3 -I tools/cloud_session_check.py --base \"$BASE_SHA\" --head \"$HEAD_SHA\" --event \"$GITHUB_EVENT_PATH\"",
    ] {
        assert!(workflow.contains(line), "{line:?} is not in the workflow");
    }
}
