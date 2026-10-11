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
