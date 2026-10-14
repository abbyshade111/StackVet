//! The cookie check's credit is whole when every route stackvet.toml names answered with careful cookies,
//! as the other group A credits are (ADR-096). Until backlog 0430 no test set a cookie on every route, so
//! the suite only ever saw this credit in part, and the census of in-part credits could not tell it from a
//! check that is in part always.

use super::*;

const COOKIE: &str = "sid=a; Path=/; Secure; HttpOnly; SameSite=Lax";

fn with_cookie(id: &str, status: u16) -> ProbeResponse {
    ProbeResponse {
        id: id.into(),
        status,
        headers: vec![
            (
                "content-type".to_owned(),
                "text/html; charset=utf-8".to_owned(),
            ),
            ("set-cookie".to_owned(), COOKIE.to_owned()),
        ],
        body: "<html>a page</html>".into(),
    }
}

fn run() -> Vec<ProbeResponse> {
    vec![
        with_cookie("home", 200),
        with_cookie("missing", 404),
        with_cookie("route:/login", 200),
        with_cookie("route:/account", 302),
    ]
}

fn routes() -> Vec<String> {
    vec!["/login".to_owned(), "/account".to_owned()]
}

/// The cookie credit, if there is one.
fn cookie_credit(credits: &[crate::Verified]) -> Option<&crate::Verified> {
    credits
        .iter()
        .find(|c| c.check_id == COOKIE_ATTRIBUTES.rule_id)
}

#[test]
fn careful_cookies_on_every_named_route_make_the_cookie_credit_whole() {
    let credits = verified_with_routes(&run(), &routes());
    // The setup: the cookies passed, so there is a credit to be whole or not.
    let credit = cookie_credit(&credits).expect("a credit for careful cookies");
    assert!(!credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("on all 2 routes stackvet.toml names"),
        "{}",
        credit.scope
    );
}

#[test]
fn a_named_route_that_did_not_answer_leaves_the_cookie_credit_in_part() {
    let mut run = run();
    run.retain(|r| r.id != "route:/account");
    let credits = verified_with_routes(&run, &routes());
    let credit = cookie_credit(&credits).expect("a credit for careful cookies");
    assert!(credit.in_part, "{credit:?}");
}
