//! Backlog 0599 (the Cursor and Flash run of 15 October 2026): the app limited user names to 3 to 30 characters, every
//! sign-up with a 38-character address in its `username` field was refused, and nine requirements stopped with a gap
//! that said nothing of why. Now a sign-up form that keeps a `username` apart from the address is sent a short name
//! there, and a run that still cannot sign in quotes what the sign-up page said.

use super::fake_app::*;
use super::tests::with_signup;
use super::*;

const RULE_WORDS: &str = "Username must be between 3 and 30 characters";

/// Sign-up and sign-in by user name, with `extra` beside it in the sign-up form.
fn by_username(signup_extra: &[(&str, &str)]) -> UsersSection {
    let mut u = with_signup();
    let mut form: BTreeMap<String, String> = [
        ("username", "{user}"),
        ("password", "{password}"),
        ("csrf_token", "{csrf}"),
    ]
    .iter()
    .chain(signup_extra)
    .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
    .collect();
    u.signup.as_mut().unwrap().form = std::mem::take(&mut form);
    u.login.as_mut().unwrap().form = [
        ("username", "{user}"),
        ("password", "{password}"),
        ("csrf_token", "{csrf}"),
    ]
    .iter()
    .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
    .collect();
    u
}

fn run_by_username(users: &UsersSection) -> (Outcome, FakeApp) {
    let mut app = FakeApp::new(Flaws {
        usernames_up_to_30: true,
        ..Default::default()
    });
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    // Addresses as long as the run's stand-ins, 38 characters and more.
    acc.a.user = "sv-probe-a-0123456789abcdef@example.test".into();
    acc.b.user = "sv-probe-b-0123456789abcdef@example.test".into();
    // The setup: the stand-in address is longer than the app's rule allows a user name.
    assert!(
        acc.a.user.chars().count() > 30,
        "the setup: the address fits the rule"
    );
    let o = run(
        &mut app,
        users,
        &acc,
        false,
        &super::tests::with_words(&["Acme Notes"]),
    );
    (o, app)
}

#[test]
fn a_user_name_kept_apart_from_the_address_is_sent_short_and_the_run_signs_in() {
    let (o, _) = run_by_username(&by_username(&[("email", "{user}")]));
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("signed in as A and opened /account (2")),
        "A did not sign in and open /account"
    );
    assert!(
        !o.not_assessed.iter().any(|(ids, _)| ids == NEEDS_A_SESSION),
        "the signed-in checks stopped"
    );
}

#[test]
fn a_user_name_that_is_the_only_name_keeps_the_address_and_the_gap_quotes_the_sign_up() {
    // With no address field beside it, the `username` is how the app knows the account: the address goes in it, as
    // before, and this app's rule refuses it. The gap now says why, in the page's words.
    let (o, _) = run_by_username(&by_username(&[]));
    let (_, why) = o
        .not_assessed
        .iter()
        .find(|(ids, _)| ids == NEEDS_A_SESSION)
        .expect("the run signed in, so there is no gap to read");
    assert!(
        why.contains("the sign-up of the first test user answered 422"),
        "the gap does not say what the sign-up answered"
    );
    assert!(
        why.contains(RULE_WORDS),
        "the gap does not quote the sign-up page"
    );
}

#[test]
fn the_short_name_fits_a_user_name_rule_and_differs_from_one_address_to_the_next() {
    let a = short_name("sv-probe-a-0123456789abcdef@example.test");
    let b = short_name("sv-probe-b-0123456789abcdef@example.test");
    assert_ne!(a, b);
    assert_eq!(a, short_name("sv-probe-a-0123456789abcdef@example.test"));
    for name in [&a, &b] {
        assert!((3..20).contains(&name.chars().count()), "{name}");
        assert!(name.chars().all(|c| c.is_ascii_alphanumeric()), "{name}");
        assert!(
            name.starts_with(|c: char| c.is_ascii_alphabetic()),
            "{name}"
        );
    }
}

#[test]
fn only_a_sign_up_with_a_user_name_and_an_address_wants_short_names() {
    let t = |fields: &[(&str, &str)]| RequestTemplate {
        method: "POST".into(),
        path: "/signup".into(),
        form: fields
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        json: BTreeMap::new(),
    };
    assert!(wants_short_names(&t(&[
        ("username", "{user}"),
        ("email", "{user}")
    ])));
    assert!(wants_short_names(&t(&[
        ("user[user_name]", "{user}"),
        ("user[email]", "{user}")
    ])));
    assert!(!wants_short_names(&t(&[
        ("username", "{user}"),
        ("password", "{password}")
    ])));
    assert!(!wants_short_names(&t(&[
        ("email", "{user}"),
        ("password", "{password}")
    ])));
}

#[test]
fn a_log_line_naming_the_short_name_names_the_account() {
    let address = "sv-log-nobody-4a91@example.test";
    let markers = crate::logs::Markers {
        failed_sign_in: Some(address.into()),
        ..Default::default()
    };
    let by_address = crate::logs::evaluate(
        &markers,
        &format!("2026-09-26T10:00:01Z sign-in failed for {address}\n"),
    );
    let by_short = crate::logs::evaluate(
        &markers,
        &format!(
            "2026-09-26T10:00:01Z sign-in failed for {}\n",
            short_name(address)
        ),
    );
    let ids = |o: &crate::logs::LogOutcome| {
        let mut ids: Vec<String> = o.verified.iter().map(|v| v.check_id.clone()).collect();
        ids.sort();
        ids
    };
    assert!(
        !ids(&by_address).is_empty(),
        "the setup: the address alone is credited"
    );
    assert_eq!(ids(&by_short), ids(&by_address));
}
