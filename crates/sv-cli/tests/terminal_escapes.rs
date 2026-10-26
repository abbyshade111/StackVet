//! What `sv` prints never carries a control character from the app to the terminal (the deep review's
//! improvement 5). A file name is the app's to choose, and on Linux and macOS it can hold an escape
//! character: printed as it is, it can rewrite what is on screen or retitle the window.

#[cfg(unix)]
#[test]
fn a_file_name_with_an_escape_in_it_is_printed_with_the_escape_written_out() {
    use std::process::Command;
    let dir = std::env::temp_dir().join(format!("sv-terminal-escape-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("app.py"), "print('hello')\n").unwrap();
    // A link `sv` does not follow, so its name is listed: an escape that would set the window's title.
    let name = "notes\u{1b}]0;all clear\u{7}.py";
    std::os::unix::fs::symlink("app.py", dir.join(name)).unwrap();
    assert!(
        std::fs::symlink_metadata(dir.join(name)).is_ok(),
        "the link with the escape in its name was made"
    );

    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["check"])
        .arg(&dir)
        .output()
        .expect("sv runs");
    std::fs::remove_dir_all(&dir).ok();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    // The name is there, so the line that would have carried the escape was printed.
    assert!(
        stdout.contains("notes\\u{001b}]0;all clear\\u{0007}.py"),
        "{stdout}{stderr}"
    );
    for (what, text) in [("stdout", &stdout), ("stderr", &stderr)] {
        assert!(
            !text
                .chars()
                .any(|c| c.is_control() && c != '\n' && c != '\t'),
            "{what} carries a control character: {text:?}"
        );
    }
}

/// Runs `sv` with stdin, stdout and stderr on a pseudo-terminal, typing each line of `typed` as a person would: once
/// `sv` is waiting at a question (what it printed has stopped, on a line not yet ended), and the end of input (Ctrl-D)
/// once they are all typed. Typed any sooner, `sv review` drops it as typed ahead (backlog 0675). A run still going
/// after a minute is stopped, so a test fails, not hangs.
#[cfg(unix)]
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
    if waiting and time.monotonic() - last > 0.3:
        # The next answer, or, with none left, the end of input (Ctrl-D), as a person would leave.
        os.write(main, answers.pop(0).encode() if answers else b"\x04")
        last = time.monotonic()
sys.stdout.write(out.decode(errors="replace"))
sys.exit(p.wait())
"#;

/// `sv review` runs only in a terminal, so this gives it one, a pseudo-terminal (`PTY`), typing each answer at its
/// question (backlog 0675). What it shows is in the app's own words: here a reason the AI coding tool wrote into
/// stackvet.toml, with an escape that would retitle the window.
#[cfg(unix)]
#[test]
fn the_review_shows_an_escape_in_a_proposal_rather_than_sending_it() {
    use std::process::Command;
    let root = std::env::temp_dir().join(format!("sv-terminal-review-{}", std::process::id()));
    std::fs::remove_dir_all(&root).ok();
    let (app, home) = (root.join("app"), root.join("home"));
    std::fs::create_dir_all(&app).unwrap();
    std::fs::create_dir_all(&home).unwrap();
    std::fs::write(app.join("app.py"), "print('hello')\n").unwrap();
    std::fs::write(
        app.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"Reviewed\"\n[stack]\nlanguages = [\"python\"]\n\n\
         [[finding-review]]\nrule = \"ast.open-redirect\"\nfile = \"app.py\"\nfingerprint = \"abc\"\n\
         verdict = \"false-alarm\"\nwhy = \"Safe: \\u001b]0;all clear\\u0007 the redirect is fixed\"\n",
    )
    .unwrap();
    // No passphrase on the key it makes, then Enter: leave the proposal as it is.
    let out = Command::new("python3")
        .arg("-c")
        .arg(PTY)
        .arg("none\n\n")
        .arg(env!("CARGO_BIN_EXE_sv"))
        .arg("review")
        .arg(&app)
        .env("HOME", &home)
        .env_remove("XDG_CONFIG_HOME")
        .output()
        .expect("python3 runs");
    std::fs::remove_dir_all(&root).ok();
    let shown = String::from_utf8_lossy(&out.stdout);
    // The proposal was shown, so the line that would have carried the escape was written.
    assert!(
        shown.contains("Safe: \\u{001b}]0;all clear\\u{0007} the redirect is fixed"),
        "{shown:?}"
    );
    assert!(
        !shown.contains('\u{1b}') && !shown.contains('\u{7}'),
        "{shown:?}"
    );
}
