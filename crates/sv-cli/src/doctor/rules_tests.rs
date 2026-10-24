//! Backlog 0667: `sv doctor` and `stackvet_status` say whether the rules for the AI coding tool are where each
//! tool reads them, and never write them: every case below leaves the folder's files byte for byte as they were.

use super::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const TOPIC: &str = "rules for the AI tool";

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-doctor-rules-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn section() -> String {
    format!(
        "# Ours\n\n{}\nThe rules.\n{}\n",
        sv_frameworks::names::RULES_BEGIN,
        sv_frameworks::names::RULES_END
    )
}

/// Every file under `dir` and its bytes, to show nothing was written.
fn files(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            out.extend(files(&path));
        } else {
            out.insert(path.clone(), std::fs::read(&path).unwrap());
        }
    }
    out
}

/// The rules line for `dir`, after showing the answer wrote nothing.
fn asked(dir: &Path) -> (State, String) {
    let before = files(dir);
    let line = rules_for_the_tool(dir);
    assert_eq!(files(dir), before, "the answer wrote into the folder");
    std::fs::remove_dir_all(dir).ok();
    assert_eq!(line.topic, TOPIC);
    assert!(
        !line.quotes_app,
        "the line is sv's own words, never the app's"
    );
    (line.state, line.said)
}

#[test]
fn no_agents_md_is_not_ready_and_names_sv_rules() {
    let (state, said) = asked(&folder("none"));
    assert_eq!(state, State::NotReady);
    assert!(said.contains("There is no AGENTS.md"), "{said}");
    assert!(said.contains("`sv rules`"), "{said}");
}

#[test]
fn an_agents_md_without_the_section_is_not_ready() {
    let dir = folder("no-section");
    std::fs::write(dir.join("AGENTS.md"), "# Ours\n\nUse tabs.\n").unwrap();
    let (state, said) = asked(&dir);
    assert_eq!(state, State::NotReady);
    assert!(said.contains("no section"), "{said}");
}

#[test]
fn the_section_is_ready_and_the_old_markers_count_too() {
    let dir = folder("section");
    std::fs::write(dir.join("AGENTS.md"), section()).unwrap();
    assert_eq!(asked(&dir).0, State::Ready);

    let dir = folder("old-markers");
    let old = format!(
        "{}\nThe rules.\n{}\n",
        sv_frameworks::names::OLD_RULES_BEGIN,
        sv_frameworks::names::OLD_RULES_END
    );
    std::fs::write(dir.join("AGENTS.md"), old).unwrap();
    assert_eq!(asked(&dir).0, State::Ready);
}

#[test]
fn a_claude_md_that_does_not_bring_agents_md_in_is_not_ready() {
    let dir = folder("claude");
    std::fs::write(dir.join("AGENTS.md"), section()).unwrap();
    std::fs::write(dir.join("CLAUDE.md"), "Be brief.\n").unwrap();
    let (state, said) = asked(&dir);
    assert_eq!(state, State::NotReady);
    assert!(
        said.contains("Claude Code does not see the rules"),
        "{said}"
    );

    // With the import, ready.
    let dir = folder("claude-imports");
    std::fs::write(dir.join("AGENTS.md"), section()).unwrap();
    std::fs::write(dir.join("CLAUDE.md"), "@AGENTS.md\n").unwrap();
    assert_eq!(asked(&dir).0, State::Ready);
}

#[test]
fn the_other_tools_notes_are_passed_on_by_file_name_alone() {
    let dir = folder("notes");
    std::fs::write(dir.join("AGENTS.md"), section()).unwrap();
    std::fs::write(dir.join("AGENTS.override.md"), "Override.\n").unwrap();
    std::fs::write(dir.join(".aider.conf.yml"), "model: x\n").unwrap();
    let (state, said) = asked(&dir);
    // A note is advice: the rules are where Claude Code, Cursor, and the rest read them.
    assert_eq!(state, State::Ready);
    assert!(said.contains("Codex:"), "{said}");
    assert!(said.contains("Aider:"), "{said}");
    assert!(
        !said.contains("Override."),
        "a file's own text is never quoted: {said}"
    );
}

#[cfg(unix)]
#[test]
fn an_agents_md_that_is_a_link_is_not_read_through() {
    let dir = folder("link");
    let elsewhere = folder("link-target");
    std::fs::write(elsewhere.join("theirs.md"), section()).unwrap();
    std::os::unix::fs::symlink(elsewhere.join("theirs.md"), dir.join("AGENTS.md")).unwrap();
    let (state, said) = asked(&dir);
    std::fs::remove_dir_all(&elsewhere).ok();
    assert_eq!(state, State::CannotTell);
    assert!(said.contains("is a link"), "{said}");
}
