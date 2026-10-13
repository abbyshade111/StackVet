//! Every release build in a workflow uses the committed lock file (backlog 0358). The native build `rust.yml` compares
//! with the image was the one without `--locked`: with a stale lock it would have been built, quietly, from crate
//! versions other than the image's, and the comparison would not have been like with like.

use std::path::Path;

/// The line numbers (from 1) of `cargo build` commands without `--locked`.
fn unlocked(text: &str) -> Vec<usize> {
    text.lines()
        .enumerate()
        .filter(|(_, line)| {
            let code = line.split('#').next().unwrap_or("");
            code.contains("cargo build") && !code.contains("--locked")
        })
        .map(|(i, _)| i + 1)
        .collect()
}

#[test]
fn the_reader_sees_a_build_without_the_lock() {
    assert_eq!(
        unlocked(
            "run: cargo build --release -p sv-cli\nrun: cargo build --release --locked -p sv-cli\n# cargo build in a comment\n"
        ),
        vec![1]
    );
}

#[test]
fn every_cargo_build_in_the_workflows_is_locked() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/workflows");
    let mut builds = 0;
    let mut failures = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("the workflows folder is readable") {
        let path = entry.expect("a workflow entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("yml") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("a workflow is readable");
        builds += text.matches("cargo build").count();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        failures.extend(
            unlocked(&text)
                .into_iter()
                .map(|line| format!("{name}:{line}")),
        );
    }
    // The setup: the release builds are there to be read (rust.yml, windows.yml, audit.yml, release.yml).
    assert!(builds >= 4, "found {builds} cargo builds in the workflows");
    assert!(
        failures.is_empty(),
        "cargo build without --locked: {failures:?}"
    );
}
