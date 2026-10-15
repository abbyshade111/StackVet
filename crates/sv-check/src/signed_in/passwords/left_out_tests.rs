//! Sweep 3 (backlog 0561, ADR-119), inventory item 3: the password change is also sent with the current password
//! left out. An app that checks the current password only when it is sent refuses a wrong one and takes the change
//! without one; the wrong one alone credited it.

use super::super::fake_app::*;
use super::super::tests::run_signing_up;
use super::*;

#[test]
fn a_change_taken_with_the_current_password_left_out_is_found() {
    let flaws = Flaws {
        change_current_only_if_given: true,
        ..Default::default()
    };
    for o in [run_signing_up(flaws), run_against(flaws, &users())] {
        // The setup: the wrong current password was refused, so only the left-out change can have taken.
        assert!(
            o.steps
                .iter()
                .any(|s| s == "asked to change the password giving a wrong current one (403)"),
            "{:#?}",
            o.steps
        );
        let found = o
            .findings
            .iter()
            .find(|f| f.rule_id == CHANGE_WITHOUT_CURRENT.rule_id)
            .unwrap_or_else(|| panic!("{:#?}", o.steps));
        assert_eq!(
            found.title,
            "The password can be changed with the current one left out"
        );
        assert!(
            !o.verified
                .iter()
                .any(|v| v.check_id == CHANGE_WITHOUT_CURRENT.rule_id),
            "{:?}",
            o.verified
        );
    }
}

#[test]
fn a_change_refused_both_ways_is_credited_and_says_so() {
    let o = run_against(Flaws::default(), &users());
    assert!(
        o.steps
            .iter()
            .any(|s| s == "asked to change the password with the current one left out (400)"),
        "{:#?}",
        o.steps
    );
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == CHANGE_WITHOUT_CURRENT.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        credit.scope.contains("and one leaving it out"),
        "{}",
        credit.scope
    );
}

#[test]
fn only_the_fields_that_carry_the_password_are_left_out() {
    let mut t = RequestTemplate::default();
    t.form.insert("current".into(), "{password}".into());
    t.form.insert("new".into(), "{new_password}".into());
    let left = without_password(&t).expect("something to leave out");
    assert_eq!(left.form.keys().collect::<Vec<_>>(), ["new"]);
    t.form.remove("current");
    assert!(without_password(&t).is_none());
}

#[test]
fn an_email_change_taken_with_the_password_left_out_is_found() {
    let o = run_signing_up(Flaws {
        email_password_only_if_given: true,
        ..Default::default()
    });
    // The setup: the wrong password was refused, so only the left-out change can have taken.
    assert!(
        o.steps
            .iter()
            .any(|s| s == "asked to change the email address giving a wrong password (403)"),
        "{:#?}",
        o.steps
    );
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == EMAIL_CHANGE_WITHOUT_PASSWORD.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert_eq!(
        found.title,
        "The email address can be changed with the password left out"
    );
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == EMAIL_CHANGE_WITHOUT_PASSWORD.rule_id),
        "{:?}",
        o.verified
    );
}

#[test]
fn an_email_change_refused_both_ways_is_credited_and_says_so() {
    let o = run_signing_up(Flaws::default());
    assert!(
        o.steps
            .iter()
            .any(|s| s == "asked to change the email address with the password left out (400)"),
        "{:#?}",
        o.steps
    );
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == EMAIL_CHANGE_WITHOUT_PASSWORD.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        credit.scope.contains("and one leaving it out"),
        "{}",
        credit.scope
    );
}
