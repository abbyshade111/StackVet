//! A session that timed out, answered with "Your session has expired" served with 200, has ended: later reads of
//! the private page count as opening it only when they show the page it opened at the start (backlog 0560,
//! ADR-119). The session kept busy, read at the same moment, is the control.

use super::super::fake_app::*;
use super::super::tests::timeouts;
use super::*;

fn slow(expired_on_200: bool) -> Outcome {
    let mut app = FakeApp::new(Flaws {
        expired_on_200,
        ..Default::default()
    });
    app.idle_limit = Some(15 * 60);
    app.lifetime_limit = Some(60 * 60);
    let acc = accounts();
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let admin = acc.admin.clone().unwrap();
    app.users.insert(admin.user, (admin.password, true));
    super::super::run_with(
        &mut app,
        &users(),
        &acc,
        true,
        &timeouts(Some(15), Some(60)),
        true,
    )
}

fn ids(o: &Outcome, verified: bool) -> Vec<&str> {
    let wanted = [NO_IDLE_TIMEOUT.rule_id, NO_SESSION_LIFETIME.rule_id];
    if verified {
        o.verified
            .iter()
            .map(|v| v.check_id.as_str())
            .filter(|id| wanted.contains(id))
            .collect()
    } else {
        o.findings
            .iter()
            .map(|f| f.rule_id.as_str())
            .filter(|id| wanted.contains(id))
            .collect()
    }
}

#[test]
fn a_session_expired_page_served_with_200_is_the_session_ended() {
    let o = slow(true);
    assert!(
        o.steps.iter().any(|s| s.contains("a session left alone")),
        "the idle check did not run: {:?}",
        o.steps
    );
    assert!(
        ids(&o, false).is_empty(),
        "an expired session's 200 page was read as the page opening: {:?}",
        o.findings
    );
    assert_eq!(
        ids(&o, true),
        vec![NO_IDLE_TIMEOUT.rule_id, NO_SESSION_LIFETIME.rule_id],
        "{:?}",
        o.not_assessed
    );
}
