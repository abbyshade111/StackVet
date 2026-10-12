//! `sv review --help` says what the review records and where the whole flow is written down, and the guide section it
//! names is there (backlog 0254).

use std::path::Path;
use std::process::Command;

#[test]
fn review_help_says_what_it_records_and_names_the_guide_section() {
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["review", "--help"])
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{said}");
    for words in [
        "sv review [PATH]",
        "a finding you set aside",
        "[[finding-review]]",
        "`confirmed`",
        "security-notes.md",
        "a seal",
        "docs/GETTING-STARTED.md, section 5, \"Answer the questions\"",
    ] {
        assert!(said.contains(words), "{words:?} is missing from:\n{said}");
    }
    // The section named is really there, under that number and title.
    let guide = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/GETTING-STARTED.md"),
    )
    .unwrap();
    assert!(
        guide.contains("\n## 5. Answer the questions\n"),
        "the guide's section 5 moved"
    );
}

#[test]
fn another_commands_help_is_its_one_entry() {
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["bundle", "--help"])
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&out.stdout);
    assert!(said.starts_with("USAGE:\n  sv bundle"), "{said}");
    assert!(!said.contains("GETTING-STARTED"), "{said}");
}
