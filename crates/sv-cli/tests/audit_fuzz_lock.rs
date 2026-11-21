//! The weekly audit compares the fuzzing crate's lock with the known vulnerabilities too (backlog 0707; ADR-077,
//! Later, 21 November 2026), in a step and a folder of its own, so a finding there is the fuzzing crate's and never
//! read as one in what `sv` ships. Measured on 21 November 2026: `sv audit` over the two copied files compared all 117
//! packages, and with a crate version that has a published advisory added to a scratch copy, it named it and exited 1.

use std::path::PathBuf;

fn audit_workflow() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows/audit.yml");
    std::fs::read_to_string(path).unwrap()
}

#[test]
fn the_audit_compares_the_fuzzing_crates_lock_in_a_step_of_its_own() {
    let workflow = audit_workflow();
    let own = workflow
        .find("- name: Audit sv's own crates")
        .expect("the setup: the audit of sv's own crates is there");
    let fuzz = workflow
        .find("- name: Audit the fuzzing crate")
        .expect("the audit compares the fuzzing crate's lock");
    assert!(own < fuzz, "the fuzzing crate's audit comes after sv's own");
    let step = &workflow[fuzz..];
    for line in [
        "git ls-files -z 'fuzz/Cargo.toml' 'fuzz/Cargo.lock'",
        "test -s \"$RUNNER_TEMP/sv-fuzz/Cargo.lock\"",
        "sv audit \"$RUNNER_TEMP/sv-fuzz\" --advisories \"$RUNNER_TEMP/osv\"",
        "if: ${{ !cancelled() }}",
    ] {
        assert!(step.contains(line), "{line:?} is not in the step:\n{step}");
    }
    // sv's own audit folder never holds the fuzzing crate's files, so its result stays sv's alone.
    let own_step = &workflow[own..fuzz];
    assert!(
        !own_step.contains("fuzz/"),
        "sv's own audit copies fuzz/:\n{own_step}"
    );
}

#[test]
fn a_change_to_the_fuzzing_crates_lock_runs_the_audit() {
    let workflow = audit_workflow();
    let paths: Vec<&str> = workflow
        .lines()
        .filter(|l| l.trim_start().starts_with("paths:"))
        .collect();
    assert_eq!(
        paths.len(),
        2,
        "the setup: push and pull_request each list paths"
    );
    for line in paths {
        for file in ["'fuzz/Cargo.lock'", "'fuzz/Cargo.toml'"] {
            assert!(line.contains(file), "{file} is not in {line}");
        }
    }
}
