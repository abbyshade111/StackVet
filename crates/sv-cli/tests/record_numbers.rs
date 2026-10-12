//! Record, backlog, and design-entry numbers taken twice are seen before a merge (backlog 0265; ADR-061, Later).
//! `tools/record_numbers.py` does the reading, and the "Decision records" check runs it on every pull request; this
//! runs its self-test and `next`, which must give a number past every one in the tree.

use std::process::Command;

fn root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run(args: &[&str]) -> (bool, String) {
    let out = Command::new("python3")
        .arg("-I")
        .arg(root().join("tools/record_numbers.py"))
        .args(args)
        .current_dir(root())
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
fn the_number_check_passes_its_own_test() {
    let (ok, said) = run(&["--self-test"]);
    assert!(ok, "{said}");
    assert!(said.contains("record_numbers self-test: ok"), "{said}");
}

#[test]
fn next_gives_a_design_number_past_every_one_here() {
    let (ok, said) = run(&["next", "design"]);
    assert!(ok, "{said}");
    let next: u32 = said
        .lines()
        .next()
        .and_then(|l| l.strip_prefix("design "))
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("a number: {said}"));
    let highest = std::fs::read_dir(root().join("docs/design"))
        .unwrap()
        .filter_map(|e| e.ok()?.file_name().to_str()?.get(..4)?.parse::<u32>().ok())
        .max()
        .expect("design entries");
    assert!(next > highest, "{next} after {highest}");
}
