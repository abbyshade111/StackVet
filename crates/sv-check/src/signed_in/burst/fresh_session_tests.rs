//! A limit refusing the end of a burst is credited only when a fresh sign-in is refused too (backlog 0573): a
//! limit kept per session, not per user, starts again at each sign-in, so one user goes on creating.

use super::super::fake_app::*;
use super::tests::why_not;
use super::*;

/// The scripted app with ten notes a minute, changed by `adjust`, against the ten stackvet.toml states.
fn run(adjust: impl FnOnce(&mut FakeApp)) -> (Outcome, FakeApp) {
    let acc = accounts();
    let mut app = FakeApp::new(Flaws::default());
    app.notes_per_minute = Some(10);
    adjust(&mut app);
    app.users
        .insert(acc.a.user.clone(), (acc.a.password.clone(), false));
    app.users
        .insert(acc.b.user.clone(), (acc.b.password.clone(), false));
    let policy = sv_manifest::PolicySection {
        requests_per_minute: Some(10),
        ..Default::default()
    };
    let o = super::super::run(&mut app, &users(), &acc, true, &policy);
    (o, app)
}

fn sent(app: &FakeApp, id: &str) -> bool {
    app.clock_log.iter().any(|(i, _)| i == id)
}

#[test]
fn a_limit_kept_per_session_is_not_credited() {
    let (o, app) = run(|a| a.notes_limit_per_session = true);
    // The setup: the burst met the limit, and the fresh sign-in and its one create were sent.
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("created 11 records") && s.contains("10 went through")),
        "{:?}",
        o.steps
    );
    assert!(sent(&app, "login-burst-again") && sent(&app, "burst-again"));
    assert!(
        !verified_ids(&o).contains(&CREATE_UNLIMITED.rule_id),
        "a limit per session was credited"
    );
    assert!(
        why_not(&o).contains("the limit counts each session, not the user"),
        "{}",
        why_not(&o)
    );
}

#[test]
fn a_limit_kept_per_user_is_credited_and_says_the_fresh_sign_in_was_refused() {
    let (o, app) = run(|_| {});
    assert!(sent(&app, "login-burst-again") && sent(&app, "burst-again"));
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == CREATE_UNLIMITED.rule_id)
        .unwrap_or_else(|| panic!("not credited: {}", why_not(&o)));
    assert!(
        credit
            .scope
            .contains("after signing in again was refused too"),
        "{}",
        credit.scope
    );
}

#[test]
fn without_a_fresh_sign_in_nothing_is_credited() {
    // Sign-ins refused from the end of the burst on: the burst still meets the limit, but the fresh
    // sign-in cannot show it holds for a new session. The fake's clock is its own, so the moment is
    // read from a first run.
    let (_, first) = run(|_| {});
    let at = first
        .clock_log
        .iter()
        .find(|(i, _)| i == "burst-11")
        .map(|(_, at)| *at)
        .expect("the first run sent the whole burst");
    let (o, app) = run(|a| a.sign_ins_refused_from = Some(at));
    assert!(sent(&app, "burst-11") && sent(&app, "login-burst-again"));
    assert!(
        !sent(&app, "burst-again"),
        "a create was sent without a fresh session"
    );
    assert!(!verified_ids(&o).contains(&CREATE_UNLIMITED.rule_id));
    assert!(
        why_not(&o).contains("could not sign in again"),
        "{}",
        why_not(&o)
    );
}

#[test]
fn a_new_sign_in_that_ends_the_old_session_leaves_the_credit_standing() {
    // An app that keeps one session a user: the new session takes the old one's place.
    let (o, _) = run(|a| a.one_session_per_user = true);
    assert!(
        verified_ids(&o).contains(&CREATE_UNLIMITED.rule_id),
        "{}",
        why_not(&o)
    );
}

#[test]
fn the_fresh_sign_in_is_made_only_when_a_limit_would_be_credited() {
    // No limit at all: the finding stands on the burst alone, and no sign-in is spent on it.
    let (o, app) = run(|a| a.notes_per_minute = None);
    assert!(rule_ids(&o).contains(&CREATE_UNLIMITED.rule_id));
    assert!(!sent(&app, "login-burst-again"));
}
