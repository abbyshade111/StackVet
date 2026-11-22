//! `sv mcp` notes, outside the app's folder, where the build-loop record has got to (ADR-159, decision 12; ADR-076,
//! Later, 22 November 2026; backlog 0716): run as a person's AI coding tool runs it, with its records kept under a
//! folder of the test's own.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use sv_frameworks::paths::Canonical;

const SV: &str = env!("CARGO_BIN_EXE_sv");

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-note-outside-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

/// Starts `sv mcp` for `app`, makes `calls` calls to a tool, and waits for it to end.
fn serve(app: &Path, state: &Path, calls: usize, record: Option<&str>) {
    let mut command = Command::new(SV);
    command
        .args(["mcp", "--root", app.to_str().unwrap()])
        .env("XDG_STATE_HOME", state)
        .env("XDG_CONFIG_HOME", state)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    match record {
        Some(value) => command.env("SV_BUILD_LOOP_RECORD", value),
        None => command.env_remove("SV_BUILD_LOOP_RECORD"),
    };
    let mut child = command.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","id":0,"method":"initialize","params":{{"protocolVersion":"2025-06-18","capabilities":{{}},"clientInfo":{{"name":"t","version":"1"}}}}}}"#
    )
    .unwrap();
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","method":"notifications/initialized"}}"#
    )
    .unwrap();
    for id in 1..=calls {
        writeln!(
            stdin,
            r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"stackvet_spec","arguments":{{}}}}}}"#
        )
        .unwrap();
    }
    drop(stdin);
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "sv mcp ended with {:?}", out.status);
}

/// The notes kept for `app` under `state`, as lines of JSON.
fn notes(app: &Path, state: &Path) -> Vec<serde_json::Value> {
    let name = av_record::plain::sha256(app.display().to_string().as_bytes());
    let path = state
        .join("stackvet/loop-notes")
        .join(format!("{}.jsonl", &name[..16]));
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).expect("a line of JSON"))
        .collect()
}

#[test]
fn the_server_notes_each_line_of_the_record_outside_the_app_s_folder() {
    let app = folder("noted");
    let state = folder("noted-state");
    serve(&app, &state, 2, None);
    // The setup: the server wrote the record, in the app's folder.
    let record = av_record::build_loop::read(&app);
    assert!(
        record.calls >= 2,
        "the setup: the calls were written down: {record:?}"
    );
    let noted = notes(&app, &state);
    assert_eq!(
        noted.len(),
        record.chained,
        "one place noted for each line written: {noted:?}"
    );
    let last = noted.last().unwrap();
    assert_eq!(last["lines"].as_u64(), Some(record.chained as u64));
    assert_eq!(last["head"].as_str(), record.chain_head.as_deref());
    // Outside the app's folder, and nothing of what the record holds.
    let whole = noted.iter().map(|n| n.to_string()).collect::<String>();
    assert!(!whole.contains("stackvet_spec"), "{whole}");
    assert!(!state.starts_with(&app));
}

#[test]
fn with_the_record_turned_off_nothing_is_noted() {
    let app = folder("off");
    let state = folder("off-state");
    serve(&app, &state, 2, Some("off"));
    assert_eq!(
        av_record::build_loop::read(&app).calls,
        0,
        "the setup: no record was written"
    );
    assert_eq!(notes(&app, &state).len(), 0);
    assert!(
        !state.join("stackvet/loop-notes").exists(),
        "no folder of notes was made"
    );
}
