//! Backlog 0322: `tools/cvss4_tables.py` copies from a checkout of FIRST's calculator only at the commit the committed
//! tables name, or at one named on purpose with `--commit`; a clone at another commit would write other tables with
//! that commit in their header, unnoticed. The copy itself needs the calculator and `node`, so this runs the script's
//! self-test, which holds the refusal and reads the committed tables' commit.

use std::process::Command;

#[test]
fn the_cvss_copier_refuses_a_checkout_at_another_commit() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = Command::new("python3")
        .arg("-I")
        .arg(root.join("tools/cvss4_tables.py"))
        .arg("--self-test")
        .current_dir(&root)
        .output()
        .expect("python3 runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    assert!(said.contains("cvss4_tables.py: self-test passed"), "{said}");
}
