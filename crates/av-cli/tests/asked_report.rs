//! `av report --hook-record` on the permissions asked for (ADR-159, part 12; backlog 0718), run as a person runs it:
//! what it says of an asking the tool ran after and one it did not, that it never calls one a refusal, and that it
//! says nothing of them without the option.

use std::path::{Path, PathBuf};
use std::process::Command;
use sv_frameworks::paths::Canonical;

fn folder(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("av-asked-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

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

/// Writes the hook record of `app` under `state`, one line for each hook input, as `sv hook-record` makes them.
fn record(state: &Path, app: &Path, inputs: &[serde_json::Value]) {
    let key = av_record::plain::sha256(app.display().to_string().as_bytes());
    let path = state
        .join(sv_frameworks::names::CONFIG_DIR)
        .join("hook-records")
        .join(format!("{}.jsonl", &key[..16]));
    let rules = sv_check::secrets::SecretRules::shared().expect("the rules sv ships");
    for (n, input) in inputs.iter().enumerate() {
        let at = format!("2026-11-22T03:57:{n:02}Z");
        let line = av_record::hook_record::line_of(input, app, &at, rules).expect("a line");
        av_record::hook_record::append(&path, line).expect("the line is written");
    }
}

fn asks(tool: &str, input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({ "hook_event_name": "PermissionRequest", "session_id": "s", "tool_name": tool, "tool_input": input })
}

fn ran(tool: &str, input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({ "hook_event_name": "PostToolUse", "session_id": "s", "tool_name": tool, "tool_input": input })
}

#[test]
fn an_asking_that_did_not_run_is_listed_with_what_came_next_and_is_never_called_a_refusal() {
    let (app, state) = (folder("listed"), folder("listed-state"));
    let delete = serde_json::json!({ "command": "rm -rf data" });
    record(
        &state,
        &app,
        &[
            asks("mcp__stackvet__stackvet_status", serde_json::json!({})),
            ran("mcp__stackvet__stackvet_status", serde_json::json!({})),
            asks("Bash", delete.clone()),
            asks("Bash", delete.clone()),
            ran(
                "Bash",
                serde_json::json!({ "command": "find data -delete" }),
            ),
        ],
    );
    // Not asked for: nothing of it.
    let (_, out, _) = av(&["report", app.to_str().unwrap()], &state);
    assert!(
        !out.contains("rm -rf data") && !out.contains("You were asked for permission"),
        "{out}"
    );

    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--hook-record"], &state);
    assert_eq!(code, 0, "{out}{err}");
    for said in [
        "You were asked for permission 3 times; the tool then ran after 2, and did not after 1.",
        "you said no, or the prompt was closed without an answer, or the session ended with it open",
        "asked, and did not run: Bash, to be given rm -rf data",
        "Next in that session: asked for Bash: rm -rf data",
        "you were asked to let Bash run and it did not run; the same call was asked for again once",
    ] {
        assert!(out.contains(said), "missing {said:?} in:\n{out}");
    }
    // The run of another command answers the newer asking, so one asking is left, and what the tool did instead is
    // there to read. Nothing here says the person refused: no hook says so.
    for line in out.lines().filter(|l| l.contains("rm -rf data")) {
        assert!(!line.contains("refus"), "a line calls it a refusal: {line}");
    }
    let (_, out, _) = av(
        &["report", app.to_str().unwrap(), "--hook-record", "--json"],
        &state,
    );
    let report: serde_json::Value = serde_json::from_str(&out).expect("JSON");
    assert_eq!(report["asked"]["asked"], 3, "{report}");
    assert_eq!(report["asked"]["not_run"][0]["asked_again"], 1, "{report}");
}

#[test]
fn a_hook_record_with_no_asking_says_the_hook_may_not_write_them_down() {
    let (app, state) = (folder("none"), folder("none-state"));
    record(
        &state,
        &app,
        &[ran("Bash", serde_json::json!({ "command": "ls" }))],
    );
    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--hook-record"], &state);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains("Permissions asked for: none written down.")
            && out.contains("print it again with `sv connect claude --hook`"),
        "{out}"
    );
}

#[test]
fn an_asking_with_no_summary_is_listed_by_its_tool_alone() {
    let (app, state) = (folder("bare"), folder("bare-state"));
    record(
        &state,
        &app,
        &[
            asks(
                "ExitPlanMode",
                serde_json::json!({ "plan": "the tool's own words" }),
            ),
            asks(
                "ExitPlanMode",
                serde_json::json!({ "plan": "the tool's own words" }),
            ),
        ],
    );
    let (code, out, err) = av(&["report", app.to_str().unwrap(), "--hook-record"], &state);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains("asked, and did not run: ExitPlanMode (2026-11-22T03:57:00Z)"),
        "{out}"
    );
    assert!(
        !out.contains("to be given  (") && !out.contains("It was to be given: ("),
        "{out}"
    );
    assert!(!out.contains("the tool's own words"), "{out}");
}
