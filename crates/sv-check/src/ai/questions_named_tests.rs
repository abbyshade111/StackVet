//! The eight AI findings that named no question until 20 October 2026 now name the questions whose answers showed them
//! (backlog 0381), so "What StackVet saw" quotes those answers: the burst past the owner's limit, a saved injection,
//! an agent's endless loop, the kill switch ignored, a flagged injection passed on, an incomplete record of a model
//! call, and the test MCP server's two results reaching the model.

use super::tests::{
    Flaws, Logs, ask_and_read, ask_limited, ask_mcp, ask_switched, call_markers, notes_run,
    record_run,
};
use super::*;

/// The questions a finding of `rule` in `o` names, or a panic saying the finding was not made.
fn named(o: &Outcome, rule: &str) -> Vec<String> {
    o.findings
        .iter()
        .find(|f| f.rule_id == rule)
        .unwrap_or_else(|| panic!("{rule} was not found: {:?}", o.steps))
        .evidence
        .clone()
}

#[test]
fn the_burst_names_its_first_and_last_messages() {
    let o = ask_limited(Flaws::default(), 10);
    assert_eq!(
        named(&o, UNLIMITED.rule_id),
        ["ai-plain-100", "ai-plain-110"]
    );
}

#[test]
fn a_saved_injection_names_the_question_that_found_it() {
    let mut flaws = Flaws::default();
    flaws.notes_unscreened = true;
    assert_eq!(
        named(&notes_run(flaws), STORED_INJECTION.rule_id),
        ["ai-recall-38"]
    );
}

#[test]
fn an_endless_loop_names_its_question() {
    let mut f = Flaws::default();
    f.tool_marked_read_only = true;
    f.unbounded_tool_loop = true;
    let o = record_run(f, true);
    assert_eq!(named(&o, AGENT_UNBOUNDED.rule_id), ["ai-fetchloop-26"]);
}

#[test]
fn an_ignored_kill_switch_names_the_message_sent_to_the_switched_copy() {
    let mut second = Flaws::default();
    second.ignores_kill_switch = true;
    let o = ask_switched(Flaws::default(), second, true);
    assert_eq!(named(&o, KILL_SWITCH.rule_id), ["ai-kill-switch"]);
}

#[test]
fn a_flag_passed_on_names_the_injection_and_the_plain_message_after_it() {
    let mut o = Outcome::default();
    let markers = LogMarkers {
        injection_reached: Some(true),
        ..call_markers()
    };
    logged(
        &markers,
        "2026-10-09T12:00:01Z WARN prompt injection detected abc123",
        &mut o,
    );
    assert_eq!(
        named(&o, FLAG_NOT_GATING.rule_id),
        ["ai-inject-4", "ai-plain-5"]
    );
}

#[test]
fn an_incomplete_record_of_a_call_names_the_plain_message_that_made_it() {
    let o = ask_and_read(Flaws::default(), Logs::NoProvider);
    assert_eq!(named(&o, CALL_LOG.rule_id), ["ai-plain-1"]);
}

#[test]
fn each_mcp_result_reaching_the_model_names_its_own_question() {
    let mut f = Flaws::default();
    f.mcp_unvalidated = true;
    let bad = ask_mcp(f);
    assert_eq!(named(&bad, MCP_UNVALIDATED.rule_id), ["ai-mcpbad-7"]);
    let mut f = Flaws::default();
    f.mcp_unscreened = true;
    let inject = ask_mcp(f);
    assert_eq!(named(&inject, MCP_UNSCREENED.rule_id), ["ai-mcpinject-8"]);
}
