//! The credit census holds the list of checks only ever in part (backlog 0430): the documents print
//! a check as able to mark a requirement *checked* unless `tools/coverage.py` lists it as in part, and
//! about 75 checks that only ever credit in part were printed that way, with no gate on the list.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// What `--credits` says about `check` given a log of `lines`, each a line of the census without its
/// place; every one is given a place in code that ships, so none is left out as a test's own.
fn said(tag: &str, check: &str, lines: &[&str]) -> String {
    let dir = std::env::temp_dir().join(format!("sv-census-in-part-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let log = dir.join("credits.log");
    let text: String = lines
        .iter()
        .map(|l| format!("{l}\tcrates/sv-check/src/probes.rs:1\n"))
        .collect();
    std::fs::write(&log, text).unwrap();
    std::fs::write(dir.join("credits.log.withheld"), "").unwrap();
    let out = Command::new("python3")
        .arg("-I")
        .arg(repo().join("tools/coverage.py"))
        .arg("--credits")
        .arg(&log)
        .output()
        .expect("python3 runs");
    std::fs::remove_dir_all(&dir).ok();
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .filter(|l| l.contains(check))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn a_check_the_suite_only_saw_credit_in_part_must_be_listed_as_in_part() {
    // The setup: a check that gives credit, cites both, and is not listed as in part.
    let check = "probe.admin-action-ordinary-user";
    let script = std::fs::read_to_string(repo().join("tools/coverage.py")).unwrap();
    let listed = script
        .lines()
        .find(|l| l.starts_with("RUST_IN_PART = "))
        .expect("the setup: tools/coverage.py lists RUST_IN_PART");
    assert!(!listed.contains(check), "the setup: {check} is not listed");
    let made = format!("{check}\tV8.2.1,V8.3.1");
    let in_part = format!("{check}\tV8.2.1,V8.3.1\tin part");
    let whole = format!("{check}\tV8.2.1,V8.3.1\twhole");

    let only = said("only", check, &[&made, &in_part]);
    assert!(
        only.contains(&format!(
            "{check}: every one of its 1 credits the suite saw was in part"
        )),
        "{only}"
    );
    // One whole credit of two, and one marked and then made whole: neither is only ever in part.
    let one_whole = said("one-whole", check, &[&made, &in_part, &made]);
    assert!(!one_whole.contains("was in part"), "{one_whole}");
    let made_whole = said("made-whole", check, &[&made, &in_part, &whole]);
    assert!(!made_whole.contains("was in part"), "{made_whole}");
}

#[test]
fn a_check_listed_as_in_part_that_the_suite_saw_credit_whole_is_a_fault() {
    let check = "probe.ai-tool-timeout";
    let made = format!("{check}\tC9.1.1");
    let in_part = format!("{check}\tC9.1.1\tin part");

    let whole = said("listed-whole", check, &[&made]);
    assert!(
        whole.contains(&format!(
            "{check} is listed as only ever in part, and the suite saw 1 of its credits whole"
        )),
        "the setup or the gate: {check} must be listed in RUST_IN_PART\n{whole}"
    );
    // The control: in part, as listed, and nothing is said of it.
    let as_listed = said("listed-in-part", check, &[&made, &in_part]);
    assert!(!as_listed.contains("in part"), "{as_listed}");
}
