//! `sv probe` exits 2 when it could not reach the address (backlog 0235), so a CI step that runs it fails rather than
//! passing on an address it never touched. Reached, it exits 0 whatever it found.
//!
//! The address is a public one, which `sv` resolves before it asks anything, as `probe_addresses.rs` does; `curl` is
//! a stand-in on the PATH, so no real site is asked.

// The stand-in `curl` below is a shell script put first on the PATH, and its permissions are Unix
// permissions, so the whole file is for Unix. Windows does not run this check. This is a
// test-only gate: `sv probe` itself is unchanged.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-probe-exit-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A `curl` that answers with a page, or, when `answers` is false, cannot connect (its exit status 7).
fn fake_curl(dir: &Path, answers: bool) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let body = if answers {
        "printf 'HTTP/1.1 200 OK\\r\\ncontent-type: text/html\\r\\n\\r\\n'"
    } else {
        "exit 7"
    };
    let script = format!(
        "#!/bin/sh\ncase \"$1\" in --version) echo 'curl 8.0.0 (x86_64-pc-linux-gnu) libcurl/8.0.0'; exit 0;; esac\n{body}\n"
    );
    let bin = dir.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let curl = bin.join("curl");
    std::fs::write(&curl, script).unwrap();
    std::fs::set_permissions(&curl, std::fs::Permissions::from_mode(0o755)).unwrap();
    bin
}

fn probe(dir: &Path, answers: bool) -> std::process::Output {
    let bin = fake_curl(dir, answers);
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["probe", "https://example.com"])
        .env("PATH", path)
        .output()
        .unwrap()
}

#[test]
fn an_address_that_cannot_be_reached_exits_2_and_one_that_answers_exits_0() {
    let dir = scratch("unreached");
    let unreached = probe(&dir, false);
    let said = String::from_utf8_lossy(&unreached.stdout).to_string();
    // The setup: the message the command prints for it is there.
    assert!(
        said.contains("could not reach that address"),
        "the setup did not reach the unreached branch: {said}"
    );
    assert_eq!(unreached.status.code(), Some(2), "{said}");

    let dir = scratch("reached");
    let reached = probe(&dir, true);
    assert_eq!(
        reached.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&reached.stdout)
    );
}
