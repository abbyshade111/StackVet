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

/// What the owner's trial of 22 November 2026 showed: an asking for a file write kept the text to be written.
#[test]
fn an_asking_to_write_or_edit_a_file_keeps_the_file_s_name_and_nothing_of_what_would_be_written() {
    for (tool, input) in [
        (
            "Write",
            json!({ "file_path": "/app/notes/trial.txt", "content": "the file's own text" }),
        ),
        (
            "Edit",
            json!({ "file_path": "/app/notes/trial.txt", "old_string": "the file's own text", "new_string": "the file's own text, changed" }),
        ),
        (
            "NotebookEdit",
            json!({ "notebook_path": "/app/notes/trial.txt", "new_source": "the file's own text" }),
        ),
    ] {
        let asked = line(json!({
            "hook_event_name": "PermissionRequest", "session_id": "s", "tool_name": tool, "tool_input": input
        }))
        .unwrap();
        assert_eq!(asked["summary"], "notes/trial.txt", "{tool}");
        assert!(
            !asked.to_string().contains("the file's own text"),
            "{tool}: {asked}"
        );
        // The fingerprint is still of everything it was given, so the same write is the same call.
        assert_eq!(asked["print"].as_str().map(str::len), Some(64));
    }
    // A tool server's tool is not known to name a file this way: its input is summarized as text, cut short.
    let asked = line(json!({
        "hook_event_name": "PermissionRequest", "session_id": "s", "tool_name": "mcp__files__write",
        "tool_input": { "file_path": "/app/a.txt" }
    }))
    .unwrap();
    assert_eq!(asked["summary"], r#"{"file_path":"/app/a.txt"}"#);
}

/// What the owner's first real session showed, 22 November 2026: the AI coding tool's question to the person was
/// kept as a permission asked for, with the start of the question's text.
#[test]
fn the_tool_s_own_question_is_not_an_asking_and_its_other_tools_keep_no_words_of_the_tool_s() {
    let question = json!({ "questions": [{ "header": "Starting point", "question": "Shall I keep the existing sign-in?" }] });
    assert_eq!(
        line(json!({
            "hook_event_name": "PermissionRequest", "session_id": "s", "tool_name": "AskUserQuestion",
            "tool_input": question
        })),
        None,
        "a question to the person is not a permission asked for"
    );
    // And when it is answered, nothing is kept for it either: no asking is waiting for it.
    let answered = line(json!({
        "hook_event_name": "PostToolUse", "session_id": "s", "tool_name": "AskUserQuestion", "tool_input": question
    }))
    .unwrap();
    assert!(!answered.to_string().contains("Shall I keep"), "{answered}");
    assert!(!asked_and_waiting(b"", "s", "AskUserQuestion"));

    // Another tool of its own that names no command and no file: the name, the fingerprint, and no summary.
    let plan = line(json!({
        "hook_event_name": "PermissionRequest", "session_id": "s", "tool_name": "ExitPlanMode",
        "tool_input": { "plan": "First I will rewrite the sign-in, the tool's own words" }
    }))
    .expect("asking to leave plan mode is an asking");
    assert_eq!(plan["summary"], "");
    assert!(!plan.to_string().contains("the tool's own words"), "{plan}");
    assert_eq!(plan["print"].as_str().map(str::len), Some(64));
}
