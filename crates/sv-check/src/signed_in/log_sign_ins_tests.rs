//! Backlog 0232 part 5, the sign-ins (ADR-053, Later): V16.3.1 asks that all authentication operations be logged. Its
//! credit was always in part, from one refused sign-in for a name with no account and one accepted. Now the
//! password-guessing check's first wrong password, at an account it made, is bracketed too, and the credit is whole
//! only when that was found as well and stackvet.toml names no other way to authenticate.

use super::Outcome;
use super::fake_app::{FakeApp, Flaws, LogStyle, accounts};

const RULE: &str = "probe.authentication-logged";

/// Sign-up, a stated limit (or none) so the guessing runs, and none of the other ways to authenticate, but a
/// password change when `keep_password_change`.
fn run(
    style: LogStyle,
    flaws: Flaws,
    keep_password_change: bool,
    limit: Option<u32>,
) -> (Outcome, String) {
    let mut app = FakeApp::new(flaws);
    app.log_style = Some(style);
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    let mut u = super::tests::with_signup();
    u.email_code = None;
    u.totp = None;
    u.reset = None;
    u.change_email = None;
    u.change_two_factor = None;
    u.change_recovery = None;
    if !keep_password_change {
        u.change_password = None;
    }
    let policy = sv_manifest::PolicySection {
        failed_sign_ins: limit,
        within_minutes: Some(15),
        ..Default::default()
    };
    let out = super::run(&mut app, &u, &acc, false, &policy);
    (out, app.log.join("\n"))
}

/// The credit, and whether it is in part.
fn credit(out: &Outcome, log: &str) -> Option<(String, bool)> {
    let o = crate::logs::evaluate(&out.log_markers, log);
    o.verified
        .iter()
        .find(|v| v.check_id == RULE)
        .map(|v| (v.scope.clone(), v.in_part))
}

/// The setup: the first guess was bracketed and both its markers reached the log.
fn bracketed(out: &Outcome, log: &str) {
    let w = out
        .log_markers
        .wrong_password_window
        .as_ref()
        .expect("the setup: the first guess was bracketed");
    assert!(
        log.contains(&w.open) && log.contains(&w.close),
        "the setup: the markers are in the log"
    );
    assert!(out.log_markers.wrong_password.is_some());
}

#[test]
fn every_sign_in_found_and_nothing_else_named_is_whole() {
    for style in [LogStyle::Full, LogStyle::Private] {
        let (out, log) = run(style, Flaws::default(), false, Some(3));
        bracketed(&out, &log);
        assert!(
            out.log_markers.untried.is_empty(),
            "{:?}",
            out.log_markers.untried
        );
        let (scope, in_part) = credit(&out, &log).expect("credited");
        assert!(!in_part, "{style:?}: {scope}");
        assert!(
            scope.contains("a wrong password for that account"),
            "{scope}"
        );
    }
}

#[test]
fn a_wrong_password_for_a_real_account_not_logged_keeps_it_in_part() {
    let flaws = Flaws {
        logs_no_wrong_password: true,
        ..Flaws::default()
    };
    for style in [LogStyle::Full, LogStyle::Private] {
        let (out, log) = run(style, flaws, false, Some(3));
        bracketed(&out, &log);
        let (scope, in_part) = credit(&out, &log).expect("still credited in part");
        assert!(in_part, "{style:?}: {scope}");
        assert!(scope.contains("tried and not found"), "{scope}");
    }
}

#[test]
fn another_way_to_authenticate_named_keeps_it_in_part() {
    let (out, log) = run(LogStyle::Full, Flaws::default(), true, Some(3));
    bracketed(&out, &log);
    assert_eq!(out.log_markers.untried, vec!["a password change"]);
    let (scope, in_part) = credit(&out, &log).expect("credited in part");
    assert!(in_part, "{scope}");
    assert!(scope.contains("a password change, not tried"), "{scope}");
}

#[test]
fn with_no_guessing_the_wrong_password_is_said_not_tried() {
    let (out, log) = run(LogStyle::Full, Flaws::default(), false, None);
    assert!(out.log_markers.wrong_password.is_none());
    let (scope, in_part) = credit(&out, &log).expect("credited in part");
    assert!(in_part, "{scope}");
    assert!(scope.contains("could not be tried"), "{scope}");
}
