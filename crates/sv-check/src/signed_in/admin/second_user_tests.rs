//! Sweep 3 (backlog 0561, ADR-119), from its inventory: the second user is shown signed in before their refusals are
//! read. A sign-in that quietly failed leaves a session of nobody, every record is refused to nobody, and V8.2.2 was
//! credited on that.

use super::super::fake_app::*;
use super::*;

/// A run in which the second user has no account in the app, so their sign-in fails, however it is answered.
fn run_without_b() -> Outcome {
    let mut app = FakeApp::new(Flaws::default());
    let acc = accounts();
    app.users
        .insert(acc.a.user.clone(), (acc.a.password.clone(), false));
    let admin = acc.admin.clone().unwrap();
    app.users.insert(admin.user, (admin.password, true));
    run(&mut app, &users(), &acc, true, &Default::default())
}

#[test]
fn a_second_user_never_signed_in_credits_nothing_and_says_so() {
    let o = run_without_b();
    // The setup: the first user made the record the second is asked about.
    assert!(
        o.steps.iter().any(|s| s.starts_with("A created a record")),
        "{:#?}",
        o.steps
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("the second test user's session did not open")),
        "{:#?}",
        o.steps
    );
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == OTHER_USERS_DATA.rule_id),
        "{:?}",
        o.verified
    );
    assert!(
        o.not_assessed
            .iter()
            .any(|(ids, why)| ids == "V8.2.2" && why.contains("could not be shown signed in")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_flow_tried_by_a_second_user_never_signed_in_credits_nothing() {
    let o = run_without_b();
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == super::super::STEP_SKIPPED.rule_id),
        "{:?}",
        o.verified
    );
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("B could not be shown signed in again")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_second_user_signed_in_is_still_credited() {
    let o = run_against(Flaws::default(), &users());
    for rule in [OTHER_USERS_DATA.rule_id, super::super::STEP_SKIPPED.rule_id] {
        assert!(
            o.verified.iter().any(|v| v.check_id == rule),
            "{rule}: {:?}",
            o.not_assessed
        );
    }
}
