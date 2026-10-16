//! Backlog 0609, end to end through the binary: the second-factor question (V6.3.3) says plainly when none of the
//! app's packages is a library for one, every package list read, and that this is no answer; names the library when
//! there is one; and says nothing when part of the app is in a package list the locator does not read.

use serde_json::Value;
use std::process::Command;

const NONE: &str = "None of the app's packages is a library for one-time codes";

/// A Flask app with `packages` in requirements.txt and, with `go`, a Go module beside it.
fn app(name: &str, packages: &str, go: bool) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-second-factor-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"x\"\n\n[capabilities]\nauth = true\n",
    )
    .unwrap();
    std::fs::write(dir.join("requirements.txt"), packages).unwrap();
    std::fs::write(
        dir.join("app.py"),
        "from flask import Flask\napp = Flask(__name__)\n",
    )
    .unwrap();
    if go {
        std::fs::write(
            dir.join("go.mod"),
            "module example.com/x\n\ngo 1.22\n\nrequire github.com/pquerna/otp v1.4.0\n",
        )
        .unwrap();
        std::fs::write(dir.join("main.go"), "package main\n\nfunc main() {}\n").unwrap();
    }
    dir
}

fn sv(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .output()
        .expect("sv runs")
}

/// The report's folder, made, and its report.json.
fn report(dir: &std::path::Path) -> (std::path::PathBuf, Value) {
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
    let json =
        serde_json::from_str(&std::fs::read_to_string(out.join("report.json")).unwrap()).unwrap();
    (out, json)
}

/// V6.3.3 as the report asks it, on whichever list it is on.
fn question(report: &Value) -> Value {
    ["questions_for_you", "only_you_can_check"]
        .iter()
        .flat_map(|list| report[list].as_array().cloned().unwrap_or_default())
        .find(|q| q["id"] == "V6.3.3")
        .expect("V6.3.3 is asked of an app with no answers")
}

#[test]
fn no_second_factor_library_is_said_plainly_and_as_no_answer() {
    let dir = app("none", "flask==3.0.3\n", false);
    let (out, json) = report(&dir);
    let found = &question(&json)["in_the_packages"];
    assert_eq!(found["packages"], Value::Array(Vec::new()), "{found}");
    let says = found["says"].as_str().expect("the sentence for none");
    assert!(
        says.starts_with(NONE) && says.contains("sign-in service"),
        "{says}"
    );
    // It stays the owner's word.
    let status = json["requirements"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == "V6.3.3")
        .map(|r| r["status"].clone());
    assert_ne!(status, Some(Value::from("verified")));

    let html = std::fs::read_to_string(out.join("report.html")).unwrap();
    assert!(html.contains(": none. None of the app"), "report.html");
    let compliance = std::fs::read_to_string(out.join("compliance.md")).unwrap();
    assert!(
        compliance.contains(": none. None of the app"),
        "compliance.md"
    );
    // The "found by its name" sentence is for a package that was found.
    let v633 = &compliance[compliance.find(NONE).unwrap()..];
    let v633 = &v633[..v633.find('\n').unwrap_or(v633.len())];
    assert!(!v633.contains(sv_check::human::PACKAGES_SAY), "{v633}");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_second_factor_library_is_named_and_none_is_not_said() {
    let dir = app("pyotp", "flask==3.0.3\npyotp==2.9.0\n", false);
    let (_, json) = report(&dir);
    let found = &question(&json)["in_the_packages"];
    let packages = found["packages"].as_array().expect("the library is named");
    assert!(
        packages[0].as_str().unwrap().starts_with("pyotp 2.9.0"),
        "{packages:?}"
    );
    assert!(
        !found["says"].as_str().unwrap().starts_with(NONE),
        "{found}"
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_package_list_the_locator_does_not_read_keeps_none_from_being_said() {
    let dir = app("go", "flask==3.0.3\n", true);
    let (_, json) = report(&dir);
    // The setup worked: the scan found the Go module (it says so of its missing lockfile). With no `go.sum`, its library
    // adds nothing to the bill of materials, so the module itself is what must keep "none" from being said.
    assert!(
        json["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["title"].as_str().is_some_and(|t| t.contains("`go.mod`"))),
        "the Go module was found"
    );
    let found = &question(&json)["in_the_packages"];
    assert!(
        found.is_null(),
        "a Go module's library is in no list it reads: {found}"
    );
    std::fs::remove_dir_all(&dir).ok();
}
