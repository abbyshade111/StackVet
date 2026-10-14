//! A threat checked in part counts its requirements checked only in part apart from those checked whole (backlog 0447).
//! Backlog 0333 made "checked in part" say how many of the threat's requirements were checked; a requirement that was
//! itself only checked in part (one sample, ADR-053) was counted with them, so "2 of 3 requirements checked" could mean
//! two requirements each checked in part.

use std::path::PathBuf;
use sv_frameworks::{Condition, ConditionContext};
use sv_report::threats::{ThreatRules, ThreatStatus, evaluate};
use sv_report::{RequirementLine, Status};

fn rules() -> ThreatRules {
    ThreatRules::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/knowledge/threats.json"),
    )
    .expect("the threat rules load")
}

fn line(id: &str, status: Status) -> RequirementLine {
    RequirementLine {
        id: id.into(),
        description: String::new(),
        chapter: String::new(),
        level: 1,
        status,
        findings: Vec::new(),
        information: Vec::new(),
        checked_by: Vec::new(),
        tested_by: Vec::new(),
        supported_by: Vec::new(),
        documented_by: Vec::new(),
        attested_by: Vec::new(),
        by_hand: Vec::new(),
        withheld_by: Vec::new(),
        whose_word: None,
    }
}

/// What the threat table says for T-01 (sign-in) with its three requirements at level 1 as given.
fn shown(statuses: [Status; 3]) -> String {
    let mut ctx = ConditionContext::default();
    ctx.set(Condition::from_name("auth").unwrap(), true);
    let requirements: Vec<RequirementLine> = ["V6.2.1", "V6.2.4", "V7.2.3"]
        .iter()
        .zip(statuses)
        .map(|(id, status)| line(id, status))
        .collect();
    let lines = evaluate(&rules(), &ctx, &requirements);
    let t = lines.iter().find(|t| t.id == "T-01").expect("T-01");
    assert_eq!(t.status, ThreatStatus::CheckedInPart, "the setup");
    t.shown()
}

#[test]
fn a_requirement_checked_in_part_is_not_counted_as_checked() {
    assert_eq!(
        shown([
            Status::CheckedInPart,
            Status::CheckedInPart,
            Status::NotVerified
        ]),
        "checked in part: 0 of 3 requirements checked, 2 checked in part"
    );
    assert_eq!(
        shown([Status::Checked, Status::CheckedInPart, Status::NotVerified]),
        "checked in part: 1 of 3 requirements checked, 1 checked in part"
    );
}

#[test]
fn with_none_checked_in_part_the_line_is_as_0333_made_it() {
    assert_eq!(
        shown([Status::NotVerified, Status::Checked, Status::NotVerified]),
        "checked in part: 1 of 3 requirements checked"
    );
}
