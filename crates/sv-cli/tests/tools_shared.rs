//! `tools/_shared.py` holds what the scripts in `tools/` share: `write_text` (UTF-8 and Unix line endings) and `slug`
//! (a title made into a file name). Twelve scripts once had a copy of the first and two of the second, and the two
//! `slug`s had come to differ (backlog 0464). This runs its self-test, and fails when a script defines either again
//! (`docs_page.py`'s `slug(text, seen)`, which makes a heading's id, is another function).

use std::path::PathBuf;
use std::process::Command;

fn tools() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tools")
}

#[test]
fn the_shared_helpers_pass_their_own_test_run_as_ci_runs_the_tools() {
    let out = Command::new("python3")
        .arg("-I")
        .arg(tools().join("_shared.py"))
        .output()
        .expect("python3 runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    assert!(said.contains("_shared self-test: ok"), "{said}");
}

#[test]
fn no_script_keeps_a_copy_of_its_own() {
    let mut read = 0;
    let mut copies = Vec::new();
    for entry in std::fs::read_dir(tools()).unwrap().flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !name.ends_with(".py") || name == "_shared.py" {
            continue;
        }
        read += 1;
        let text = std::fs::read_to_string(&path).unwrap();
        for helper in ["def write_text(", "def slug(title)"] {
            if text.lines().any(|l| l.starts_with(helper)) {
                copies.push(format!("{name}: {helper}"));
            }
        }
    }
    assert!(read > 10, "the setup read only {read} scripts");
    assert!(
        copies.is_empty(),
        "import these from tools/_shared.py instead: {copies:?}"
    );
}
