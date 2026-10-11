//! `sv probe` exits 2 when it could not reach the address (backlog 0235), so a CI step that runs it fails rather than
//! passing on an address it never touched. Reached, it exits 0 whatever it found.
//!
//! The address is a public one, which `sv` resolves before it asks anything, as `probe_addresses.rs` does; `curl` is
//! a stand-in on the PATH, so no real site is asked. The stand-in is a small program this test compiles with `rustc`
//! before it runs. A shell script cannot be run as `curl` on Windows, so the same program serves on Unix and Windows.

use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-probe-exit-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The stand-in `curl`. Asked for its version, it names one. Asked for a page, it answers with one when
/// `PROBE_EXIT_ANSWERS` is `1`, and otherwise cannot connect, exiting with status 7 as `curl` does.
const STAND_IN: &str = r#"
fn main() {
    if std::env::args().nth(1).as_deref() == Some("--version") {
        println!("curl 8.0.0 (x86_64-pc-linux-gnu) libcurl/8.0.0");
        return;
    }
    if std::env::var("PROBE_EXIT_ANSWERS").as_deref() == Ok("1") {
        print!("HTTP/1.1 200 OK\r\ncontent-type: text/html\r\n\r\n");
    } else {
        std::process::exit(7);
    }
}
"#;

/// Compiles the stand-in as `curl` in a `bin` folder under `dir`, and gives that folder's path.
fn fake_curl(dir: &Path) -> PathBuf {
    let bin = dir.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let source = dir.join("stand_in.rs");
    std::fs::write(&source, STAND_IN).unwrap();
    let curl = bin.join(format!("curl{}", std::env::consts::EXE_SUFFIX));
    let built = Command::new("rustc")
        .args(["--edition", "2021", "-o"])
        .arg(&curl)
        .arg(&source)
        .output()
        .expect("rustc runs");
    assert!(
        built.status.success(),
        "the stand-in did not compile: {}",
        String::from_utf8_lossy(&built.stderr)
    );
    bin
}

fn probe(dir: &Path, answers: bool) -> std::process::Output {
    let mut paths = vec![fake_curl(dir)];
    if let Some(path) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&path));
    }
    let path = std::env::join_paths(paths).expect("the PATH joins");
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["probe", "https://example.com"])
        .env("PATH", path)
        .env("PROBE_EXIT_ANSWERS", if answers { "1" } else { "0" })
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
