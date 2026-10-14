//! The setup page's prompt 7, "Record my answers as mine" (backlog 0474): the step that turns the AI tool's word into
//! the person's is the one most likely to be skipped, so the tool walks the person to it. It cannot run `sv review`
//! (terminal only, by design), so the prompt has it list what is waiting, give the one command, wait, and check
//! again; and it never has the tool write an answer down as the person's.

use std::path::Path;
use std::process::Command;

fn page() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join("docs/prompts/setup.md")).unwrap()
}

/// Prompt 7 as one line of words, as the AI tool reads it. The markers are asserted, so a renamed one fails here.
fn prompt_seven() -> String {
    let page = page();
    let start = page
        .find("<!-- setup-step-7:start -->")
        .expect("prompt 7's start marker");
    let end = page
        .find("<!-- setup-step-7:end -->")
        .expect("prompt 7's end marker");
    assert!(start < end, "the markers in order");
    page[start..end]
        .lines()
        .filter(|l| l.starts_with('>'))
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn prompt_seven_comes_after_prompt_six_and_is_in_the_table() {
    let page = page();
    let six = page.find("<!-- setup-step-6:start -->").expect("prompt 6");
    let seven = page.find("<!-- setup-step-7:start -->").expect("prompt 7");
    assert!(six < seven, "prompt 7 after prompt 6");
    assert!(
        page.contains("| 7. Record my answers as mine |"),
        "the table names prompt 7"
    );
    assert!(
        page.contains("## 7. Record my answers as mine"),
        "its own section"
    );
}

#[test]
fn prompt_seven_walks_the_person_to_sv_review_and_never_records_for_them() {
    let text = prompt_seven();
    for said in [
        "Read stackvet.toml and security-notes.md",
        "one at a time and in plain words",
        "only I can do that, with `sv review`",
        "`sv review .`",
        "`stackvet review .` in PowerShell",
        "say that you cannot run it for me, and wait",
        "run stackvet_check again and tell me what changed",
    ] {
        assert!(text.contains(said), "{said}: {text}");
    }
    // It must never have the tool write an answer down as the person's.
    for never in ["by = \"owner\"", "Written by: owner", "Written by: me"] {
        assert!(!text.contains(never), "{never} in prompt 7: {text}");
    }
    // The command it gives is one `sv` takes.
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(["review", "--help"])
        .output()
        .expect("sv runs");
    assert!(
        out.status.success(),
        "`sv review` is not a command sv takes"
    );
}
