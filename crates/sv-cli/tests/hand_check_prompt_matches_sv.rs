//! Setup prompt 5 and `stackvet_check`'s own text for checks by hand tell the AI coding tool the same thing (backlog
//! 0603, the owner's choice of 15 October 2026): it may record a check it made by reading the code, as its own word,
//! and propose with `confirmed` that the person make it too. Until then the prompt said "never mark a check by hand as
//! done yourself" while sv's text said how to.

use std::path::Path;

fn flat(text: &str) -> String {
    text.lines()
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn the_prompt_and_sv_s_question_text_agree_on_a_check_the_tool_made() {
    let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking");
    let report = sv_cli::assemble_report(
        &app,
        &sv_cli::ReportOptions::reading_only("a test"),
        &sv_cli::Loaded::load().unwrap(),
    )
    .unwrap();
    let sv_says = flat(&sv_report::interview::text(&report));
    // The setup: the example app has checks by hand, or the text below is not given and proves nothing.
    assert!(
        sv_says.contains("A check you made yourself, reading the code, is by = \"ai-tool\""),
        "{sv_says}"
    );
    assert!(
        sv_says.contains("add confirmed = { by = \"ai-tool\""),
        "{sv_says}"
    );

    let page = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/prompts/setup.md"),
    )
    .unwrap();
    let prompt = flat(&page);
    assert!(
        prompt.contains("for a check you made reading the code (its lines in `how`; it counts as your word, not mine)"),
        "prompt 5 no longer lets the tool record a check it made, which sv's own text does"
    );
    assert!(
        !prompt.contains("Never mark a check by hand as done yourself"),
        "prompt 5 forbids again what sv's own text tells the tool to do"
    );
    assert!(
        prompt.contains("`confirmed = { by = \"ai-tool\""),
        "the proposal is gone from prompt 5"
    );
}
