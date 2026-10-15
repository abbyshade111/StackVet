//! The ways into the app the code shows, beside the sign-in pathway questions (backlog 0491): in `sv questions` and
//! report.html, as where to look and never as evidence.

use std::path::PathBuf;
use std::process::Command;

const APP: &str = "\
from flask import Flask, request
app = Flask(__name__)

@app.post(\"/login\")
def login():
    return \"signed in\"

@app.get(\"/auth/google/callback\")
def google():
    return \"signed in with Google\"

@app.get(\"/notes\")
def notes():
    if request.headers.get(\"X-API-Key\"):
        return \"notes\"
    return \"no\"
";

fn app(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-sign-in-paths-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), APP).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Notes\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n\
         [capabilities]\nauth = true\n",
    )
    .unwrap();
    dir
}

fn sv(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .output()
        .expect("sv runs");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// The text of one question in `sv questions`.
fn question<'a>(out: &'a str, id: &str) -> &'a str {
    let start = out
        .find(&format!(" - {id}:"))
        .unwrap_or_else(|| panic!("{id} is asked:\n{out}"));
    let rest = &out[start..];
    &rest[..rest[1..].find("\n - ").map_or(rest.len(), |e| e + 1)]
}

#[test]
fn both_pathway_questions_show_the_ways_in() {
    let dir = app("questions");
    let (ok, out, err) = sv(&["questions", dir.to_str().unwrap()]);
    std::fs::remove_dir_all(&dir).ok();
    assert!(ok, "{err}");
    for id in ["V6.1.3", "V6.3.4"] {
        let q = question(&out, id);
        assert!(q.contains("The ways into the app StackVet found"), "{q}");
        assert!(q.contains("a route, POST /login (app.py, line 4)"), "{q}");
        assert!(
            q.contains("a route, GET /auth/google/callback (app.py, line 8)"),
            "{q}"
        );
        assert!(
            q.contains("an API key read by the name `X-API-Key` (app.py, line 14)"),
            "{q}"
        );
        assert!(
            !q.contains("/notes"),
            "a route not about sign-in is listed: {q}"
        );
    }
}

#[test]
fn report_html_shows_them_and_report_json_holds_none_and_nothing_is_credited() {
    let dir = app("html");
    let out_dir = dir.join("report");
    let (ok, _, err) = sv(&[
        "report",
        dir.to_str().unwrap(),
        "--no-tools",
        "--out",
        out_dir.to_str().unwrap(),
    ]);
    let html = std::fs::read_to_string(out_dir.join("report.html")).unwrap_or_default();
    let json = std::fs::read_to_string(out_dir.join("report.json")).unwrap_or_default();
    let compliance = std::fs::read_to_string(out_dir.join("compliance.md")).unwrap_or_default();
    std::fs::remove_dir_all(&dir).ok();
    assert!(ok, "{err}");
    assert!(
        html.contains("<li>a route, POST /login (<code>app.py</code>, line 4)</li>"),
        "the ways in are not in report.html"
    );
    assert!(!json.is_empty(), "report.json was not written");
    assert!(
        !json.contains("a route, POST /login"),
        "the ways in are in report.json"
    );
    for id in ["V6.1.3", "V6.3.4"] {
        let status = sv_check::test_support::status_of(&compliance, id);
        assert!(status.starts_with("not verified"), "{id}: {status}");
    }
}
