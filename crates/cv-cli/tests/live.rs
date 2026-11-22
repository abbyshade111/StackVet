//! `cv live github` end to end (backlog 0717; ADR-157, decision 5): the real `cv`, with a stand-in for `curl`
//! (`CV_CURL_FOR_TEST`, honored by a debug build only) that answers from canned settings and records what it was given.
//! The token must reach it on standard input and never in its arguments, and must appear nowhere in what `cv` prints or
//! writes but as its first characters and its length.
//!
//! The stand-in is a Python script, as `sv`'s fetch tests use, so these run on Unix only.

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

const FAKE_CURL: &str = r#"#!/usr/bin/env python3
import json, os, sys
args = sys.argv[1:]
log = os.environ["FAKE_LOG"]
url = args[-1]
headers_file = args[args.index("--dump-header") + 1]
body_file = args[args.index("--output") + 1]
stdin = sys.stdin.read()
want = os.environ["FAKE_TOKEN"]
with open(log, "a") as f:
    f.write(json.dumps({"args": args, "token_in_args": any(want in a for a in args),
                        "bearer_on_stdin": ("Authorization: Bearer " + want) in stdin}) + "\n")
answers = json.load(open(os.environ["FAKE_ANSWERS"]))
path = url[len("https://api.github.com"):]
status, extra, body = answers.get(path, [404, {}, {"message": "Not Found"}])
with open(headers_file, "w") as f:
    f.write("HTTP/2 %d\r\n" % status + "".join("%s: %s\r\n" % kv for kv in extra.items()) + "\r\n")
with open(body_file, "w") as f:
    f.write(json.dumps(body))
sys.stdout.write("%d " % status)
"#;

