//! The census reads a file's code up to where its own tests begin, and a `#[cfg(test)] mod name;` that
//! only declares a file of tests kept elsewhere is not that place (backlog 0430). Until then every file was
//! cut at its first `#[cfg(test)]`, so `sv-cli`'s `assemble.rs`, which declares one at line 53, was read as
//! 53 lines, and the credit it makes whole on the owner's word was left out as a test's own; about 11,700
//! lines of 13 files were read as test code.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn a_credit_below_a_declared_test_module_is_counted_as_shipping_code() {
    let file = "crates/sv-cli/src/assemble.rs";
    let code = std::fs::read_to_string(repo().join(file)).unwrap();
    let declared = code
        .find("#[cfg(test)]\n#[path")
        .expect("the setup: assemble.rs declares a test module in a file of its own");
    let line = code
        .lines()
        .position(|l| l.contains("credit.whole_after_all();"))
        .expect("the setup: assemble.rs makes a credit whole")
        + 1;
    assert!(
        code[..declared].lines().count() < line,
        "the setup: the declaration comes before the credit"
    );
    // A check that is only ever a finding, credited there: counted, it is a fault the census names.
    let dir = std::env::temp_dir().join(format!("sv-census-declared-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let log = dir.join("credits.log");
    std::fs::write(
        &log,
        format!("probe.password-hints\tV6.4.2\t{file}:{line}\n"),
    )
    .unwrap();
    std::fs::write(dir.join("credits.log.withheld"), "").unwrap();
    let out = Command::new("python3")
        .arg("-I")
        .arg(repo().join("tools/coverage.py"))
        .arg("--credits")
        .arg(&log)
        .output()
        .expect("python3 runs");
    std::fs::remove_dir_all(&dir).ok();
    let said = String::from_utf8_lossy(&out.stderr);
    assert!(
        said.contains("probe.password-hints is listed as only ever a finding"),
        "{said}"
    );
}
