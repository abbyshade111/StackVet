//! The browser-validation probe reads its answer beside the ordinary sign-up's (backlog 0523, 0560).

use super::super::fake_app::*;
use super::super::tests::with_signup;
use super::*;

fn refused_on_200() -> Flaws {
    Flaws {
        signup_refused_on_200: true,
        ..Default::default()
    }
}

#[test]
fn a_form_shown_again_with_200_is_not_an_acceptance() {
    let users = with_signup();
    assert!(
        !users.private.is_empty(),
        "the fixture lists no private page"
    );
    let o = run_with_users(refused_on_200(), &users);
    // The probe got as far as the broken value, or there is nothing to read.
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("breaking the form's own maxlength=40")),
        "{:?}",
        o.steps
    );
    assert!(
        !rule_ids(&o).contains(&CLIENT_SIDE_VALIDATION.rule_id),
        "a refusal on a 200 page was read as accepted: {:?}",
        o.findings
    );
    // What was compared, and the sign-in that settled it.
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("200 where the control redirected to /login")),
        "{:?}",
        o.steps
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("breaking the form's own maxlength=40: refused")),
        "{:?}",
        o.steps
    );
}

#[test]
fn with_no_private_page_to_open_it_cannot_tell_and_says_so() {
    let mut users = with_signup();
    users.private = Vec::new();
    let o = run_with_users(refused_on_200(), &users);
    assert!(!rule_ids(&o).contains(&CLIENT_SIDE_VALIDATION.rule_id));
    let why = o
        .not_assessed
        .iter()
        .find(|(id, why)| id == "V2.2.2" && why.contains("unlike the ordinary sign-up"))
        .unwrap_or_else(|| panic!("V2.2.2 not named: {:?}", o.not_assessed));
    assert!(
        why.1.contains("200 where the control redirected to /login"),
        "{}",
        why.1
    );
}

#[test]
fn a_value_accepted_the_way_the_ordinary_sign_up_was_is_still_found() {
    let o = run_with_users(
        Flaws {
            validation_only_in_browser: true,
            signup_refused_on_200: true,
            ..Default::default()
        },
        &with_signup(),
    );
    let finding = o
        .findings
        .iter()
        .find(|f| f.rule_id == CLIENT_SIDE_VALIDATION.rule_id)
        .unwrap_or_else(|| panic!("{:?}", o.findings));
    assert!(
        finding.description.contains(
            "answered as the ordinary sign-up did (303), and signing in as the account it made"
        ),
        "{}",
        finding.description
    );
}

#[test]
fn a_refusal_that_redirects_like_a_sign_up_that_worked_is_settled_by_signing_in() {
    let o = run_with_users(
        Flaws {
            signup_refused_alike: true,
            ..Default::default()
        },
        &with_signup(),
    );
    assert!(
        o.steps.iter().any(|s| s.contains(
            "answered as the ordinary sign-up did (303), so signed in as the account it would have made"
        )),
        "{:?}",
        o.steps
    );
    assert!(
        !rule_ids(&o).contains(&CLIENT_SIDE_VALIDATION.rule_id),
        "an account that was never made was read as accepted: {:?}",
        o.findings
    );
}
