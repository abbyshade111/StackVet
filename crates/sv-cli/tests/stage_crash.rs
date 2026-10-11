//! A check that crashes costs that check, not the report (backlog 0234): the report is still written, and the gap for
//! the check says it crashed and where. Under a debug build, `SV_PANIC_IN_STAGE` makes one check panic inside the
//! guard, so the path runs here; a release `sv` has no such switch.

use std::path::{Path, PathBuf};
use std::process::Command;

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).unwrap();
        }
    }
}

/// Runs `sv report` on a copy of an example app, with `crash` naming the check to make panic, if any.
fn report(name: &str, crash: Option<&str>) -> (std::process::Output, PathBuf) {
    let root = std::env::temp_dir().join(format!("sv-stage-crash-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&root).ok();
    let app = root.join("app");
    copy_dir(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/notes-with-users"),
        &app,
    );
    let out = root.join("report");
    let mut sv = Command::new(env!("CARGO_BIN_EXE_sv"));
    sv.env_remove("RUST_BACKTRACE")
        .env("XDG_CONFIG_HOME", root.join("config"))
        .args([
            "report",
            app.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ]);
    match crash {
        Some(check) => sv.env("SV_PANIC_IN_STAGE", check),
        None => sv.env_remove("SV_PANIC_IN_STAGE"),
    };
    (sv.output().expect("sv runs"), out)
}

#[test]
fn a_check_that_crashes_costs_itself_and_the_report_is_still_written() {
    let (crashed, out) = report("crashed", Some("running_app"));
    let said = String::from_utf8_lossy(&crashed.stderr).to_string();
    // The whole run did not end in sv's own failure, which is what a panic that escaped the guard would do.
    assert_ne!(crashed.status.code(), Some(3), "{said}");
    let json = std::fs::read_to_string(out.join("report.json"))
        .unwrap_or_else(|e| panic!("no report written: {e}\n{said}"));
    // The gap for the check says it crashed, with what and where, so a reader sees it.
    assert!(json.contains("\"reason\": \"crashed\""), "{json}");
    assert!(
        json.contains("the app's own checks: the check crashed"),
        "{json}"
    );
    assert!(
        json.contains("a panic asked for by SV_PANIC_IN_STAGE"),
        "{json}"
    );
    // The rest of the report is there: the findings and the examined list come from the stages that did run.
    assert!(json.contains("\"examined\""), "{json}");
}

#[test]
fn with_no_check_made_to_crash_no_crash_is_reported() {
    let (clean, out) = report("clean", None);
    let json = std::fs::read_to_string(out.join("report.json")).unwrap_or_default();
    // The setup: a report was written, so the absence below means something.
    assert!(
        !json.is_empty(),
        "{}",
        String::from_utf8_lossy(&clean.stderr)
    );
    assert!(!json.contains("\"reason\": \"crashed\""), "{json}");
}

/// Each reader of the app's files, crashed in turn: the report is still written, the reader's gap says it crashed, the
/// `examined` entry for it says it was not run, and what it would have said is not credited (backlog 0234).
#[test]
fn a_reader_of_the_app_that_crashes_costs_its_own_results_and_the_report_says_so() {
    for reader in [
        "scan_report",
        "bill_of_materials",
        "secrets",
        "config",
        "code",
    ] {
        let (crashed, out) = report(&format!("reader-{reader}"), Some(reader));
        let said = String::from_utf8_lossy(&crashed.stderr).to_string();
        assert_ne!(crashed.status.code(), Some(3), "{reader}: {said}");
        let json = std::fs::read_to_string(out.join("report.json"))
            .unwrap_or_else(|e| panic!("{reader}: no report written: {e}\n{said}"));
        assert!(json.contains("\"reason\": \"crashed\""), "{reader}: {json}");
        assert!(json.contains("the check crashed"), "{reader}: {json}");
        // The reader's `examined` entry says it was not run, for each reader that has one.
        let has_entry = reader != "scan_report";
        assert!(
            !has_entry || json.contains("crashed, so nothing it reads was checked"),
            "{reader}: the examined list does not say it was not run: {json}"
        );
    }
}

/// The list of the app's files crashed: no reader runs on it, since an empty list is not an empty app. The report
/// is still written, the gap names the list, and each reader's `examined` entry says it did not run because of it
/// (backlog 0234).
#[test]
fn a_crashed_file_list_stops_every_reader_and_the_report_says_so() {
    let (crashed, out) = report("reader-file_list", Some("file_list"));
    let said = String::from_utf8_lossy(&crashed.stderr).to_string();
    assert_ne!(crashed.status.code(), Some(3), "{said}");
    let json = std::fs::read_to_string(out.join("report.json"))
        .unwrap_or_else(|e| panic!("no report written: {e}\n{said}"));
    assert!(
        json.contains("the list of the app's files: the check crashed"),
        "{json}"
    );
    assert!(
        json.contains(
            "did not run, because the list of the app's files crashed, so nothing it reads was checked"
        ),
        "the readers' examined entries do not say they did not run: {json}"
    );
    // Nothing was read, so nothing can be a finding. A reader that ran on the empty list would say the app has no config
    // file, which is a finding about the list, not the app. The setup: a clean run of the same app does find things,
    // so an empty list here means something.
    let found = |json: &str| -> usize {
        let report: serde_json::Value = serde_json::from_str(json).expect("report.json parses");
        report["findings"].as_array().map_or(0, Vec::len)
    };
    let clean = report("reader-file_list-clean", None).1;
    let clean_json = std::fs::read_to_string(clean.join("report.json")).unwrap_or_default();
    assert!(
        found(&clean_json) > 0,
        "the clean run found nothing: {clean_json}"
    );
    assert_eq!(found(&json), 0, "a reader ran on the empty list: {json}");
}

/// The known-vulnerability check crashed: the report is still written, the gap says where, and the `examined` entry
/// for advisories says nothing was compared (backlog 0234).
#[test]
fn a_crashed_known_vulnerability_check_costs_itself_and_the_report_says_so() {
    let (crashed, out) = report("advisories", Some("advisories"));
    let said = String::from_utf8_lossy(&crashed.stderr).to_string();
    assert_ne!(crashed.status.code(), Some(3), "{said}");
    let json = std::fs::read_to_string(out.join("report.json"))
        .unwrap_or_else(|e| panic!("no report written: {e}\n{said}"));
    assert!(
        json.contains("the known-vulnerability check: the check crashed"),
        "{json}"
    );
    assert!(
        json.contains("the known-vulnerability check crashed, so nothing was compared"),
        "{json}"
    );
    assert!(
        json.contains("a panic asked for by SV_PANIC_IN_STAGE"),
        "{json}"
    );
}
