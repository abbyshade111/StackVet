//! What the owner's trial in Antigravity on 13 October 2026 showed the build prompt and prompt 6 must say (backlog
//! 0407): each is something the AI tool did there that a person who is not a programmer would not have caught.
//! `setup_prompts.rs` holds the guide's step 4 and the setup page's step 5 to each other word for word; this holds
//! what they must contain, so a later edit cannot drop one of these quietly.

use std::path::Path;

fn read(relative: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(relative)).unwrap()
}

/// The quoted prompt between two markers, as one line of words.
fn between(text: &str, start: &str, end: &str) -> String {
    let from = text.find(start).unwrap_or_else(|| panic!("no {start}")) + start.len();
    let to = from + text[from..].find(end).unwrap_or_else(|| panic!("no {end}"));
    text[from..to]
        .lines()
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn the_build_prompt_says_what_the_trial_showed_it_must() {
    let prompt = between(
        &read("docs/prompts/setup.md"),
        "<!-- setup-step-5:start -->",
        "<!-- setup-step-5:end -->",
    );
    assert!(prompt.len() > 500, "the setup read the prompt: {prompt}");
    for (what, words) in [
        (
            "ask before deciding where the app runs or who signs in",
            "Ask me before you decide where the app runs, whether anyone signs in",
        ),
        (
            "name the routes, so the running checks reach them",
            "Name the app's pages and routes",
        ),
        (
            "only the stackvet_ tools",
            "never run commands inside its container",
        ),
        (
            "never silence or hide a finding",
            "Never silence a finding or hide it from a scanner",
        ),
        ("no # nosec", "`# nosec`"),
        (
            "say what changed",
            "tell me in plain words what you changed",
        ),
        ("ask before deleting", "Ask me before you delete anything"),
        (
            "a reading of the code is not a check",
            "say it is your reading, not a check",
        ),
        (
            "never by = owner",
            "Never mark a check by hand as done, or write `by = \"owner\"`",
        ),
        ("never secure", "Never tell me the app is secure"),
    ] {
        assert!(
            prompt.contains(words),
            "the build prompt no longer says to {what}: {words:?}"
        );
    }
}

#[test]
fn prompt_6_opens_the_report_in_a_browser_and_asks_before_deleting() {
    let prompt = between(
        &read("docs/prompts/setup.md"),
        "<!-- setup-step-6:start -->",
        "<!-- setup-step-6:end -->",
    );
    assert!(
        prompt.contains("open `report.html` in my web browser for me"),
        "{prompt}"
    );
    assert!(prompt.contains("not in this chat"), "{prompt}");
    assert!(
        prompt.contains("Ask me before you delete the report's folder"),
        "{prompt}"
    );
}
