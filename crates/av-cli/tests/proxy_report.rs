//! `av report --proxy-record` (ADR-159; backlog 0716, part 2), run as a person runs it: the pass-through's record is
//! read only when asked for, what it says of the calls, a call turned down and made again, a tool the task file does
//! not name, and `--help` for a command.

use av_record::passthrough::{Answer, Call, append, line_of};
use std::path::{Path, PathBuf};
use std::process::Command;
use sv_frameworks::paths::Canonical;

fn folder(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("av-proxy-report-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

/// Runs `av` with `state` as the folder its records are kept under, so a test never reads this computer's.
fn av(args: &[&str], state: &Path) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_av"))
        .args(args)
        .env("XDG_STATE_HOME", state)
        .env("XDG_CONFIG_HOME", state)
        .env_remove("SV_TRUSTED_SEALS")
        .output()
        .expect("av runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Where `av`, told its records are under `state`, keeps the pass-through's record of `app`.
fn record_in(state: &Path, app: &Path) -> PathBuf {
    let key = av_record::plain::sha256(app.display().to_string().as_bytes());
    state
        .join(sv_frameworks::names::CONFIG_DIR)
        .join("proxy-records")
        .join(format!("{}.jsonl", &key[..16]))
}

fn write(path: &Path, calls: &[(&str, &str, Option<Answer>)]) {
    for (n, (tool, arguments, answer)) in calls.iter().enumerate() {
        let call = Call {
            tool: (*tool).to_owned(),
            args_sha256: av_record::plain::sha256(arguments.as_bytes()),
            args_bytes: arguments.len(),
            summary: (*arguments).to_owned(),
        };
        let at = format!("2026-11-22T00:00:{n:02}Z");
        append(path, line_of(&at, &call, *answer)).expect("the line is written");
    }
}

const REFUSED: Option<Answer> = Some(Answer::Refused { code: -32602 });

fn a_record(state: &Path, app: &Path) {
    write(
        &record_in(state, app),
        &[
            ("stackvet_check", r#"{"section":"all"}"#, Some(Answer::Ok)),
            ("delete_file", r#"{"path":"data"}"#, REFUSED),
            ("delete_file", r#"{"path":"data"}"#, Some(Answer::ToolError)),
        ],
    );
}

#[test]
fn the_pass_through_s_record_is_read_only_when_asked_for() {
    let (app, state) = (folder("consent"), folder("consent-state"));
    a_record(&state, &app);
    let (code, out, err) = av(&["report", app.to_str().unwrap()], &state);
    // Not asked for: nothing of it is said, and with nothing else here nothing was said about the tool.
    assert_eq!(code, 2, "{out}{err}");
    assert!(
        out.contains("Not read: it is read only when you ask, with --proxy-record."),
        "{out}"
    );
    for held_back in [
        "delete_file",
        "stackvet_check: 1 call",
        "3 calls written down",
    ] {
        assert!(
            !out.contains(held_back),
            "{held_back} was said unasked: {out}"
        );
    }
    assert!(
        out.contains(
            "the pass-through's record: not read, so not set against a checkpoint; it is read only when you ask, \
             with --proxy-record"
        ),
        "{out}"
    );

    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--proxy-record"], &state);
    assert_eq!(code, 0, "{out}{err}");
    for said in [
        "The tool's calls to other tools (the pass-through's record)",
        "How far to rely on it: Kept by av proxy, outside the folder",
        "3 calls written down, from 2026-11-22T00:00:00Z to 2026-11-22T00:00:02Z: 1 answered, 2 turned down by the \
         server.",
        "so av does not tell them apart",
        "Every line's chain checks out.",
        "delete_file: 2 calls, 2 turned down",
        "stackvet_check: 1 call",
        "Not held to the task file",
        "1 call the server turned down was made again with the same arguments",
        "Trying again after a no: 1 line",
        "delete_file was turned down by the server (refused) and called again with the same arguments once in the \
         same session; the last was answered tool-error.",
        "before av proxy marked its starts (read as one session; it may be several): 3 calls",
        "the pass-through's record: no checkpoint.",
        // What no record can show is said whether or not the record was read.
        "A no you gave at your AI coding tool's own permission prompt",
    ] {
        assert!(out.contains(said), "missing {said:?} in:\n{out}");
    }
}

#[test]
fn a_tool_the_task_file_does_not_name_is_listed_as_outside_it() {
    let (app, state) = (folder("outside"), folder("outside-state"));
    a_record(&state, &app);
    std::fs::write(
        app.join("agentvet.toml"),
        "envelope-version = 1\n[task]\ngoal = \"Tidy.\"\n[may]\ncall = [\"stackvet_check\"]\n",
    )
    .unwrap();
    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--proxy-record"], &state);
    assert_eq!(code, 0, "{out}{err}");
    for said in [
        "It may call through av proxy: stackvet_check.",
        "Called, and not named by [may] call: delete_file.",
    ] {
        assert!(out.contains(said), "missing {said:?} in:\n{out}");
    }
    assert!(
        !out.contains("Which tools may the tool call through av proxy?"),
        "what the file says is not asked again: {out}"
    );
    let (_, out, _) = av(
        &["report", app.to_str().unwrap(), "--proxy-record", "--json"],
        &state,
    );
    let report: serde_json::Value = serde_json::from_str(&out).expect("JSON");
    assert_eq!(report["proxy_record"]["record"]["calls"], 3, "{report}");
    assert_eq!(
        report["proxy_record"]["record"]["outside"][0]["name"], "delete_file",
        "{report}"
    );
    // The same call is a line of the group "outside the task", which the page shows once, in the section above.
    let groups = report["groups"].to_string();
    assert!(
        groups.contains("delete_file was called 2 times, and [may] call does not name it"),
        "{groups}"
    );
    assert!(
        report["proxy_record"]["witness"]
            .as_str()
            .is_some_and(|w| w.contains("never reached the server")),
        "{report}"
    );
}

#[test]
fn a_record_with_no_file_is_said_not_to_be_there_and_is_not_an_empty_one() {
    let (app, state) = (folder("absent"), folder("absent-state"));
    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--proxy-record"], &state);
    assert_eq!(code, 2, "{out}{err}");
    assert!(
        out.contains("Not read: there is no pass-through record for this app on this computer"),
        "{out}"
    );
    assert!(
        !out.contains("How far to rely on it: Kept by av proxy"),
        "{out}"
    );
}

#[test]
fn a_tool_s_name_with_an_escape_character_does_not_reach_the_terminal() {
    let (app, state) = (folder("escape"), folder("escape-state"));
    write(
        &record_in(&state, &app),
        &[("check\u{1b}[2Jgone", "{}", Some(Answer::Ok))],
    );
    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--proxy-record"], &state);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains("gone: 1 call"),
        "the setup's tool is listed: {out}"
    );
    assert!(!out.contains('\u{1b}'), "an escape character was printed");
}

#[test]
fn help_asked_of_a_command_prints_the_help_and_succeeds() {
    let state = folder("help-state");
    for args in [
        &["proxy", "--help"][..],
        &["proxy", "-h"],
        &["report", "--help"],
        &["checkpoint", "--help"],
        &["--help"],
    ] {
        let (code, out, err) = av(args, &state);
        assert_eq!(code, 0, "{args:?}: {out}{err}");
        assert!(
            out.contains("av proxy [--app <folder>] --"),
            "{args:?}: {out}"
        );
        assert_eq!(err, "", "{args:?}");
    }
    // After `--`, a `--help` is the tool server's own, and av's help is not printed for it.
    let (code, out, _) = av(&["proxy", "--", "/nowhere/at/all/server", "--help"], &state);
    assert_ne!(code, 0);
    assert!(!out.contains("av proxy [--app <folder>] --"), "{out}");
}

#[test]
fn two_sessions_are_counted_apart_and_a_no_from_the_first_is_not_a_retry_in_the_second() {
    let (app, state) = (folder("sessions"), folder("sessions-state"));
    let path = record_in(&state, &app);
    let no = Some(Answer::ToolError);
    append(
        &path,
        av_record::passthrough::started_line("2026-11-22T00:33:00Z"),
    )
    .unwrap();
    write(&path, &[("stackvet_check", r#"{"path":"/home"}"#, no)]);
    append(
        &path,
        av_record::passthrough::started_line("2026-11-22T02:04:00Z"),
    )
    .unwrap();
    write(
        &path,
        &[
            ("stackvet_check", "{}", Some(Answer::Ok)),
            ("stackvet_check", r#"{"path":"/home"}"#, no),
        ],
    );
    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--proxy-record"], &state);
    assert_eq!(code, 0, "{out}{err}");
    for said in [
        "3 calls written down",
        "In 2 sessions, each one start of av proxy:",
        "started 2026-11-22T00:33:00Z: 1 call, 0 answered, 1 turned down",
        "started 2026-11-22T02:04:00Z: 2 calls, 1 answered, 1 turned down",
        "the same call was made once in a later session, which may not have known of the no",
    ] {
        assert!(out.contains(said), "missing {said:?} in:\n{out}");
    }
    assert!(
        !out.contains("in the same session;"),
        "a later session's call was said to be made again after the no: {out}"
    );
}

/// `av --version` names the commit it was built from (backlog 0724, part 2): a person reporting a fault needs it,
/// and the owner's second trial had to work it out by hand.
#[test]
fn the_version_names_the_commit_av_was_built_from() {
    let state = folder("version-state");
    let (code, out, err) = av(&["--version"], &state);
    assert_eq!(code, 0, "{err}");
    let line = out.trim();
    let commit = line
        .strip_prefix(&format!("av {} (commit ", env!("CARGO_PKG_VERSION")))
        .and_then(|rest| rest.strip_suffix(')'))
        .unwrap_or_else(|| panic!("not a version and a commit: {line}"));
    // Built in a checkout, as these tests are, it is a commit's hash; built from an archive with none given, it
    // says so and does not guess.
    assert!(
        commit == "unknown" || (commit.len() >= 7 && commit.chars().all(|c| c.is_ascii_hexdigit())),
        "{commit}"
    );
    // And the same one is in a report's JSON.
    let app = folder("version");
    let (_, out, _) = av(&["report", app.to_str().unwrap(), "--json"], &state);
    let report: serde_json::Value = serde_json::from_str(&out).expect("JSON");
    assert_eq!(report["commit"], commit, "{report}");
}
