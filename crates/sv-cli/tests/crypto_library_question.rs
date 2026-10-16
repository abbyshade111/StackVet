//! Backlog 0611: the hand check on using a well-known cryptography library (V11.2.1) shows the draft cryptographic
//! inventory, its libraries and call sites, as the key-management questions do.

use std::path::PathBuf;
use std::process::Command;

fn app() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-crypto-library-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("app.py"),
        "import hashlib\n\ndef fingerprint(data):\n    return hashlib.sha256(data).hexdigest()\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Keys\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n[capabilities]\nauth = true\n",
    )
    .unwrap();
    dir
}

#[test]
fn the_library_question_shows_the_draft_inventory() {
    let dir = app();
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["questions", dir.to_str().unwrap()])
        .output()
        .expect("sv runs");
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    let start = text
        .find(" - V11.2.1:")
        .unwrap_or_else(|| panic!("V11.2.1 is asked:\n{text}"));
    let rest = &text[start..];
    let question = &rest[..rest[1..].find("\n - ").map_or(rest.len(), |e| e + 1)];
    assert!(
        question.contains("A first draft of a cryptographic inventory"),
        "{question}"
    );
    assert!(
        question.contains("hashing, `hashlib.sha256` (app.py, line 4)"),
        "{question}"
    );
}
