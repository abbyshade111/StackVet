//! The AI checks keep the prompt `sv` wrote beside each answer (backlog 0623), so "What StackVet saw" shows what was
//! asked as well as what came back: every question the suite asks, the hidden-input ones and the injections among
//! them, comes back from the recorder with the prompt it composed.

use super::tests::{Flaws, ask_recorded};

#[test]
fn every_ai_question_keeps_the_prompt_sv_wrote_for_it() {
    let (_, recorded) = ask_recorded(Flaws::default());
    let ai: Vec<_> = recorded
        .iter()
        .filter(|r| r.id.starts_with("ai-"))
        .collect();
    // The setup: the suite asked its questions through the recorder.
    assert!(
        ai.len() >= 10,
        "{:?}",
        recorded.iter().map(|r| &r.id).collect::<Vec<_>>()
    );
    for r in &ai {
        let prompt = r
            .prompt
            .as_deref()
            .unwrap_or_else(|| panic!("{} kept no prompt", r.id));
        // Each prompt carries the marker the suite writes for its question's tag, so it is the one composed for it.
        assert!(prompt.contains("SV-PROBE-"), "{}: {prompt}", r.id);
    }
    // The plain message, word for word as the suite wrote it.
    let plain = ai
        .iter()
        .find(|r| r.id == "ai-plain-1")
        .expect("the plain message was asked");
    assert!(
        plain
            .prompt
            .as_deref()
            .is_some_and(|p| p.starts_with("Hello, what can you help me with? ")),
        "{:?}",
        plain.prompt
    );
}
