//! Every admin action the owner lists is tried, so with two or more refused the credit stands for what
//! V8.3.1 asks, and with one it is one sample and *checked in part* (ADR-053, Later). Until backlog 0430 the
//! fake app listed one, so the suite only ever saw this credit in part, and the census of in-part credits
//! could not tell it from a check that is in part always.

use super::fake_app::*;
use super::*;

/// `users`, with its admin action listed twice. The fake app reads a path without its query, so the second
/// is an action of its own to the check and the same one to the app.
fn two_actions() -> UsersSection {
    let mut u = users();
    let mut again = u.admin_actions[0].clone();
    again.path = format!("{}?again", again.path);
    u.admin_actions.push(again);
    u
}

fn admin_credit(o: &Outcome) -> Option<&crate::Verified> {
    o.verified
        .iter()
        .find(|v| v.check_id == ADMIN_ACTION.rule_id)
}

#[test]
fn two_admin_actions_refused_make_the_credit_whole() {
    let o = run_against(Flaws::default(), &two_actions());
    // The setup: both actions were refused to the ordinary user, so the credit is there to be whole.
    let credit = admin_credit(&o).expect("a credit for the admin actions");
    assert!(
        credit.scope.starts_with("2 admin actions"),
        "{}",
        credit.scope
    );
    assert!(!credit.in_part, "{credit:?}");
    // The control: the one action `users` lists leaves it in part.
    let one = run_against(Flaws::default(), &users());
    assert!(admin_credit(&one).expect("a credit").in_part);
}
