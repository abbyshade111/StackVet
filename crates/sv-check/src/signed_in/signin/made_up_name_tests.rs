//! With no `signup`, the wrong-password check (V6.3.1) guesses against a name no account has, and an
//! app that counts wrong passwords on each real account never locks that name. Until backlog 0384
//! this read as "Wrong passwords can be tried without limit" against an app that locks real accounts
//! after the stated number (the second loop trial, `haiku-a-1`).

use super::super::fake_app::Flaws;
use super::super::tests::{run_with, seeded_with};

fn policy() -> sv_manifest::PolicySection {
    sv_manifest::PolicySection {
        failed_sign_ins: Some(3),
        within_minutes: Some(15),
        ..Default::default()
    }
}

const RULE: &str = "probe.failed-sign-ins-unlimited";

#[test]
fn a_made_up_name_answered_alike_is_not_read_as_no_limit() {
    let flaws = Flaws {
        locks_out_after: Some(3),
        limits_only_real_accounts: true,
        ..Flaws::default()
    };
    let out = seeded_with(flaws, &policy());
    // The setup: the guesses went to a name no account has, and were all answered alike.
    assert!(
        out.steps
            .iter()
            .any(|s| s
                .starts_with("made 5 wrong sign-in attempts against a user name no account has")),
        "{:?}",
        out.steps
    );
    assert!(
        out.findings.iter().all(|f| f.rule_id != RULE),
        "an app that locks real accounts was reported as having no limit"
    );
    assert!(out.verified.iter().all(|v| v.check_id != RULE));
    let why = out
        .not_assessed
        .iter()
        .find(|(ids, _)| ids.contains("V6.3.1"))
        .map(|(_, why)| why)
        .unwrap_or_else(|| panic!("V6.3.1 is said to be not assessed: {:?}", out.not_assessed));
    assert!(
        why.contains("a user name no account has") && why.contains("`signup`"),
        "{why}"
    );
}

#[test]
fn a_made_up_name_still_shows_a_limit_by_address_and_a_real_account_still_shows_none() {
    // An app that pushes back on the made-up name is read as pushing back, as before: whatever it
    // counts by, it refused past the stated number. (The credit itself is then withheld here, since
    // the same lock refused the suite's own sign-ins; ADR-021, "a sign-in the limiter refuses".)
    let by_name = seeded_with(
        Flaws {
            locks_out_after: Some(3),
            ..Flaws::default()
        },
        &policy(),
    );
    assert!(
        by_name.steps.iter().any(|s| s
            .starts_with("made 5 wrong sign-in attempts against a user name no account has")
            && s.ends_with("the app pushed back")),
        "{:?}",
        by_name.steps
    );
    assert!(by_name.findings.iter().all(|f| f.rule_id != RULE));
    // With `signup`, the check guesses against an account it made, and an app with no limit at all
    // is still found.
    let none = run_with(Flaws::default(), &policy());
    assert!(none.findings.iter().any(|f| f.rule_id == RULE));
    // And an app that locks only real accounts is credited there, since the account is real.
    let real = run_with(
        Flaws {
            locks_out_after: Some(3),
            limits_only_real_accounts: true,
            ..Flaws::default()
        },
        &policy(),
    );
    assert!(
        real.verified.iter().any(|v| v.check_id == RULE),
        "{:?}",
        real.not_assessed
    );
}
