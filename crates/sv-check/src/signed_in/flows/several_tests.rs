//! ADR-118 (backlog 0232, part 3): every flow stackvet.toml names is tried, and the credit is whole only when two or
//! more were named, each could be judged, and every one held.

use super::super::fake_app::*;
use super::*;

fn step(path: &str) -> RequestTemplate {
    RequestTemplate {
        method: "POST".into(),
        path: path.into(),
        form: [("csrf_token".to_owned(), "{csrf}".to_owned())].into(),
        json: Default::default(),
    }
}

/// The two-step application the stand-in app answers beside its checkout.
fn application() -> sv_manifest::FlowSection {
    sv_manifest::FlowSection {
        steps: vec![step("/apply/1"), step("/apply/2")],
        completed: "/applications/".into(),
    }
}

/// The users section with both flows under `flows`, and no `flow`.
fn two_flows() -> UsersSection {
    let mut u = users();
    let checkout = u.flow.take().expect("the stand-in names a checkout");
    u.flows = vec![checkout, application()];
    u
}

fn credit(o: &Outcome) -> Option<&crate::Verified> {
    o.verified
        .iter()
        .find(|v| v.check_id == STEP_SKIPPED.rule_id)
}

/// Both flows were really gone through in order, or nothing below means anything.
fn assert_both_finished_in_order(o: &Outcome) {
    for name in ["/checkout/3", "/apply/2"] {
        assert!(
            o.steps.iter().any(|s| s.contains("in order as A: finished")
                && s.contains(&format!("the flow ending at {name}"))),
            "{name} was not finished in order: {:?}",
            o.steps
        );
    }
}

#[test]
fn two_flows_that_both_hold_are_checked_whole() {
    let o = run_against(Flaws::default(), &two_flows());
    assert_both_finished_in_order(&o);
    let credit = credit(&o).expect("credited");
    assert!(!credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("the flow ending at /checkout/3")
            && credit.scope.contains("the flow ending at /apply/2"),
        "{}",
        credit.scope
    );
    assert!(!rule_ids(&o).contains(&STEP_SKIPPED.rule_id));
}

/// The credit rests on each flow's fresh sign-ins as B, the second flow's as well as the first's, so a crash at either
/// withholds it (ADR-021) and the credit names both (ADR-082).
#[test]
fn the_credit_rests_on_every_flows_sign_ins() {
    use crate::signed_in::recording::Recorded;
    let ids = [
        "login-flow-b0",
        "login-flow-b3",
        "login-flow-1-b0",
        "flow-a-0",
    ];
    let exchanges: Vec<Recorded> = ids
        .iter()
        .map(|id| Recorded {
            id: (*id).to_owned(),
            method: "POST".to_owned(),
            path: "/login".to_owned(),
            status: Some(303),
            headers: Vec::new(),
            body: String::new(),
            prompt: None,
        })
        .collect();
    let mut credits = vec![crate::Verified::new(
        STEP_SKIPPED.rule_id,
        &[],
        "a test".to_owned(),
    )];
    crate::signed_in::name_credits(&mut credits, &exchanges);
    assert_eq!(
        credits[0].evidence,
        ["login-flow-b0", "login-flow-b3", "login-flow-1-b0"]
    );
}

#[test]
fn two_that_hold_and_one_that_cannot_be_judged_is_still_in_part() {
    // A flow of one step has no step to skip: it is named as not judged, and keeps the credit in part even beside
    // two that held.
    let mut u = two_flows();
    u.flows.push(sv_manifest::FlowSection {
        steps: vec![step("/apply/1")],
        completed: "/applications/".into(),
    });
    let o = run_against(Flaws::default(), &u);
    assert_both_finished_in_order(&o);
    let credit = credit(&o).expect("credited");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit
            .scope
            .contains("not judged: the flow ending at /apply/1"),
        "{}",
        credit.scope
    );
}

#[test]
fn a_second_flow_that_can_be_skipped_is_found_naming_it() {
    let o = run_against(
        Flaws {
            apply_unguarded: true,
            ..Default::default()
        },
        &two_flows(),
    );
    assert_both_finished_in_order(&o);
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == STEP_SKIPPED.rule_id)
        .expect("the finding");
    assert!(
        found.description.contains("The flow ending at /apply/2"),
        "{}",
        found.description
    );
    assert!(
        found
            .evidence
            .iter()
            .all(|e| e.starts_with("login-flow-1-b")),
        "{:?}",
        found.evidence
    );
    assert!(credit(&o).is_none(), "credited beside its own finding");
}

#[test]
fn a_flow_that_cannot_be_judged_keeps_the_credit_in_part_and_is_named() {
    // The checkout never finishes, even in order: nothing can be told from skipping its steps.
    let o = run_against(
        Flaws {
            flow_broken: true,
            ..Default::default()
        },
        &two_flows(),
    );
    let credit = credit(&o).expect("credited for the application");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit
            .scope
            .contains("not judged: the flow ending at /checkout/3"),
        "{}",
        credit.scope
    );
    assert!(
        o.not_assessed
            .iter()
            .any(|(id, why)| id == "V2.3.1" && why.contains("the flow ending at /checkout/3")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn both_flow_and_flows_is_a_problem_in_the_settings() {
    let mut u = two_flows();
    u.flow = Some(application());
    assert!(
        u.problems()
            .iter()
            .any(|p| p.contains("both `flow` and `flows`")),
        "{:?}",
        u.problems()
    );
    assert!(
        two_flows().problems().is_empty(),
        "{:?}",
        two_flows().problems()
    );
}
