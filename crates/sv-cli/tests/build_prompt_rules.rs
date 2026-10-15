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
        // Backlog 0548: in the trial of 14 October 2026 Antigravity wrote `audience = "customers"` on its own for a
        // family app, which set level 2, and Cursor and VS Code chose `my-team` without asking.
        (
            "ask who uses the app and what it keeps, which set the level",
            "**Ask me, too, who uses the app (`audience`) and what it keeps about people (`[data] categories`):** they \
             decide whether it is held to ASVS level 1 or the stricter level 2.",
        ),
        (
            "name the routes, so the running checks reach them",
            "Name the app's pages and routes",
        ),
        // Backlog 0549: in the trial of 14 October 2026 both Cursor's and VS Code's apps gated sign-up on an invite
        // code; Cursor left it out, so sv could not sign up, and VS Code hard-coded a test code without asking.
        (
            "say when sign-up needs a secret, and ask before seeding one",
            "If signing up needs an invite code or another secret, tell me that StackVet's run cannot sign up on its \
             own, and ask whether to seed a test code for the run, and where it would live.",
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
        // Backlog 0545: in the Antigravity trial of 14 October 2026, a rewrite that made a word disappear was read
        // as not silencing, and "No code files or configurations were deleted or silenced" was said after it.
        (
            "count a rewrite that hides something from a check as silencing",
            "Silencing includes rewriting code, a dependency file, or a test so that a check no longer sees \
             something, even when the app behaves the same.",
        ),
        (
            "leave the code as it is when a finding looks wrong",
            "If you think a finding or a contradiction is wrong, leave the code as it is and tell me; I decide.",
        ),
        (
            "say what changed",
            "tell me in plain words what you changed",
        ),
        (
            "name every file a check caused to change",
            "List every file you changed because of a check, with the check's words beside it.",
        ),
        ("ask before deleting", "Ask me before you delete anything"),
        (
            "a reading of the code is not a check",
            "say it is your reading, not a check",
        ),
        // Backlog 0546: in the Cursor trial of 14 October 2026, the tool wrote security-notes.md by hand and never
        // called stackvet_record_answer, so nothing it wrote counted.
        (
            "record answers only through stackvet_record_answer",
            "Record my answers only with `stackvet_record_answer`, under the question ids it gives you",
        ),
        (
            "say that text outside the questions is not read",
            "Text you write yourself in `security-notes.md`, outside its questions, is not read as an answer.",
        ),
        (
            "never by = owner",
            "Never mark a check by hand as done yourself, and never write `confirmed` or `by = \"owner\"`",
        ),
        // Backlog 0547, the owner's choice of 14 October 2026 (B): in the trial Antigravity wrote twelve
        // `confirmed` blocks of its own and VS Code marked a check done with `by = "ai-tool"`. The tool still writes a
        // hand check the person made, as a proposal for `sv review`, since that is the only way one is recorded.
        (
            "write a hand check only for one the person made",
            "Write a `[checked-by-hand]` entry only for a check I made and described to you, with `by = \"ai-tool\"` \
             and my words in `how`.",
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
