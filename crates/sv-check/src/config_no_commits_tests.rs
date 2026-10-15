//! A git repository with no commits yet is told apart from a folder that is not one (backlog 0596): the trial of 15
//! October 2026 read "not a git repository" of a repository the tool had made and not yet committed to.

use super::ai_tests::{commit, rules, run};
use super::*;

fn folder(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "sv-config-no-commits-{name}-{}",
        std::process::id()
    ));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn why(outcome: Outcome) -> String {
    match outcome {
        Outcome::NotAssessed(why) => why,
        _ => panic!("not assessed, with a reason, when there is no history to read"),
    }
}

#[test]
fn a_repository_with_no_commits_yet_is_said_to_be_one() {
    let dir = folder("empty");
    run(&dir, &["init", "-q", "-b", "main"]);
    // A file added and not committed: the repository is there, its history is not.
    std::fs::write(dir.join("app.py"), "print('hi')\n").unwrap();
    run(&dir, &["add", "app.py"]);
    let said = why(ai_commits_off_merge(&dir, rules()));
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        said.contains("a git repository with no commits yet"),
        "{said}"
    );
    assert!(!said.contains("not a git repository"), "{said}");
}

#[test]
fn a_folder_that_is_not_a_repository_is_still_said_to_be_none() {
    let dir = folder("none");
    let said = why(ai_commits_off_merge(&dir, rules()));
    std::fs::remove_dir_all(&dir).ok();
    assert!(said.contains("not a git repository"), "{said}");
}

#[test]
fn after_the_first_commit_the_history_is_read() {
    let dir = folder("one");
    run(&dir, &["init", "-q", "-b", "main"]);
    commit(&dir, "a.txt", "Start the app");
    let outcome = ai_commits_off_merge(&dir, rules());
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        matches!(outcome, Outcome::Passed(_)),
        "a history with no AI commit is a check that ran"
    );
}
