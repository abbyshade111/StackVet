//! `sv rules` and the AI coding tools' own files (ADR-144, backlog 0665). Claude Code reads `AGENTS.md`
//! only when the app has no `CLAUDE.md`, so a `CLAUDE.md` the app already has gets a marked
//! `@AGENTS.md` import; no tool's file is created, none is written through a link, and each tool that
//! will still not see the rules is named.

use std::path::{Path, PathBuf};
use std::process::Command;

fn sv(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .output()
        .expect("sv runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn folder(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-rules-tools-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn rules(dir: &Path) -> String {
    sv(&["rules", dir.to_str().unwrap()])
}

fn read(dir: &Path, name: &str) -> String {
    std::fs::read_to_string(dir.join(name)).unwrap()
}

#[test]
fn a_claude_md_the_app_has_gets_the_import_once_and_keeps_the_owners_text() {
    let dir = folder("import");
    let owner = "# Notes for Claude\n\nRun the tests with `make test`.\n";
    std::fs::write(dir.join("CLAUDE.md"), owner).unwrap();
    let said = rules(&dir);
    assert!(said.contains("Added the line `@AGENTS.md`"), "{said}");
    let written = read(&dir, "CLAUDE.md");
    assert!(written.starts_with(owner.trim_end()), "{written}");
    assert!(
        written.lines().any(|l| l == "@AGENTS.md"),
        "the import is a line of its own:\n{written}"
    );
    // The rules themselves are in AGENTS.md alone, never copied.
    assert!(!written.contains("pull_request_target"), "{written}");
    assert!(read(&dir, "AGENTS.md").contains("pull_request_target"));

    // Again: nothing changes, and the import is not added twice.
    rules(&dir);
    assert_eq!(read(&dir, "CLAUDE.md"), written);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_import_from_the_claude_folder_reaches_up_to_agents_md() {
    let dir = folder("dot-claude");
    std::fs::create_dir_all(dir.join(".claude")).unwrap();
    std::fs::write(dir.join(".claude/CLAUDE.md"), "Be brief.\n").unwrap();
    rules(&dir);
    let written = read(&dir, ".claude/CLAUDE.md");
    assert!(written.lines().any(|l| l == "@../AGENTS.md"), "{written}");
    assert!(!dir.join("CLAUDE.md").exists(), "no CLAUDE.md was created");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn no_claude_md_is_created_and_one_that_imports_already_is_left_alone() {
    let dir = folder("none");
    let said = rules(&dir);
    assert!(
        dir.join("AGENTS.md").exists(),
        "the setup: the rules were written"
    );
    assert!(!dir.join("CLAUDE.md").exists(), "{said}");
    assert!(!said.contains("Claude Code"), "{said}");

    let dir2 = folder("already");
    let owner = "@AGENTS.md\n\nUse plan mode for billing.\n";
    std::fs::write(dir2.join("CLAUDE.md"), owner).unwrap();
    let said = rules(&dir2);
    assert!(said.contains("already brings in AGENTS.md"), "{said}");
    assert_eq!(read(&dir2, "CLAUDE.md"), owner);
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_dir_all(&dir2).ok();
}

#[cfg(unix)]
#[test]
fn a_claude_md_that_is_a_link_is_not_written_through() {
    let dir = folder("link");
    let elsewhere = folder("link-target");
    let target = elsewhere.join("theirs.md");
    std::fs::write(&target, "Somebody else's file.\n").unwrap();
    std::os::unix::fs::symlink(&target, dir.join("CLAUDE.md")).unwrap();
    let said = rules(&dir);
    assert!(said.contains("CLAUDE.md is a link"), "{said}");
    assert_eq!(
        std::fs::read_to_string(&target).unwrap(),
        "Somebody else's file.\n"
    );
    assert!(
        std::fs::symlink_metadata(dir.join("CLAUDE.md"))
            .unwrap()
            .file_type()
            .is_symlink(),
        "the link is still a link"
    );
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_dir_all(&elsewhere).ok();
}

#[test]
fn each_tool_that_will_not_see_the_rules_is_named_and_its_file_left_alone() {
    let dir = folder("notes");
    let files = [
        ("CLAUDE.local.md", "Mine.\n"),
        ("AGENTS.override.md", "Override.\n"),
        (".aider.conf.yml", "model: something\n"),
        (".clinerules", "Be careful.\n"),
    ];
    for (name, text) in files {
        std::fs::write(dir.join(name), text).unwrap();
    }
    let said = rules(&dir);
    for tool in ["Claude Code:", "Codex:", "Aider:", "Cline:"] {
        assert!(said.contains(tool), "{tool} is not named:\n{said}");
    }
    assert!(said.contains("read: AGENTS.md"), "{said}");
    for (name, text) in files {
        assert_eq!(read(&dir, name), text, "{name} was changed");
    }
    assert!(!dir.join("CLAUDE.md").exists());
    std::fs::remove_dir_all(&dir).ok();
}

#[cfg(unix)]
#[test]
fn a_claude_folder_that_is_a_link_is_not_written_into() {
    let dir = folder("folder-link");
    let elsewhere = folder("folder-link-target");
    std::fs::write(elsewhere.join("CLAUDE.md"), "Somebody else's file.\n").unwrap();
    std::os::unix::fs::symlink(&elsewhere, dir.join(".claude")).unwrap();
    let said = rules(&dir);
    assert!(said.contains(".claude is a link"), "{said}");
    assert_eq!(
        std::fs::read_to_string(elsewhere.join("CLAUDE.md")).unwrap(),
        "Somebody else's file.\n"
    );
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_dir_all(&elsewhere).ok();
}
