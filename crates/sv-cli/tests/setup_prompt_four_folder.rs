//! Setup prompt 4 tells the AI tool not to read or use a folder `stackvet_status` names when it is not this one
//! (backlog 0592): in the Cursor run of 15 October 2026 the tool's account-wide settings still served the last trial's
//! app, and the tool read that app and copied it into the new folder as its own work.

use std::path::Path;

fn prompt_four() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let page = std::fs::read_to_string(root.join("docs/prompts/setup.md")).unwrap();
    let start = page
        .find("<!-- setup-step-4:start -->")
        .expect("start marker");
    let end = page.find("<!-- setup-step-4:end -->").expect("end marker");
    // One line of words, as the AI tool reads it.
    page[start..end]
        .lines()
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn prompt_4_says_not_to_read_or_use_another_apps_folder() {
    let prompt = prompt_four();
    assert!(
        prompt.contains("which folder it says it serves"),
        "the setup: prompt 4 asks which folder the server serves: {prompt}"
    );
    for words in [
        "Then stop and tell me, and do not read or use the folder it names",
        "it holds an earlier app, not this one",
    ] {
        assert!(
            prompt.contains(words),
            "{words:?} is missing from prompt 4: {prompt}"
        );
    }
}

#[test]
fn the_trial_write_up_says_the_tools_own_settings_are_part_of_the_blank_slate() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let page =
        std::fs::read_to_string(root.join("docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md"))
            .unwrap();
    let start = page
        .find("## The blank slate, for any later trial")
        .expect("the section");
    let section = &page[start..start + page[start..].find("\n## ").unwrap_or(page.len() - start)];
    for words in [
        "MCP settings",
        "for the whole account",
        "must name the new folder before prompt 5",
    ] {
        assert!(section.contains(words), "{words:?}: {section}");
    }
}