fn scratch(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cv-live-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn fine() -> String {
    ["github", "_pat_", "11BBBB", &"z".repeat(70)].concat()
}

fn answers(scopes: Option<&str>) -> serde_json::Value {
    let r = "/repos/o/r";
    let mut repo_headers = serde_json::json!({"x-accepted-github-permissions": "metadata=read"});
    if let Some(scopes) = scopes {
        repo_headers["x-oauth-scopes"] = scopes.into();
    }
    serde_json::json!({
        r: [200, repo_headers, {"default_branch": "main", "security_and_analysis": {"secret_scanning": {"status": "enabled"}, "secret_scanning_push_protection": {"status": "enabled"}}}],
        format!("{r}/rulesets?includes_parents=true&per_page=30"): [200, {}, [{"id": 1}]],
        format!("{r}/rulesets/1"): [200, {}, {"name": "Main", "target": "branch", "enforcement": "active", "rules": [{"type": "pull_request"}], "bypass_actors": []}],
        format!("{r}/rules/branches/main"): [200, {}, [{"type": "pull_request"}]],
        format!("{r}/actions/permissions"): [200, {}, {"enabled": true, "allowed_actions": "all", "sha_pinning_required": false}],
        format!("{r}/actions/permissions/workflow"): [200, {}, {"default_workflow_permissions": "read", "can_approve_pull_request_reviews": false}],
        format!("{r}/actions/permissions/fork-pr-contributor-approval"): [200, {}, {"approval_policy": "first_time_contributors"}],
        format!("{r}/environments?per_page=30"): [200, {}, {"environments": []}],
    })
}

struct Ran {
    code: i32,
    said: String,
    report: serde_json::Value,
    calls: Vec<serde_json::Value>,
}

fn cv(dir: &Path, token: Option<&str>, args: &[&str], canned: &serde_json::Value) -> Ran {
    let curl = dir.join("curl");
    std::fs::write(&curl, FAKE_CURL).unwrap();
    std::fs::set_permissions(&curl, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    std::fs::write(dir.join("answers.json"), canned.to_string()).unwrap();
    let log = dir.join("calls.log");
    let out_dir = dir.join("report");
    let mut command = Command::new(env!("CARGO_BIN_EXE_cv"));
    command
        .args(args)
        .arg("--out")
        .arg(&out_dir)
        .env("CV_CURL_FOR_TEST", &curl)
        .env("FAKE_LOG", &log)
        .env("FAKE_ANSWERS", dir.join("answers.json"))
        .env("FAKE_TOKEN", token.unwrap_or("none"))
        .env_remove("GH_TOKEN")
        .env_remove("GITHUB_TOKEN");
    if let Some(token) = token {
        command.env("GH_TOKEN", token);
    }
    let out = command.output().unwrap();
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let report = std::fs::read_to_string(out_dir.join("report.json"))
        .ok()
        .map(|t| serde_json::from_str(&t).unwrap())
        .unwrap_or_default();
    let calls = std::fs::read_to_string(&log)
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    Ran {
        code: out.status.code().unwrap(),
        said,
        report,
        calls,
    }
}

#[test]
fn a_whole_reading_sends_the_token_on_stdin_only_and_lists_every_request() {
    let dir = scratch("whole");
    let token = fine();
    let ran = cv(
        &dir,
        Some(&token),
        &["live", "github", "o/r"],
        &answers(None),
    );
    assert_eq!(ran.code, 0, "{}", ran.said);
    assert!(
        !ran.calls.is_empty(),
        "the setup: the stand-in curl was run: {}",
        ran.said
    );
    for call in &ran.calls {
        assert_eq!(
            call["token_in_args"], false,
            "the token was on curl's command line: {call}"
        );
        assert_eq!(
            call["bearer_on_stdin"], true,
            "the token did not reach curl's input: {call}"
        );
        let args: Vec<&str> = call["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a.as_str().unwrap())
            .collect();
        assert!(
            args.last()
                .unwrap()
                .starts_with("https://api.github.com/repos/o/r"),
            "{args:?}"
        );
    }
    let requests = ran.report["live"]["reading"]["requests"]
        .as_array()
        .unwrap();
    assert_eq!(
        requests.len(),
        ran.calls.len(),
        "every request sent is in the report"
    );
    assert!(
        requests
            .iter()
            .all(|r| r["method"] == "GET" && r["host"] == "api.github.com")
    );
    // The token, before any finding, and never whole.
    let token_line = ran.said.find("Token: ").expect("the token is spoken of");
    let first_finding = ran.said.find("finding ").expect("a finding is listed");
    assert!(token_line < first_finding, "{}", ran.said);
    assert!(
        ran.said
            .contains("could not confirm the token is read-only"),
        "{}",
        ran.said
    );
    let written = ran.report.to_string();
    assert!(
        written.contains("cv.github-actions-from-anywhere"),
        "the setup: the report was written: {written}"
    );
    for text in [&ran.said, &written] {
        assert!(!text.contains(&token), "the token was printed or written");
        assert!(
            !text.contains(&token[..16]),
            "a long piece of the token was printed or written"
        );
    }
}

#[test]
fn a_classic_token_that_can_write_is_refused_and_says_why() {
    let dir = scratch("classic");
    let token = ["gh", "o_", &"q".repeat(36)].concat();
    let ran = cv(
        &dir,
        Some(&token),
        &["live", "github", "o/r"],
        &answers(Some("gist, read:org, repo, workflow")),
    );
    assert_eq!(
        ran.calls.len(),
        1,
        "one request, then nothing: {:?}",
        ran.calls
    );
    assert_eq!(ran.code, 2, "{}", ran.said);
    assert!(
        ran.said.contains("can write (gist, repo, workflow)"),
        "{}",
        ran.said
    );
    assert_eq!(
        ran.report["live"]["reading"]["token"]["read_only"]["is"],
        "refused"
    );
}

#[test]
fn no_token_reads_nothing() {
    let dir = scratch("none");
    let ran = cv(&dir, None, &["live", "github", "o/r"], &answers(None));
    assert_eq!(ran.code, 3, "{}", ran.said);
    assert!(ran.calls.is_empty(), "curl was run with no token");
    assert!(ran.said.contains("no token in GH_TOKEN"), "{}", ran.said);
}

#[test]
fn a_token_is_never_taken_as_an_option() {
    let dir = scratch("option");
    let ran = cv(
        &dir,
        None,
        &["live", "github", "o/r", "--token", "abc"],
        &answers(None),
    );
    assert_eq!(ran.code, 3);
    assert!(ran.calls.is_empty());
    assert!(
        ran.said.contains("never takes a token as an option"),
        "{}",
        ran.said
    );
}

#[test]
fn cv_check_never_reads_a_live_account() {
    // cv check is files only: given a token in the environment, it still asks nothing (ADR-157, decision 5).
    let dir = scratch("check");
    std::fs::write(dir.join("Dockerfile"), "FROM node:24-alpine\nUSER node\n").unwrap();
    let app = dir.clone();
    let token = fine();
    let ran = cv(
        &dir,
        Some(&token),
        &["check", app.to_str().unwrap(), "--no-tools"],
        &answers(None),
    );
    assert!(ran.calls.is_empty(), "cv check ran curl: {:?}", ran.calls);
    assert!(
        ran.said.contains("Read 1 deployment files"),
        "the setup: cv check ran: {}",
        ran.said
    );
}
