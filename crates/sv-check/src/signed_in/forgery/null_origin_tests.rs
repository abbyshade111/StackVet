//! Sweep 3 (backlog 0561, ADR-119), inventory item 6: a create request refused from another site's Origin is sent
//! once more as a sandboxed frame on any site sends it, `Origin: null` with no token. An app guarded only by an
//! Origin allow-list that also takes `null` refuses the first and takes this.

use super::super::fake_app::*;
use super::*;

#[test]
fn an_origin_list_that_takes_null_is_found() {
    let o = run_against(
        Flaws {
            origin_list_takes_null: true,
            ..Default::default()
        },
        &users(),
    );
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == FORGERY.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        found.evidence.iter().any(|e| e.ends_with("-null-origin")),
        "{:?}",
        found.evidence
    );
    assert!(
        found.description.contains("with `Origin: null`"),
        "{}",
        found.description
    );
    assert!(
        !o.verified.iter().any(|v| v.check_id == FORGERY.rule_id),
        "{:?}",
        o.verified
    );
}

#[test]
fn a_token_check_refuses_origin_null_too_and_the_credit_says_so() {
    let o = run_against(Flaws::default(), &users());
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == FORGERY.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        credit.scope.contains("sent again with `Origin: null`"),
        "{}",
        credit.scope
    );
}
