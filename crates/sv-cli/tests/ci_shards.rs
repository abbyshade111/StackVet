//! `tools/ci_shards.py`, which deals this crate's test files between the CI jobs that run them side by
//! side (ADR-106): its own checks, and every file here in exactly one of the two jobs, so none goes
//! unrun and none runs twice.

use std::collections::BTreeSet;
use std::process::Command;

fn shards(args: &[&str]) -> (bool, String) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = Command::new("python3")
        .arg("-I")
        .arg(root.join("tools/ci_shards.py"))
        .args(args)
        .output()
        .expect("python3 runs");
    (
        out.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
    )
}

#[test]
fn the_tool_passes_its_own_checks() {
    let (ok, said) = shards(&["--self-test"]);
    assert!(ok, "{said}");
    assert!(said.contains("ci_shards self-test: ok"), "{said}");
}

#[test]
fn every_test_file_here_is_in_exactly_one_job() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    // Each file is a program, and so is each folder holding a `main.rs` (backlog 0339).
    let here: BTreeSet<String> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs") || p.join("main.rs").is_file())
        .map(|p| p.file_stem().unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(
        here.contains("ci_shards"),
        "the setup: this file is among them"
    );
    assert!(
        here.contains("started_apps"),
        "the setup: the folder of container tests is among them"
    );
    let job = |n: &str| -> Vec<String> {
        let (ok, said) = shards(&["args", n, "2"]);
        assert!(ok, "{said}");
        said.split_whitespace()
            .filter(|w| *w != "--test")
            .map(str::to_owned)
            .collect()
    };
    let (one, two) = (job("1"), job("2"));
    let both: BTreeSet<&String> = one.iter().filter(|f| two.contains(f)).collect();
    assert!(both.is_empty(), "in both jobs: {both:?}");
    let dealt: BTreeSet<String> = one.iter().chain(&two).cloned().collect();
    assert_eq!(dealt, here, "dealt between the jobs is not every file here");
}
