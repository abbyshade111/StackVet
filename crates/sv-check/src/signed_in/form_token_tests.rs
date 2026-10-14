//! The form token a request sends is the one its form carries (backlog 0385). In the second loop
//! trial an account change's token was looked for at the change's own address, which those apps
//! answered 405, the session's anti-forgery cookie was taken in its place, and the app rightly
//! refused the change; the report then blamed the page. The same night, a reset whose form shows
//! only at the emailed link was refused for want of its token.

use super::account_changes::rule_for;
use super::fake_app::*;
use super::tests::with_signup;
use super::*;

#[test]
fn a_cookie_is_not_taken_for_the_form_token_while_a_page_has_one() {
    // The sign-in page sets an anti-forgery cookie whose value is not the forms' token.
    let mut app = FakeApp::new(Flaws::default());
    app.pre_login_cookie = true;
    let (o, _) = run_seeded(app, &with_account_changes(with_signup()));
    // The setup: the session held that cookie, so the old search would have stopped on it.
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("giving the right password")),
        "{:?}",
        o.steps
    );
    for kind in ["phone", "two-factor", "recovery"] {
        let rule = rule_for(kind).rule_id;
        assert!(
            verified_ids(&o).contains(&rule),
            "{kind}: {:?}\n{:?}",
            o.steps,
            o.not_assessed
        );
    }
}

#[test]
fn a_change_refused_before_it_is_tried_says_so() {
    // The change sent without its token: the app refuses it (403) whatever the password.
    let mut users = with_account_changes(with_signup());
    if let Some(change) = users.change_phone.as_mut() {
        change.form.remove("csrf_token");
    }
    let o = run_against(Flaws::default(), &users);
    let why: Vec<&String> = o
        .not_assessed
        .iter()
        .filter(|(ids, why)| ids == "V7.5.1" && why.contains("the phone number"))
        .map(|(_, why)| why)
        .collect();
    assert!(
        why.iter()
            .any(|w| w.contains("was refused (403) before it was tried")),
        "{why:?}"
    );
    assert!(!why.iter().any(|w| w.contains("did not take")), "{why:?}");
}

#[test]
fn a_reset_form_shown_only_at_the_emailed_link_is_filled_from_there() {
    let flaws = Flaws {
        reset_form_only_at_link: true,
        ..Flaws::default()
    };
    let o = run_against(flaws, &users());
    let steps = o.steps.join("\n");
    assert!(
        steps.contains("the password the reset set signed in"),
        "{steps}"
    );
    assert!(
        !o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("did not change the password")),
        "{:?}",
        o.not_assessed
    );
}
