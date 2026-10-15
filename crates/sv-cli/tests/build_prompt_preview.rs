//! The build prompt (setup prompt 5, and the guide's step 4 word for word) asks before installing, keeps tests off the
//! app's own database, and has a page opened over the running server before it is called working (backlog 0602): in
//! the clean Cursor run of 15 October 2026 the tool ran `pip install` without asking, its tests wrote into the app's
//! real database, and a sign-in page that sent an empty body was reported as working because nobody opened it.

use std::path::Path;

/// Prompt 5 as one line of words, as the AI tool reads it.
fn prompt_five() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let page = std::fs::read_to_string(root.join("docs/prompts/setup.md")).unwrap();
    let start = page
        .find("<!-- setup-step-5:start -->")
        .expect("start marker");
    let end = page.find("<!-- setup-step-5:end -->").expect("end marker");
    let words = page[start..end]
        .lines()
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        words.contains("Call `stackvet_spec`"),
        "the setup: the build prompt was read"
    );
    words
}

#[test]
fn the_build_prompt_asks_before_installing_and_opens_pages_before_calling_them_working() {
    let prompt = prompt_five();
    for words in [
        "Ask me before you delete anything, and before you install anything.",
        "Your tests use a database of their own, never the app's.",
        "Before you tell me a page works, open it the way a person would, over the running server, and tell me what you \
         saw.",
    ] {
        assert!(
            prompt.contains(words),
            "{words:?} is missing from the build prompt: {prompt}"
        );
    }
}
