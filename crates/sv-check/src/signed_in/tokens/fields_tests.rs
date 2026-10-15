//! What a signed-in run keeps of the app's token for the data-classification questions (backlog 0497): the field names
//! in its readable part, never the values, and nothing when the session is not a JWT.

use super::super::fake_app::*;
use super::*;

/// stackvet.toml for an app that signs in through JSON and answers with a token.
fn bearer_users() -> UsersSection {
    let mut u = users();
    u.login = Some(RequestTemplate {
        method: "POST".into(),
        path: "/api/login".into(),
        form: BTreeMap::new(),
        json: [("email", "{user}"), ("password", "{password}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
    });
    u.token_field = Some("token".into());
    u.logout = None;
    u.owned = None;
    u
}

#[test]
fn a_jwt_sign_in_keeps_the_field_names_and_where_the_token_travels() {
    let mut app = FakeApp::new(Flaws::default());
    app.jwt_lifetime = Some(3600);
    let acc = accounts();
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let o = run_with(
        &mut app,
        &bearer_users(),
        &acc,
        true,
        &Default::default(),
        false,
    );
    let token = o
        .token_fields
        .unwrap_or_else(|| panic!("no token fields kept: {:?}", o.steps));
    assert_eq!(token.carried, "the `Authorization` header");
    // The fake app's token says who signed in (`sub`), when (`iat`), its own id (`jti`), and when it runs out (`exp`).
    let mut fields = token.fields.clone();
    fields.sort();
    assert_eq!(fields, vec!["exp", "iat", "jti", "sub"]);
    // Names only: the account the token names is nowhere in what is kept.
    assert!(
        !format!("{token:?}").contains(&acc.a.user),
        "a value was kept: {token:?}"
    );
}

#[test]
fn a_session_cookie_that_is_not_a_jwt_keeps_nothing() {
    let o = run_against(Flaws::default(), &users());
    assert_eq!(o.token_fields, None);
}
