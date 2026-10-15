//! A first draft of the cryptographic inventory beside the key-management questions, and a key made too weak reported
//! against V11.6.1 (backlog 0494).

use std::path::PathBuf;
use std::process::Command;

const APP: &str = "\
import os, hashlib
from cryptography.hazmat.primitives.asymmetric import rsa

SECRET_KEY = os.environ['SECRET_KEY']

def fingerprint(data):
    return hashlib.sha256(data).hexdigest()

def new_key():
    return rsa.generate_private_key(public_exponent=65537, key_size=1024)
";

fn app(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-crypto-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("certs")).unwrap();
    std::fs::write(dir.join("app.py"), APP).unwrap();
    // Named as a key, never opened: its contents are not a key at all.
    std::fs::write(dir.join("certs/server.pem"), "not read").unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Keys\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n[capabilities]\nauth = true\n",
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

fn question<'a>(out: &'a str, id: &str) -> &'a str {
    let start = out
        .find(&format!(" - {id}:"))
        .unwrap_or_else(|| panic!("{id} is asked:\n{out}"));
    let rest = &out[start..];
    &rest[..rest[1..].find("\n - ").map_or(rest.len(), |e| e + 1)]
}

#[test]
fn both_key_questions_show_the_draft_inventory() {
    let dir = app("questions");
    let (ok, out, err) = sv(&["questions", dir.to_str().unwrap()]);
    std::fs::remove_dir_all(&dir).ok();
    assert!(ok, "{err}");
    for id in ["V11.1.1", "V11.1.2"] {
        let q = question(&out, id);
        assert!(
            q.contains("A first draft of a cryptographic inventory"),
            "{q}"
        );
        assert!(
            q.contains("hashing, `hashlib.sha256` (app.py, line 7)"),
            "{q}"
        );
        assert!(
            q.contains("the setting `SECRET_KEY` (app.py, line 4)"),
            "{q}"
        );
        assert!(
            q.contains("a key or certificate file, certs/server.pem (not opened)"),
            "{q}"
        );
        assert!(
            !q.contains("not read"),
            "a key file's contents reached the question: {q}"
        );
    }
}

#[test]
fn a_small_rsa_key_needs_attention_and_the_draft_credits_nothing() {
    let dir = app("report");
    let out_dir = dir.join("report");
    let (ok, _, err) = sv(&[
        "report",
        dir.to_str().unwrap(),
        "--no-tools",
        "--out",
        out_dir.to_str().unwrap(),
    ]);
    let json = std::fs::read_to_string(out_dir.join("report.json")).unwrap_or_default();
    let compliance = std::fs::read_to_string(out_dir.join("compliance.md")).unwrap_or_default();
    std::fs::remove_dir_all(&dir).ok();
    assert!(ok, "{err}");
    let report: serde_json::Value = serde_json::from_str(&json).expect("report.json is JSON");
    let weak: Vec<u64> = report["findings"]
        .as_array()
        .expect("findings")
        .iter()
        .filter(|f| f["rule_id"] == "ast.weak-key-generation")
        .map(|f| f["location"]["line"].as_u64().unwrap_or(0))
        .collect();
    assert_eq!(weak, vec![10], "{compliance}");
    let status = sv_check::test_support::status_of(&compliance, "V11.6.1");
    assert!(status.contains("needs attention"), "V11.6.1: {status}");
    for id in ["V11.1.1", "V11.1.2"] {
        let status = sv_check::test_support::status_of(&compliance, id);
        assert!(status.starts_with("not verified"), "{id}: {status}");
    }
    assert!(
        !json.contains("certs/server.pem"),
        "the draft is in report.json"
    );
}
