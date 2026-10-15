//! A way into the app the code shows that `[stack.auth] pathways` does not name is reported against V6.3.4 (backlog
//! 0491; ADR-123), and a list that names every one, or no list at all, credits nothing.

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

const RULE: &str = "config.sign-in-pathway-not-listed";

/// compliance.md and the findings' rule ids from report.json, for the app with this `[stack.auth]` section.
fn report(name: &str, auth: &str) -> (String, Vec<(String, usize)>) {
    let dir = std::env::temp_dir().join(format!("sv-pathways-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), APP).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        format!(
            "manifest-version = 1\n\n[app]\nname = \"Notes\"\naudience = \"customers\"\n\
             deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n{auth}\n\
             [capabilities]\nauth = true\n"
        ),
    )
    .unwrap();
    let out_dir: PathBuf = dir.join("report");
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["report", dir.to_str().unwrap(), "--no-tools", "--out"])
        .arg(&out_dir)
        .output()
        .expect("sv runs");
    let compliance = std::fs::read_to_string(out_dir.join("compliance.md")).unwrap_or_default();
    let json = std::fs::read_to_string(out_dir.join("report.json")).unwrap_or_default();
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let report: serde_json::Value = serde_json::from_str(&json).expect("report.json is JSON");
    let found = report["findings"]
        .as_array()
        .expect("findings")
        .iter()
        .filter(|f| f["rule_id"] == RULE)
        .map(|f| {
            (
                f["location"]["file"].as_str().unwrap_or("").to_owned(),
                f["location"]["line"].as_u64().unwrap_or(0) as usize,
            )
        })
        .collect();
    (compliance, found)
}

#[test]
fn a_way_in_the_list_does_not_name_needs_attention() {
    let (compliance, found) = report(
        "partial",
        "[stack.auth]\npathways = [\"/login\", \"/auth/google\"]\n",
    );
    // Only the API key read by header is not named.
    assert_eq!(found, vec![("app.py".to_owned(), 14)], "{compliance}");
    let status = sv_check::test_support::status_of(&compliance, "V6.3.4");
    assert!(status.contains("needs attention"), "V6.3.4: {status}");
}

#[test]
fn a_list_naming_every_way_in_and_no_list_credit_nothing() {
    for (name, auth) in [
        (
            "full",
            "[stack.auth]\npathways = [\"/login\", \"/auth/google\", \"x-api-key\"]\n",
        ),
        ("none", ""),
    ] {
        let (compliance, found) = report(name, auth);
        assert!(found.is_empty(), "{name}: {found:?}");
        let status = sv_check::test_support::status_of(&compliance, "V6.3.4");
        assert!(
            status.starts_with("not verified"),
            "{name}: V6.3.4: {status}"
        );
    }
}
