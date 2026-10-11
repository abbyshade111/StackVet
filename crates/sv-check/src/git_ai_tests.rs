//! The reader of AI-made commits on the default branch (ADR-092). Each test plants a real repository
//! in a temporary folder and reads it with the same `git` the check runs.

use super::*;

/// Runs `git` in `dir` with an identity for the commit, and asserts it worked.
fn run(dir: &Path, args: &[&str]) {
    let ok = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "user.email=t@t", "-c", "user.name=t", "-c", "commit.gpgsign=false"])
        .args(args)
        .output()
        .is_ok_and(|o| o.status.success());
    assert!(ok, "git {args:?} failed in the test's setup");
}

/// Makes a commit with `message`, changing `file` so each commit is distinct, and returns its hash.
fn commit(dir: &Path, file: &str, message: &str) -> String {
    std::fs::write(dir.join(file), format!("{message}\n")).unwrap();
    run(dir, &["add", file]);
    run(dir, &["commit", "-qm", message]);
    let out = Command::new("git").arg("-C").arg(dir).args(["rev-parse", "HEAD"]).output().unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// A repository on `main` with: an ordinary commit; a commit an AI tool made, straight onto `main`; a
/// feature branch with an AI commit, merged with a merge commit; a commit a person co-wrote with a
/// human; and an AI commit on a branch that was never merged. Returns the folder and the hashes of
/// the AI commits that reached `main` directly.
fn planted(name: &str) -> (std::path::PathBuf, Vec<String>) {
    let dir = std::env::temp_dir().join(format!("sv-git-ai-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    run(&dir, &["init", "-q", "-b", "main"]);
    commit(&dir, "a.txt", "Start the app");
    let direct_claude = commit(&dir, "b.txt", "Add sign-in\n\nCo-Authored-By: Claude <noreply@anthropic.com>");
    let direct_cursor = commit(&dir, "c.txt", "Add uploads\n\nAssisted-by: Cursor");
    let human = commit(&dir, "d.txt", "Add search\n\nCo-Authored-By: Sam Lee <sam@example.com>");
    run(&dir, &["checkout", "-q", "-b", "feature"]);
    commit(&dir, "e.txt", "Add export\n\nAssisted-by: Muse Spark 1.3");
    run(&dir, &["checkout", "-q", "main"]);
    run(&dir, &["merge", "-q", "--no-ff", "-m", "Merge pull request #7", "feature"]);
    run(&dir, &["checkout", "-q", "-b", "unmerged"]);
    commit(&dir, "f.txt", "Try a route\n\nAssisted-by: Aider");
    run(&dir, &["checkout", "-q", "main"]);
    (dir, vec![direct_claude, direct_cursor, human])
}

#[test]
fn an_ai_commit_straight_onto_main_is_listed_and_a_merged_one_is_not() {
    if Command::new("git").arg("--version").output().is_err() {
        println!("no git here; this needs it");
        return;
    }
    let (dir, _) = planted("listed");
    let found = ai_commits_off_merge(&dir).expect("git reads the planted repository");
    // The setup itself must have worked: the merged feature commit is on main, and the planted
    // commits are all there, before asserting what the check finds among them.
    let log = Command::new("git").arg("-C").arg(&dir).args(["log", "--format=%s"]).output().unwrap();
    let log = String::from_utf8_lossy(&log.stdout).to_string();
    assert!(log.contains("Add export"), "the merged commit is on main in the setup");
    assert!(log.contains("Add sign-in"), "the direct AI commit is on main in the setup");
    let subjects: Vec<&str> = found.iter().map(|c| c.subject.as_str()).collect();
    assert_eq!(subjects, vec!["Add uploads", "Add sign-in"], "the direct AI commits, newest first");
    assert!(
        found.iter().all(|c| !c.subject.contains("export")),
        "a commit that came in through a merge is not listed"
    );
    let tools: Vec<&str> = found.iter().map(|c| c.tool.as_str()).collect();
    assert_eq!(tools, vec!["Cursor", "Claude <noreply@anthropic.com>"]);
}

#[test]
fn a_person_who_co_wrote_with_a_person_is_not_listed() {
    if Command::new("git").arg("--version").output().is_err() {
        println!("no git here; this needs it");
        return;
    }
    let (dir, _) = planted("human");
    let found = ai_commits_off_merge(&dir).expect("git reads the planted repository");
    assert!(
        found.iter().all(|c| c.subject != "Add search"),
        "a Co-Authored-By line naming a person is not an AI tool"
    );
}

#[test]
fn an_unmerged_branch_is_not_read() {
    if Command::new("git").arg("--version").output().is_err() {
        println!("no git here; this needs it");
        return;
    }
    let (dir, _) = planted("unmerged");
    let found = ai_commits_off_merge(&dir).expect("git reads the planted repository");
    assert!(
        found.iter().all(|c| c.subject != "Try a route"),
        "a commit on a branch that main never took is not on main, so it is not read"
    );
}

#[test]
fn a_folder_that_is_not_a_repository_is_not_assessed() {
    let dir = std::env::temp_dir().join(format!("sv-git-ai-plain-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    assert!(
        ai_commits_off_merge(&dir).is_none(),
        "no repository is no answer, and is not an empty list"
    );
}

#[test]
fn the_trailer_lines_are_read_as_the_message_names_them() {
    assert_eq!(trailer_tool("Add x\n\nAssisted-by: Copilot"), Some("Copilot".to_owned()));
    assert_eq!(
        trailer_tool("Add x\n\nco-authored-by: Claude Sonnet <noreply@anthropic.com>"),
        Some("Claude Sonnet <noreply@anthropic.com>".to_owned())
    );
    assert_eq!(trailer_tool("Add x\n\nCo-Authored-By: Pat <pat@example.com>"), None);
    assert_eq!(trailer_tool("Add x"), None);
    assert_eq!(trailer_tool("Add x\n\nAssisted-by:"), Some("a tool not named".to_owned()));
}
