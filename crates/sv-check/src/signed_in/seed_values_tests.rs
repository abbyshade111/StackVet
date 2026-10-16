//! Backlog 0536 (ADR-124; the owner's choice (a)): a sign-up that needs a value the manifest must not carry, such as an
//! invite code, cites it as `{seed:NAME}`; the seed prints it as a `NAME=value` line inside the fence, and the run
//! fills it in. A name the seed did not print is sent as written, and the run says so.

use super::fake_app::*;
use super::tests::with_signup;
use super::*;

fn code() -> String {
    invite_code()
}

fn needs_invite() -> Flaws {
    Flaws {
        signup_needs_invite: true,
        ..Default::default()
    }
}

/// `seed` makes A, B, and the admin; `signup` is the password checks' own, and cites the seed's `INVITE`.
fn users_citing_the_seed() -> UsersSection {
    let mut u = with_signup();
    u.seed = Some("seed".into());
    u.signup
        .as_mut()
        .unwrap()
        .form
        .insert("invite_code".into(), "{seed:INVITE}".into());
    u
}

fn run_with_seed_values(values: &[(&str, String)]) -> Outcome {
    let mut app = FakeApp::new(needs_invite());
    let mut acc = accounts();
    acc.totp = None;
    for account in [&acc.a, &acc.b, acc.admin.as_ref().unwrap()] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    acc.seed_values = values
        .iter()
        .map(|(k, v)| ((*k).to_owned(), v.clone()))
        .collect();
    run(
        &mut app,
        &users_citing_the_seed(),
        &acc,
        true,
        &super::tests::with_words(&["Acme Notes"]),
    )
}

#[test]
fn the_seeds_value_is_filled_into_the_sign_up_and_the_password_checks_run() {
    let o = run_with_seed_values(&[("INVITE", code())]);
    // The control account's sign-up went through, so the long-password check was answered.
    assert!(
        verified_ids(&o).contains(&LONG_PASSWORD.rule_id),
        "the sign-ups were refused despite the seed's value"
    );
    assert!(
        !o.steps.iter().any(|s| s.contains("the seed did not print")),
        "a printed value was called missing"
    );
}

#[test]
fn a_name_the_seed_did_not_print_is_sent_as_written_and_said() {
    let o = run_with_seed_values(&[("SOMETHING_ELSE", code())]);
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("`{seed:INVITE}`") && s.contains("the seed did not print")),
        "the missing value is not said"
    );
    assert!(
        !verified_ids(&o).contains(&LONG_PASSWORD.rule_id),
        "the setup: the app refuses a sign-up without its invite code"
    );
}

#[test]
fn the_names_a_template_cites_are_read_once_each() {
    let t = RequestTemplate {
        method: "POST".into(),
        path: "/join/{seed:TEAM}".into(),
        form: [
            ("invite".to_owned(), "{seed:INVITE}".to_owned()),
            ("again".to_owned(), "x{seed:INVITE}y".to_owned()),
        ]
        .into(),
        json: BTreeMap::new(),
    };
    assert_eq!(seed_names(&t), ["INVITE", "TEAM"]);
}
