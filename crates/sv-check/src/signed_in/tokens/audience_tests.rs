//! Backlog 0500, V9.2.4: what a signed-in run keeps of who the app's own token says it is for, the services its `aud`
//! field names, or that it names none; and the sentence the audience question shows.

use super::super::fake_app::*;
use super::*;

/// The app's token as a run with `audience` in its `aud` field keeps it.
fn kept(audience: Option<serde_json::Value>) -> crate::personal_fields::TokenFields {
    let mut app = FakeApp::new(Flaws::default());
    app.jwt_lifetime = Some(3600);
    app.jwt_audience = audience;
    let acc = accounts();
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let o = run_with(&mut app, &users(), &acc, true, &Default::default(), false);
    o.token_fields
        .unwrap_or_else(|| panic!("no token kept: {:?}", o.steps))
}

#[test]
fn a_token_with_no_aud_is_said_to_name_no_service() {
    let token = kept(None);
    assert!(!token.cookie, "the setup: the session is a JWT");
    assert!(token.audience.is_empty());
    let said = token.audience_said(|n| format!("<{n}>"));
    assert!(said.contains("had no `aud` field"), "{said}");
}

#[test]
fn the_services_a_token_names_are_kept_one_or_several() {
    let one = kept(Some(serde_json::json!("notes-api")));
    assert_eq!(one.audience, ["notes-api"]);
    let said = one.audience_said(|n| format!("<{n}>"));
    assert!(said.contains("it is for <notes-api>."), "{said}");

    let several = kept(Some(serde_json::json!(["notes-api", "billing"])));
    assert_eq!(several.audience, ["notes-api", "billing"]);
    assert!(
        several
            .audience_said(|n| format!("<{n}>"))
            .contains("<notes-api>, <billing>")
    );
}
