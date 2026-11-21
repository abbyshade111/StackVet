//! `av report` (ADR-159; backlog 0705, part 2), run as a person runs it: what it says of each record, that it reads
//! the hook record only when asked, that nothing of the tool's reaches the terminal as it was written, and its exit
//! codes.

use std::path::{Path, PathBuf};
use std::process::Command;
use sv_frameworks::paths::Canonical;

fn folder(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("av-report-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

/// Runs `av` with `state` as the folder `sv` keeps its own records under, so a test never reads this computer's.
fn av(args: &[&str], state: &Path) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_av"))
        .args(args)
        .env("XDG_STATE_HOME", state)
        .output()
        .expect("av runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn git(dir: &Path, args: &[&str]) {
    let ok = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args([
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .is_ok_and(|o| o.status.success());
    assert!(ok, "git {args:?} failed in the test's setup");
}

fn no_git() -> bool {
    if Command::new("git").arg("--version").output().is_err() {
        println!("no git here; this needs it");
        return true;
    }
    false
}

/// A checkout with one commit a person made and one whose message names an AI coding tool, which adds `file`.
fn with_a_tool_commit(app: &Path, file: &str, text: &str) {
    git(app, &["init", "-q", "-b", "main"]);
    std::fs::write(app.join("app.py"), "print(1)\n").unwrap();
    git(app, &["add", "-A"]);
    git(app, &["commit", "-qm", "by hand"]);
    std::fs::write(app.join(file), text).unwrap();
    git(app, &["add", "-A"]);
    git(
        app,
        &[
            "commit",
            "-qm",
            "more\n\nCo-Authored-By: Claude <noreply@anthropic.com>",
        ],
    );
}

/// Where the hook record of `app` is kept when `sv`'s own records are under `state`, as `av` is told they are.
fn record_in(state: &Path, app: &Path) -> PathBuf {
    static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _held = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let key = "XDG_STATE_HOME";
    let before = std::env::var_os(key);
    // SAFETY: the variable is set and put back under a lock every caller here takes, and nothing else in this test
    // binary reads it: `av` is given its own as a child's environment.
    unsafe { std::env::set_var(key, state) };
    let path = av_record::hook_record::record_of(app).expect("a place for the record");
    match before {
        Some(v) => unsafe { std::env::set_var(key, v) },
        None => unsafe { std::env::remove_var(key) },
    }
    path
}

/// One line of the build-loop record: a check with `findings` findings.
fn check(time: &str, findings: usize) -> av_record::build_loop::Line {
    av_record::build_loop::Line {
        time: time.to_owned(),
        tool: "stackvet_check".to_owned(),
        counts: Some(sv_report::LoopCounts {
            findings,
            checked: 5,
            needs_attention: findings,
            not_assessed: 1,
        }),
        outcome: Some("ok".to_owned()),
        ..Default::default()
    }
}

#[test]
fn a_folder_with_nothing_about_a_tool_exits_2_and_says_what_was_not_looked_at() {
    let app = folder("nothing");
    let state = folder("nothing-state");
    let (code, out, err) = av(&["report", app.to_str().unwrap()], &state);
    assert_eq!(code, 2, "{out}{err}");
    assert!(
        out.contains("Nothing here is about the tool")
            && out.contains("That is not a sign that no tool worked here."),
        "{out}"
    );
    // What it cannot see is said every time, never left as silence.
    for gap in [
        "av reads no transcript yet",
        "there is no task file (agentvet.toml) to hold it to",
        "There is none in this folder",
        "read only when you ask, with --hook-record",
        "not a git checkout",
    ] {
        assert!(out.contains(gap), "missing {gap:?}: {out}");
    }
    assert!(
        !out.to_lowercase().contains("trust"),
        "it never says a tool can be trusted: {out}"
    );
}

#[test]
fn the_build_loop_record_and_a_tool_s_commit_are_each_said_with_how_far_to_rely_on_them() {
    if no_git() {
        return;
    }
    let app = folder("records");
    let state = folder("records-state");
    with_a_tool_commit(&app, "more.py", "print(2)\n");
    av_record::build_loop::record(&app, &check("2026-11-21T10:00:00Z", 4)).unwrap();
    av_record::build_loop::record(&app, &check("2026-11-21T10:30:00Z", 1)).unwrap();
    // The setup: the record is there, and reads as two checks.
    let seen = av_record::build_loop::read(&app);
    assert_eq!((seen.calls, seen.checks), (2, 2), "the setup: {seen:?}");

    let (code, out, err) = av(&["report", app.to_str().unwrap()], &state);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("The tool's calls to sv"), "{out}");
    assert!(
        out.contains("could rewrite it") && out.contains("history can be rewritten"),
        "each record says how far to rely on it: {out}"
    );
    assert!(
        out.contains("1 was made by an AI coding tool"),
        "the commit that names a tool is counted, and the other is not: {out}"
    );
    assert!(
        out.contains("Not read: it is read only when you ask, with --hook-record."),
        "{out}"
    );
    // With no fingerprints kept, the record cannot say which findings went, and says so rather than guessing.
    assert!(
        out.contains("Findings that went away between the tool's checks: not shown."),
        "{out}"
    );
}

#[test]
fn the_hook_record_is_read_only_when_asked_for() {
    let app = folder("hook");
    let state = folder("hook-state");
    let record = record_in(&state, &app);
    assert!(record.starts_with(&state), "the setup: {record:?}");
    let rules = sv_check::secrets::SecretRules::shared().expect("the rules sv ships");
    let line = av_record::hook_record::line_of(
        &serde_json::json!({
            "hook_event_name": "PostToolUse", "session_id": "s1", "tool_name": "Bash",
            "tool_input": { "command": "pytest -q" }
        }),
        &app,
        "2026-11-21T10:00:00Z",
        rules,
    )
    .expect("a command is a line");
    av_record::hook_record::append(&record, line).unwrap();

    let (code, out, _) = av(&["report", app.to_str().unwrap()], &state);
    assert_eq!(
        code, 2,
        "without the record, nothing here is about the tool: {out}"
    );
    assert!(
        out.contains("Not read: it is read only when you ask, with --hook-record.")
            && !out.contains("pytest"),
        "not read, and nothing of it shown: {out}"
    );

    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--hook-record"], &state);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains("The tool could remove the hook or rewrite the record"),
        "{out}"
    );
    assert!(
        out.contains("pytest -q"),
        "the last test command is shown: {out}"
    );
}

#[test]
fn a_folder_name_with_an_escape_character_does_not_reach_the_terminal() {
    // The folder's own name is the one piece of text `av` prints that no record cleaned first: the hook record
    // writes an escape character out as it keeps a command. This name carries the sequence that sets a terminal's
    // title.
    let app = folder("escape").join("app\u{1b}]0;owned\u{7}");
    std::fs::create_dir_all(&app).unwrap();
    let state = folder("escape-state");
    let (code, out, err) = av(&["report", app.to_str().unwrap()], &state);
    assert_eq!(code, 2, "{out}{err}");
    assert!(
        out.contains("owned"),
        "the setup: the folder is named in what is printed: {out}"
    );
    assert!(
        !out.contains('\u{1b}') && !out.contains('\u{7}'),
        "an escape character reached the terminal: {out:?}"
    );
    // And on the way to standard error, for a folder that is not there.
    let (code, _, err) = av(&["report", "/nowhere/\u{1b}]0;owned\u{7}"], &state);
    assert_eq!(code, 3);
    assert!(
        err.contains("owned") && !err.contains('\u{1b}') && !err.contains('\u{7}'),
        "{err:?}"
    );
}

#[test]
fn a_look_away_comment_is_listed_whatever_the_file_is_named() {
    if no_git() {
        return;
    }
    // Backlog 0713: git writes a name with a letter outside ASCII in quotes, and one with a space with a tab after
    // it; a tool that named its file so kept what it added there out of this list.
    for (test, name) in [
        ("named-plain", "plain.py"),
        ("named-accent", "caf\u{e9}.py"),
        ("named-space", "my file.py"),
    ] {
        let app = folder(test);
        let state = folder(&format!("{test}-state"));
        with_a_tool_commit(&app, name, "import subprocess  # nosec\n");
        let (code, out, err) = av(&["report", app.to_str().unwrap()], &state);
        assert_eq!(code, 0, "{out}{err}");
        assert!(
            out.contains(&format!(
                "a marker that tells a check to look away (nosec) was added in {name}"
            )),
            "{name}: {out}"
        );
    }
}

#[test]
fn the_task_envelope_is_said_with_what_it_leaves_unsaid_and_that_it_may_be_the_tool_s() {
    let app = folder("envelope");
    let state = folder("envelope-state");
    std::fs::write(
        app.join("agentvet.toml"),
        "envelope-version = 1\n[task]\ngoal = \"Add a sign-out link.\"\n[may]\nwrite = [\"src/\"]\nrun = []\n\
         [guards]\nfiles = [\"deploy/\"]\n",
    )
    .unwrap();
    let (code, out, err) = av(&["report", app.to_str().unwrap()], &state);
    // The envelope is not a record of the tool: with nothing else here, nothing was said about the tool.
    assert_eq!(code, 2, "{out}{err}");
    for said in [
        "The task: Add a sign-out link.",
        "It may write: src/.",
        "It may start: nothing.",
        "it may be the tool's own account of its task",
        "It may not change, whatever the file says: agentvet.toml,",
        "Not said, so not assumed either way: Which hosts may the tool connect to?",
        "Not said, so not assumed either way: Which actions must the tool ask you about first",
        "The file adds to those: deploy/.",
        "Nothing is held to this yet",
        "the task file was read, and nothing is held to it yet",
    ] {
        assert!(out.contains(said), "missing {said:?}: {out}");
    }
    assert!(
        !out.contains("Which folders and files may the tool write?"),
        "what the file says is not asked again: {out}"
    );
    let (_, out, _) = av(&["report", app.to_str().unwrap(), "--json"], &state);
    let report: serde_json::Value = serde_json::from_str(&out).expect("JSON");
    assert_eq!(report["envelope"]["read"], "held", "{report}");
    assert_eq!(report["envelope"]["held_to"], false, "{report}");
    assert_eq!(
        report["envelope"]["questions"].as_array().map(Vec::len),
        Some(2),
        "{report}"
    );
}

#[test]
fn an_envelope_that_cannot_be_read_is_said_and_holds_nothing() {
    let app = folder("envelope-bad");
    let state = folder("envelope-bad-state");
    // A misspelled table: read loosely, it would be an envelope that guards nothing.
    std::fs::write(
        app.join("agentvet.toml"),
        "envelope-version = 1\n[task]\ngoal = \"x\"\n[guard]\nfiles = []\n",
    )
    .unwrap();
    let (code, out, err) = av(&["report", app.to_str().unwrap()], &state);
    assert_eq!(code, 2, "{out}{err}");
    assert!(
        out.contains("Not read: agentvet.toml is not an envelope av can read")
            && out.contains("Nothing is judged against a file that cannot be read."),
        "{out}"
    );
    assert!(
        !out.contains("The task: x"),
        "nothing of it is taken as the task: {out}"
    );
    // A goal that carries an escape character is shown on one line, with the character written out.
    std::fs::write(
        app.join("agentvet.toml"),
        "envelope-version = 1\n[task]\ngoal = \"tidy\\u001b]0;owned\\u0007\\nThe task: delete everything\"\n",
    )
    .unwrap();
    let (_, out, _) = av(&["report", app.to_str().unwrap()], &state);
    assert!(out.contains("owned"), "the setup: the goal is shown: {out}");
    assert!(!out.contains('\u{1b}') && !out.contains('\u{7}'), "{out:?}");
    assert_eq!(
        out.lines()
            .filter(|l| l.trim_start().starts_with("The task: "))
            .count(),
        1,
        "a line break in the goal does not start a line of av's own: {out}"
    );
}

#[test]
fn json_holds_each_record_with_how_far_to_rely_on_it_and_what_was_not_looked_at() {
    let app = folder("json");
    let state = folder("json-state");
    av_record::build_loop::record(&app, &check("2026-11-21T10:00:00Z", 2)).unwrap();
    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--json"], &state);
    assert_eq!(code, 0, "{out}{err}");
    let report: serde_json::Value = serde_json::from_str(&out).expect("JSON, and nothing else");
    assert_eq!(report["build_loop"]["record"]["calls"], 1, "{report}");
    assert!(
        report["build_loop"]["witness"]
            .as_str()
            .is_some_and(|w| w.contains("could rewrite it")),
        "{report}"
    );
    assert!(report["hook_record"].is_null(), "not asked for: {report}");
    assert!(
        report["not_looked_at"]
            .as_array()
            .is_some_and(|gaps| gaps.len() >= 4),
        "{report}"
    );
}

#[test]
fn a_folder_that_is_not_there_or_an_unknown_option_exits_3() {
    let state = folder("failed-state");
    let (code, _, err) = av(&["report", "/nowhere/at/all"], &state);
    assert_eq!(code, 3);
    assert!(err.contains("is not a folder"), "{err}");
    let app = folder("failed");
    let (code, _, err) = av(&["report", app.to_str().unwrap(), "--transcript"], &state);
    assert_eq!(code, 3);
    assert!(err.contains("does not know --transcript"), "{err}");
    let (code, _, _) = av(&[], &state);
    assert_eq!(code, 3);
}

/// `av` opens no network connection and starts no program of its own (ADR-159, decision 10). This holds what `av`'s
/// own source can show: it names nothing that connects or starts a program, and depends only on the crates it is
/// meant to. It says nothing of the crates beneath it, which read git as `sv` does (ADR-032).
#[test]
fn av_s_own_source_names_nothing_that_connects_or_starts_a_program() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = std::fs::read_to_string(root.join("src/main.rs")).unwrap();
    for word in ["std::net", "TcpStream", "UdpSocket", "Command::new", "curl"] {
        assert!(!source.contains(word), "src/main.rs names {word}");
    }
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    let dependencies: Vec<&str> = manifest
        .split("[dependencies]")
        .nth(1)
        .expect("a dependencies table")
        .lines()
        .take_while(|l| !l.starts_with('['))
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .filter_map(|l| l.split(['=', '.']).next())
        .map(str::trim)
        .collect();
    assert_eq!(
        dependencies,
        ["serde_json", "av-record", "sv-check", "sv-report"],
        "a dependency added to av is a decision (ADR-159)"
    );
}
