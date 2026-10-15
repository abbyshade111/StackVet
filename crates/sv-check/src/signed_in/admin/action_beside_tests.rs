//! An admin action with no `check` page is read beside the admin's own same request (backlog 0560, ADR-119),
//! and one that cannot be judged that way keeps the credit from resting on the others.

use super::super::fake_app::*;
use super::*;

#[test]
fn an_action_not_judged_keeps_the_credit_from_resting_on_the_others() {
    // The same announcement listed twice: once with its `check` page, which shows the ordinary user's refused,
    // and once without, where the refusal comes as a 200 unlike the admin's redirect.
    let mut u = users();
    let mut without = u.admin_actions[0].clone();
    without.check = None;
    let alone = run_against(
        Flaws {
            admin_action_says_ok: true,
            ..Default::default()
        },
        &u,
    );
    assert!(
        alone
            .verified
            .iter()
            .any(|v| v.check_id == ADMIN_ACTION.rule_id),
        "the action with its check page is credited alone: {:?}",
        alone.not_assessed
    );
    u.admin_actions.push(without);
    let both = run_against(
        Flaws {
            admin_action_says_ok: true,
            ..Default::default()
        },
        &u,
    );
    assert!(
        both.not_assessed.iter().any(|(ids, why)| ids == "V8.3.1"
            && why.contains("answered unlike the admin's own same request")),
        "{:?}",
        both.not_assessed
    );
    assert!(
        !both
            .verified
            .iter()
            .any(|v| v.check_id == ADMIN_ACTION.rule_id),
        "{:?}",
        both.verified
    );
}
