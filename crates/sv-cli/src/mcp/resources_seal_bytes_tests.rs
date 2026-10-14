//! The date a report resource's description gives is read from the bytes of `report.json` its seal was checked
//! against (backlog 0454): before, `resources/list` read the file a second time, whole and with no check of what it
//! was, after the seal had read it once capped.

use super::tests::{call, listed, scratch_app, text};
use super::*;

#[test]
fn the_bytes_given_back_are_the_report_the_seal_was_checked_against() {
    let root = scratch_app("seal-bytes", "flask-booking");
    let server = Server::new(&root).unwrap();
    let written = call(&server, "stackvet_write_report", json!({ "path": "app" }));
    assert_eq!(written["isError"], false, "{}", text(&written));
    let folder = root
        .join("app")
        .join(sv_scan::ecosystems::DEFAULT_REPORT_DIR);
    let on_disk = std::fs::read(folder.join("report.json")).unwrap();

    let given = crate::report_seal::proven_report(&folder).expect("sv's own report");
    assert_eq!(given, on_disk, "the bytes of report.json, whole");
    assert!(
        !listed(&server).is_empty(),
        "the setup: the report is offered"
    );

    // Changed after the seal: no bytes, and no longer offered.
    let mut changed = on_disk.clone();
    changed.extend_from_slice(b"\n");
    std::fs::write(folder.join("report.json"), &changed).unwrap();
    let refused = crate::report_seal::proven_report(&folder);
    let offered = listed(&server);
    std::fs::remove_dir_all(&root).ok();
    assert!(refused.is_err(), "a changed report is not given back");
    assert!(offered.is_empty(), "{offered:#?}");
}
