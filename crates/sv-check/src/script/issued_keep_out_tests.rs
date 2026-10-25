//! Backlog 0625, part 7 (ADR-151): a sign-in code or token the app's own authorization server issued to the test
//! client is taken out of everything the run writes down, as the seed's values are: the address the browser was sent
//! to with the code, and the token address's answer.

use super::*;

fn token() -> String {
    ["tok", "9d1f", "e44a", "b0c7"].join("-")
}

fn code() -> String {
    ["code", "51ac", "7e02"].join("-")
}

#[test]
fn a_code_and_a_token_the_app_issued_are_taken_out_of_the_record() {
    let mut out = crate::signed_in::Outcome::default();
    out.exchanges.push(crate::signed_in::recording::Recorded {
        id: "oauth-authorize".into(),
        method: "GET".into(),
        path: "/o/authorize?client_id=sv-test".into(),
        status: Some(302),
        headers: vec![(
            "location".into(),
            format!(
                "http://localhost/callback?code={}&state=oauth-authorize",
                code()
            ),
        )],
        body: String::new(),
        prompt: None,
    });
    out.exchanges.push(crate::signed_in::recording::Recorded {
        id: "oauth-token".into(),
        method: "POST".into(),
        path: "/o/token".into(),
        status: Some(200),
        headers: Vec::new(),
        body: format!(r#"{{"access_token":"{}","token_type":"Bearer"}}"#, token()),
        prompt: None,
    });
    out.issued = vec![code(), token()];
    // The setup: both are there to be found.
    assert!(format!("{:?}", out.exchanges).contains(&token()));
    keep_out_issued(&mut out);
    let all = format!("{:?}", out.exchanges);
    assert!(!all.contains(&token()), "a token the app issued was kept");
    assert!(!all.contains(&code()), "a code the app issued was kept");
    assert_eq!(all.matches(ISSUED_SHOWN).count(), 2, "{all}");
    assert!(out.issued.is_empty());
}
