//! Backlog 0522, the first case of the setup sweep (0559, ADR-119): the wrong-password check (V6.3.1) makes an account
//! of its own through `signup` and guesses at it. When that sign-up made no account, the guesses fell on an address
//! with none, and the check wrote "Wrong passwords can be tried without limit ... against an account this check made for
//! it" (the Cursor trial: sign-up 403 for want of an invite code). It now signs in as the account first, and without it
//! says the question is not assessed, and why, and sends no wrong password.

use super::super::fake_app::{FakeApp, Flaws, accounts};
use super::*;

fn policy() -> sv_manifest::PolicySection {
    sv_manifest::PolicySection {
        failed_sign_ins: Some(3),
        within_minutes: Some(15),
        ..Default::default()
    }
}

/// A run whose A and B already exist, with `signup` named for the checks that make accounts of their own.
fn run_seeded_with_signup(flaws: Flaws) -> (Outcome, FakeApp) {
    let mut app = FakeApp::new(flaws);
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let out = crate::signed_in::run(
        &mut app,
        &super::super::tests::with_signup(),
        &acc,
        true,
        &policy(),
    );
    (out, app)
}

const RULE: &str = "probe.failed-sign-ins-unlimited";

#[test]
fn a_sign_up_that_made_no_account_is_not_assessed_and_nothing_is_guessed() {
    for (flaws, said) in [
        (
            Flaws {
                signup_closed: true,
                ..Default::default()
            },
            "422",
        ),
        (
            Flaws {
                signup_does_nothing: true,
                ..Default::default()
            },
            "303",
        ),
    ] {
        let (out, app) = run_seeded_with_signup(flaws);
        // The setup: A signed in, so the run reached the checks after it.
        assert!(
            out.steps.iter().any(|s| s.contains("signed in as A")),
            "the run did not get going: {:?}",
            out.steps
        );
        assert!(
            !out.findings.iter().any(|f| f.rule_id == RULE),
            "a finding about an account that was never made: {:?}",
            out.findings
        );
        assert!(!out.verified.iter().any(|v| v.check_id == RULE));
        let why = out
            .not_assessed
            .iter()
            .find(|(id, why)| id == "V6.3.1" && why.contains("was not made"))
            .unwrap_or_else(|| panic!("V6.3.1 not named: {:?}", out.not_assessed));
        assert!(why.1.contains(said), "{}", why.1);
        assert!(why.1.contains("invite code"), "{}", why.1);
        // The one failed sign-in at that address is the check's own look for the account; none of the guesses
        // that would follow it was sent.
        let guessed = format!("guessed.{}", accounts().a.user);
        let at_it = app.guessed_at.iter().filter(|g| **g == guessed).count();
        assert_eq!(at_it, 1, "wrong passwords were sent to {guessed}");
    }
}

#[test]
fn a_sign_up_that_made_the_account_is_still_guessed_at() {
    let (out, app) = run_seeded_with_signup(Flaws::default());
    assert!(!app.guessed_at.is_empty(), "nothing was guessed at");
    assert!(
        out.findings
            .iter()
            .any(|f| f.rule_id == RULE
                && f.description.contains("an account this check made for it")),
        "{:?}",
        out.findings
    );
}
