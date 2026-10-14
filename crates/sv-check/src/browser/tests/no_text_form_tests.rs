//! Backlog 0503: the signed-in checks, with no text form named in stackvet.toml, say under V3.2.2 that the typed-text
//! check was not made and which line makes it. With no `browser` table, and with one that names no form, whether or not
//! signing in worked (every run of the suite names what it could not do: `signed_in::asked_tests`); never when a form is
//! named.

use super::*;

fn said(o: &Outcome) -> Vec<&str> {
    o.not_assessed
        .iter()
        .filter(|(_, why)| why == NO_TEXT_FORM)
        .map(|(ids, _)| ids.as_str())
        .collect()
}

fn signed_in_run(users: &UsersSection, works: bool) -> Outcome {
    let mut fake = Fake {
        app: App::default(),
        jobs: Vec::new(),
    };
    let mut out = Outcome::default();
    checks(
        &mut fake,
        users,
        &signed_in(),
        &account(),
        works,
        "t0k",
        &mut out,
    );
    out
}

#[test]
fn with_no_browser_table_the_report_says_the_typed_text_check_was_not_made_and_how_to_make_it() {
    let mut u = users(None, None);
    u.browser = None;
    let o = signed_in_run(&u, true);
    assert_eq!(said(&o), ["V3.2.2"], "{:?}", o.not_assessed);
    assert!(NO_TEXT_FORM.contains("browser = { text-form = "));
    assert!(NO_TEXT_FORM.contains("[stack.run.users]"));
}

#[test]
fn with_a_browser_table_and_no_form_it_says_so_too() {
    // The setup: the table is there, and names no form.
    assert!(users(None, None).browser.unwrap().forms().is_empty());
    let o = signed_in_run(&users(None, None), true);
    assert_eq!(said(&o), ["V3.2.2"], "{:?}", o.not_assessed);
}

#[test]
fn with_a_form_named_it_says_nothing_of_it_and_without_one_it_says_so_even_when_signing_in_did_not_work()
 {
    let named = signed_in_run(&users(Some("/notes"), None), true);
    assert!(said(&named).is_empty(), "{:?}", named.not_assessed);
    // The control: with a form named, the check was made, so its own lines speak for V3.2.2.
    assert!(
        !named.verified.is_empty() || !named.findings.is_empty(),
        "{:?}",
        named.not_assessed
    );

    let mut u = users(None, None);
    u.browser = None;
    let not_signed_in = signed_in_run(&u, false);
    assert_eq!(
        said(&not_signed_in),
        ["V3.2.2"],
        "{:?}",
        not_signed_in.not_assessed
    );
}
