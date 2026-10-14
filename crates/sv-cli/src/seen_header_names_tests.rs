//! A header whose name says it holds a token, a session, or a key (backlog 0285): its value is a
//! random string no rule for keys knows, so it is cut for its name, as the nine listed session
//! headers are.

use super::*;

fn rules() -> &'static SecretRules {
    SecretRules::shared().expect("the rules sv ships")
}

/// A random token no vendor's shape describes, built from pieces so this file holds none.
fn random_token() -> String {
    ["q7Vd2", "Kx9Lp", "4Rm8T", "w1Zs6", "Hb3Nc"].concat()
}

#[test]
fn a_token_under_a_header_named_for_it_does_not_reach_the_record() {
    let rules = rules();
    let token = random_token();
    // The setup: no rule finds this token on its own, so only the header's name can cut it.
    let (alone, found) = redact_text(rules, &token);
    assert_eq!((alone.as_str(), found), (token.as_str(), 0), "the setup");
    let names = [
        "x-session-token",
        "x-access-token",
        "x-refresh-token",
        "x-amz-security-token",
        "refresh-token",
        "x-session-id",
        "x-client-secret",
    ];
    let headers: Vec<(String, String)> = names
        .iter()
        .map(|n| ((*n).to_owned(), token.clone()))
        .chain([("x-refresh-token-2".to_owned(), format!("Bearer {token}"))])
        .collect();
    let mut removed = 0;
    let kept = kept_exchange(rules, &mut removed, "q", "GET", "/", 200, &headers, "");
    for (name, value) in &kept.headers {
        assert!(!value.contains(&token), "{name}: {value}");
        assert!(value.contains("[removed, "), "{name}: {value}");
    }
    let names_kept: Vec<&str> = kept.headers.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(&names_kept[..names.len()], &names[..]);
    assert_eq!(
        kept.headers[names.len()].1,
        "Bearer [removed, 25 characters]"
    );
}

#[test]
fn a_header_that_only_names_credentials_keeps_its_value() {
    // The browser's cross-origin headers name credentials and headers; they never hold one, and a
    // person following a finding about them needs to read them.
    let rules = rules();
    let headers: Vec<(String, String)> = [
        ("access-control-allow-credentials", "true"),
        (
            "access-control-allow-headers",
            "Authorization, X-Session-Token",
        ),
        ("x-request-id", "r-1234"),
        ("content-type", "text/html"),
    ]
    .iter()
    .map(|(n, v)| ((*n).to_owned(), (*v).to_owned()))
    .collect();
    let mut removed = 0;
    let kept = kept_exchange(rules, &mut removed, "q", "GET", "/", 200, &headers, "");
    assert_eq!(kept.headers, headers);
    assert_eq!(removed, 0);
}
