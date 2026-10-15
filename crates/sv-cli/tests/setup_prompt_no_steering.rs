//! The setup prompts ask the AI coding tool to put StackVet's questions as they are, with no answer of its own marked
//! as recommended, and to check the installed commit without fetching anything (backlog 0597, from the Cursor run of
//! 15 October 2026: the tool's question dialogs offered a "(Recommended)" answer on every design question, and it cloned
//! StackVet's repository into a temporary folder to compare a commit).

fn flat(path: &str) -> String {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path),
    )
    .unwrap_or_else(|e| panic!("{path}: {e}"));
    text.lines()
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn prompt_five_asks_without_recommending_and_the_guide_says_the_same() {
    let wanted = "ask me the questions one at a time, each as StackVet words it, with no answer of yours marked as \
                  recommended: if I ask what you would choose, tell me then, with the lines it rests on.";
    for page in ["docs/prompts/setup.md", "docs/GETTING-STARTED.md"] {
        assert!(
            flat(page).contains(wanted),
            "{page} no longer says: {wanted}"
        );
    }
}

#[test]
fn prompt_six_compares_the_commit_without_fetching_anything() {
    let page = flat("docs/prompts/setup.md");
    for wanted in [
        "Do not clone, download, or fetch anything to compare.",
        "compare the commit `sv --version` prints with the one the install printed, fetching nothing to do it: if \
         they differ, stop and tell me.",
    ] {
        assert!(
            page.contains(wanted),
            "the setup page no longer says: {wanted}"
        );
    }
}
