//! V3.4.7 asks that the Content-Security-Policy say where to report what it blocks. A named route that
//! answered with no policy at all says nowhere, so the reporting credit cannot be whole "on all N routes"
//! while that route was left out of the reading (backlog 0419; ADR-096, Later). The missing policy is the
//! security-headers finding, as before.

use super::*;

const POLICY: &str = "default-src 'self'; object-src 'none'; base-uri 'none'; report-uri /csp";

fn answer(id: &str, status: u16, policy: bool) -> ProbeResponse {
    let mut headers = vec![(
        "content-type".to_owned(),
        "text/html; charset=utf-8".to_owned(),
    )];
    if policy {
        headers.push(("content-security-policy".to_owned(), POLICY.to_owned()));
    }
    ProbeResponse {
        id: id.into(),
        status,
        headers,
        body: "<html>a page</html>".into(),
    }
}

fn routes() -> Vec<String> {
    vec!["/login".to_owned(), "/account".to_owned()]
}

fn reporting(credits: &[crate::Verified]) -> Option<&crate::Verified> {
    credits.iter().find(|c| c.check_id == CSP_REPORTING.rule_id)
}

#[test]
fn a_named_route_with_no_policy_leaves_the_reporting_credit_in_part() {
    let run = vec![
        answer("home", 200, true),
        answer("missing", 404, true),
        answer("route:/login", 200, true),
        answer("route:/account", 302, false),
    ];
    let credits = verified_with_routes(&run, &routes());
    // The setup: every named route answered, and the missing policy is the security-headers finding.
    assert!(
        evaluate(&run)
            .iter()
            .any(|f| f.rule_id == SECURITY_HEADERS.rule_id),
        "the setup: /account's missing policy is a finding"
    );
    let credit =
        reporting(&credits).expect("the health path's policy reports, so there is a credit");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        !credit.scope.contains("on all 2 routes"),
        "{}",
        credit.scope
    );
}

#[test]
fn every_named_route_with_a_reporting_policy_makes_the_credit_whole() {
    let run = vec![
        answer("home", 200, true),
        answer("missing", 404, true),
        answer("route:/login", 200, true),
        answer("route:/account", 302, true),
    ];
    let credits = verified_with_routes(&run, &routes());
    let credit = reporting(&credits).expect("a credit");
    assert!(!credit.in_part, "{credit:?}");
    assert!(credit.scope.contains("on all 2 routes"), "{}", credit.scope);
}
