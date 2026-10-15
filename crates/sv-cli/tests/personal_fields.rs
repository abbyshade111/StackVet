//! The fields the app's schema shows that look like personal data, beside the data-classification questions (backlog
//! 0497): in `sv questions` and report.html, as where to look and never as evidence.

use std::path::PathBuf;
use std::process::Command;

const MODELS: &str = "\
from flask_sqlalchemy import SQLAlchemy
db = SQLAlchemy()

class Patient(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    email = db.Column(db.String(120))
    diagnosis = db.Column(db.Text)
    notes = db.Column(db.Text)
";

fn app(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("sv-personal-fields-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("models.py"), MODELS).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Clinic\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n\
         [data]\ncategories = [\"health\", \"contact\"]\n\n[capabilities]\nauth = true\n",
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
fn both_classification_questions_show_the_fields_that_look_personal() {
    let dir = app("questions");
    let (ok, out, err) = sv(&["questions", dir.to_str().unwrap()]);
    std::fs::remove_dir_all(&dir).ok();
    assert!(ok, "{err}");
    for id in ["V14.1.1", "V14.1.2"] {
        let q = question(&out, id);
        assert!(q.contains("look like personal data"), "{q}");
        assert!(q.contains("Patient.email (models.py, line 6)"), "{q}");
        assert!(q.contains("Patient.diagnosis (models.py, line 7)"), "{q}");
        assert!(
            !q.contains("Patient.notes"),
            "a field that does not look personal is listed: {q}"
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
        html.contains("<li><code>Patient.diagnosis</code> (<code>models.py</code>, line 7)</li>"),
        "the fields are not in report.html"
    );
    assert!(!json.is_empty(), "report.json was not written");
    assert!(
        !json.contains("Patient.diagnosis"),
        "the fields are in report.json"
    );
    for id in ["V14.1.1", "V14.1.2"] {
        let status = sv_check::test_support::status_of(&compliance, id);
        assert!(status.starts_with("not verified"), "{id}: {status}");
    }
}
