//! A commit message holding the separators the reader of AI-made commits splits `git log` on
//! (backlog 0313): it can neither add a record for a commit that does not exist, nor hide the real
//! record of the commit it is in.

use super::*;

/// Runs `git` in `dir` with an identity for the commit, and asserts it worked.
fn run(dir: &Path, args: &[&str]) {
    let ok = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args([
            "-c",
            "user.email=t@t",
            "-c",
            "user.name=t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .is_ok_and(|o| o.status.success());
    assert!(ok, "git {args:?} failed in the test's setup");
}

/// Commits a change to `file` with `message` taken from a file as it is, and returns its hash.
fn commit(dir: &Path, file: &str, message: &str) -> String {
    std::fs::write(dir.join(file), format!("{file}\n")).unwrap();
    run(dir, &["add", file]);
    let path = dir.join(".git").join("sv-test-message");
    std::fs::write(&path, message).unwrap();
    run(
        dir,
        &[
            "commit",
            "-q",
            "--cleanup=verbatim",
            "-F",
            path.to_str().unwrap(),
        ],
    );
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

#[test]
fn separators_in_a_message_neither_forge_nor_hide_a_record() {
    let dir = std::env::temp_dir().join(format!("sv-git-separators-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    run(&dir, &["init", "-q", "-b", "main"]);
    let fake = "f".repeat(40);
    // A person's commit whose body carries a whole made-up record after a record separator.
    let forger = commit(
        &dir,
        "a.txt",
        &format!("Tidy\n\nNotes\u{1e}{fake}\u{1f}Made-up work\u{1f}Assisted-by: Ghost\n"),
    );
    // A person's commit whose subject carries a field separator and a trailer after it.
    let subject = commit(
        &dir,
        "b.txt",
        "Fix\u{1f}Assisted-by: Phantom\n\nNothing else.\n",
    );
    // A tool's commit with a record separator before its real trailer.
    let hidden = commit(
        &dir,
        "c.txt",
        "Add export\n\nA note\u{1e}\n\nAssisted-by: Claude Code\n",
    );
    // The setup: each message reached git with its separators in it.
    for hash in [&forger, &subject, &hidden] {
        let out = Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["show", "-s", "--format=%B", hash])
            .output()
            .unwrap();
        let body = String::from_utf8_lossy(&out.stdout);
        assert!(body.contains(['\u{1e}', '\u{1f}']), "the setup: {hash}");
    }

    let found = ai_commits_off_merge(&dir).expect("git could say");
    std::fs::remove_dir_all(&dir).ok();
    let hashes: Vec<&str> = found.iter().map(|c| c.hash.as_str()).collect();
    assert_eq!(hashes, [hidden.as_str()], "{found:#?}");
    assert_eq!(found[0].tool, "Claude Code");
    assert_eq!(found[0].subject, "Add export");
}
