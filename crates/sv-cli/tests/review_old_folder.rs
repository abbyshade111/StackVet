//! `sv review` reads the key folder under its old name, `~/.config/securevibe` (ADR-062), and says
//! nothing of it: a note on every review read as an error (backlog 0256; ADR-062, Later). It moves
//! nothing and makes nothing under the new name.
//!
//! Unix only: these tests type into `sv review` through a pseudo-terminal (Python's `pty`), which
//! Windows does not have; there they would not run `sv` at all, and the test that looks for nothing
//! said would pass on nothing (backlog 0120). How `sv review` behaves at a Windows console (it says
//! the passphrase will show as it is typed, ADR-043, Later) is not tested anywhere yet.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;
use sv_frameworks::names::{CONFIG_DIR, OLD_CONFIG_DIR};

/// Runs `sv` with stdin, stdout and stderr on a pseudo-terminal, typing each line of `typed` as a person would: once
/// `sv` is waiting at a question (what it printed has stopped, on a line not yet ended). Typed any sooner, `sv review`
/// drops it as typed ahead (backlog 0675). A run still going after a minute is stopped, so a test fails, not hangs.
const PTY: &str = r#"
import os, pty, select, subprocess, sys, time
answers = sys.argv[1].splitlines(keepends=True)
main, child = pty.openpty()
p = subprocess.Popen(sys.argv[2:], stdin=child, stdout=child, stderr=child, close_fds=True)
os.close(child)
out = b""
last = time.monotonic()
started = last
while True:
    ready, _, _ = select.select([main], [], [], 0.1)
    if ready:
        try:
            data = os.read(main, 4096)
        except OSError:
            break
        if not data:
            break
        out += data
        last = time.monotonic()
        continue
    if p.poll() is not None:
        break
    if time.monotonic() - started > 60:
        p.kill()
        break
    waiting = out and not out.replace(b"\r", b"").endswith(b"\n")
    if answers and waiting and time.monotonic() - last > 0.3:
        os.write(main, answers.pop(0).encode())
        last = time.monotonic()
p.wait()
sys.stdout.buffer.write(out)
sys.exit(p.returncode)
"#;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-review-old-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("app")).unwrap();
    std::fs::write(
        dir.join("app/stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"Reviewed\"\n[stack]\nlanguages = [\"python\"]\n",
    )
    .unwrap();
    std::fs::write(dir.join("app/app.py"), "print(1)\n").unwrap();
    dir
}

fn review(dir: &Path) -> String {
    let run = Command::new("python3")
        .arg("-c")
        .arg(PTY)
        .arg("none\n\n")
        .arg(env!("CARGO_BIN_EXE_sv"))
        .arg("review")
        .arg(dir.join("app"))
        .env("XDG_CONFIG_HOME", dir.join("config"))
        .env_remove(sv_check::signed::TRUSTED_VARIABLE)
        .output()
        .expect("python3 runs");
    String::from_utf8_lossy(&run.stdout).into_owned()
}

#[test]
fn the_old_key_folder_in_use_is_read_and_not_said() {
    let dir = scratch("old");
    let old = dir.join("config").join(OLD_CONFIG_DIR);
    let new = dir.join("config").join(CONFIG_DIR);
    std::fs::create_dir_all(&old).unwrap();
    let said = review(&dir);
    // The setup: the review really ran at a terminal, so its silence means something.
    assert!(!said.trim().is_empty(), "nothing came back from the review");
    assert!(
        !said.contains("runs only in a terminal"),
        "no terminal: {said}"
    );
    assert!(!said.contains("read under its old name"), "{said}");
    assert!(!said.contains("mv "), "{said}");
    assert!(
        !new.exists(),
        "nothing was moved or made under the new name"
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_computer_with_no_old_folder_hears_nothing_of_it() {
    let dir = scratch("new");
    let said = review(&dir);
    assert!(!said.contains("read under its old name"), "{said}");
    std::fs::remove_dir_all(&dir).ok();
}
