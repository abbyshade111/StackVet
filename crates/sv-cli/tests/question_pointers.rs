//! Backlog 0487, end to end through the binary: a question to the owner carries the lines of the app's code it is about,
//! with the sentence that they are where to look and not evidence, redacted, and only as file and line in report.json.

use serde_json::Value;
use std::process::Command;

const SAYS: &str = "where to look, not evidence";

/// An app whose authorization check sits beside a password, so the lines shown have something to redact.
fn app() -> (std::path::PathBuf, String) {
    let dir = std::env::temp_dir().join(format!("sv-question-pointers-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"x\"\n",
    )
    .unwrap();
    // Built from pieces, so no file in the repository holds a value shaped like a password.
    let password: String = ["Zq7", "vX2m", "Lp9w", "Rt4k", "Yb8n"].concat();
    std::fs::write(
        dir.join("app.py"),
        format!(
            "from flask import Flask\n\
             app = Flask(__name__)\n\
             password = \"{password}\"\n\
             def require_role(role):\n\
             \x20   return role\n\
             \n\
             @app.route('/admin')\n\
             @require_role('admin')\n\
             def admin():\n\
             \x20   return 'ok'\n"
        ),
    )
    .unwrap();
    (dir, password)
}

fn sv(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .output()
        .expect("sv runs")
}

#[test]
fn a_question_shows_the_lines_it_is_about_and_says_they_are_not_evidence() {
    let (dir, password) = app();
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

    let json_text = std::fs::read_to_string(out.join("report.json")).unwrap();
    let report: Value = serde_json::from_str(&json_text).unwrap();
    let question = report["questions_for_you"]
        .as_array()
        .expect("the questions are listed")
        .iter()
        .find(|q| q["id"] == "V8.3.1")
        .expect("V8.3.1 is asked of an app with no answers");
    let places = question["rests_on"]["places"]
        .as_array()
        .expect("V8.3.1 carries the places its locator found");
    // The function written comes first, then the place it is used.
    assert_eq!(places[0]["file"], "app.py");
    assert_eq!(places[0]["line"], 4);
    assert_eq!(places[1]["line"], 8);
    // File and line, never the code: report.json holds none of the app's lines.
    assert!(
        !json_text.contains("def require_role"),
        "report.json holds a line of the app's code"
    );
    // report.html shows the lines under "What only you can check"; compliance.md, a table, the file and line only.
    let html = std::fs::read_to_string(out.join("report.html")).unwrap();
    assert!(html.contains("<strong>In the code</strong>") && html.contains(SAYS));
    assert!(html.contains("def require_role(role):"));
    assert!(!html.contains(&password), "report.html shows the password");
    let compliance = std::fs::read_to_string(out.join("compliance.md")).unwrap();
    assert!(compliance.contains("app.py, line 4") && compliance.contains(SAYS));
    assert!(!compliance.contains("def require_role"));
    // It credits nothing: the requirement is no more met for having lines to look at.
    let status = report["requirements"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == "V8.3.1")
        .map(|r| r["status"].clone());
    assert_eq!(status, Some(Value::from("not-verified")));

    // The questions as the AI coding tool is given them: the lines, the sentence, and no password.
    let asked = sv(&["questions", dir.to_str().unwrap()]);
    assert!(
        asked.status.success(),
        "{}",
        String::from_utf8_lossy(&asked.stderr)
    );
    let text = String::from_utf8_lossy(&asked.stdout);
    let v831 = &text[text.find(" - V8.3.1: ").expect("V8.3.1 is asked")..];
    let v831 = &v831[..v831[1..].find("\n - ").map_or(v831.len(), |n| n + 1)];
    assert!(v831.contains(SAYS), "{v831}");
    assert!(v831.contains("def require_role(role):"), "{v831}");
    assert!(v831.contains("app.py, line 4"), "{v831}");
    assert!(
        !text.contains(&password),
        "the password beside the check was shown"
    );
}
