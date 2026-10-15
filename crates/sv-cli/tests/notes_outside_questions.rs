//! Backlog 0542: the report, and so `stackvet_check`'s "What was not examined", says when security-notes.md holds
//! headings of its own before any question, with how many and the first, and how to make the decisions count. In the
//! Cursor trial of 14 October 2026 the tool wrote the owner's real choices that way, and they counted for nothing
//! with no word but the counts.

use std::path::Path;

/// A copy of the flask-booking example's files, with `notes` as its security-notes.md when given.
fn report_with(tag: &str, notes: Option<&str>) -> sv_report::Report {
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let app = std::env::temp_dir().join(format!("sv-notes-outside-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&app);
    std::fs::create_dir_all(&app).unwrap();
    for entry in std::fs::read_dir(&example).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            std::fs::copy(entry.path(), app.join(entry.file_name())).unwrap();
        }
    }
    if let Some(notes) = notes {
        std::fs::write(app.join("security-notes.md"), notes).unwrap();
    }
    let report = sv_cli::assemble_report(
        &app,
        &sv_cli::ReportOptions::reading_only("a test"),
        &sv_cli::Loaded::load().unwrap(),
    )
    .unwrap();
    let _ = std::fs::remove_dir_all(&app);
    assert!(
        !report.app_name.is_empty(),
        "the example's report was assembled"
    );
    report
}

fn the_gap(report: &sv_report::Report) -> Option<&sv_report::Gap> {
    report
        .gaps
        .iter()
        .find(|gap| gap.what.contains("before any of its questions"))
}

#[test]
fn notes_under_headings_of_their_own_are_said_to_count_for_nothing() {
    let report = report_with(
        "by-hand",
        Some(
            "# Security notes\n\n## Session length\n\nSessions end after 30 minutes idle.\n\n## Guessing limit\n\nFive wrong passwords lock the account.\n",
        ),
    );
    let gap = the_gap(&report).expect("the report says the notes count for nothing");
    assert!(
        gap.what
            .contains("2 headings of your own in security-notes.md"),
        "{}",
        gap.what
    );
    assert!(
        gap.why.contains("\"Session length\""),
        "the first heading is named: {}",
        gap.why
    );
    assert!(
        gap.why.contains("`stackvet_record_answer`") && gap.why.contains("`sv notes`"),
        "{}",
        gap.why
    );
}

#[test]
fn no_notes_file_or_one_without_such_headings_says_nothing_of_it() {
    // The control: the line is for the hand-written file alone.
    assert!(the_gap(&report_with("none", None)).is_none());
    assert!(
        the_gap(&report_with(
            "title-only",
            Some("# Security notes\n\nNothing yet.\n")
        ))
        .is_none()
    );
}
