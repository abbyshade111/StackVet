//! Backlog 0527: a refused sign-in is told by the field names sent and the page's own words, never a value.

use super::*;

fn login(form: &[(&str, &str)], json: &[(&str, &str)]) -> RequestTemplate {
    RequestTemplate {
        method: "POST".into(),
        path: "/login".into(),
        form: form
            .iter()
            .map(|(k, v)| ((*k).into(), (*v).into()))
            .collect(),
        json: json
            .iter()
            .map(|(k, v)| ((*k).into(), (*v).into()))
            .collect(),
    }
}

fn answered(status: u16, body: &str) -> ProbeResponse {
    ProbeResponse {
        id: "login-a".into(),
        status,
        headers: Vec::new(),
        body: body.into(),
    }
}

#[test]
fn the_trial_page_is_told_by_its_fields_and_words() {
    // As the trial's app answered: a page of inline styles, then the one sentence that says why.
    let page = format!(
        "<html><head><style>{}</style></head><body><div class=\"error\">The CSRF token is missing or invalid.</div>\
         <form><input name=\"username\" value=\"sv-a\"></form></body></html>",
        "body { margin: 0 } ".repeat(300)
    );
    let said = answer(
        &login(
            &[
                ("username", "{user}"),
                ("password", "{password}"),
                ("csrf_token", "{csrf}"),
            ],
            &[],
        ),
        &answered(400, &page),
        false,
    );
    assert!(
        said.starts_with(
            "the sign-in answered 400; it sent the fields `csrf_token`, `password`, `username`"
        ),
        "{said}"
    );
    assert!(
        said.contains("\u{201c}The CSRF token is missing or invalid."),
        "{said}"
    );
    assert!(!said.contains("margin"), "styles are not words: {said}");
    assert!(
        !said.contains("{password}") && !said.contains("{user}"),
        "values are never named: {said}"
    );
}

#[test]
fn a_page_that_repeats_the_password_is_not_quoted() {
    // The test password is built at run time, so nothing here looks like a key.
    let password = ["hunter", "-", "2024"].concat();
    let page = answered(200, &format!("<p>Wrong password: {password}</p>"));
    let repeats = repeats_password(&page, &password);
    assert!(repeats, "the setup: the page repeats it");
    let said = sent_and_said(&login(&[("password", "{password}")], &[]), &page, repeats);
    assert_eq!(
        said,
        "it sent the field `password`, and the page repeated the test account's password, so its words are not quoted"
    );
    assert!(!said.contains("hunter"), "{said}");
    // The control: a page that does not repeat it is quoted, and a password too short to tell is not looked for.
    let other = answered(200, "<p>Wrong password.</p>");
    assert!(!repeats_password(&other, &password));
    assert!(!repeats_password(&answered(200, "<p>pw</p>"), "pw"));
}

#[test]
fn characters_that_hide_or_move_text_are_dropped() {
    let said = page_said(&answered(200, "<p>Wrong\u{202e} password\u{7}.</p>"))
        .expect("the page has words");
    assert_eq!(said, "Wrong password.");
}

#[test]
fn json_fields_are_named_and_an_empty_page_says_so() {
    let said = answer(
        &login(&[], &[("email", "{user}"), ("password", "{password}")]),
        &answered(401, ""),
        false,
    );
    assert_eq!(
        said,
        "the sign-in answered 401; it sent the fields `email`, `password`, and the page showed no words"
    );
}

#[test]
fn a_run_whose_sign_in_is_refused_says_why_in_its_step_and_its_reason() {
    use crate::signed_in::fake_app::{Flaws, run_against, users};
    let o = run_against(
        Flaws {
            broken_login: true,
            ..Default::default()
        },
        &users(),
    );
    // The setup: the sign-in was refused, so the signed-in checks were not assessed.
    let refused = o
        .steps
        .iter()
        .find(|s| s.starts_with("signing in as A was refused (403"))
        .unwrap_or_else(|| panic!("no refused step: {:?}", o.steps));
    assert!(
        refused.contains("): it sent the fields `csrf_token`, `email`, `password`")
            && refused.contains("the page said: \u{201c}no\u{201d}"),
        "{refused}"
    );
    let reason = o
        .not_assessed
        .iter()
        .map(|(_, why)| why)
        .find(|why| why.contains("did not open"))
        .unwrap_or_else(|| panic!("no reason: {:?}", o.not_assessed));
    assert!(
        reason
            .contains("To tell which: the sign-in answered 403; it sent the fields `csrf_token`, `email`, `password`"),
        "{reason}"
    );
}

#[test]
fn a_crash_at_sign_in_failed_and_was_not_refused() {
    // ADR-021: a crash's answer is never read as the app refusing.
    use crate::signed_in::fake_app::{Flaws, run_against, users};
    let o = run_against(
        Flaws {
            broken_login: true,
            falls_over_after: Some((0, 500)),
            ..Default::default()
        },
        &users(),
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("signing in as A failed (500")),
        "{:?}",
        o.steps
    );
    assert!(
        !o.steps.iter().any(|s| s.contains("was refused (500")),
        "{:?}",
        o.steps
    );
}
