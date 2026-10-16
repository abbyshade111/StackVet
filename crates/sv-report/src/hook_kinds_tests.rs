//! Backlog 0607: a claim about security the tool ended a turn with, set beside what this report shows for the
//! requirements it speaks to, in `sv`'s words, never the tool's.

use crate::hook::{CLAIM_KINDS, HookSeen, SaidKind, sentences};
use crate::{RequirementLine, Status};

fn requirement(id: &str, status: Status) -> RequirementLine {
    RequirementLine {
        id: id.into(),
        description: String::new(),
        chapter: String::new(),
        level: 1,
        status,
        findings: Vec::new(),
        information: Vec::new(),
        checked_by: Vec::new(),
        supported_by: Vec::new(),
        tested_by: Vec::new(),
        documented_by: Vec::new(),
        attested_by: Vec::new(),
        by_hand: Vec::new(),
        withheld_by: Vec::new(),
        whose_word: None,
    }
}

#[test]
fn a_claim_is_set_beside_this_report_s_verdict_for_each_requirement_it_speaks_to() {
    let seen = HookSeen {
        said: vec![SaidKind {
            kind: "rate-limiting".into(),
            turns: 2,
        }],
        ..Default::default()
    };
    let requirements = [requirement("V2.4.1", Status::NeedsAttention)];
    let (lines, _) = sentences(&seen, &requirements);
    let line = lines
        .iter()
        .find(|l| l.contains("rate limiting"))
        .expect("the claim is said");
    assert!(line.contains("The tool ended 2 turns"), "{line}");
    assert!(line.contains("V2.4.1, needs attention"), "{line}");
    assert!(
        line.contains("V6.3.1, not one this app is held to"),
        "{line}"
    );
    assert!(line.contains("never its words"), "{line}");
}

#[test]
fn every_kind_names_requirements_and_a_topic() {
    for kind in CLAIM_KINDS {
        assert!(!kind.requirements.is_empty(), "{}", kind.kind);
        assert!(!kind.topic.is_empty(), "{}", kind.kind);
    }
}

#[test]
fn a_kind_the_report_does_not_know_says_nothing() {
    let seen = HookSeen {
        said: vec![SaidKind {
            kind: "made-up".into(),
            turns: 1,
        }],
        ..Default::default()
    };
    let (lines, _) = sentences(&seen, &[]);
    assert!(!lines.iter().any(|l| l.contains("made-up")), "{lines:?}");
}
