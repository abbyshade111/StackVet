//! Backlog 0572 (ADR-119): with no `signup`, the wrong-password check guesses at a name no account has, and a
//! push-back there was credited without a right password ever being tried. Now A signs in once, from the same
//! address, after the push-back. A refused shows a limit by address that stops the right password too, and the credit
//! says so. A let in is what a limit that refuses only wrong passwords does, and also what a sound limit counting each
//! name does: the two cannot be told apart, so the credit is withheld rather than a finding made.

use super::fake_app::{FakeApp, Flaws, accounts, users};

const RULE: &str = "probe.failed-sign-ins-unlimited";

/// A and B seeded, no `signup`, a limit of 3, and the guessing check alone, so nothing earlier has used up a limit
/// by address: the outcome, and the ids of the requests the app was sent.
fn run(flaws: Flaws) -> (super::Outcome, Vec<String>) {
    let mut app = FakeApp::new(flaws);
    let acc = accounts();
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let u = users();
    assert!(u.signup.is_none(), "the setup: no `signup`");
    let policy = sv_manifest::PolicySection {
        failed_sign_ins: Some(3),
        within_minutes: Some(15),
        ..Default::default()
    };
    let mut out = super::Outcome::default();
    super::signin::brute_force_check(&mut app, &u, &acc, &policy, &mut out);
    let sent = app.clock_log.iter().map(|(id, _)| id.clone()).collect();
    (out, sent)
}

fn credit(out: &super::Outcome) -> Option<&str> {
    out.verified
        .iter()
        .find(|v| v.check_id == RULE)
        .map(|v| v.scope.as_str())
}

fn withheld(out: &super::Outcome) -> Option<&str> {
    out.not_assessed
        .iter()
        .find(|(ids, why)| ids == "V6.3.1" && why.contains("against a user name no account has"))
        .map(|(_, why)| why.as_str())
}

fn found(out: &super::Outcome) -> bool {
    out.findings.iter().any(|f| f.rule_id == RULE)
}

/// The setup every case rests on: the app pushed back on the guesses, and then A's sign-in was sent.
fn pushed_back_and_tried(out: &super::Outcome, sent: &[String]) {
    assert!(
        out.steps
            .iter()
            .any(|s| s.contains("wrong sign-in attempts")
                && s.contains("pushed back")
                && !s.contains("did not push back")),
        "the setup: the app pushed back: {:?}",
        out.steps
    );
    assert!(
        sent.iter().any(|id| id == "guess-right"),
        "the setup: A's sign-in was sent after the guesses: {sent:?}"
    );
}

#[test]
fn a_limit_by_address_that_stops_the_right_password_too_is_credited_and_says_so() {
    let (out, sent) = run(Flaws {
        locks_out_after: Some(3),
        limits_by_address: true,
        ..Flaws::default()
    });
    pushed_back_and_tried(&out, &sent);
    let what = credit(&out).unwrap_or_else(|| panic!("not credited: {:?}", out.not_assessed));
    assert!(
        what.contains("from the same address straight after was refused too"),
        "{what}"
    );
    assert!(!found(&out));
}

#[test]
fn a_limit_by_address_that_refuses_only_wrong_passwords_is_not_credited() {
    let (out, sent) = run(Flaws {
        locks_out_after: Some(3),
        limits_by_address: true,
        lockout_checks_password_first: true,
        ..Flaws::default()
    });
    pushed_back_and_tried(&out, &sent);
    assert_eq!(
        credit(&out),
        None,
        "credited a limit the right guess walks through"
    );
    let why = withheld(&out).unwrap_or_else(|| panic!("not said: {:?}", out.not_assessed));
    assert!(why.contains("cannot be told apart"), "{why}");
    assert!(!found(&out), "the two cannot be told apart, so no finding");
}

#[test]
fn a_sound_limit_counting_each_name_is_not_accused_but_not_credited_either() {
    let (out, sent) = run(Flaws {
        locks_out_after: Some(3),
        ..Flaws::default()
    });
    pushed_back_and_tried(&out, &sent);
    assert!(
        !found(&out),
        "a sound limit was accused: {:?}",
        out.findings
    );
    assert_eq!(credit(&out), None);
    assert!(withheld(&out).is_some(), "{:?}", out.not_assessed);
}
