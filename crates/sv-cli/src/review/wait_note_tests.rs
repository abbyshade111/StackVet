//! Backlog 0675: before `sv review` looks for the line an entry is about, it says so, and in the terminal every
//! question is asked through `typed_ahead`, so what was typed before it appeared is not its answer.

use super::tests::{HEAD, LINE, Scratch, WHY, proposal, with_app};
use super::typed_ahead;
use std::sync::atomic::Ordering;

#[test]
fn the_wait_for_an_entrys_line_is_said_before_the_line_and_the_question() {
    let s = Scratch::new("wait-note");
    with_app(&s, &format!("{HEAD}{}", proposal(WHY)));
    let (result, out) = s.run("owner\n");
    result.unwrap();
    let note = out
        .find("Reading app.py to find the line this entry is about...")
        .unwrap_or_else(|| panic!("no line before the wait: {out}"));
    let line = out
        .find(&format!("Line 2: {LINE}"))
        .expect("the setup: the line is found and shown");
    let question = out
        .find("Type your name, or `owner`")
        .expect("the question");
    assert!(note < line && line < question, "{out}");
}

#[test]
fn in_the_terminal_every_question_drops_what_was_typed_ahead() {
    let s = Scratch::new("typed-ahead-wiring");
    with_app(&s, &format!("{HEAD}{}", proposal(WHY)));
    let before = typed_ahead::DROPS.load(Ordering::Relaxed);
    // As `cmd_review` sets it in a terminal. Here stdin is not the input read, and dropping from it changes nothing.
    typed_ahead::ON.store(true, Ordering::Relaxed);
    let (result, out) = s.run("owner\n");
    typed_ahead::ON.store(false, Ordering::Relaxed);
    result.unwrap();
    assert!(
        out.contains("Recorded 1 of 1"),
        "the setup: a question was asked and answered: {out}"
    );
    assert!(
        typed_ahead::DROPS.load(Ordering::Relaxed) > before,
        "no question dropped what was typed ahead"
    );
}
