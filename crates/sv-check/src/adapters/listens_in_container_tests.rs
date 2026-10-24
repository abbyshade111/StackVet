//! Backlog 0658, ADR-111 (Later): Bandit's B104 in an app stackvet.toml runs in a container is listed apart as worth a
//! look, and still counts; anywhere else it is a thing to fix.

use super::*;
use crate::finding::{Confidence, Finding, Location, Outranked, Severity};

fn b104(rule: &str) -> Finding {
    Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: rule.into(),
        title: "Possible binding to all interfaces".into(),
        severity: Severity::Medium,
        confidence: Confidence::Medium,
        location: Location {
            file: "app.py".into(),
            line: 25,
        },
        secret: None,
        requirement_ids: vec!["V13.4.6".into()],
        cwe: vec!["CWE-605".into()],
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    }
}

#[test]
fn listening_everywhere_in_a_container_is_worth_a_look_and_still_counts() {
    let mut findings = vec![b104(LISTENS_EVERYWHERE), b104("bandit.B108")];
    set_apart_listening_in_container(&mut findings, true);
    assert_eq!(findings[0].outranked, Some(Outranked::ListensInContainer));
    assert!(findings[0].apart(), "listed apart");
    assert!(findings[0].withholds_credit(), "and still counted");
    assert_eq!(findings[1].outranked, None, "another rule is untouched");
}

#[test]
fn outside_a_container_or_seen_by_another_tool_it_is_a_thing_to_fix() {
    let mut alone = vec![b104(LISTENS_EVERYWHERE)];
    set_apart_listening_in_container(&mut alone, false);
    assert_eq!(alone[0].outranked, None, "no [stack.run]");
    let mut agreed = b104(LISTENS_EVERYWHERE);
    agreed.also_reported_by = vec!["semgrep.python.lang.security.audit.bind-all".into()];
    let mut agreed = vec![agreed];
    set_apart_listening_in_container(&mut agreed, true);
    assert_eq!(agreed[0].outranked, None, "another tool found it too");
}
