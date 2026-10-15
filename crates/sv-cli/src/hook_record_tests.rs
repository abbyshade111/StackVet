//! The hook record (backlog 0566, ADR-122): what a line keeps of what Claude Code's hook hands over, the chain, and what
//! a report reads back.

use crate::hook_record::{append, line_of, read, runs_tests};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use sv_report::hook::{Earlier, Tip};

fn rules() -> &'static sv_check::secrets::SecretRules {
    sv_check::secrets::SecretRules::shared().expect("the rules sv ships")
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "sv-hook-record-{name}-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn tool(event: &str, tool: &str, input: Value) -> Value {
    json!({ "hook_event_name": event, "session_id": "s1", "tool_name": tool, "tool_input": input })
}

const APP: &str = "/home/someone/app";

#[test]
fn a_line_keeps_the_command_or_the_file_and_nothing_the_tool_wrote_or_said() {
    let app = Path::new(APP);
    let write = line_of(
        &tool(
            "PostToolUse",
            "Write",
            json!({ "file_path": "/home/someone/app/src/db.py", "content": "SECRET" }),
        ),
        app,
        "T",
        rules(),
    )
    .expect("a write is recorded");
    assert_eq!(write["file"], "src/db.py");
    assert!(!write.to_string().contains("SECRET"), "{write}");

    let outside = line_of(
        &tool("PostToolUse", "Edit", json!({ "file_path": "/etc/hosts" })),
        app,
        "T",
        rules(),
    )
    .unwrap();
    assert_eq!(
        outside["file"], "/etc/hosts",
        "a write outside the app keeps its whole path"
    );

    let failed = line_of(
        &tool(
            "PostToolUseFailure",
            "Bash",
            json!({ "command": "npm test" }),
        ),
        app,
        "T",
        rules(),
    )
    .unwrap();
    assert_eq!(failed["event"], "PostToolUseFailure");
    assert_eq!(failed["command"], "npm test");

    let stop = line_of(
        &json!({ "hook_event_name": "Stop", "session_id": "s1", "last_assistant_message": "All tests pass." }),
        app,
        "T",
        rules(),
    )
    .unwrap();
    assert!(
        !stop.to_string().contains("tests pass"),
        "the tool's words are not kept: {stop}"
    );

    for (event, name) in [
        ("PostToolUse", "Read"),
        ("PreToolUse", "Bash"),
        ("UserPromptSubmit", ""),
    ] {
        assert!(
            line_of(
                &tool(event, name, json!({ "command": "ls", "file_path": "/x" })),
                app,
                "T",
                rules()
            )
            .is_none(),
            "{event} {name} is not recorded"
        );
    }
}

#[test]
fn a_key_in_a_command_is_cut_and_a_long_command_cut_short() {
    let key = ["sk", "ant", "api03", "Hp4Tz8Wq2Lr6Vn1Kd5Yb0Xm3"].join("-");
    let line = line_of(
        &tool(
            "PostToolUse",
            "Bash",
            json!({ "command": format!("curl -H 'x-api-key: {key}' https://x.test") }),
        ),
        Path::new(APP),
        "T",
        rules(),
    )
    .unwrap();
    let kept = line["command"].as_str().unwrap();
    assert!(!kept.contains(&key[8..]), "{kept}");
    assert!(kept.contains("sk-a"), "{kept}");
    let long = line_of(
        &tool(
            "PostToolUse",
            "Bash",
            json!({ "command": "x".repeat(1000) }),
        ),
        Path::new(APP),
        "T",
        rules(),
    )
    .unwrap();
    assert!(long["command"].as_str().unwrap().chars().count() < 400);
}

#[test]
fn test_runners_are_told_by_their_own_words() {
    for yes in [
        "pytest -q",
        "python -m pytest",
        "npm test",
        "npm run test -- --watch=false",
        "cargo test -p x",
        "go test ./...",
        "./node_modules/.bin/jest",
        "python manage.py test",
    ] {
        assert!(runs_tests(yes), "{yes}");
    }
    for no in [
        "npm install",
        "cat tests/test_app.py",
        "pip install pytest-cov",
        "echo contest",
        "git commit -m 'test'",
    ] {
        assert!(!runs_tests(no), "{no}");
    }
}

