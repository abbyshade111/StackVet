//! How `sv review` reads the last report it counts from (backlog 0451): only one `sv` can show it wrote, and only an
//! ordinary file, so a `report.json` the AI coding tool wrote is not repeated as `sv`'s count, and a pipe in its place
//! is not waited on.

use crate::review::findings_without_entries;
use std::path::Path;

use sv_check::test_support::scratch;

#[test]
fn a_sealed_report_is_counted_from() {
    // Keys of this test process's own, so no key is read or made on the computer running it.
    sv_check::seal::key_folder_for_tests(
        std::env::temp_dir().join(format!("sv-review-read-test-keys-{}", std::process::id())),
    );
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let app = scratch("sealed");
    for entry in std::fs::read_dir(&example).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            std::fs::copy(entry.path(), app.join(entry.file_name())).unwrap();
        }
    }
    let folder = app.join(sv_frameworks::names::REPORT_DIR);
    let mut quiet = |_: &str| {};
    sv_cli::report_folder::write_report_folder(
        &app,
        &folder,
        "a test",
        "elsewhere",
        false,
        || {
            sv_cli::assemble_report(
                &app,
                &sv_cli::ReportOptions::reading_only("a test"),
                &sv_cli::Loaded::load()?,
            )
        },
        &mut quiet,
    )
    .unwrap();
    assert!(
        sv_cli::report_seal::proven(&folder).is_ok(),
        "the setup: a report sealed where it was written"
    );
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(folder.join("report.json")).unwrap())
            .unwrap();
    let n = report["findings"].as_array().unwrap().len();
    assert!(n > 0, "the setup: the example has findings to count");
    let expected = if n == 1 {
        "lists 1 finding that counts".to_owned()
    } else {
        format!("lists {n} findings that count")
    };

    let said = findings_without_entries(&app);
    assert!(said.contains(&expected), "{said}");

    // The same files, one changed after the seal: no longer counted from.
    let mut changed = report.clone();
    changed["findings"] = serde_json::json!([]);
    std::fs::write(folder.join("report.json"), changed.to_string()).unwrap();
    let said = findings_without_entries(&app);
    std::fs::remove_dir_all(&app).ok();
    assert!(said.contains("No report sv can show it wrote"), "{said}");
    assert!(!said.contains("lists no finding"), "{said}");
}

#[cfg(unix)]
#[test]
fn a_pipe_in_place_of_the_report_is_not_waited_on() {
    let app = scratch("pipe");
    let folder = app.join(sv_frameworks::names::REPORT_DIR);
    std::fs::create_dir_all(&folder).unwrap();
    let made = std::process::Command::new("mkfifo")
        .arg(folder.join("report.json"))
        .status()
        .expect("mkfifo");
    assert!(made.success(), "the setup: a pipe named report.json");
    // Run where a hang would show as a failure, not as a test that never ends.
    let (sent, got) = std::sync::mpsc::channel();
    let dir = app.clone();
    std::thread::spawn(move || {
        let _ = sent.send(findings_without_entries(&dir));
    });
    let said = got
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("sv review waited on a pipe");
    std::fs::remove_dir_all(&app).ok();
    assert!(said.contains("No report sv can show it wrote"), "{said}");
}
