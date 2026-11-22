//! What a report is told of the pass-through's record (backlog 0716, part 2): the counts, a call turned down and
//! made again, the calls outside `[may] call`, and a record that was changed.

use super::*;
use crate::passthrough::{Answer, Call, append, line_of};

fn record(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("av-proxy-seen-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("record.jsonl")
}

fn call(tool: &str, arguments: &str) -> Call {
    Call {
        tool: tool.to_owned(),
        args_sha256: crate::plain::sha256(arguments.as_bytes()),
        args_bytes: arguments.len(),
        summary: arguments.to_owned(),
    }
}

fn write(path: &Path, calls: &[(&str, &str, Option<Answer>)]) {
    for (n, (tool, arguments, answer)) in calls.iter().enumerate() {
        let at = format!("2026-11-22T00:00:{n:02}Z");
        append(path, line_of(&at, &call(tool, arguments), *answer)).expect("the line is written");
    }
}

const NO: Option<Answer> = Some(Answer::ToolError);
const REFUSED: Option<Answer> = Some(Answer::Refused { code: -32602 });
const OK: Option<Answer> = Some(Answer::Ok);

#[test]
fn a_record_that_is_not_there_is_not_read_and_says_why() {
    let seen = read(&record("absent"), None);
    assert!(
        seen.not_read
            .as_deref()
            .is_some_and(|why| why.contains("there is no pass-through record")),
        "{seen:?}"
    );
    assert_eq!(seen.calls, 0);
}

#[test]
fn the_calls_are_counted_by_tool_and_by_how_each_was_answered() {
    let path = record("counts");
    write(
        &path,
        &[
            ("check", r#"{"section":"all"}"#, OK),
            ("check", r#"{"path":"/elsewhere"}"#, NO),
            ("read_file", r#"{"path":"a"}"#, REFUSED),
            ("read_file", r#"{"path":"b"}"#, None),
        ],
    );
    let seen = read(&path, None);
    assert_eq!(seen.not_read, None);
    assert_eq!(
        (seen.calls, seen.chained, seen.broken_at),
        (4, 4, None),
        "{seen:?}"
    );
    assert_eq!(
        (seen.ok, seen.tool_error, seen.refused, seen.unanswered),
        (1, 1, 1, 1)
    );
    // An error of the protocol and a result marked as failed are one count: a server may say no either way.
    assert_eq!(seen.turned_down(), 2);
    assert_eq!(
        seen.tools,
        vec![
            Tool {
                name: "check".to_owned(),
                calls: 2,
                turned_down: 1
            },
            Tool {
                name: "read_file".to_owned(),
                calls: 2,
                turned_down: 1
            },
        ]
    );
    assert_eq!(seen.first.as_deref(), Some("2026-11-22T00:00:00Z"));
    assert_eq!(seen.last.as_deref(), Some("2026-11-22T00:00:03Z"));
    // No task file said which tools may be called: nothing is called outside it, and nothing inside.
    assert_eq!(seen.outside, None);
    assert_eq!(seen.again, Vec::<Again>::new());
}

#[test]
fn a_call_turned_down_and_made_again_with_the_same_arguments_is_listed_and_no_other_is() {
    let path = record("again");
    write(
        &path,
        &[
            // Turned down, then made twice more: listed, with how the last was answered.
            ("delete", r#"{"path":"data"}"#, REFUSED),
            ("delete", r#"{"path":"data"}"#, REFUSED),
            ("delete", r#"{"path":"data"}"#, OK),
            // Turned down once and left: not listed.
            ("push", r#"{"to":"main"}"#, NO),
            // Turned down, then made with other arguments: a different call, not listed.
            ("check", r#"{"path":"/elsewhere"}"#, NO),
            ("check", r#"{"path":"."}"#, OK),
            // Answered, then made again and turned down: the no came last, so nothing was tried after it.
            ("read_file", r#"{"path":"a"}"#, OK),
            ("read_file", r#"{"path":"a"}"#, NO),
            // The same arguments to another tool are another call.
            ("remove", r#"{"path":"data"}"#, OK),
        ],
    );
    let seen = read(&path, None);
    assert_eq!(
        seen.again,
        vec![Again {
            tool: "delete".to_owned(),
            summary: r#"{"path":"data"}"#.to_owned(),
            at: "2026-11-22T00:00:00Z".to_owned(),
            answer: "refused".to_owned(),
            run: 1,
            times: 2,
            later: 0,
            last_answer: "ok".to_owned(),
        }],
        "{seen:?}"
    );
    assert_eq!(seen.again_more, 0);
}

#[test]
fn a_tool_called_that_the_task_file_does_not_name_is_outside_it() {
    let path = record("outside");
    write(
        &path,
        &[
            ("check", "{}", OK),
            ("delete", "{}", OK),
            ("delete", "{}", OK),
        ],
    );
    let allowed = ["check".to_owned()];
    let seen = read(&path, Some(&allowed));
    assert_eq!(
        seen.outside,
        Some(vec![Tool {
            name: "delete".to_owned(),
            calls: 2,
            turned_down: 0
        }])
    );
    // An empty list is an answer: no tool may be called, so each one called is outside it.
    let seen = read(&path, Some(&[]));
    assert_eq!(seen.outside.as_ref().map(Vec::len), Some(2));
    // Names are compared as written.
    let seen = read(&path, Some(&["Check".to_owned(), "delete".to_owned()]));
    assert_eq!(
        seen.outside
            .unwrap()
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<_>>(),
        ["check"]
    );
}

#[test]
fn a_line_changed_after_it_was_written_shows_and_the_calls_are_still_counted() {
    let path = record("changed");
    write(
        &path,
        &[
            ("delete", "{}", REFUSED),
            ("check", "{}", OK),
            ("check", "{}", OK),
        ],
    );
    assert_eq!(read(&path, None).broken_at, None, "the setup's chain holds");
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(
        text.contains(r#""answer":"refused""#),
        "the setup wrote a no"
    );
    // The no is made to read as a yes, as a tool hiding it would.
    std::fs::write(
        &path,
        text.replace(r#""answer":"refused""#, r#""answer":"ok""#),
    )
    .unwrap();
    let seen = read(&path, None);
    assert_eq!(seen.broken_at, Some(1), "{seen:?}");
    assert_eq!(seen.chained, 0);
    assert_eq!(seen.calls, 3);
}

#[test]
fn a_name_in_a_record_that_was_rewritten_is_taken_no_longer_than_the_pass_through_writes_one() {
    let path = record("long");
    let long = "x".repeat(5000);
    std::fs::write(
        &path,
        format!(
            "{}\n",
            serde_json::json!({ "at": "t", "tool": long, "answer": "ok" })
        ),
    )
    .unwrap();
    let seen = read(&path, None);
    assert_eq!(seen.calls, 1);
    assert!(
        seen.tools[0].name.chars().count() < 200,
        "{}",
        seen.tools[0].name.len()
    );
}

/// A record of two runs of `av proxy`: each begins with the line that says it started.
fn two_runs(
    path: &Path,
    first: &[(&str, &str, Option<Answer>)],
    second: &[(&str, &str, Option<Answer>)],
) {
    append(
        path,
        crate::passthrough::started_line("2026-11-22T00:00:00Z"),
    )
    .unwrap();
    write(path, first);
    append(
        path,
        crate::passthrough::started_line("2026-11-22T02:00:00Z"),
    )
    .unwrap();
    write(path, second);
}

#[test]
fn a_call_turned_down_in_one_session_and_made_in_a_later_one_is_not_said_to_be_made_again_in_it() {
    // What the owner's second trial showed: a no in one session, the same call 90 minutes later in another.
    let path = record("later");
    two_runs(
        &path,
        &[("check", r#"{"path":"/home"}"#, NO)],
        &[("check", "{}", OK), ("check", r#"{"path":"/home"}"#, NO)],
    );
    let seen = read(&path, None);
    assert_eq!(
        seen.broken_at, None,
        "a start line is chained as any line is"
    );
    assert_eq!(seen.calls, 3, "a start line is not a call");
    assert_eq!(
        seen.runs,
        vec![
            Run {
                started: Some("2026-11-22T00:00:00Z".to_owned()),
                calls: 1,
                ok: 0,
                turned_down: 1,
                unanswered: 0
            },
            Run {
                started: Some("2026-11-22T02:00:00Z".to_owned()),
                calls: 2,
                ok: 1,
                turned_down: 1,
                unanswered: 0
            },
        ]
    );
    assert_eq!(seen.again.len(), 1, "{seen:?}");
    let again = &seen.again[0];
    assert_eq!(
        (again.run, again.times, again.later),
        (1, 0, 1),
        "{again:?}"
    );
}

#[test]
fn a_call_made_again_within_a_session_is_counted_there_whatever_an_earlier_session_did() {
    let path = record("within");
    two_runs(
        &path,
        &[("delete", "{}", REFUSED)],
        &[("delete", "{}", REFUSED), ("delete", "{}", OK)],
    );
    let seen = read(&path, None);
    let by_run: Vec<(usize, usize, usize)> = seen
        .again
        .iter()
        .map(|a| (a.run, a.times, a.later))
        .collect();
    // The first session's no came back twice later; the second session's own no was made again once in it.
    assert_eq!(by_run, [(1, 0, 2), (2, 1, 0)], "{seen:?}");
}

#[test]
fn calls_from_before_av_proxy_marked_its_starts_are_one_run_with_no_start() {
    let path = record("unmarked");
    write(&path, &[("check", "{}", NO), ("check", "{}", NO)]);
    let seen = read(&path, None);
    assert_eq!(seen.runs.len(), 1);
    assert_eq!(seen.runs[0].started, None);
    // Nothing tells its sessions apart, so the two are read as one session's.
    assert_eq!((seen.again[0].times, seen.again[0].later), (1, 0));
}
