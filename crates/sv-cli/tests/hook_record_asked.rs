//! The hook `sv connect claude --hook` prints, run as Claude Code runs it, for the permissions a person is asked for
//! (backlog 0718): an asking is a line for any tool; a tool with no line of its own is written down only when it
//! ran after an asking; and `sv report --hook-record` counts commands and file writes as it did.

#![cfg(unix)]

use serde_json::Value;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn copy(from: &std::path::Path, to: &std::path::Path) {
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

struct Place {
    app: PathBuf,
    home: PathBuf,
    hook: String,
}

fn place(name: &str) -> Place {
    let scratch = std::env::temp_dir().join(format!("sv-hook-asked-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&scratch).ok();
    let (app, home) = (scratch.join("app"), scratch.join("home"));
    // An example app, so `sv report` has something to read.
    copy(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking"),
        &app,
    );
    std::fs::create_dir_all(&home).unwrap();
    let printed = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["connect", "claude", "--hook"])
        .current_dir(&app)
        .env_remove("SV_CONNECT_IN_CONTAINER")
        .output()
        .expect("sv runs");
    assert!(
        printed.status.success(),
        "{}",
        String::from_utf8_lossy(&printed.stderr)
    );
    let block: Value = serde_json::from_slice(&printed.stdout).expect("the block is JSON");
    let hook = block["hooks"]["PermissionRequest"][0]["hooks"][0]["command"]
        .as_str()
        .expect("a hook for the asking")
        .to_owned();
    Place { app, home, hook }
}

impl Place {
    /// Runs the printed hook through the shell with `input`, as Claude Code runs a command hook.
    fn fire(&self, input: &str) {
        let mut child = Command::new("sh")
            .args(["-c", &self.hook])
            .current_dir(&self.app)
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", self.home.join("state"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("sh runs");
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        // A hook that printed something could be read by Claude Code as a decision about the call.
        assert!(
            out.stdout.is_empty(),
            "the hook printed: {}",
            String::from_utf8_lossy(&out.stdout)
        );
    }

    fn lines(&self) -> Vec<Value> {
        let records = self.home.join("state/stackvet/hook-records");
        let Ok(found) = std::fs::read_dir(&records) else {
            return Vec::new();
        };
        let files: Vec<_> = found.flatten().collect();
        assert_eq!(files.len(), 1, "one record for the one app");
        std::fs::read_to_string(files[0].path())
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).expect("a line of JSON"))
            .collect()
    }
}

const ASK_CHECK: &str = r#"{"hook_event_name":"PermissionRequest","session_id":"s","tool_name":"mcp__stackvet__stackvet_check","tool_input":{"section":"all"}}"#;
const RAN_CHECK: &str = r#"{"hook_event_name":"PostToolUse","session_id":"s","tool_name":"mcp__stackvet__stackvet_check","tool_input":{"section":"all"},"tool_response":{"content":"the tool's own words"}}"#;

#[test]
fn a_tool_with_no_line_of_its_own_is_written_down_only_after_an_asking_and_only_once() {
    let place = place("only");
    // It ran without the person being asked: nothing is written, so the record is not a list of every tool used.
    place.fire(RAN_CHECK);
    assert_eq!(place.lines().len(), 0);

    place.fire(ASK_CHECK);
    place.fire(RAN_CHECK);
    // And again unasked: the asking was answered already.
    place.fire(RAN_CHECK);
    let lines = place.lines();
    let events: Vec<&str> = lines.iter().map(|l| l["event"].as_str().unwrap()).collect();
    assert_eq!(events, ["Asked", "Ran"], "{lines:?}");
    assert_eq!(lines[0]["print"], lines[1]["print"]);
    assert_eq!(lines[0]["summary"], r#"{"section":"all"}"#);
    let all = format!("{lines:?}");
    assert!(
        !all.contains("the tool's own words"),
        "an answer's text was kept: {all}"
    );
}

#[test]
fn an_asking_that_is_not_followed_by_a_run_stays_as_the_only_line_and_sv_s_report_counts_as_before()
{
    let place = place("refused");
    // As the owner's trial went: asked, said no, and nothing more for that call.
    place.fire(ASK_CHECK);
    place.fire(r#"{"hook_event_name":"PermissionRequest","session_id":"s","tool_name":"Bash","tool_input":{"command":"pytest -q"}}"#);
    place.fire(r#"{"hook_event_name":"PostToolUse","session_id":"s","tool_name":"Bash","tool_input":{"command":"pytest -q"}}"#);
    let lines = place.lines();
    let events: Vec<&str> = lines.iter().map(|l| l["event"].as_str().unwrap()).collect();
    assert_eq!(events, ["Asked", "Asked", "PostToolUse"], "{lines:?}");

    // sv's own report reads the same record and counts one command, no file written: an asking is not a command,
    // and is not a file write.
    let report = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["report", "--no-tools", "--hook-record"])
        .current_dir(&place.app)
        .env("HOME", &place.home)
        .env("XDG_STATE_HOME", place.home.join("state"))
        .env("XDG_CONFIG_HOME", place.home.join(".config"))
        .output()
        .expect("sv runs");
    assert!(
        report.status.code().is_some_and(|c| c <= 2),
        "{}",
        String::from_utf8_lossy(&report.stderr)
    );
    let json: Value = serde_json::from_str(
        &std::fs::read_to_string(place.app.join("stackvet-report/report.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        json["hook_record"]["commands"], 1,
        "{}",
        json["hook_record"]
    );
    assert_eq!(json["hook_record"]["writes"], 0, "{}", json["hook_record"]);
}
