//! `docs/prompts/design-time.md` shows each design-time prompt word for word as `data/design-prompts.json` has it, the
//! file `sv` serves them from. The page is written by hand; by 11 October 2026 it still said `securevibe.toml` in every
//! prompt, two weeks after the rename, and nothing had noticed (backlog 0034, part 5).

use std::path::PathBuf;

fn read(path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The quoted prompt under a page's `### title`, with the quote marks taken off: `None` when the title is not there.
fn shown(page: &str, title: &str) -> Option<String> {
    let start = page.find(&format!("\n### {title}\n\n"))? + title.len() + 7;
    let lines: Vec<&str> = page[start..]
        .lines()
        .take_while(|l| l.starts_with('>'))
        .map(|l| l.strip_prefix("> ").unwrap_or(l.trim_start_matches('>')))
        .collect();
    (!lines.is_empty()).then(|| lines.join("\n"))
}

#[test]
fn every_design_time_prompt_is_shown_on_the_page_as_the_data_file_has_it() {
    let page = read("docs/prompts/design-time.md");
    let data: serde_json::Value = serde_json::from_str(&read("data/design-prompts.json")).unwrap();
    let prompts = data["prompts"].as_array().expect("the prompts");
    // The setup: the file holds the prompts, so an empty list cannot pass for agreement.
    assert!(prompts.len() >= 14, "{} prompts", prompts.len());
    for prompt in prompts {
        let title = prompt["title"].as_str().unwrap();
        let text = prompt["prompt"].as_str().unwrap();
        let on_page = shown(&page, title)
            .unwrap_or_else(|| panic!("the page has no prompt under \"### {title}\""));
        assert_eq!(
            on_page, text,
            "the page and data/design-prompts.json differ on \"{title}\""
        );
    }
}

#[test]
fn a_prompt_that_differs_by_one_word_is_told_apart() {
    // The comparison itself can fail: the reading of the page is not so loose that anything matches.
    let page = "intro\n\n### A title\n\n> Line one,\n> line two.\n>\n> Line four.\n\nAfter.\n";
    assert_eq!(
        shown(page, "A title").as_deref(),
        Some("Line one,\nline two.\n\nLine four.")
    );
    assert_ne!(
        shown(page, "A title").as_deref(),
        Some("Line one,\nline 2.\n\nLine four.")
    );
    assert_eq!(shown(page, "Another title"), None);
}
