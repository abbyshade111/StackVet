//! A private page asked for with a header naming a user is read beside the signed-in user's own page (backlog
//! 0560, ADR-119): "Unknown user" served with 200 is a 2xx too.

use super::super::fake_app::*;
use super::*;

#[test]
fn unknown_user_served_with_200_is_not_the_page_opening() {
    let o = run_against(
        Flaws {
            identity_header_unknown_user: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s.contains("carrying") && s.contains("that name a user")),
        "the check did not run: {:?}",
        o.steps
    );
    assert!(
        !rule_ids(&o).contains(&IDENTITY_HEADER.rule_id),
        "\"Unknown user\" on a 200 was read as the header letting the request in: {:?}",
        o.findings
    );
    assert!(
        o.not_assessed.iter().any(|(_, why)| why.contains(
            "answered 200 with a page that shows little of what the signed-in user gets"
        )),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_header_that_opens_the_page_itself_is_still_found() {
    let o = run_against(
        Flaws {
            trusts_identity_header: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        rule_ids(&o).contains(&IDENTITY_HEADER.rule_id),
        "{:?}",
        o.not_assessed
    );
}
