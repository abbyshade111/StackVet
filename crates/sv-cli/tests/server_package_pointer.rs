//! Backlog 0490, end to end through the binary: the request-smuggling question (V4.2.1) names the app's web server
//! package and its version from the bill of materials, says plainly why nothing StackVet does can check it, and stays
//! the owner's word: the requirement is no more met for either.

use serde_json::Value;
use std::process::Command;

/// A Flask app; with `server`, `gunicorn` is pinned beside it.
fn app(name: &str, server: bool) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-server-package-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"x\"\n",
    )
    .unwrap();
    let packages = if server {
        "flask==3.0.3\ngunicorn==22.0.0\n"
    } else {
        "flask==3.0.3\n"
    };
    std::fs::write(dir.join("requirements.txt"), packages).unwrap();
    std::fs::write(
        dir.join("app.py"),
        "from flask import Flask\napp = Flask(__name__)\n",
    )
    .unwrap();
    dir
}

fn sv(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .output()
        .expect("sv runs")
}

/// The report's folder, made.
fn report(dir: &std::path::Path) -> std::path::PathBuf {
    let out = dir.join("report");
    let run = sv(&[
        "report",
        dir.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ]);
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    out
}

fn question(report: &Value) -> Value {
    report["questions_for_you"]
        .as_array()
        .expect("the questions are listed")
        .iter()
        .find(|q| q["id"] == "V4.2.1")
        .cloned()
        .expect("V4.2.1 is asked of an app with no answers")
}

#[test]
fn the_smuggling_question_names_the_server_and_says_why_nothing_here_can_check_it() {
    let dir = app("gunicorn", true);
    let out = report(&dir);
    let json: Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("report.json")).unwrap()).unwrap();
    let q = question(&json);
    let packages = q["in_the_packages"]["packages"]
        .as_array()
        .expect("V4.2.1 carries the web server package");
    assert_eq!(packages.len(), 1, "{packages:?}");
    let named = packages[0].as_str().unwrap();
    assert!(named.starts_with("gunicorn 22.0.0"), "{named}");
    // requirements.txt is what was asked for, not a lockfile, and the line says so.
    assert!(named.contains("no lockfile"), "{named}");
    let why = q["why_not_checked"].as_str().expect("the reason is given");
    assert!(why.contains("not in the app's folder"), "{why}");
    // It stays the owner's word.
    let status = json["requirements"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == "V4.2.1")
        .map(|r| r["status"].clone());
    assert_eq!(status, Some(Value::from("not-verified")));

    let html = std::fs::read_to_string(out.join("report.html")).unwrap();
    assert!(
        html.contains("<strong>In the packages</strong>"),
        "report.html"
    );
    assert!(html.contains("<code>gunicorn 22.0.0"), "report.html");
    assert!(
        html.contains("Why StackVet cannot check it:"),
        "report.html"
    );
    let compliance = std::fs::read_to_string(out.join("compliance.md")).unwrap();
    assert!(compliance.contains("**In the packages**") && compliance.contains("gunicorn 22.0.0"));
    assert!(compliance.contains("**Why StackVet cannot check it:**"));

    // The questions as the AI coding tool is given them.
    let asked = sv(&["questions", dir.to_str().unwrap()]);
    assert!(
        asked.status.success(),
        "{}",
        String::from_utf8_lossy(&asked.stderr)
    );
    let text = String::from_utf8_lossy(&asked.stdout);
    let v421 = &text[text.find(" - V4.2.1: ").expect("V4.2.1 is asked")..];
    let v421 = &v421[..v421[1..].find("\n - ").map_or(v421.len(), |n| n + 1)];
    assert!(
        v421.contains("In the packages") && v421.contains("gunicorn 22.0.0"),
        "{v421}"
    );
    assert!(v421.contains(sv_check::human::PACKAGES_SAY), "{v421}");
    assert!(v421.contains("Why StackVet cannot check it:"), "{v421}");
}

#[test]
fn with_no_server_package_nothing_is_named_and_the_reason_still_is() {
    let dir = app("none", false);
    let out = report(&dir);
    let json: Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("report.json")).unwrap()).unwrap();
    let q = question(&json);
    assert!(q["in_the_packages"].is_null(), "{q}");
    assert!(q["why_not_checked"].is_string(), "{q}");
}
