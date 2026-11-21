//! `tools/rulesets_check.py` and `.github/rulesets.json`, the rules each repository should have on GitHub (backlog
//! 0690). The weekly workflow compares the file with GitHub; this runs the script's self-test, which reads the file,
//! and holds the file to what the decision records and the workflow say of it. Nothing here opens a connection.

use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

#[test]
fn the_rulesets_check_passes_its_own_test() {
    let out = Command::new("python3")
        .arg("-I")
        .arg(root().join("tools/rulesets_check.py"))
        .arg("--self-test")
        .output()
        .expect("python3 runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    assert!(said.contains("rulesets_check self-test: ok"), "{said}");
}

#[test]
fn the_file_is_governed_and_the_workflow_runs_the_check() {
    // A rule changed on GitHub owes a record only because these records govern the file.
    for record in ["docs/adr/ADR-051.md", "docs/adr/ADR-108.md"] {
        let text = read(record);
        let governs = text
            .split("**Governs:**")
            .nth(1)
            .and_then(|rest| rest.split("\n\n").next())
            .unwrap_or_else(|| panic!("{record} has a Governs paragraph"));
        assert!(
            governs.contains("`.github/rulesets.json`"),
            "{record} does not govern .github/rulesets.json: {governs}"
        );
    }
    let workflow = read(".github/workflows/rulesets.yml");
    assert!(workflow.contains("schedule:"), "it runs each week");
    assert!(
        workflow.contains("python3 -I tools/rulesets_check.py\n"),
        "it runs the comparison"
    );
    assert!(
        workflow.contains("permissions:\n  contents: read"),
        "its token is read-only"
    );
    assert!(!workflow.contains("secrets."), "no secret is added");
}

#[test]
fn the_required_checks_in_the_file_are_jobs_that_exist() {
    // The required checks the file lists for this repository are ones its workflows report, so the file and CI agree.
    let file = read(".github/rulesets.json");
    assert!(
        file.contains("\"abbyshade111/StackVet\""),
        "the setup: {file}"
    );
    for (context, workflow) in [
        (
            "\"context\": \"Decision records\"",
            ".github/workflows/decision-records.yml",
        ),
        ("\"context\": \"test\"", ".github/workflows/rust.yml"),
    ] {
        assert!(file.contains(context), "the file requires {context}");
        assert!(root().join(workflow).is_file(), "{workflow} reports it");
    }
}
