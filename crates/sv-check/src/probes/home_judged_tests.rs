//! The header and cookie credits rest on the health path's own answer. When the rate limiter answered it
//! and it was left out, and the site root was not a page, the credits were read from the named routes
//! alone, whole, and worded "the app's answers on , and on all 2 routes" (backlog 0444).

use super::*;

const COOKIE: &str = "sid=a; Path=/; Secure; HttpOnly; SameSite=Lax";

fn careful(id: &str, status: u16) -> ProbeResponse {
    ProbeResponse {
        id: id.into(),
        status,
        headers: [
            (
                "content-security-policy",
                "default-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'; report-uri /csp",
            ),
            ("x-content-type-options", "nosniff"),
            ("referrer-policy", "strict-origin-when-cross-origin"),
            ("content-type", "text/html; charset=utf-8"),
            ("set-cookie", COOKIE),
        ]
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect(),
        body: "<html>a page</html>".into(),
    }
}

fn routes() -> Vec<String> {
    vec!["/login".to_owned(), "/account".to_owned()]
}

fn credit<'a>(credits: &'a [crate::Verified], rule: &str) -> Option<&'a crate::Verified> {
    credits.iter().find(|c| c.check_id == rule)
}

#[test]
fn without_the_health_path_s_answer_neither_credit_is_given() {
    // The health path's answer was left out; the root answered 404; both named routes answered, careful.
    let run = vec![
        careful("root", 404),
        careful("route:/login", 200),
        careful("route:/account", 302),
    ];
    let credits = verified_with_routes(&run, &routes());
    for rule in [SECURITY_HEADERS.rule_id, COOKIE_ATTRIBUTES.rule_id] {
        assert!(credit(&credits, rule).is_none(), "{rule}: {credits:#?}");
    }
    // The control: with the health path's answer, both are credited, and the words name it.
    let mut with_home = run.clone();
    with_home.push(careful("home", 200));
    let credits = verified_with_routes(&with_home, &routes());
    for rule in [SECURITY_HEADERS.rule_id, COOKIE_ATTRIBUTES.rule_id] {
        let c = credit(&credits, rule).unwrap_or_else(|| panic!("{rule}: {credits:#?}"));
        assert!(!c.scope.contains(" on ,"), "{}", c.scope);
    }
}
