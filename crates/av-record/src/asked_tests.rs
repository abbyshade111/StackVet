//! What the hook record shows of the permissions asked for (backlog 0718): an asking the tool then ran after, one it
//! did not, the same call coming again, and what a hook cannot tell.

use super::*;
use crate::hook_record::line_of;
use serde_json::json;

fn rules() -> &'static sv_check::secrets::SecretRules {
    sv_check::secrets::SecretRules::shared().expect("the rules sv ships")
}

/// The record's bytes for these hook inputs, each as `sv hook-record` would write it, in order.
fn record(inputs: &[Value]) -> Vec<u8> {
    let mut text = String::new();
    for (n, input) in inputs.iter().enumerate() {
        let at = format!("2026-11-22T03:57:{n:02}Z");
        let line = line_of(input, Path::new("/app"), &at, rules()).expect("a line");
        text.push_str(&line.to_string());
        text.push('\n');
    }
    text.into_bytes()
}

fn asks(session: &str, tool: &str, input: Value) -> Value {
    json!({ "hook_event_name": "PermissionRequest", "session_id": session, "tool_name": tool, "tool_input": input })
}

fn ran(session: &str, tool: &str, input: Value) -> Value {
    json!({ "hook_event_name": "PostToolUse", "session_id": session, "tool_name": tool, "tool_input": input })
}

const CHECK: &str = "mcp__stackvet__stackvet_check";
const STATUS: &str = "mcp__stackvet__stackvet_status";

#[test]
fn an_asking_the_tool_ran_after_is_counted_as_run_and_one_it_did_not_is_listed() {
    // As the owner's trial of 22 November 2026 went: a status approved, a check refused, then a turn's end.
    let seen = of(&record(&[
        asks("s", STATUS, json!({})),
        ran("s", STATUS, json!({})),
        asks("s", CHECK, json!({ "section": "all" })),
        json!({ "hook_event_name": "Stop", "session_id": "s" }),
    ]));
    assert_eq!((seen.asked, seen.ran), (2, 1), "{seen:?}");
    assert_eq!(seen.not_run.len(), 1);
    let no = &seen.not_run[0];
    assert_eq!(no.tool, CHECK);
    assert_eq!(no.summary, r#"{"section":"all"}"#);
    assert_eq!((no.asked_again, no.ran_later), (0, 0));
    assert_eq!(no.next.as_deref(), Some("the turn ended"));
    assert!(!no.last_of_session);
}

#[test]
fn the_same_call_asked_for_again_or_run_later_in_the_session_is_counted_and_another_call_is_not() {
    let seen = of(&record(&[
        asks("s", "Bash", json!({ "command": "rm -rf data" })),
        // Asked again, the same thing, and not let again.
        asks("s", "Bash", json!({ "command": "rm -rf data" })),
        // Another command: not the same call.
        asks("s", "Bash", json!({ "command": "ls" })),
        ran("s", "Bash", json!({ "command": "ls" })),
        // And then the first one ran.
        asks("s", "Bash", json!({ "command": "rm -rf data" })),
        ran("s", "Bash", json!({ "command": "rm -rf data" })),
    ]));
    assert_eq!((seen.asked, seen.ran), (4, 2), "{seen:?}");
    assert_eq!(seen.not_run.len(), 2);
    let first = &seen.not_run[0];
    assert_eq!(first.summary, "rm -rf data");
    // Asked twice more, and ran once.
    assert_eq!((first.asked_again, first.ran_later), (2, 1), "{first:?}");
    assert_eq!(first.next.as_deref(), Some("asked for Bash: rm -rf data"));
    let second = &seen.not_run[1];
    assert_eq!((second.asked_again, second.ran_later), (1, 1), "{second:?}");
}

#[test]
fn another_session_s_run_does_not_answer_an_asking() {
    // The same call, the same fingerprint, run in another session without that session asking: it is not the answer
    // to this session's asking.
    let seen = of(&record(&[
        asks("one", CHECK, json!({})),
        ran("two", CHECK, json!({})),
    ]));
    assert_eq!((seen.asked, seen.ran), (1, 0), "{seen:?}");
    let no = &seen.not_run[0];
    // Nothing follows it in its own session: the prompt may still be open, and the other session's call is not
    // this one coming again.
    assert!(no.last_of_session, "{no:?}");
    assert_eq!((no.asked_again, no.ran_later), (0, 0));
}

#[test]
fn a_run_whose_input_was_changed_after_the_asking_still_answers_the_newest_asking_for_that_tool() {
    let seen = of(&record(&[
        asks(
            "s",
            "Write",
            json!({ "file_path": "/app/a.py", "content": "x" }),
        ),
        ran(
            "s",
            "Write",
            json!({ "file_path": "/app/a.py", "content": "x changed" }),
        ),
    ]));
    assert_eq!((seen.asked, seen.ran), (1, 1), "{seen:?}");
    assert_eq!(seen.not_run, Vec::<NotRun>::new());
}

#[test]
fn a_record_with_no_asking_shows_none_and_does_not_call_that_a_refusal() {
    let seen = of(&record(&[ran("s", "Bash", json!({ "command": "ls" }))]));
    assert_eq!((seen.asked, seen.ran), (0, 0));
    assert_eq!(seen.not_run, Vec::<NotRun>::new());
}

#[test]
fn a_key_in_what_a_tool_was_to_be_given_is_cut_from_the_asking() {
    // Built from pieces, so no key is written here whole.
    let key = format!("sk-ant-api03-{}", "A1b2C3d4E5f6G7h8".repeat(6));
    let bytes = record(&[asks(
        "s",
        "Bash",
        json!({ "command": format!("curl -H 'x-api-key: {key}' https://api.example") }),
    )]);
    let text = String::from_utf8(bytes).unwrap();
    assert!(
        text.contains("curl -H"),
        "the setup's command is there: {text}"
    );
    assert!(!text.contains(&key), "the key was written down");
}
