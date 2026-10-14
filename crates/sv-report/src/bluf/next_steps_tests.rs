//! Backlog 0424: "What to do next" sends the person to their AI coding tool for the questions and the tests, says to
//! record their own answers with `sv review` when an answer rests on the tool's word, and says to run the full report
//! when requirements only it can reach are left. Each step only when there is work for it.

use super::tests::{report, unverified};
use super::*;
use crate::{Counts, NotRunThisTime, RunStatus};

fn says<'a>(steps: &'a [NextStep], start: &str) -> Option<&'a NextStep> {
    steps.iter().find(|s| s.what.starts_with(start))
}

#[test]
fn the_tool_s_word_standing_behind_answers_brings_the_sv_review_step() {
    assert!(
        says(
            &next_steps(&report(Counts::default())),
            "Record the answers that are yours"
        )
        .is_none(),
        "a step with nothing to record"
    );
    let r = report(Counts {
        stated: 3,
        ..Counts::default()
    });
    let steps = next_steps(&r);
    let step = says(&steps, "Record the answers that are yours")
        .expect("no step for the answers resting on the tool's word");
    assert!(
        step.what
            .contains("3 requirements rest on your AI coding tool's word alone"),
        "{}",
        step.what
    );
    assert!(
        step.where_to_look.contains("`sv review`"),
        "{}",
        step.where_to_look
    );
    assert!(step.where_to_look.contains("cannot do this for you"));
}

#[test]
fn requirements_only_a_full_run_reaches_bring_the_full_run_step_with_the_command() {
    let mut r = report(Counts::default());
    r.not_run_this_time = Some(NotRunThisTime {
        kinds: vec!["the running app".to_owned()],
        only_they_reach: 0,
    });
    assert!(
        says(&next_steps(&r), "Run the full report").is_none(),
        "a step for a run that reaches nothing more"
    );
    r.not_run_this_time = Some(NotRunThisTime {
        kinds: vec!["the running app".to_owned()],
        only_they_reach: 12,
    });
    // The command as the MCP server words it, with the app's folder in it.
    let why =
        "The MCP server never starts the app; the person can, with `cd '/a/b' && sv report --run`.";
    r.run_status = Some(RunStatus::NotAsked {
        why: why.to_owned(),
    });
    let steps = next_steps(&r);
    let step = says(&steps, "Run the full report").expect("no step for the full run");
    assert!(
        step.what
            .contains("12 requirements can only be checked that way"),
        "{}",
        step.what
    );
    assert_eq!(step.where_to_look, why);
}

#[test]
fn the_questions_and_the_tests_are_the_ai_tool_s_to_ask_and_write() {
    let mut r = report(Counts::default());
    r.requirements.push(unverified("V8.3.1"));
    let steps = next_steps(&r);
    let answer =
        says(&steps, "Answer the 1 question at level 1").expect("no step for the questions");
    assert!(
        answer
            .where_to_look
            .contains("ask your AI coding tool to ask you them"),
        "{}",
        answer.where_to_look
    );
    assert!(answer.where_to_look.contains("`sv questions`"));
    assert!(
        !answer.where_to_look.contains("sv notes"),
        "the old terminal-only words"
    );
}
