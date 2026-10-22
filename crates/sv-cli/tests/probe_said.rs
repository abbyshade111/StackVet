//! What `sv probe` says it found at an address that answers (backlog 0324): the things to fix, what it checked and found
//! nothing wrong with, what it could not settle, and that it says nothing about the code. `probe_exit.rs` held the line
//! for an address it could not reach and the exit codes; nothing held these before `cmd_probe` handed them to
//! `say_probe_result`. The same stand-in `curl` answers, so no real site is asked; the address is resolved first, as
//! `sv` always does.

use std::path::{Path, PathBuf};
use std::process::Command;

use sv_check::test_support::scratch;

/// The stand-in `curl`, as `probe_exit.rs` has it.
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
    let bin = fake_curl(dir);
    let curl = bin.join(format!("curl{}", std::env::consts::EXE_SUFFIX));
    // The debug `sv` this test runs asks the stand-in instead of the system's `curl`, on every platform.
    Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["probe", "https://example.com"])
        .env("SV_CURL_FOR_TEST", &curl)
        .env("PROBE_EXIT_ANSWERS", if answers { "1" } else { "0" })
        .output()
        .unwrap()
}

#[test]
fn an_address_that_answers_is_told_what_to_fix_what_held_and_what_could_not_be_settled() {
    let dir = scratch("said");
    let reached = probe(&dir, true);
    let said = String::from_utf8_lossy(&reached.stdout).to_string();
    // The setup: the stand-in answered, so the site was reached.
    assert_eq!(reached.status.code(), Some(0), "{said}");
    assert!(said.contains("It asked for:"), "{said}");
    // A page served over plain HTTP, as the stand-in serves one, is a thing to fix, with its fix.
    assert!(said.contains(" to fix:\n"), "{said}");
    assert!(
        said.contains("[high] The site is served over plain HTTP"),
        "{said}"
    );
    assert!(
        said.contains("  → Serve nothing over plain HTTP but a permanent redirect"),
        "{said}"
    );
    assert!(
        said.contains(
            "What it checked and found nothing wrong with:\n  probe.certificate-not-trusted"
        ),
        "{said}"
    );
    assert!(said.contains("What it could not settle:\n"), "{said}");
    assert!(
        said.contains("V3.7.4 — Whether the site is on the HSTS preload list"),
        "{said}"
    );
    assert!(
        said.trim_end()
            .ends_with("Run `sv report` in the app folder for that, and read the two together."),
        "{said}"
    );
    // What it found comes after what it asked for, and the line about the code last.
    let (asked, fix, code) = (
        said.find("It asked for:").unwrap(),
        said.find(" to fix:").unwrap(),
        said.find("This says nothing about the code.").unwrap(),
    );
    assert!(asked < fix && fix < code, "{said}");
}
