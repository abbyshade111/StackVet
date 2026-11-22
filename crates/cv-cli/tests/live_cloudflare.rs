//! `cv live cloudflare` end to end (backlog 0732; ADR-157, decision 5, and its Later entry of 22 November 2026): the real
//! `cv`, with a stand-in for `curl` (`CV_CURL_FOR_TEST`, honored by a debug build only) that answers from canned settings
//! and records what it was given. As for GitHub (`live.rs`): the token reaches it on standard input and never in its
//! arguments, and appears nowhere in what `cv` prints or writes but as its first characters and its length.
//!
//! The stand-in is a Python script, so these run on Unix only.

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

const BASE: &str = "https://api.cloudflare.com/client/v4";

const FAKE_CURL: &str = r#"#!/usr/bin/env python3
import json, os, sys
args = sys.argv[1:]
url = args[-1]
headers_file = args[args.index("--dump-header") + 1]
body_file = args[args.index("--output") + 1]
stdin = sys.stdin.read()
want = os.environ["FAKE_TOKEN"]
with open(os.environ["FAKE_LOG"], "a") as f:
    f.write(json.dumps({"args": args, "token_in_args": any(want in a for a in args),
                        "bearer_on_stdin": ("Authorization: Bearer " + want) in stdin,
                        "auth_key_sent": "X-Auth-Key" in stdin}) + "\n")
answers = json.load(open(os.environ["FAKE_ANSWERS"]))
path = url[len("https://api.cloudflare.com/client/v4"):]
status, body = answers.get(path, [404, {"success": False, "errors": [{"code": 7003, "message": "Could not route"}]}])
with open(headers_file, "w") as f:
    f.write("HTTP/2 %d\r\ncontent-type: application/json\r\n\r\n" % status)
with open(body_file, "w") as f:
    f.write(json.dumps(body))
sys.stdout.write("%d " % status)
"#;

fn scratch(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cv-live-cf-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn user_token() -> String {
    ["cf", "ut_", &"W".repeat(40)].concat()
}

fn answers(ssl: &str) -> serde_json::Value {
    let z = "/zones/z1";
    let v = |value: serde_json::Value| serde_json::json!([200, {"success": true, "result": {"value": value}}]);
    serde_json::json!({
        "/zones?name=stackvet.dev": [200, {"success": true, "result": [{"id": "z1", "account": {"id": "a1"}, "plan": {"name": "Free Website"}}]}],
        "/user/tokens/verify": [200, {"success": true, "result": {"id": "t1", "status": "active"}}],
        format!("{z}/settings/ssl"): v(ssl.into()),
        format!("{z}/settings/ssl_automatic_mode"): v("custom".into()),
        format!("{z}/settings/min_tls_version"): v("1.2".into()),
        format!("{z}/settings/tls_1_3"): v("on".into()),
        format!("{z}/settings/always_use_https"): v("on".into()),
        format!("{z}/settings/security_header"): v(serde_json::json!({"strict_transport_security": {"enabled": true, "max_age": 31536000, "include_subdomains": true, "preload": false, "nosniff": true}})),
        format!("{z}/settings/automatic_https_rewrites"): v("on".into()),
        format!("{z}/rulesets/phases/http_request_cache_settings/entrypoint"): [200, {"success": true, "result": {"rules": []}}],
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
    sv_check::test_support::executable(&curl, FAKE_CURL);
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
        .env_remove("CLOUDFLARE_API_TOKEN")
        .env_remove("CLOUDFLARE_API_KEY")
        .env_remove("CLOUDFLARE_EMAIL");
    if let Some(token) = token {
        command.env("CLOUDFLARE_API_TOKEN", token);
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
    let token = user_token();
    let ran = cv(
        &dir,
        Some(&token),
        &["live", "cloudflare", "stackvet.dev"],
        &answers("full"),
    );
    assert_eq!(ran.code, 0, "{}", ran.said);
    assert_eq!(
        ran.calls.len(),
        10,
        "the zone, the verify, seven settings, and the cache rules: {}",
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
        assert_eq!(
            call["auth_key_sent"], false,
            "a Global API Key's header was sent: {call}"
        );
        let url = call["args"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();
        assert!(url.starts_with(BASE), "{url}");
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
            .all(|r| r["method"] == "GET" && r["host"] == "api.cloudflare.com")
    );
    // The token, before any finding, and never whole; Full is a finding against V12.3.2.
    let token_line = ran.said.find("Token: ").expect("the token is spoken of");
    let first_finding = ran.said.find("finding ").expect("a finding is listed");
    assert!(token_line < first_finding, "{}", ran.said);
    assert!(
        ran.said.contains("could not confirm it is read-only"),
        "{}",
        ran.said
    );
    let written = ran.report.to_string();
    assert!(
        written.contains("cv.cloudflare-origin-certificate-unchecked"),
        "{written}"
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
fn a_global_api_key_is_refused_and_nothing_is_sent() {
    let dir = scratch("global");
    let key = ["cf", "k_", &"e".repeat(40)].concat();
    let ran = cv(
        &dir,
        Some(&key),
        &["live", "cloudflare", "stackvet.dev"],
        &answers("strict"),
    );
    assert!(ran.calls.is_empty(), "curl was run: {:?}", ran.calls);
    assert_eq!(ran.code, 2, "{}", ran.said);
    assert!(ran.said.contains("Global API Key"), "{}", ran.said);
    assert_eq!(
        ran.report["live"]["reading"]["token"]["standing"]["is"],
        "refused"
    );
}

#[test]
fn no_token_reads_nothing_and_a_token_is_never_an_option() {
    let dir = scratch("none");
    let ran = cv(
        &dir,
        None,
        &["live", "cloudflare", "stackvet.dev"],
        &answers("strict"),
    );
    assert_eq!(ran.code, 3, "{}", ran.said);
    assert!(ran.calls.is_empty());
    assert!(
        ran.said.contains("no token in CLOUDFLARE_API_TOKEN"),
        "{}",
        ran.said
    );
    let ran = cv(
        &dir,
        None,
        &["live", "cloudflare", "stackvet.dev", "--token", "abc"],
        &answers("strict"),
    );
    assert_eq!(ran.code, 3);
    assert!(
        ran.said.contains("put it in CLOUDFLARE_API_TOKEN"),
        "{}",
        ran.said
    );
}
