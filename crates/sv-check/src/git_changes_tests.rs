//! The reader of what each commit changed (backlog 0630, ADR-131). Each test plants a real repository in a temporary
//! folder and reads it with the same `git` the section runs.

use super::*;
use std::process::Command;

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

fn head(dir: &Path) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// A repository with a person's commit, then an AI tool's commit that deletes a test, rewrites a line beginning
/// `-- ` (which a diff prints as `--- `), and adds a marker; returns the folder and the first commit's hash.
fn planted(name: &str) -> (std::path::PathBuf, String) {
    let dir = std::env::temp_dir().join(format!("sv-git-changes-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("tests")).unwrap();
    run(&dir, &["init", "-q", "-b", "main"]);
    std::fs::write(dir.join("app.sql"), "-- the schema\nselect 1;\n").unwrap();
    std::fs::write(
        dir.join("tests/test_app.py"),
        "def test_it():\n    assert True\n",
    )
    .unwrap();
    run(&dir, &["add", "-A"]);
    run(&dir, &["commit", "-qm", "Start the app"]);
    let first = head(&dir);
    std::fs::write(dir.join("app.sql"), "-- the new schema\nselect 1;\n").unwrap();
    std::fs::write(dir.join("app.py"), "eval(x)  # nosec\n").unwrap();
    std::fs::remove_file(dir.join("tests/test_app.py")).unwrap();
    run(&dir, &["add", "-A"]);
    run(
        &dir,
        &[
            "commit",
            "-qm",
            "Tidy up\n\nCo-Authored-By: Claude <noreply@anthropic.com>",
        ],
    );
    std::fs::write(dir.join("notes.txt"), "a person's note\n").unwrap();
    run(&dir, &["add", "-A"]);
    run(&dir, &["commit", "-qm", "Add a note"]);
    (dir, first)
}

#[test]
fn a_tool_commit_s_files_and_lines_are_read_and_a_person_s_are_not() {
    let (dir, first) = planted("read");
    let (commits, from_found) =
        since(&dir, Some(&first), 20).expect("git could read the planted repository");
    assert!(from_found);
    // The setup: the two commits after the first, newest first, and only the second names a tool.
    assert_eq!(commits.len(), 2, "{commits:?}");
    assert_eq!(commits[0].tool, None);
    assert!(
        commits[0].files.is_empty(),
        "a person's commit is not read for its files"
    );
    let tool = &commits[1];
    assert_eq!(tool.subject, "Tidy up");
    assert_eq!(tool.tool.as_deref(), Some("Claude <noreply@anthropic.com>"));
    assert!(
        tool.files.contains(&('D', "tests/test_app.py".to_owned())),
        "{:?}",
        tool.files
    );
    assert!(tool.files.contains(&('A', "app.py".to_owned())));
    assert!(tool.files.contains(&('M', "app.sql".to_owned())));
    assert!(
        tool.added
            .contains(&("app.py".to_owned(), "eval(x)  # nosec".to_owned()))
    );
    // A removed line that begins `-- ` is a line of the file, not a header.
    assert!(
        tool.removed
            .contains(&("app.sql".to_owned(), "-- the schema".to_owned())),
        "{:?}",
        tool.removed
    );
    assert!(
        tool.added
            .contains(&("app.sql".to_owned(), "-- the new schema".to_owned()))
    );
    // A deleted file's lines are kept under its own name.
    assert!(
        tool.removed
            .contains(&("tests/test_app.py".to_owned(), "    assert True".to_owned()))
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_commit_not_in_the_history_reads_the_last_commits_instead() {
    let (dir, _) = planted("not-found");
    let (commits, from_found) =
        since(&dir, Some("0123456789abcdef0123456789abcdef01234567"), 20).unwrap();
    assert!(!from_found);
    assert_eq!(commits.len(), 3, "every commit, up to the most asked for");
    let (commits, _) = since(&dir, None, 2).unwrap();
    assert_eq!(commits.len(), 2);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_folder_with_no_commits_says_nothing() {
    let dir = std::env::temp_dir().join(format!("sv-git-changes-empty-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    assert_eq!(since(&dir, None, 20), None);
    run(&dir, &["init", "-q", "-b", "main"]);
    assert_eq!(since(&dir, None, 20), None);
    std::fs::remove_dir_all(&dir).ok();
}
