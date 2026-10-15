//! A turn the AI coding tool ends saying the app's tests pass, read against its own session's commands (backlog 0565,
//! ADR-122, Later): only the kind of claim is kept, never the tool's words.

use crate::hook_record::{TESTS_PASS, append, claims_in, line_of, read};
use serde_json::json;
use std::path::{Path, PathBuf};
use sv_report::hook::Mismatch;

fn rules() -> &'static sv_check::secrets::SecretRules {
    sv_check::secrets::SecretRules::shared().expect("the rules sv ships")
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "sv-hook-claims-{name}-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_claim_that_the_tests_pass_is_told_from_one_that_they_do_not_or_will() {
    for said in [
        "Done. All 12 tests pass.",
        "The test suite is passing now!",
        "I ran the tests and they passed",
        "Tests are green; the sign-in page works.",
    ] {
        assert_eq!(claims_in(said), [TESTS_PASS], "{said}");
    }
    for said in [
        "Two tests fail.",
        "The tests don't pass yet.",
        "I did not run the tests.",
        "No tests were run.",
        "I'll make the tests pass next.",
        "The tests should pass once the database is up.",
        "The page works and the login passes its checks.",
        "",
    ] {
        assert!(claims_in(said).is_empty(), "{said}");
    }
}

#[test]
fn a_turn_s_end_keeps_the_kind_of_claim_and_none_of_the_words() {
    let line = line_of(
        &json!({ "hook_event_name": "Stop", "session_id": "s", "last_assistant_message": "All 12 tests pass. Secret plan: x." }),
        Path::new("/app"),
        "T",
        rules(),
    )
    .unwrap();
    assert_eq!(line["claims"], json!([TESTS_PASS]));
    let text = line.to_string();
    assert!(!text.contains("12") && !text.contains("Secret"), "{text}");
    let quiet = line_of(
        &json!({ "hook_event_name": "Stop", "session_id": "s", "last_assistant_message": "I changed the page." }),
        Path::new("/app"),
        "T",
        rules(),
    )
    .unwrap();
    assert!(quiet.get("claims").is_none(), "{quiet}");
}

#[test]
fn each_claim_is_read_against_its_own_session() {
    let dir = scratch("sessions");
    let record = dir.join("r.jsonl");
    let tool = |session: &str, event: &str, name: &str, input: serde_json::Value, at: &str| {
        let line = line_of(
            &json!({ "hook_event_name": event, "session_id": session, "tool_name": name, "tool_input": input }),
            Path::new("/app"),
            at,
            rules(),
        )
        .unwrap();
        append(&record, line).unwrap();
    };
    let said = |session: &str, at: &str| {
        let line = line_of(
            &json!({ "hook_event_name": "Stop", "session_id": session, "last_assistant_message": "All tests pass." }),
            Path::new("/app"),
            at,
            rules(),
        )
        .unwrap();
        append(&record, line).unwrap();
    };
    // One: the tests failed, then it said they pass.
    tool(
        "one",
        "PostToolUseFailure",
        "Bash",
        json!({ "command": "pytest" }),
        "T1",
    );
    said("one", "T2");
    // Two: no test ran in its session (one's test does not count for it).
    said("two", "T3");
    // Three: the tests ran clean, then a file was written, then it said they pass.
    tool(
        "three",
        "PostToolUse",
        "Bash",
        json!({ "command": "npm test" }),
        "T4",
    );
    tool(
        "three",
        "PostToolUse",
        "Edit",
        json!({ "file_path": "/app/a.js" }),
        "T5",
    );
    said("three", "T6");
    // Four: the tests ran clean, and it said so.
    tool(
        "four",
        "PostToolUse",
        "Bash",
        json!({ "command": "cargo test" }),
        "T7",
    );
    said("four", "T8");

    let seen = read(&record, None);
    assert_eq!(seen.claims, 4);
    let found: Vec<(&str, &Mismatch)> = seen
        .mismatches
        .iter()
        .map(|m| (m.at.as_str(), &m.what))
        .collect();
    assert_eq!(
        found,
        [
            ("T2", &Mismatch::LastTestFailed { at: "T1".into() }),
            ("T3", &Mismatch::NoTestRan),
            (
                "T6",
                &Mismatch::WrittenSince {
                    at: "T4".into(),
                    files: 1
                }
            ),
        ]
    );
    std::fs::remove_dir_all(&dir).ok();
}
