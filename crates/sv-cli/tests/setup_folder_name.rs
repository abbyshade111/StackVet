//! Backlog 0558: the setup page and prompt 2 named `my-app` as if it were the folder's name. In the VS Code trial of
//! 14 October 2026 the tool found a folder of that kind already there and the owner had to say "choose a different
//! name". Both now say the name is the person's to choose.

use std::path::Path;

fn page() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join("docs/prompts/setup.md")).unwrap()
}

/// The text as one line of words, without the quote marks a prompt's lines start with.
fn words(text: &str) -> String {
    text.lines()
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn the_folder_may_be_called_anything_on_the_page_and_in_prompt_2() {
    let page = page();
    let start = page
        .find("<!-- setup-step-2:start -->")
        .expect("prompt 2's start marker");
    let end = page
        .find("<!-- setup-step-2:end -->")
        .expect("prompt 2's end marker");
    let prompt_2 = words(&page[start..end]);
    // The setup: prompt 2 was found, and it is the one that offers to make the folder.
    assert!(prompt_2.contains("First, the folder."), "{prompt_2}");
    assert!(
        prompt_2.contains("called `my-app` or any name I choose: ask me which"),
        "prompt 2 names `my-app` as if it were the name: {prompt_2}"
    );
    let before_prompts = words(&page[..start]);
    assert!(
        before_prompts.contains("called `my-app` or any name you like"),
        "the page names `my-app` as if it were the name"
    );
}
