//! The header, cookie, and content-type checks also read the anonymous answer for each route stackvet.toml names,
//! whatever its status, and their credits are whole only when every named route answered and passed (ADR-095,
//! backlog 0232 part 1).

use super::*;

/// What a careful app sends on every answer, a redirect included.
const CAREFUL: &[(&str, &str)] = &[
    (
        "content-security-policy",
        "default-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'; report-uri /csp",
    ),
    ("x-content-type-options", "nosniff"),
    ("referrer-policy", "strict-origin-when-cross-origin"),
    ("cross-origin-opener-policy", "same-origin"),
    ("content-type", "text/html; charset=utf-8"),
];

fn answer(id: &str, status: u16, headers: &[(&str, &str)], body: &str) -> ProbeResponse {
    ProbeResponse {
        id: id.into(),
        status,
        headers: headers
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        body: body.into(),
    }
}

/// A careful answer, less the header named in `without` (`""` for none).
fn careful(id: &str, status: u16, without: &str) -> ProbeResponse {
    let headers: Vec<(&str, &str)> = CAREFUL
        .iter()
        .copied()
        .filter(|(k, _)| *k != without)
        .collect();
    answer(id, status, &headers, "<html>a page</html>")
}

const ROUTES: [&str; 2] = ["/login", "/account"];

fn routes() -> Vec<String> {
    ROUTES.iter().map(|r| (*r).to_owned()).collect()
}

/// The health path, the missing page, and an answer for each route: the sign-in page, and a redirect from the
/// private page, both careful.
fn careful_run() -> Vec<ProbeResponse> {
    vec![
        careful("home", 200, ""),
        careful("missing", 404, ""),
        careful("route:/login", 200, ""),
        careful("route:/account", 302, ""),
    ]
}

/// The group A credits, by rule, and whether each is in part.
fn group_a(credits: &[crate::Verified]) -> Vec<(&str, bool)> {
    let a = [
        SECURITY_HEADERS.rule_id,
        CONTENT_TYPE.rule_id,
        OPENER_POLICY.rule_id,
        CSP_REPORTING.rule_id,
    ];
    credits
        .iter()
        .filter(|c| a.contains(&c.check_id.as_str()))
        .map(|c| (c.check_id.as_str(), c.in_part))
        .collect()
}

#[test]
fn every_named_route_answered_and_careful_makes_the_credits_whole() {
    let credits = verified_with_routes(&careful_run(), &routes());
    let a = group_a(&credits);
    // The setup: all four were credited, so "whole" below is about each of them.
    assert_eq!(a.len(), 4, "{a:?}");
    assert!(a.iter().all(|(_, in_part)| !in_part), "{a:?}");
    let headers = credits
        .iter()
        .find(|c| c.check_id == SECURITY_HEADERS.rule_id)
        .unwrap();
    assert!(
        headers
            .scope
            .contains("on all 2 routes stackvet.toml names"),
        "{}",
        headers.scope
    );
}

#[test]
fn a_named_route_that_did_not_answer_leaves_the_credits_in_part() {
    let mut run = careful_run();
    run.retain(|r| r.id != "route:/account");
    let credits = verified_with_routes(&run, &routes());
    let a = group_a(&credits);
    assert_eq!(a.len(), 4, "{a:?}");
    assert!(a.iter().all(|(_, in_part)| *in_part), "{a:?}");
}

#[test]
fn with_no_named_routes_the_credits_stay_in_part() {
    let credits = verified(&careful_run());
    let a = group_a(&credits);
    assert_eq!(a.len(), 4, "{a:?}");
    assert!(a.iter().all(|(_, in_part)| *in_part), "{a:?}");
}

#[test]
fn a_redirect_from_a_named_route_without_a_header_is_found_and_not_credited() {
    let mut run = careful_run();
    run.retain(|r| r.id != "route:/account");
    run.push(careful("route:/account", 302, "x-content-type-options"));
    let found = evaluate(&run);
    let headers = found
        .iter()
        .find(|f| f.rule_id == SECURITY_HEADERS.rule_id)
        .expect("the security-headers finding");
    assert_eq!(headers.evidence, ["route:/account"]);
    assert!(
        headers
            .description
            .contains("answer for /account came back without X-Content-Type-Options"),
        "{}",
        headers.description
    );
    let credited = verified_with_routes(&run, &routes());
    assert!(
        !credited
            .iter()
            .any(|c| c.check_id == SECURITY_HEADERS.rule_id),
        "credited beside its own finding"
    );
}

#[test]
fn a_policy_on_a_named_route_that_reports_nowhere_is_found() {
    let mut run = careful_run();
    run.retain(|r| r.id != "route:/login");
    run.push(answer(
        "route:/login",
        200,
        &[(
            "content-security-policy",
            "default-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'",
        )],
        "<html>sign in</html>",
    ));
    let found = evaluate(&run);
    let csp = found
        .iter()
        .find(|f| f.rule_id == CSP_REPORTING.rule_id)
        .expect("the CSP reporting finding");
    assert_eq!(csp.evidence, ["route:/login"]);
    assert!(
        !verified_with_routes(&run, &routes())
            .iter()
            .any(|c| c.check_id == CSP_REPORTING.rule_id),
        "credited beside its own finding"
    );
}

#[test]
fn the_routes_are_read_from_the_manifest_once_each_without_placeholders_or_what_is_asked_already() {
    let text = r#"
manifest-version = 1
[app]
name = "Routes"
audience = "customers"
deployment = "internet"
[stack]
languages = ["python"]
[stack.run]
image = "python:3.12-slim"
start = "python app.py"
health = "/healthz"
[stack.run.users]
seed = "python seed.py"
login = { path = "/login", form = { email = "{user}", password = "{password}" } }
signup = { path = "/signup?next=/", form = { email = "{user}", password = "{password}" } }
logout = { path = "/", form = {} }
private = ["/account", "/notes/{id}", "/login"]
admin = ["/admin", "/healthz"]
change-password = { path = "/password", form = { current = "{password}", new = "{new_password}" } }
"#;
    let manifest =
        sv_manifest::Manifest::parse(text, std::path::Path::new("stackvet.toml")).unwrap();
    let users = manifest
        .stack
        .run
        .users
        .as_ref()
        .expect("the users section");
    assert_eq!(
        named_routes(users, "/healthz"),
        ["/login", "/signup", "/password", "/account", "/admin"]
    );
    let asked = route_requests(&named_routes(users, "/healthz"));
    assert!(
        asked
            .iter()
            .all(|r| r.method == "GET" && r.headers.is_empty() && r.body.is_none())
    );
    assert_eq!(asked[0].id, "route:/login");
}
