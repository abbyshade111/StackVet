//! What `sv review` says when nothing is waiting (ADR-096): findings with no entry are never offered here.

use super::*;

fn app(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-review-gap-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn it_counts_the_findings_the_last_report_lists() {
    let dir = app("counted");
    let folder = dir.join(sv_frameworks::names::REPORT_DIR);
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("report.json"), r#"{"findings": [{}, {}]}"#).unwrap();
    let said = findings_without_entries(&dir);
    assert!(said.contains("lists 2 findings that count"), "{said}");
    assert!(said.contains("never offered here"), "{said}");
    assert!(said.contains("fixed in the code"), "{said}");
}

#[test]
fn with_no_report_it_says_so_rather_than_a_count() {
    let said = findings_without_entries(&app("none"));
    assert!(said.contains("No report was read"), "{said}");
    assert!(!said.contains("lists"), "no count without a report: {said}");
}
