//! A finding names the answer it rests on (backlog 0442). A question answered by a rate limiter is waited out and
//! asked again, and the recording keeps the second answer as `…-2`; the finding named the first, so "What StackVet
//! saw" quoted the limiter's 429 or 503 and not the answer that was judged. And the finding on an injection that
//! got through in another language named all four, whichever got through.

use super::tests::{Flaws, ask_recorded, found};
use super::*;

fn flaws(set: impl FnOnce(&mut Flaws)) -> Flaws {
    let mut flaws = Flaws::default();
    set(&mut flaws);
    flaws
}

fn answer_of(recorded: &[crate::signed_in::recording::Recorded], id: &str) -> Option<u16> {
    recorded.iter().find(|r| r.id == id).and_then(|r| r.status)
}

#[test]
fn a_second_ask_is_kept_under_the_id_a_finding_names() {
    // The recording's own numbering, which `asked_again` names.
    let id = question_id("PLAIN", 32);
    assert_eq!(asked_again(&id), "ai-plain-32-2");
}

#[test]
fn the_failure_finding_after_a_wait_names_the_answer_it_judged() {
    // Busy for a moment after its service fails, then down for good: the plain message after the failure is answered
    // by a limiter, waited out, asked again, and answered 500.
    let (outcome, recorded) = ask_recorded(flaws(|f| {
        f.busy_after_model_error = 5;
        f.down_after_model_error = true;
    }));
    let f = outcome
        .findings
        .iter()
        .find(|f| f.rule_id == FAILURE_HANDLED.rule_id)
        .unwrap_or_else(|| panic!("{:?}", outcome.steps));
    // The setup: the first ask was the limiter's, and it stays recorded.
    assert_eq!(
        answer_of(&recorded, "ai-plain-32"),
        Some(503),
        "{recorded:?}"
    );
    assert_eq!(
        answer_of(&recorded, "ai-plain-32-2"),
        Some(500),
        "{recorded:?}"
    );
    assert_eq!(
        f.evidence,
        ["ai-fail-31", "ai-plain-32-2"],
        "the finding names the answer it rests on, not the limiter's"
    );
}

#[test]
fn the_failure_finding_with_no_wait_names_the_one_ask() {
    let (outcome, recorded) = ask_recorded(flaws(|f| {
        f.down_after_model_error = true;
    }));
    let f = outcome
        .findings
        .iter()
        .find(|f| f.rule_id == FAILURE_HANDLED.rule_id)
        .unwrap_or_else(|| panic!("{:?}", outcome.steps));
    assert!(answer_of(&recorded, "ai-plain-32-2").is_none());
    assert_eq!(f.evidence, ["ai-fail-31", "ai-plain-32"]);
}

#[test]
fn the_error_shown_names_only_the_failed_question() {
    // The error is in the failed question's answer; the plain message after it shows nothing of it.
    let (outcome, _) = ask_recorded(flaws(|f| {
        f.passes_model_error = true;
    }));
    let f = outcome
        .findings
        .iter()
        .find(|f| f.rule_id == FAILURE_SHOWN.rule_id)
        .unwrap_or_else(|| panic!("{:?}", outcome.steps));
    assert_eq!(f.evidence, ["ai-fail-31"]);
}

#[test]
fn the_languages_finding_names_only_the_questions_that_got_through() {
    let (outcome, _) = ask_recorded(flaws(|f| {
        f.screen_misses_base64 = true;
    }));
    assert!(
        found(&outcome).contains(&LANGUAGES.rule_id),
        "{:?}",
        outcome.steps
    );
    let f = outcome
        .findings
        .iter()
        .find(|f| f.rule_id == LANGUAGES.rule_id)
        .unwrap();
    // Base64 is the fourth of the four, number 14.
    assert_eq!(f.evidence, ["ai-inject-14"], "{}", f.description);
}
