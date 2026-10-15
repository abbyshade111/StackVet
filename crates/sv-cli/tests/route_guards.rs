//! The app's routes and what is written on each, shown beside the authorization questions (backlog 0493): in `sv
//! questions` and report.html, as where to look and never as evidence.

use std::path::PathBuf;
use std::process::Command;

const APP: &str = "\
from flask import Flask
app = Flask(__name__)

@app.route(\"/\")
def home():
    return \"hi\"

@app.route(\"/admin\", methods=[\"GET\", \"POST\"])
@login_required
@roles_required(\"admin\")
def admin():
    return \"admin\"
";

fn app(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-route-guards-{name}-{}", std::process::id()));
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

#[test]
fn the_questions_show_each_route_and_what_is_written_on_it() {
    let dir = app("questions");
    let (ok, out, err) = sv(&["questions", dir.to_str().unwrap()]);
    std::fs::remove_dir_all(&dir).ok();
    assert!(ok, "{err}");
    let start = out
        .find(" - V8.1.1:")
        .unwrap_or_else(|| panic!("V8.1.1 is asked:\n{out}"));
    let question = &out[start..];
    let question = &question[..question[1..]
        .find("\n - ")
        .map_or(question.len(), |e| e + 1)];
    assert!(
        question.contains("The routes StackVet could read"),
        "{question}"
    );
    assert!(
        question.contains("GET / (app.py, line 4): none found in this file"),
        "{question}"
    );
    assert!(
        question.contains(
            "GET, POST /admin (app.py, line 8): login_required, roles_required(\"admin\")"
        ),
        "{question}"
    );
    assert!(question.contains("not evidence"), "{question}");
}

#[test]
fn report_html_shows_the_table_beside_the_question() {
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
        html.contains("<th>who may use it, as the code says</th>"),
        "no table in report.html"
    );
    assert!(
        html.contains(
            "<code>login_required</code>, <code>roles_required(&quot;admin&quot;)</code>"
        ),
        "the guards are not shown"
    );
    // report.json holds none of the app's code.
    assert!(!json.is_empty(), "report.json was not written");
    assert!(
        !json.contains("roles_required"),
        "the guards are in report.json"
    );
    // Where to look, never credit: the requirement is as verified as it was without the table.
    for id in ["V8.1.1", "V8.1.2"] {
        let status = sv_check::test_support::status_of(&compliance, id);
        assert!(status.starts_with("not verified"), "{id}: {status}");
    }
}
