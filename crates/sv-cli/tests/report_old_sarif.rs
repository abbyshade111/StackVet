//! Backlog 0409: the SARIF file is `stackvet.sarif`. A report folder written before the rename holds
//! `findings.sarif`, which is still `sv`'s own: the folder is written over, and the old file, which would hold
//! the old report's findings beside the new report, is removed.

use std::path::{Path, PathBuf};
use sv_scan::ecosystems::{DEFAULT_REPORT_DIR, OLD_SARIF_FILE, REPORT_MARKER};

use sv_check::test_support::scratch;

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            copy(&path, &to.join(entry.file_name()));
        } else {
            std::fs::copy(&path, to.join(entry.file_name())).unwrap();
        }
    }
}

/// A copy of the example app `partly-passing`, with a report folder as `sv` left it before the rename.
fn app_with_an_old_report(root: &Path) -> PathBuf {
    let from = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/partly-passing");
    let app = root.join("app");
    copy(&from, &app);
    let folder = app.join(DEFAULT_REPORT_DIR);
    std::fs::remove_dir_all(&folder).ok();
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join(REPORT_MARKER), "sv\n").unwrap();
    std::fs::write(folder.join("report.json"), "{}\n").unwrap();
    std::fs::write(folder.join(OLD_SARIF_FILE), "{\"old\": true}\n").unwrap();
    app
}

#[test]
fn a_report_folder_from_before_the_rename_is_written_over_and_loses_the_old_sarif_file() {
    let root = scratch("");
    let app = app_with_an_old_report(&root);
    let folder = app.join(DEFAULT_REPORT_DIR);
    assert!(folder.join(OLD_SARIF_FILE).is_file(), "the setup");
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("report")
        .arg(&app)
        .output()
        .unwrap();
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.status.code().is_some_and(|c| c < 2),
        "sv report refused or failed on a folder holding the old name: {said}"
    );
    let sarif: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(folder.join("stackvet.sarif")).unwrap())
            .expect("stackvet.sarif is SARIF");
    assert_eq!(sarif["version"], "2.1.0");
    assert!(
        !folder.join(OLD_SARIF_FILE).exists(),
        "the old report's findings.sarif was left beside the new report: {said}"
    );
    std::fs::remove_dir_all(&root).ok();
}
