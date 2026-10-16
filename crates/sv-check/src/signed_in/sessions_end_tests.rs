//! Backlog 0492 (ADR-125): a user ends their other sessions, giving the password again (V7.5.2), and an admin ends a
//! user's (V7.4.5), tried from `end-other-sessions` and `admin-end-sessions`.

use super::fake_app::*;
use super::tests::with_end_sessions;
use super::*;

fn run(flaws: Flaws) -> Outcome {
    run_against(flaws, &with_end_sessions(users()))
}

fn credited(o: &Outcome, rule: &str) -> Option<String> {
    o.verified
        .iter()
        .find(|v| v.check_id == rule)
        .map(|v| v.scope.clone())
}

#[test]
fn a_correct_app_is_credited_in_part_for_both_and_says_what_was_not_tried() {
    let o = run(Flaws::default());
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("signed in as A twice")),
        "the setup: A signed in twice"
    );
    let own = credited(&o, SESSIONS_END_WITHOUT_PASSWORD.rule_id).expect("V7.5.2 not credited");
    assert!(own.contains("viewing the sessions not tried"), "{own}");
    let admin = credited(&o, ADMIN_END_SESSIONS_IGNORED.rule_id).expect("V7.4.5 not credited");
    assert!(admin.contains("every user's not tried"), "{admin}");
    for rule in [
        SESSIONS_END_WITHOUT_PASSWORD.rule_id,
        ADMIN_END_SESSIONS_IGNORED.rule_id,
    ] {
        assert!(credited_in_part(&o, rule), "{rule} credited whole");
    }
    assert!(
        !rule_ids(&o).iter().any(|r| [
            SESSIONS_END_WITHOUT_PASSWORD.rule_id,
            OTHER_SESSIONS_NOT_ENDED.rule_id,
            ADMIN_END_SESSIONS_IGNORED.rule_id
        ]
        .contains(r)),
        "a correct app was reported"
    );
}

#[test]
fn other_sessions_ended_with_a_wrong_password_are_found() {
    let o = run(Flaws {
        end_sessions_without_password: true,
        ..Default::default()
    });
    assert!(rule_ids(&o).contains(&SESSIONS_END_WITHOUT_PASSWORD.rule_id));
    assert!(credited(&o, SESSIONS_END_WITHOUT_PASSWORD.rule_id).is_none());
}

#[test]
fn a_request_that_ends_nothing_is_found() {
    let o = run(Flaws {
        end_sessions_does_nothing: true,
        ..Default::default()
    });
    assert!(rule_ids(&o).contains(&OTHER_SESSIONS_NOT_ENDED.rule_id));
    assert!(credited(&o, SESSIONS_END_WITHOUT_PASSWORD.rule_id).is_none());
}

#[test]
fn an_admin_request_that_ends_nothing_is_found() {
    let o = run(Flaws {
        admin_end_sessions_does_nothing: true,
        ..Default::default()
    });
    assert!(rule_ids(&o).contains(&ADMIN_END_SESSIONS_IGNORED.rule_id));
    assert!(credited(&o, ADMIN_END_SESSIONS_IGNORED.rule_id).is_none());
}

#[test]
fn a_request_the_app_refuses_is_not_judged_either_way() {
    // The manifest's request leaves out the password field this app needs: refused, so nothing ended to judge.
    let mut users = with_end_sessions(users());
    users
        .end_other_sessions
        .as_mut()
        .unwrap()
        .form
        .remove("password");
    let o = run_against(Flaws::default(), &users);
    assert!(!rule_ids(&o).contains(&OTHER_SESSIONS_NOT_ENDED.rule_id));
    assert!(credited(&o, SESSIONS_END_WITHOUT_PASSWORD.rule_id).is_none());
    assert!(
        o.not_assessed
            .iter()
            .any(|(ids, why)| ids == "V7.5.2" && why.contains("nothing ended to judge")),
        "the refusal is not said"
    );
}
