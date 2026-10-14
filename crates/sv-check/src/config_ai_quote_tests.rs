//! A commit subject quoted into the note on AI-made commits has any key in it masked, and is cut short (backlog
//! 0443). AI tools do write a key into a commit subject, and a subject is in the history, which is never rewritten:
//! quoted as it was, the key reached every report file, and the bundle step then refused the zip with advice that
//! could not be followed.

use super::ai_tests::{commit, finding_of, rules, run};
use super::*;

fn git_here() -> bool {
    std::process::Command::new("git")
        .arg("--version")
        .output()
        .is_ok()
}

fn repo_with(name: &str, message: &str) -> std::path::PathBuf {
    let dir =
        std::env::temp_dir().join(format!("sv-config-ai-quote-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    run(&dir, &["init", "-q", "-b", "main"]);
    commit(&dir, "a.txt", "Start the app");
    commit(&dir, "b.txt", message);
    dir
}

#[test]
fn a_key_in_a_commit_subject_is_masked_in_the_note() {
    if !git_here() {
        println!("no git here; this needs it");
        return;
    }
    // Built from pieces, so no key is written in this file.
    let key = format!("{}_{}_{}", "sk", "live", "4eC39HqLyjWDarjtT1zdp7dc");
    let dir = repo_with(
        "key",
        &format!("Add Stripe key {key}\n\nAssisted-by: Cursor"),
    );
    let rules = rules();
    // The setup: the rules know the key's shape, so the test is about the note and not about them.
    assert_eq!(crate::secrets::redact_text(&rules, &key).1, 1);
    let f = finding_of(ai_commits_off_merge(&dir, &rules));
    std::fs::remove_dir_all(&dir).ok();
    // A failure shows the note masked, so not even this test's made-up key is printed.
    let shown = crate::secrets::redact_text(&rules, &f.description).0;
    assert!(!f.description.contains(&key), "{shown}");
    assert!(
        !f.description.contains(&key[8..]),
        "no part of the key past what a masked one shows: {shown}"
    );
    // What must not be lost: the subject's words, the hash, and the tool's name.
    assert!(
        f.description.contains("Add Stripe key"),
        "{}",
        f.description
    );
    assert!(f.description.contains("(Cursor)"), "{}", f.description);
    assert!(f.description.contains('`'), "the hash: {}", f.description);
}

#[test]
fn a_long_subject_is_cut_short() {
    if !git_here() {
        println!("no git here; this needs it");
        return;
    }
    let long = "word ".repeat(200);
    let dir = repo_with("long", &format!("{}\n\nAssisted-by: Cursor", long.trim()));
    let f = finding_of(ai_commits_off_merge(&dir, &rules()));
    std::fs::remove_dir_all(&dir).ok();
    let quoted = f
        .description
        .split_once("` ")
        .and_then(|(_, rest)| rest.split_once(" (Cursor)"))
        .map(|(subject, _)| subject)
        .unwrap_or_else(|| panic!("the note quotes the subject: {}", f.description));
    assert!(quoted.ends_with('…'), "{quoted}");
    assert_eq!(
        quoted.chars().count(),
        161,
        "a thousand-character subject is quoted at 160 characters and a mark: {quoted}"
    );
    assert!(f.description.contains("(Cursor)"), "{}", f.description);
}
