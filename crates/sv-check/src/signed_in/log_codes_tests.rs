//! Backlog 0232 part 5, the codes (ADR-053, Later): a sign-in with an emailed code and one with a two-factor code are
//! each bracketed in the log twice, around a code the app took and one it refused, and V16.3.1's credit is whole only
//! when both of each were found, beside the three password sign-ins.

use super::Outcome;
use super::fake_app::{FakeApp, Flaws, LogStyle, accounts, users};
use crate::logs::{EMAILED_CODE, TWO_FACTOR_CODE};

const RULE: &str = "probe.authentication-logged";

/// A and B seeded, a two-factor account enrolled, sign-up, the emailed code and the two-factor code, a stated limit,
/// and no other way to authenticate named.
fn run(style: LogStyle, flaws: Flaws) -> (Outcome, String) {
    let mut app = FakeApp::new(flaws);
    app.log_style = Some(style);
    let mut acc = accounts();
    acc.admin = None;
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    // Enrolled as `seed` would, as `run_seeded` does.
    let totp = acc.totp.as_ref().expect("the setup: a two-factor account");
    app.users.insert(
        totp.account.user.clone(),
        (totp.account.password.clone(), false),
    );
    app.totp
        .insert(totp.account.user.clone(), totp.secret.clone());
    let mut u = users();
    u.signup = super::tests::with_signup().signup;
    u.admin = Vec::new();
    u.admin_actions = Vec::new();
    u.change_password = None;
    u.reset = None;
    u.change_email = None;
    u.change_two_factor = None;
    u.change_recovery = None;
    assert!(u.email_code.is_some() && u.totp.is_some() && u.seed.is_some());
    let policy = sv_manifest::PolicySection {
        failed_sign_ins: Some(3),
        within_minutes: Some(15),
        ..Default::default()
    };
    let out = super::run(&mut app, &u, &acc, true, &policy);
    (out, app.log.join("\n"))
}

fn credit(out: &Outcome, log: &str) -> Option<(String, bool)> {
    let o = crate::logs::evaluate(&out.log_markers, log);
    o.verified
        .iter()
        .find(|v| v.check_id == RULE)
        .map(|v| (v.scope.clone(), v.in_part))
}

/// The setup: both ways were marked, nothing else is left untried, and every window's markers reached the log.
fn marked(out: &Outcome, log: &str, refused_too: bool) {
    let m = &out.log_markers;
    assert!(m.untried.is_empty(), "the setup: {:?}", m.untried);
    for what in [EMAILED_CODE, TWO_FACTOR_CODE] {
        let f = m
            .factors
            .iter()
            .find(|f| f.what == what)
            .unwrap_or_else(|| panic!("the setup: {what} was marked: {:?}", m.factors));
        let windows: Vec<_> = f.accepted.iter().chain(&f.refused).collect();
        assert_eq!(
            windows.len(),
            if refused_too { 2 } else { 1 },
            "{what}: {f:?}"
        );
        for w in windows {
            assert!(
                log.contains(&w.open) && log.contains(&w.close),
                "{what}: markers not in the log"
            );
        }
    }
}

#[test]
fn every_way_to_sign_in_found_is_whole() {
    for style in [LogStyle::Full, LogStyle::Private] {
        let (out, log) = run(style, Flaws::default());
        marked(&out, &log, true);
        let (scope, in_part) = credit(&out, &log).expect("credited");
        assert!(!in_part, "{style:?}: {scope}");
        assert!(
            scope.contains(&format!(
                "for {EMAILED_CODE} and {TWO_FACTOR_CODE}, a code taken and a code refused"
            )),
            "{scope}"
        );
    }
}

#[test]
fn codes_the_log_does_not_record_keep_it_in_part() {
    let flaws = Flaws {
        logs_no_codes: true,
        ..Flaws::default()
    };
    for style in [LogStyle::Full, LogStyle::Private] {
        let (out, log) = run(style, flaws);
        marked(&out, &log, true);
        let (scope, in_part) = credit(&out, &log).expect("credited in part");
        assert!(in_part, "{style:?}: {scope}");
        for what in [EMAILED_CODE, TWO_FACTOR_CODE] {
            assert!(
                scope.contains(&format!("{what}: one it took was tried and not found")),
                "{scope}"
            );
        }
    }
}

#[test]
fn a_code_never_refused_is_said_not_tried() {
    // Old two-factor codes taken, and an emailed code taken in another session: no refusal to read for either.
    let flaws = Flaws {
        totp_any_age: true,
        code_unbound: true,
        ..Flaws::default()
    };
    let (out, log) = run(LogStyle::Full, flaws);
    marked(&out, &log, false);
    let (scope, in_part) = credit(&out, &log).expect("credited in part");
    assert!(in_part, "{scope}");
    for what in [EMAILED_CODE, TWO_FACTOR_CODE] {
        assert!(
            scope.contains(&format!("{what}: one it refused could not be tried")),
            "{scope}"
        );
    }
}

#[test]
fn an_access_line_with_a_status_code_is_not_a_code_given() {
    // `status_code` and `error_code` carry the word `code` on a line that records only that a request was answered.
    let window = |which: &str| crate::logs::Window {
        open: format!("/sv-log-before-{which}-t"),
        close: format!("/sv-log-after-{which}-t"),
        login_path: "/login/verify".to_owned(),
    };
    let markers = crate::logs::Markers {
        failed_sign_in: Some("nobody@example.test".to_owned()),
        successful_sign_in: Some("ok@example.test".to_owned()),
        wrong_password: Some("ok@example.test".to_owned()),
        factors: vec![crate::logs::FactorMarkers {
            what: EMAILED_CODE.to_owned(),
            accepted: Some(window("code-ok")),
            refused: Some(window("code-wrong")),
        }],
        ..Default::default()
    };
    let log = [
        "sign-in failed for nobody@example.test",
        "signed in ok@example.test",
        "sign-in failed for ok@example.test",
        "GET /sv-log-before-code-ok-t 404",
        r#"{"path":"/login/verify","status_code":303,"error_code":null}"#,
        "GET /sv-log-after-code-ok-t 404",
        "GET /sv-log-before-code-wrong-t 404",
        r#"{"path":"/login/verify","status_code":401,"error_code":"E401"}"#,
        "GET /sv-log-after-code-wrong-t 404",
    ]
    .join("\n");
    let o = crate::logs::evaluate(&markers, &log);
    let v = o
        .verified
        .iter()
        .find(|v| v.check_id == RULE)
        .expect("the setup: the three password sign-ins credit it");
    assert!(v.in_part, "{}", v.scope);
    for which in ["one it took", "one it refused"] {
        assert!(
            v.scope
                .contains(&format!("{EMAILED_CODE}: {which} was tried and not found"))
                || v.scope
                    .contains(&format!("and {which} was tried and not found")),
            "{which}: {}",
            v.scope
        );
    }
}
