//! `tools/ci_scope.py`, which says whether a pull request needs the whole suite (ADR-104): its own checks,
//! and the repository's own backlog read through it, the way CI reads a claim.

use std::io::Write;
use std::process::{Command, Stdio};

fn scope(input: &str, args: &[&str]) -> (bool, String) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut child = Command::new("python3")
        .arg("-I")
        .arg(root.join("tools/ci_scope.py"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("python3 runs");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), said)
}

#[test]
fn the_tool_passes_its_own_checks() {
    let (ok, said) = scope("", &["--self-test"]);
    assert!(ok, "{said}");
    assert!(said.contains("ci_scope self-test: ok"), "{said}");
}

#[test]
fn a_claim_of_a_real_item_takes_the_short_path_and_one_more_file_takes_the_long_one() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/backlog");
    // A real item's path, as `git diff --name-only` names it, so the pattern is held to the folder as
    // it really is and not only to the self-test's samples.
    let item = std::fs::read_dir(&root)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .find(|name| name.ends_with(".md") && name.as_bytes()[0].is_ascii_digit())
        .expect("an open backlog item to read");
    let claim = format!("docs/backlog/{item}\n");
    let (ok, said) = scope(&claim, &[]);
    assert!(ok, "{said}");
    assert_eq!(said.trim(), "false", "{claim}");

    let with_code = format!("{claim}crates/sv-cli/src/main.rs\n");
    let (ok, said) = scope(&with_code, &[]);
    assert!(ok, "{said}");
    assert_eq!(said.trim(), "true", "{with_code}");

    let (ok, said) = scope("", &[]);
    assert!(ok, "{said}");
    assert_eq!(said.trim(), "true", "no paths at all runs everything");
}
