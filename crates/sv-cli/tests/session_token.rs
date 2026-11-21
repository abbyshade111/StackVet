//! `tools/session_token.py`, which runs one `gh` or `git` command as the sessions' GitHub App (ADR-158; backlog
//! 0692). Its self-test signs with a throwaway key it makes, checks the signature with `openssl`, and checks that the
//! token reaches `git` only through the environment; it never reads the owner's keychain and opens no connection.
//!
//! The script runs on the owner's Mac and hands the key to `openssl` through `/dev/fd`, which Windows does not have,
//! so on Windows this test is not built rather than reported as passing.

#![cfg(unix)]

use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn the_session_token_script_passes_its_own_test() {
    let out = Command::new("python3")
        .arg("-I")
        .arg(root().join("tools/session_token.py"))
        .arg("--self-test")
        .output()
        .expect("python3 runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    assert!(said.contains("session_token self-test: ok"), "{said}");
}

#[test]
fn the_script_never_falls_back_to_the_owners_sign_in() {
    // The promise of ADR-158, decision 2: with no token, stop. A fallback would be a call to `gh` with no GH_TOKEN,
    // or a read of `gh auth token`; neither may be in the script.
    let script = std::fs::read_to_string(root().join("tools/session_token.py")).unwrap();
    assert!(
        script.contains("raise Stop("),
        "the setup: the script stops with reasons"
    );
    for never in ["gh auth token", "auth status", "print(token", "print(key"] {
        assert!(!script.contains(never), "{never:?} in the script");
    }
}
