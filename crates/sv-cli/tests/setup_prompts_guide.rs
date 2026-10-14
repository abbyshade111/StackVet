//! The guide's word on the setup prompts held to the setup page's (backlog 0428). The guide once said the prompts had
//! not been tried in any AI coding tool while the setup page said where they had, which sent the very person they were
//! written for to the terminal steps. This holds the guide's "Tried so far" to the page's: the same tool, computer,
//! date, and model, the same "the setup worked", the warning about the build that followed, and no tool named as
//! untried that the page says was tried.

use std::path::Path;

fn read(relative: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(relative)).unwrap()
}

/// The paragraph that starts with `opening`, as one line of words. The setup is asserted: an opening renamed would
/// otherwise leave the test reading nothing.
fn paragraph(text: &str, opening: &str) -> String {
    let start = text
        .find(opening)
        .unwrap_or_else(|| panic!("a paragraph starting {opening:?}"));
    let rest = &text[start..];
    let end = rest.find("\n\n").unwrap_or(rest.len());
    rest[..end].split_whitespace().collect::<Vec<_>>().join(" ")
}

fn guide() -> String {
    paragraph(
        &read("docs/GETTING-STARTED.md"),
        "**Or let your AI coding tool do steps 1 to 3.**",
    )
}

fn page() -> String {
    paragraph(&read("docs/prompts/setup.md"), "**Tried so far:**")
}

/// Where, on what, when, and with which model: the page's first sentence up to how far the trial went.
fn trial(page: &str) -> String {
    let after = page.strip_prefix("**Tried so far:** ").unwrap();
    let end = after
        .find(", from ")
        .expect("the page says how far the trial went");
    after[..end].to_owned()
}

#[test]
fn the_guide_names_the_trial_the_setup_page_names() {
    let (guide, page) = (guide(), page());
    let trial = trial(&page);
    assert!(
        trial.starts_with("in ") && trial.contains(" 20"),
        "the page's trial names a tool and a date: {trial:?}"
    );
    assert!(
        guide.contains(&format!("**Tried so far:** {trial}.")),
        "the guide's \"Tried so far\" says what the setup page's does ({trial:?}): {guide}"
    );
    assert!(
        !guide.contains("not yet been tried in any"),
        "the guide says the prompts were never tried, the page says where they were: {guide}"
    );
}

#[test]
fn the_guide_keeps_both_halves_of_what_the_trial_showed() {
    let (guide, page) = (guide(), page());
    assert!(page.contains("The setup worked."), "{page}");
    assert!(guide.contains("The setup worked."), "{guide}");
    // The warning about the build is the half that must not be lost.
    assert!(page.contains("we don't recommend"), "{page}");
    assert!(
        guide.contains("build that followed went badly") && guide.contains("does not recommend"),
        "the guide keeps the warning about the build: {guide}"
    );
}

#[test]
fn no_tool_the_guide_calls_untried_is_one_the_page_says_was_tried() {
    let (guide, page) = (guide(), page());
    let marker = "have not yet been tried in ";
    let start = guide
        .find(marker)
        .expect("the guide names the tools not yet tried")
        + marker.len();
    let list = &guide[start..start + guide[start..].find('.').unwrap()];
    let tools: Vec<&str> = list
        .split(", ")
        .map(|t| t.trim_start_matches("or ").trim())
        .collect();
    assert!(tools.len() >= 2, "a list of tools: {list:?}");
    for tool in tools {
        assert!(
            !page.contains(tool),
            "the guide calls {tool} untried, and the setup page's trial names it: {page}"
        );
    }
}
