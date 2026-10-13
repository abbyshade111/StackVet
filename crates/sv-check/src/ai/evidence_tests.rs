//! The AI checks' findings name the questions whose answers showed them (backlog 0381), and those questions are
//! recorded, so "What StackVet saw" can quote them. Before, every `probe.ai-*` finding carried no evidence and the
//! suite's exchanges were not kept at all.

use super::tests::{Flaws, ask_recorded, one_flaw};
use super::*;

#[test]
fn each_fault_names_questions_that_were_recorded() {
    for (flaws, rule) in [
        (one_flaw("unbounded"), UNBOUNDED.rule_id),
        (one_flaw("passes_replies_on"), LEAKED.rule_id),
        (one_flaw("fetches_images"), FETCHED.rule_id),
    ] {
        let (outcome, recorded) = ask_recorded(flaws);
        // The setup: the suite ran, its questions were recorded, and the fault was found.
        assert!(!recorded.is_empty(), "{rule}: nothing was recorded");
        let f = outcome
            .findings
            .iter()
            .find(|f| f.rule_id == rule)
            .unwrap_or_else(|| panic!("{rule} was not found"));
        assert!(!f.evidence.is_empty(), "{rule}: no question named");
        let ids: Vec<&str> = recorded.iter().map(|r| r.id.as_str()).collect();
        for id in &f.evidence {
            assert!(
                ids.contains(&id.as_str()),
                "{rule} names {id}, which was not recorded: {ids:?}"
            );
        }
    }
}

#[test]
fn every_question_is_kept_under_its_kind_and_number() {
    let (_, recorded) = ask_recorded(Flaws::default());
    // The first question of the suite is the plain message, number 1.
    assert!(
        recorded.iter().any(|r| r.id == "ai-plain-1"),
        "{:?}",
        recorded.iter().map(|r| &r.id).collect::<Vec<_>>()
    );
    // And a number asked twice, after a limiter's wait, is kept apart rather than overwritten.
    let mut ids: Vec<&str> = recorded.iter().map(|r| r.id.as_str()).collect();
    let all = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), all, "every recorded question has its own id");
}
