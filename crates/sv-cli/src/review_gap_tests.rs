//! What `sv review` says when nothing is waiting (ADR-098): findings with no entry are never offered here.

use super::*;

fn app(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-review-gap-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_report_json_sv_cannot_show_it_wrote_is_not_counted_from() {
    // The AI coding tool can write report.json; its word is not repeated as sv's count (backlog 0451). A sealed report
    // is counted: `review_report_read_tests.rs`.
    let dir = app("counted");
    let folder = dir.join(sv_frameworks::names::REPORT_DIR);
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("report.json"), r#"{"findings": [{}, {}]}"#).unwrap();
    let said = findings_without_entries(&dir);
    assert!(said.contains("No report sv can show it wrote"), "{said}");
    assert!(!said.contains("lists 2"), "{said}");
    assert!(said.contains("never offered here"), "{said}");
    assert!(said.contains("fixed in the code"), "{said}");
}

#[test]
fn with_no_report_it_says_so_rather_than_a_count() {
    let said = findings_without_entries(&app("none"));
    assert!(
        said.contains("No report sv can show it wrote was read"),
        "{said}"
    );
    assert!(!said.contains("lists"), "no count without a report: {said}");
}