#[test]
fn a_report_counts_what_the_record_holds_and_whether_its_chain_holds() {
    let dir = scratch("read");
    let record = dir.join("r.jsonl");
    let app = Path::new(APP);
    for (event, name, input) in [
        (
            "PostToolUse",
            "Write",
            json!({ "file_path": "/home/someone/app/a.py" }),
        ),
        ("PostToolUseFailure", "Bash", json!({ "command": "pytest" })),
        (
            "PostToolUse",
            "Edit",
            json!({ "file_path": "/home/someone/app/a.py" }),
        ),
        ("PostToolUse", "Bash", json!({ "command": "ls" })),
    ] {
        append(
            &record,
            line_of(
                &tool(event, name, input),
                app,
                "2026-10-15T10:00:00Z",
                rules(),
            )
            .unwrap(),
        )
        .unwrap();
    }
    append(
        &record,
        json!({ "at": "2026-10-15T10:05:00Z", "event": "Stop" }),
    )
    .unwrap();
    let seen = read(&record, None);
    assert!(seen.not_read.is_none(), "{seen:?}");
    assert_eq!((seen.commands, seen.writes, seen.turns), (2, 2, 1));
    assert_eq!(seen.tests, 1);
    assert!(seen.last_test_failed);
    assert_eq!(seen.written_after_last_test, 1);
    assert_eq!(seen.last.as_deref(), Some("2026-10-15T10:05:00Z"));
    assert_eq!((seen.chained, seen.broken_at), (5, None));
    let tip = seen.tip.clone().expect("the chain's end");
    assert_eq!(tip.lines, 5);

    // Grown since: the earlier end is still on the chain.
    append(&record, json!({ "at": "T", "event": "Stop" })).unwrap();
    assert_eq!(
        read(&record, Some(&tip)).earlier,
        Some(Earlier::Kept { lines: 5 })
    );

    // A line changed: the chain breaks there, and the earlier end is no longer on it.
    let text = std::fs::read_to_string(&record)
        .unwrap()
        .replacen("pytest", "pytest -k fast", 1);
    std::fs::write(&record, text).unwrap();
    let changed = read(&record, Some(&tip));
    assert_eq!(changed.broken_at, Some(2));
    assert_eq!(changed.earlier, Some(Earlier::NotKept { lines: 5 }));

    // Cut from the end, below the earlier end: not on it either.
    let kept: Vec<String> = std::fs::read_to_string(&record)
        .unwrap()
        .lines()
        .take(1)
        .map(str::to_owned)
        .collect();
    std::fs::write(&record, kept.join("\n") + "\n").unwrap();
    let cut = read(
        &record,
        Some(&Tip {
            lines: 5,
            chain: tip.chain.clone(),
        }),
    );
    assert_eq!(cut.earlier, Some(Earlier::NotKept { lines: 5 }));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn no_record_is_said_and_counts_nothing() {
    let dir = scratch("none");
    let seen = read(&dir.join("none.jsonl"), None);
    assert!(
        seen.not_read
            .as_deref()
            .is_some_and(|w| w.contains("no hook record")),
        "{seen:?}"
    );
    assert_eq!(
        (seen.commands, seen.writes, seen.tip.is_none()),
        (0, 0, true)
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[cfg(unix)]
#[test]
fn a_link_where_the_record_goes_is_refused_and_the_record_is_the_person_s_alone() {
    use std::os::unix::fs::PermissionsExt;
    let dir = scratch("link");
    let target = dir.join("elsewhere");
    std::fs::write(&target, "").unwrap();
    std::os::unix::fs::symlink(&target, dir.join("linked.jsonl")).unwrap();
    assert!(append(&dir.join("linked.jsonl"), json!({ "event": "Stop" })).is_err());
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "");
    let record = dir.join("own/r.jsonl");
    append(&record, json!({ "event": "Stop" })).unwrap();
    assert_eq!(
        std::fs::metadata(&record).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(
        std::fs::metadata(dir.join("own"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn hooks_at_once_still_make_one_chain() {
    // Claude Code can run several tools at once, and each runs the hook; the lock keeps two from chaining to one line.
    let dir = scratch("at-once");
    let record = dir.join("r.jsonl");
    let threads: Vec<_> = (0..8)
        .map(|i| {
            let record = record.clone();
            std::thread::spawn(move || {
                for j in 0..25 {
                    append(
                        &record,
                        json!({ "at": "T", "event": "Stop", "n": i * 100 + j }),
                    )
                    .unwrap();
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    let seen = read(&record, None);
    assert_eq!((seen.turns, seen.chained, seen.broken_at), (200, 200, None));
    std::fs::remove_dir_all(&dir).ok();
}
