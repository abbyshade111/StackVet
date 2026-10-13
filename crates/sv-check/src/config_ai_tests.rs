//! The note on AI-made commits outside a merge (ADR-092): an `Info` finding when the history names
//! such a commit, a check that ran and found nothing when none does (ADR-092, Later), and not assessed when the
//! folder is no repository.

use super::*;

fn run(dir: &Path, args: &[&str]) {
    let ok = std::process::Command::new("git")
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

fn commit(dir: &Path, file: &str, message: &str) {
    std::fs::write(dir.join(file), format!("{message}\n")).unwrap();
    run(dir, &["add", file]);
    run(dir, &["commit", "-qm", message]);
}

fn repo(name: &str, ai_commit: bool) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-config-ai-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    run(&dir, &["init", "-q", "-b", "main"]);
    commit(&dir, "a.txt", "Start the app");
    if ai_commit {
        commit(&dir, "b.txt", "Add sign-in\n\nAssisted-by: Cursor");
    }
    dir
}

fn finding_of(outcome: Outcome) -> Finding {
    match outcome {
        Outcome::Failed(f) => *f,
        _ => panic!("the note is a finding when the history is read"),
    }
}

#[test]
fn an_ai_commit_outside_a_merge_is_a_info_note_that_names_it() {
    if std::process::Command::new("git")
        .arg("--version")
        .output()
        .is_err()
    {
        println!("no git here; this needs it");
        return;
    }
    let dir = repo("named", true);
    let f = finding_of(ai_commits_off_merge(&dir));
    assert_eq!(f.rule_id, "config.ai-commits-off-merge");
    assert_eq!(f.severity, Severity::Info, "a note, not a failure");
    assert!(
        f.title.starts_with("1 commit(s)"),
        "the title counts it: {}",
        f.title
    );
    assert!(
        f.description.contains("Add sign-in"),
        "the note names the commit"
    );
    assert!(f.description.contains("Cursor"), "the note names the tool");
    assert!(
        f.requirement_ids.is_empty(),
        "the note cites no requirement and credits none"
    );
}

#[test]
fn a_branch_with_no_ai_commit_is_a_check_that_found_nothing_not_a_finding() {
    if std::process::Command::new("git")
        .arg("--version")
        .output()
        .is_err()
    {
        println!("no git here; this needs it");
        return;
    }
    let dir = repo("clean", false);
    // None outside a merge is good news, so it must not reach the report as a thing to fix (ADR-092, Later), and it
    // names no requirement, so it credits none.
    match ai_commits_off_merge(&dir) {
        Outcome::Passed(ids) => assert!(ids.is_empty(), "it credits no requirement: {ids:?}"),
        Outcome::Failed(f) => panic!("good news reported as a finding: {}", f.title),
        Outcome::NotAssessed(why) => panic!("the test's repository could not be read: {why}"),
    }
    // The setup: the same history with an AI commit outside a merge is found, so the check really looked.
    let found = repo("clean-but-one", true);
    let f = finding_of(ai_commits_off_merge(&found));
    assert_eq!(f.severity, Severity::Info);
}

#[test]
fn a_folder_that_is_no_repository_is_not_assessed() {
    let dir = std::env::temp_dir().join(format!("sv-config-ai-plain-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    match ai_commits_off_merge(&dir) {
        Outcome::NotAssessed(why) => assert!(why.contains("not a git repository")),
        _ => panic!("no repository is not assessed, and is not a note"),
    }
}
