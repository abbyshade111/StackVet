//! The lines `sv hook-record` makes for a permission asked for and for a tool that ran after one (backlog 0718), and
//! which of them it keeps.

use crate::hook_record::{ASKED, RAN, asked_and_waiting, line_of};
use serde_json::{Value, json};
use std::path::Path;

fn rules() -> &'static sv_check::secrets::SecretRules {
    sv_check::secrets::SecretRules::shared().expect("the rules sv ships")
}

fn line(input: Value) -> Option<Value> {
    line_of(&input, Path::new("/app"), "2026-11-22T03:57:21Z", rules())
}

#[test]
fn an_asking_keeps_the_tool_a_fingerprint_and_a_summary_and_never_the_input_whole() {
    let long = "x".repeat(5000);
    let asked = line(json!({
        "hook_event_name": "PermissionRequest", "session_id": "s",
        "tool_name": "mcp__files__write", "tool_input": { "file": "a.txt", "text": long }
    }))
    .expect("an asking is a line, for any tool");
    assert_eq!(asked["event"], ASKED);
    assert_eq!(asked["tool"], "mcp__files__write");
    assert_eq!(asked["print"].as_str().map(str::len), Some(64));
    let summary = asked["summary"].as_str().unwrap();
    assert!(
        summary.contains("a.txt") && summary.chars().count() < 300,
        "{}",
        summary.len()
    );
    // A command is summarized as a command's line is: the command, not the JSON around it.
    let asked = line(json!({
        "hook_event_name": "PermissionRequest", "session_id": "s",
        "tool_name": "Bash", "tool_input": { "command": "git push\n--force" }
    }))
    .unwrap();
    assert_eq!(asked["summary"], "git push --force");
}

#[test]
fn a_run_carries_the_fingerprint_of_its_asking_and_a_tool_with_no_line_of_its_own_carries_nothing_else()
 {
    let input = json!({ "section": "all" });
    let asked = line(json!({
        "hook_event_name": "PermissionRequest", "session_id": "s", "tool_name": "mcp__sv__check", "tool_input": input
    }))
    .unwrap();
    let ran = line(json!({
        "hook_event_name": "PostToolUse", "session_id": "s", "tool_name": "mcp__sv__check", "tool_input": input,
        "tool_response": { "content": "the tool's own words" }
    }))
    .unwrap();
    assert_eq!(ran["event"], RAN);
    assert_eq!(ran["print"], asked["print"]);
    let keys: Vec<&String> = ran.as_object().unwrap().keys().collect();
    assert_eq!(
        keys,
        ["at", "event", "print", "session", "tool"],
        "nothing of what it was given or answered"
    );
    // A command's own line gains the fingerprint and is otherwise what it was.
    let command = line(json!({
        "hook_event_name": "PostToolUseFailure", "session_id": "s", "tool_name": "Bash",
        "tool_input": { "command": "pytest -q" }
    }))
    .unwrap();
    assert_eq!(command["event"], "PostToolUseFailure");
    assert_eq!(command["command"], "pytest -q");
    assert!(command["print"].is_string());
}

#[test]
fn a_tool_with_no_line_of_its_own_is_kept_only_while_an_asking_for_it_is_unanswered() {
    let record = |lines: &[Value]| {
        lines
            .iter()
            .map(|l| format!("{l}\n"))
            .collect::<String>()
            .into_bytes()
    };
    let asked = json!({ "event": ASKED, "session": "s", "tool": "mcp__sv__check" });
    let ran = json!({ "event": RAN, "session": "s", "tool": "mcp__sv__check" });
    // Never asked: the record does not become a list of every tool used.
    assert!(!asked_and_waiting(&record(&[]), "s", "mcp__sv__check"));
    assert!(asked_and_waiting(
        &record(std::slice::from_ref(&asked)),
        "s",
        "mcp__sv__check"
    ));
    // Answered already.
    assert!(!asked_and_waiting(
        &record(&[asked.clone(), ran]),
        "s",
        "mcp__sv__check"
    ));
    // Another session's asking, or another tool's, is not this one's.
    assert!(!asked_and_waiting(
        &record(std::slice::from_ref(&asked)),
        "t",
        "mcp__sv__check"
    ));
    assert!(!asked_and_waiting(
        &record(&[asked]),
        "s",
        "mcp__sv__status"
    ));
}
