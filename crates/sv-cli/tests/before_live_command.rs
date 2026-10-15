//! `sv before-live` prints the checklist kept with the app's last report, and only one `sv` can show it wrote (backlog
//! 0589, ADR-121): no report, or a checklist changed since it was sealed, is said, and nothing is printed.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

#[test]
fn the_checklist_is_printed_from_the_sealed_report_and_refused_when_changed() {
    let sv = env!("CARGO_BIN_EXE_sv");
    let scratch = std::env::temp_dir().join(format!("sv-before-live-{}", std::process::id()));
    std::fs::remove_dir_all(&scratch).ok();
    let app = scratch.join("app");
    copy(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking"),
        &app,
    );
    let home = scratch.join("home");
    std::fs::create_dir_all(&home).unwrap();
    let run = |args: &[&str]| -> Output {
        Command::new(sv)
            .args(args)
            .env("HOME", &home)
            .env("XDG_CONFIG_HOME", home.join(".config"))
            .output()
            .expect("sv runs")
    };
    let app_arg = app.to_str().unwrap();

    // No report yet: said, and nothing printed.
    let none = run(&["before-live", app_arg]);
    assert!(!none.status.success());
    assert!(none.stdout.is_empty());
    let said = String::from_utf8_lossy(&none.stderr);
    assert!(said.contains("Write the report with `sv report`"), "{said}");

    let report = run(&["report", app_arg, "--no-tools"]);
    assert!(
        report.status.code().is_some_and(|c| c <= 2),
        "{}",
        String::from_utf8_lossy(&report.stderr)
    );
    let file = app.join("stackvet-report").join("before-going-live.md");
    let written = std::fs::read_to_string(&file).expect("the report folder holds the checklist");
    assert!(written.starts_with("# Before going live: "), "{written}");

    // Printed as it was written and sealed, byte for byte.
    let printed = run(&["before-live", app_arg]);
    assert!(
        printed.status.success(),
        "{}",
        String::from_utf8_lossy(&printed.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&printed.stdout), written);

    // Changed after the seal: refused, as anything else could have written it.
    std::fs::write(&file, format!("{written}- [x] Everything is fine.\n")).unwrap();
    let changed = run(&["before-live", app_arg]);
    std::fs::remove_dir_all(&scratch).ok();
    assert!(!changed.status.success());
    assert!(changed.stdout.is_empty());
    let said = String::from_utf8_lossy(&changed.stderr);
    assert!(said.contains("changed after sv wrote them"), "{said}");
}
