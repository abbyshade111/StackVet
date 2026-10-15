//! Backlog 0563: `sv` printing to a pipe whose reader has gone (`sv --version | head -1`) ends quietly with 141, the
//! usual code for it, never "sv itself failed", and never 0. The reading end is closed before `sv` starts, so the
//! first line it prints meets a closed pipe every time.

use std::process::{Command, Stdio};

/// `sv` run with `args`, its output a pipe nobody reads: its status and what it wrote to stderr.
fn into_closed_pipe(args: &[&str]) -> (Option<i32>, String) {
    let (reader, writer) = std::io::pipe().expect("a pipe");
    drop(reader);
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .stdout(Stdio::from(writer))
        .stderr(Stdio::piped())
        .output()
        .expect("sv runs");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn a_closed_pipe_ends_quietly_and_never_as_a_fault() {
    for args in [&["--version"][..], &["--help"][..]] {
        let (code, said) = into_closed_pipe(args);
        assert_eq!(code, Some(141), "`sv {}`: {said}", args.join(" "));
        assert!(
            !said.contains("sv itself failed"),
            "`sv {}`: {said}",
            args.join(" ")
        );
        assert!(said.is_empty(), "nothing is said: {said}");
    }
}

#[test]
fn a_reader_that_stays_still_gets_everything() {
    // The control: the same command, read to the end, prints its line and ends 0.
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("--version")
        .output()
        .expect("sv runs");
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).starts_with("sv "));
}
