//! Signed in, as for a stranger: a finding on a private page's headers, or on the session cookie, names
//! the requirements it found failing, not every one its rule can credit (backlog 0432; ADR-047, Later).

use super::fake_app::*;
use super::*;

fn ids_of(o: &Outcome, rule: &str) -> Vec<String> {
    o.findings
        .iter()
        .find(|f| f.rule_id == rule)
        .unwrap_or_else(|| panic!("the setup: no {rule} finding: {:#?}", o.findings))
        .requirement_ids
        .clone()
}

#[test]
fn a_private_page_short_only_of_the_policy_minimum_fails_v3_4_3_alone() {
    let o = run_against(
        Flaws {
            private_page_policy_without_base_uri: true,
            ..Flaws::default()
        },
        &users(),
    );
    assert_eq!(ids_of(&o, PRIVATE_PAGE_HEADERS.rule_id), ["V3.4.3"]);
}

#[test]
fn a_session_cookie_missing_only_http_only_fails_v3_3_4_alone() {
    let o = run_against(
        Flaws {
            no_httponly: true,
            ..Flaws::default()
        },
        &users(),
    );
    assert_eq!(ids_of(&o, SESSION_COOKIE.rule_id), ["V3.3.4"]);
}
