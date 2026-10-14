//! A credit "on all N routes stackvet.toml names" names the route answers it rests on in its evidence
//! (ADR-082), so `seen.json` can show a reader what a whole credit was read from (backlog 0467).

use super::*;

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
            ("cross-origin-opener-policy", "same-origin"),
            ("content-type", "text/html; charset=utf-8"),
        ]
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect(),
        body: "<html>a page</html>".into(),
    }
}

#[test]
fn each_credit_read_from_the_routes_names_their_answers() {
    let run = vec![
        careful("home", 200),
        careful("missing", 404),
        careful("route:/login", 200),
        careful("route:/account", 302),
    ];
    let routes = vec!["/login".to_owned(), "/account".to_owned()];
    let credits = verified_with_routes(&run, &routes);
    for rule in [
        SECURITY_HEADERS.rule_id,
        CONTENT_TYPE.rule_id,
        OPENER_POLICY.rule_id,
        CSP_REPORTING.rule_id,
    ] {
        let credit = credits
            .iter()
            .find(|c| c.check_id == rule)
            .unwrap_or_else(|| panic!("the setup: {rule} was not credited: {credits:#?}"));
        // The setup: the credit is whole, read on every named route.
        assert!(!credit.in_part, "{credit:?}");
        for id in ["home", "route:/login", "route:/account"] {
            assert!(
                credit.evidence.iter().any(|e| e == id),
                "{rule}: {id} not in {:?}",
                credit.evidence
            );
        }
    }
}
