//! Each user takes the action alone and gives it back through `reset` before the race (backlog 0574, the owner's
//! answer): a user refused in the race may otherwise be one who may not take it at all.

use super::super::fake_app::*;
use super::*;

/// The fake's cancel route, as `reset`, or one at `path` the app does not have.
fn with_reset(path: &str) -> UsersSection {
    let mut u = users();
    if let Some(once) = u.once.as_mut() {
        once.reset = Some(RequestTemplate {
            method: "POST".into(),
            path: path.into(),
            form: [("slot", "1"), ("csrf_token", "{csrf}")]
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
            json: BTreeMap::new(),
        });
    }
    u
}

fn run(users: &UsersSection, adjust: impl FnOnce(&mut FakeApp, &Accounts)) -> Outcome {
    let acc = accounts();
    let mut app = FakeApp::new(Flaws::default());
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    adjust(&mut app, &acc);
    let mut out = Outcome::default();
    once_check(&mut app, users, &acc, &mut out);
    out
}

fn why_not(o: &Outcome) -> &str {
    o.not_assessed
        .iter()
        .find(|(ids, _)| ids == "V2.3.4")
        .map_or("", |(_, why)| why.as_str())
}

fn credit(o: &Outcome) -> &crate::Verified {
    o.verified
        .iter()
        .find(|v| v.check_id == DONE_TWICE.rule_id)
        .unwrap_or_else(|| panic!("not credited: {}\n{:?}", why_not(o), o.steps))
}

#[test]
fn with_a_reset_each_user_takes_it_alone_first_and_the_credit_says_so() {
    let o = run(&with_reset("/unbook"), |_, _| {});
    // The setup: both took it alone and gave it back, and then the race ran.
    assert_eq!(
        o.steps
            .iter()
            .filter(|s| s.contains("gave it back"))
            .count(),
        2,
        "{:?}",
        o.steps
    );
    assert!(
        credit(&o)
            .scope
            .contains("Each user had first taken it alone")
    );
}

#[test]
fn a_user_who_may_not_take_it_at_all_is_not_read_as_losing_the_race() {
    // Only the first user may book: without the solo try, the second's refusals read as the race lost.
    let o = run(&with_reset("/unbook"), |app, acc| {
        app.booking_only_for = Some(acc.a.user.clone())
    });
    assert!(!verified_ids(&o).contains(&DONE_TWICE.rule_id));
    assert!(!rule_ids(&o).contains(&DONE_TWICE.rule_id));
    assert!(
        why_not(&o).contains("sent alone by the second user")
            && why_not(&o).contains("may not do it at all"),
        "{}",
        why_not(&o)
    );
}

#[test]
fn a_reset_that_is_not_accepted_leaves_the_race_unjudged() {
    let o = run(&with_reset("/no-such-cancel"), |_, _| {});
    assert!(!verified_ids(&o).contains(&DONE_TWICE.rule_id));
    assert!(why_not(&o).contains("was not accepted"), "{}", why_not(&o));
}

#[test]
fn without_a_reset_the_credit_says_neither_user_was_shown_able() {
    let o = run(&users(), |_, _| {});
    assert!(
        credit(&o)
            .scope
            .contains("Neither user was shown able to take it alone"),
        "{}",
        credit(&o).scope
    );
    assert!(credit(&o).in_part);
}
