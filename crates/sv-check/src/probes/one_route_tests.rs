//! With one route stackvet.toml names, the header, cookie, content-type, and reporting credits are in part,
//! whatever it answered: the whole credit needs two or more named routes, as groups B, E, and F do (backlog
//! 0467; ADR-096, Later, 14 October 2026).

use super::*;

/// What a careful app sends on every answer, a cookie included.
const CAREFUL: &[(&str, &str)] = &[
    (
        "content-security-policy",
        "default-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'; report-uri /csp",
    ),
    ("x-content-type-options", "nosniff"),
    ("referrer-policy", "strict-origin-when-cross-origin"),
    ("cross-origin-opener-policy", "same-origin"),
    ("content-type", "text/html; charset=utf-8"),
    (
        "set-cookie",
        "__Host-session=x; Path=/; Secure; HttpOnly; SameSite=Lax",
    ),
];

fn careful(id: &str, status: u16) -> ProbeResponse {
    ProbeResponse {
        id: id.into(),
        status,
        headers: CAREFUL
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        body: "<html>a page</html>".into(),
    }
}

/// The credits the named routes can make whole, by rule, with whether each is in part and its words.
fn route_credits(routes: &[&str]) -> Vec<(String, bool, String)> {
    let mut run = vec![careful("home", 200), careful("missing", 404)];
    run.extend(routes.iter().map(|r| careful(&format!("route:{r}"), 200)));
    let routes: Vec<String> = routes.iter().map(|r| (*r).to_owned()).collect();
    let rules = [
        SECURITY_HEADERS.rule_id,
        COOKIE_ATTRIBUTES.rule_id,
        CONTENT_TYPE.rule_id,
        OPENER_POLICY.rule_id,
        CSP_REPORTING.rule_id,
    ];
    verified_with_routes(&run, &routes)
        .into_iter()
        .filter(|c| rules.contains(&c.check_id.as_str()))
        .map(|c| (c.check_id, c.in_part, c.scope))
        .collect()
}

#[test]
fn one_named_route_answered_and_careful_leaves_the_credits_in_part() {
    let two = route_credits(&["/login", "/account"]);
    // The setup: with two careful routes all five are credited, and whole, so the marks below are the
    // number of routes and nothing else.
    assert_eq!(two.len(), 5, "{two:?}");
    assert!(two.iter().all(|(_, in_part, _)| !in_part), "{two:?}");

    let one = route_credits(&["/login"]);
    assert_eq!(one.len(), 5, "{one:?}");
    for (rule, in_part, scope) in &one {
        assert!(in_part, "{rule} is whole on one route: {scope}");
        // Its words still name the route it read, and never as "all 1 route".
        assert!(
            scope.contains("on the one route stackvet.toml names"),
            "{rule}: {scope}"
        );
        assert!(!scope.contains("all 1 route"), "{rule}: {scope}");
    }
}
