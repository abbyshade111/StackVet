//! Backlog 0524, the first case of sweep 3 (0561, ADR-119): after the limit on wrong passwords pushed back, the
//! account's own password is sent once. A lockout applied only to wrong passwords (the VS Code app of the trial of
//! 14 October 2026 checks the password first) lets the right guess in, and limits nothing.

use super::super::fake_app::{FakeApp, Flaws, accounts};
use super::*;

fn policy() -> sv_manifest::PolicySection {
    sv_manifest::PolicySection {
        failed_sign_ins: Some(3),
        within_minutes: Some(15),
        ..Default::default()
    }
}

fn run_with_lockout(password_first: bool) -> Outcome {
    let mut app = FakeApp::new(Flaws {
        locks_out_after: Some(3),
        lockout_checks_password_first: password_first,
        ..Default::default()
    });
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    crate::signed_in::run(
        &mut app,
        &super::super::tests::with_signup(),
        &acc,
        true,
        &policy(),
    )
}

const RULE: &str = "probe.failed-sign-ins-unlimited";

#[test]
fn a_lockout_that_lets_the_right_password_in_is_found_and_not_credited() {
    let out = run_with_lockout(true);
    // The setup: the guesses were made against an account of the check's own, and the app pushed back.
    assert!(
        out.steps.iter().any(|s| s.contains(
            "wrong sign-in attempts against an account this check \
             made for it; the app pushed back"
        )),
        "{:#?}",
        out.steps
    );
    let found = out
        .findings
        .iter()
        .find(|f| f.rule_id == RULE)
        .unwrap_or_else(|| panic!("{:#?}", out.steps));
    assert_eq!(
        found.title,
        "After the limit, the right password still signs in"
    );
    assert!(
        found.evidence.iter().any(|e| e == "guess-right"),
        "{found:?}"
    );
    assert!(
        !out.verified.iter().any(|v| v.check_id == RULE),
        "credited beside its own finding: {:?}",
        out.verified
    );
}

#[test]
fn a_lockout_that_refuses_the_right_password_too_is_credited_and_says_so() {
    let out = run_with_lockout(false);
    assert!(
        !out.findings.iter().any(|f| f.rule_id == RULE),
        "{:?}",
        out.findings
    );
    let credit = out
        .verified
        .iter()
        .find(|v| v.check_id == RULE)
        .unwrap_or_else(|| panic!("{:#?}", out.steps));
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit
            .scope
            .contains("the account's own password, sent once straight after, was refused too"),
        "{}",
        credit.scope
    );
}
