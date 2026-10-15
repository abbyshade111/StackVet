//! Sweep 3 (backlog 0561, ADR-119), from its inventory: the wrong-code check already sends the right emailed code
//! after the guesses, and credited a limit that refused the wrong codes even when the right one still signed in.
//! That limit stops nobody, as for passwords (backlog 0524).

use super::super::fake_app::*;
use super::super::tests::run_with;
use super::*;

fn policy() -> sv_manifest::PolicySection {
    sv_manifest::PolicySection {
        failed_codes: Some(3),
        ..Default::default()
    }
}

#[test]
fn a_code_limit_that_still_takes_the_right_code_is_found_and_not_credited() {
    let o = run_with(
        Flaws {
            code_limit_checks_code_first: true,
            ..Default::default()
        },
        &policy(),
    );
    // The setup: the guesses were pushed back on, and then the right code was taken.
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("pushed back (429), and then took the right code")),
        "{:#?}",
        o.steps
    );
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == EMAIL_CODE_GUESSING.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert_eq!(
        found.title,
        "After the limit, the right emailed code still signs in"
    );
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == EMAIL_CODE_GUESSING.rule_id),
        "{:?}",
        o.verified
    );
}

#[test]
fn a_code_limit_that_refuses_the_right_code_too_is_still_credited() {
    let o = run_with(Flaws::default(), &policy());
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == EMAIL_CODE_GUESSING.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        credit.scope.contains("then refused the right code"),
        "{}",
        credit.scope
    );
}
