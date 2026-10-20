//! Values no sensible form would take, saved through `owned.create` (V2.2.1, backlog 0489): a long text stored whole
//! is a finding and refused is credited in part; a declared range is credited in part only when both values outside
//! it are refused and one inside is saved, and a value outside it saved is a finding.

use super::fake_app::*;
use super::*;

fn found(o: &Outcome, rule: &str) -> Option<Finding> {
    o.findings.iter().find(|f| f.rule_id == rule).cloned()
}

/// `users()`, with a `priority` field in the create form and `limits` holding it to 1 to 5.
fn with_priority(limits: Vec<sv_manifest::FieldLimit>) -> UsersSection {
    let mut u = users();
    let owned = u.owned.as_mut().expect("users() has an owned record");
    owned
        .create
        .form
        .insert("priority".to_owned(), "3".to_owned());
    owned.limits = limits;
    u
}

fn one_to_five() -> Vec<sv_manifest::FieldLimit> {
    vec![sv_manifest::FieldLimit {
        field: "priority".to_owned(),
        min: 1,
        max: 5,
    }]
}

fn app() -> FakeApp {
    FakeApp::new(Flaws::default())
}

#[test]
fn a_long_text_kept_whole_is_found_and_one_refused_is_credited_in_part() {
    let mut any_length = app();
    any_length.longest_note = None;
    let (o, _) = run_seeded(any_length, &users());
    let f = found(&o, INPUT_LENGTH.rule_id).unwrap_or_else(|| panic!("{:?}", o.steps));
    assert_eq!(f.requirement_ids, ["V2.2.1"]);
    assert_eq!(f.severity, Severity::Low);
    assert_eq!(f.evidence, ["input-long-create", "input-long-read"]);
    assert!(!verified_ids(&o).contains(&INPUT_LENGTH.rule_id));

    let (o, _) = run_seeded(app(), &users());
    assert!(
        found(&o, INPUT_LENGTH.rule_id).is_none(),
        "{:?}",
        o.findings
    );
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == INPUT_LENGTH.rule_id)
        .unwrap_or_else(|| panic!("{:?}", o.steps));
    assert!(credit.in_part, "{credit:?}");
    // The other records were saved as before: only the long one was refused.
    assert!(found(&o, STORED_HTML.rule_id).is_none());
}

#[test]
fn a_value_outside_the_declared_range_saved_is_found() {
    let (o, _) = run_seeded(app(), &with_priority(one_to_five()));
    let f = found(&o, INPUT_RANGE.rule_id).unwrap_or_else(|| panic!("{:?}", o.steps));
    assert!(
        f.description.contains("set to 0 and 6"),
        "{}",
        f.description
    );
    assert_eq!(f.evidence.len(), 4, "{:?}", f.evidence);
    assert!(!verified_ids(&o).contains(&INPUT_RANGE.rule_id));
}

#[test]
fn a_range_held_on_the_server_is_credited_in_part_for_the_declared_field_only() {
    let mut careful = app();
    careful.priority_range = Some((1, 5));
    let (o, _) = run_seeded(careful, &with_priority(one_to_five()));
    assert!(found(&o, INPUT_RANGE.rule_id).is_none(), "{:?}", o.findings);
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == INPUT_RANGE.rule_id)
        .unwrap_or_else(|| panic!("{:?}", o.steps));
    assert!(
        credit.in_part && credit.scope.contains("`priority` (1 to 5)"),
        "{credit:?}"
    );
}

#[test]
fn a_refusal_of_the_value_inside_the_range_too_credits_nothing() {
    // The control: an app that refuses 1, the least the range declares, would look like one holding the range when it
    // holds another. It still saves the base record (priority 3), so the check runs.
    let mut refuses = app();
    refuses.priority_range = Some((2, 5));
    let (o, _) = run_seeded(refuses, &with_priority(one_to_five()));
    assert!(!verified_ids(&o).contains(&INPUT_RANGE.rule_id));
    assert!(found(&o, INPUT_RANGE.rule_id).is_none());
    assert!(
        o.not_assessed
            .iter()
            .any(|(id, why)| id == "V2.2.1" && why.contains("inside it, was not saved")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_limit_on_a_field_the_form_does_not_send_is_said_and_not_tried() {
    let mut u = users();
    u.owned.as_mut().unwrap().limits = one_to_five();
    let (o, _) = run_seeded(app(), &u);
    assert!(
        o.not_assessed
            .iter()
            .any(|(id, why)| id == "V2.2.1" && why.contains("`owned.create` does not send")),
        "{:?}",
        o.not_assessed
    );
    assert!(found(&o, INPUT_RANGE.rule_id).is_none());
}

#[test]
fn the_far_end_coming_back_is_what_shows_the_text_kept_whole() {
    let answer = |body: &str| {
        Some(ProbeResponse {
            id: "r".into(),
            status: 200,
            headers: Vec::new(),
            body: body.into(),
        })
    };
    let whole = long_value();
    assert_eq!(came_back(&answer(&whole)), Came::Whole);
    assert_eq!(came_back(&answer(&whole[..5000])), Came::CutShort);
    assert_eq!(came_back(&answer("nothing here")), Came::NotSeen);
    // A refusal shows nothing, whatever its body repeats.
    let mut refused = answer(&whole);
    refused.as_mut().unwrap().status = 422;
    assert_eq!(came_back(&refused), Came::NotSeen);
}
