//! Backlog 0404: the record of the build loop is kept in the app's `.stackvet` folder, so clearing the report
//! folder leaves it; a record kept in the report folder before then is read, and moved whole when the next line is
//! written; and with no record but other signs that `sv` was used, the report says the record may have been deleted.

use super::*;
use sv_check::test_support::scratch;

fn app() -> std::path::PathBuf {
    let dir = scratch("");
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"t\"\n",
    )
    .unwrap();
    dir
}

fn call(time: &str) -> Line {
    Line {
        time: time.to_owned(),
        tool: "stackvet_check".to_owned(),
        counts: Some(LoopCounts::default()),
        ..Line::default()
    }
}

#[test]
fn clearing_the_report_folder_leaves_the_record() {
    let dir = app();
    record(&dir, &call("10:00")).unwrap();
    record(&dir, &call("10:10")).unwrap();
    // A report folder, as `sv` writes one, cleared by the AI coding tool before a report.
    let report = sv_scan::ecosystems::default_report_dir_in(&dir);
    std::fs::create_dir_all(&report).unwrap();
    std::fs::write(report.join("report.json"), "{}\n").unwrap();
    std::fs::remove_dir_all(&report).unwrap();
    assert!(
        sv_scan::ecosystems::build_loop_record_in(&dir).is_file(),
        "the setup: the record is not where it is kept"
    );
    assert_eq!(
        read(&dir).calls,
        2,
        "the record went with the report folder"
    );
}

#[test]
fn a_record_in_the_report_folder_is_read_and_moved_whole_with_its_chain() {
    let dir = app();
    // Two chained lines written where the record was kept before: made by writing them here and moving them back.
    record(&dir, &call("10:00")).unwrap();
    record(&dir, &call("10:10")).unwrap();
    let report = sv_scan::ecosystems::default_report_dir_in(&dir);
    std::fs::create_dir_all(&report).unwrap();
    let old = report.join(sv_scan::ecosystems::BUILD_LOOP_RECORD);
    std::fs::rename(sv_scan::ecosystems::build_loop_record_in(&dir), &old).unwrap();
    assert!(old.is_file(), "the setup: no record in the old place");

    let before = read(&dir);
    assert_eq!(
        (before.calls, before.chained),
        (2, 2),
        "the old place is not read"
    );

    record(&dir, &call("10:20")).unwrap();
    assert!(!old.exists(), "the record was left in the report folder");
    let after = read(&dir);
    assert_eq!(
        (after.calls, after.chained, after.chain_broken_at),
        (3, 3, None),
        "the moved record lost a line or its chain"
    );
}

#[test]
fn with_no_record_the_signs_that_sv_was_used_are_named_and_the_report_says_it_may_have_been_deleted()
 {
    let dir = app();
    // Nothing shows it: no sign is named, and the report says nothing shows sv was used.
    assert_eq!(read(&dir).signs_of_use, Vec::<String>::new());
    let said = sv_report::build_loop_said(&read(&dir));
    assert!(said.starts_with("Nothing shows that sv was used"), "{said}");

    // An answer the AI coding tool marked as its own, as sv asks it to.
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"t\"\n[design]\n\"V8.3.1\" = { answer = \"yes\", by = \"ai-tool\" }\n",
    )
    .unwrap();
    let summary = read(&dir);
    assert_eq!(summary.calls, 0);
    assert_eq!(summary.signs_of_use.len(), 1, "{:?}", summary.signs_of_use);
    let said = sv_report::build_loop_said(&summary);
    assert!(
        said.starts_with("No record of the build was found; it may have been deleted."),
        "{said}"
    );
    assert!(said.contains("stackvet.toml holds answers marked as the AI coding tool's"));
    assert!(!said.contains("Nothing shows that sv was used"), "{said}");
}

#[test]
fn the_folder_the_record_is_kept_in_is_not_read_as_the_app_s_code() {
    let dir = app();
    record(&dir, &call("10:00")).unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    assert!(
        !listing
            .app_files()
            .any(|f| f.relative.starts_with(sv_scan::ecosystems::STATE_DIR)),
        "the record was listed as one of the app's files"
    );
}
