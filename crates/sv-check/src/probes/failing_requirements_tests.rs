//! A finding on the headers or the cookie attributes names the requirements it found failing, not every
//! one its rule can credit (backlog 0432; ADR-047, Later). One missing `Referrer-Policy` marked V3.4.3,
//! V3.4.4, and V3.4.6 *needs attention* too, for headers the page had.

use super::*;

const CAREFUL: &[(&str, &str)] = &[
    (
        "content-security-policy",
        "default-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'; report-uri /csp",
    ),
    ("x-content-type-options", "nosniff"),
    ("referrer-policy", "strict-origin-when-cross-origin"),
    ("content-type", "text/html; charset=utf-8"),
];

fn home(without: &str, cookie: Option<&str>) -> Vec<ProbeResponse> {
    let mut headers: Vec<(String, String)> = CAREFUL
        .iter()
        .filter(|(k, _)| *k != without)
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect();
    if let Some(cookie) = cookie {
        headers.push(("set-cookie".to_owned(), cookie.to_owned()));
    }
    vec![ProbeResponse {
        id: "home".into(),
        status: 200,
        headers,
        body: "<html>a page</html>".into(),
    }]
}

fn ids_of(found: &[Finding], rule: &str) -> Vec<String> {
    found
        .iter()
        .find(|f| f.rule_id == rule)
        .unwrap_or_else(|| panic!("the setup: no {rule} finding: {found:#?}"))
        .requirement_ids
        .clone()
}

#[test]
fn a_page_missing_only_its_referrer_policy_fails_v3_4_5_alone() {
    let found = evaluate(&home("referrer-policy", None));
    assert_eq!(ids_of(&found, SECURITY_HEADERS.rule_id), ["V3.4.5"]);
    // The control: a page with none of them names all four, in the rule's order.
    let bare = vec![ProbeResponse {
        id: "home".into(),
        status: 200,
        headers: vec![("content-type".into(), "text/html".into())],
        body: "<html>a page</html>".into(),
    }];
    assert_eq!(
        ids_of(&evaluate(&bare), SECURITY_HEADERS.rule_id),
        SECURITY_HEADERS.requirement_ids
    );
}

#[test]
fn a_cookie_missing_only_same_site_fails_v3_3_2_alone() {
    let found = evaluate(&home("", Some("sid=a; Path=/; Secure; HttpOnly")));
    assert_eq!(ids_of(&found, COOKIE_ATTRIBUTES.rule_id), ["V3.3.2"]);
    let found = evaluate(&home("", Some("sid=a; Path=/; Secure; SameSite=Lax")));
    assert_eq!(ids_of(&found, COOKIE_ATTRIBUTES.rule_id), ["V3.3.4"]);
}
