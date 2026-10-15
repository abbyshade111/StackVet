//! The AI tool's first calls, before `stackvet.toml` exists, are in the record of the folder the server serves
//! (backlog 0586, ADR-076): in the Opus run of 15 October 2026 the record began after the manifest was written, and
//! the `stackvet_status`, `stackvet_guidance`, and `stackvet_spec` calls before it were missing.

use super::tests::{call, scratch_app};
use super::*;

fn lines(app: &Path) -> Vec<crate::build_loop::Line> {
    std::fs::read_to_string(sv_scan::ecosystems::build_loop_record_in(app))
        .unwrap_or_default()
        .lines()
        .filter_map(crate::build_loop::Line::parse)
        .collect()
}

#[test]
fn the_calls_before_stackvet_toml_are_written_down_and_say_there_was_none() {
    // The app's own folder served, its stackvet.toml not written yet.
    let root = scratch_app("first-calls", "tested-notes");
    let app = root.join("app");
    let manifest = app.join("stackvet.toml");
    let kept = std::fs::read_to_string(&manifest).unwrap();
    std::fs::remove_file(&manifest).unwrap();
    let server = Server::new(&app).unwrap().recording(true);
    call(&server, "stackvet_status", json!({}));
    call(&server, "stackvet_spec", json!({}));
    let before = lines(&app);
    assert_eq!(before.len(), 2, "{before:?}");
    assert!(before.iter().all(|l| l.no_manifest), "{before:?}");
    assert_eq!(
        before.iter().map(|l| l.tool.as_str()).collect::<Vec<_>>(),
        ["stackvet_status", "stackvet_spec"]
    );
    // The manifest written: the next call is the app's, and says nothing of one missing.
    std::fs::write(&manifest, kept).unwrap();
    call(&server, "stackvet_status", json!({}));
    let after = lines(&app);
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(after.len(), 3, "{after:?}");
    assert!(!after[2].no_manifest, "{after:?}");
}

#[test]
fn a_folder_below_the_one_served_with_no_stackvet_toml_is_not_written_down() {
    let root = scratch_app("first-calls-below", "tested-notes");
    std::fs::create_dir_all(root.join("other")).unwrap();
    let server = Server::new(&root).unwrap().recording(true);
    call(&server, "stackvet_check", json!({ "path": "other" }));
    let none = sv_scan::ecosystems::build_loop_record_in(&root.join("other"));
    let made = none.exists();
    std::fs::remove_dir_all(&root).ok();
    assert!(
        !made,
        "a folder that is neither the app nor the one served got a record"
    );
}

#[test]
fn a_line_says_there_was_no_stackvet_toml_and_reads_back() {
    let line = crate::build_loop::Line {
        time: "2026-10-15T18:00:00Z".into(),
        tool: "stackvet_status".into(),
        no_manifest: true,
        ..Default::default()
    };
    let json = line.to_json();
    assert!(json.contains("\"no_manifest\":true"), "{json}");
    assert_eq!(crate::build_loop::Line::parse(&json), Some(line));
}
