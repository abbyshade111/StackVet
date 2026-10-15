//! Sweep 3 (backlog 0561, ADR-119), inventory item 5: the WebSocket's made-up session is the browser's cookies with
//! those set at sign-in made up, not the first cookie alone. When the first is one set before sign-in, a session of
//! it alone is no session, and a socket that takes any session cookie at all was credited as refusing a made-up one.

use super::super::fake_app::*;
use super::*;

fn run_with(flaws: Flaws) -> Outcome {
    let mut app = FakeApp::new(flaws);
    // A cookie set on the sign-in page before the session's, so the browser's first cookie is not the session.
    app.pre_login_cookie = true;
    let mut u = users_full();
    u.private_websocket = Some("/ws".into());
    run_seeded(app, &u).0
}

#[test]
fn a_socket_that_takes_any_session_cookie_is_found_past_a_cookie_set_before_sign_in() {
    let o = run_with(Flaws {
        ws_any_cookie: true,
        ..Default::default()
    });
    assert!(
        o.steps
            .iter()
            .any(|s| s.ends_with("with a session value this check made up: accepted")),
        "{:#?}",
        o.steps
    );
    assert!(
        o.findings
            .iter()
            .any(|f| f.rule_id == WS_WITHOUT_SESSION.rule_id),
        "{:#?}",
        o.findings
    );
}

#[test]
fn a_socket_that_checks_the_session_still_refuses_it() {
    let o = run_with(Flaws::default());
    assert!(
        o.steps
            .iter()
            .any(|s| s.ends_with("with a session value this check made up: refused")),
        "{:#?}",
        o.steps
    );
    assert!(
        !o.findings
            .iter()
            .any(|f| f.rule_id == WS_WITHOUT_SESSION.rule_id),
        "{:#?}",
        o.findings
    );
}
