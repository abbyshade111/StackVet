//! A refused port 80 is credited in part, as a permanent redirect there is (backlog 0466; ADR-090, Later,
//! 14 October 2026).

use super::tests_support::*;
use super::*;

#[test]
fn a_refused_port_80_is_credited_in_part_as_the_permanent_redirect_is() {
    let plain_credit = |plain: Answer| {
        let mut s = site(&[
            ("https://example.test/", ok(&[])),
            ("http://example.test/", plain),
        ]);
        run(&mut s, &target())
            .verified
            .into_iter()
            .find(|v| v.check_id == PLAIN_HTTP_SERVED.rule_id)
    };
    let refused = plain_credit(Answer {
        revocation: None,
        status: 0,
        headers: Vec::new(),
        failure: Some(
            "Failed to connect to example.test port 80 after 3 ms: Connection refused".to_owned(),
        ),
    });
    // The setup: the refusal is credited at all, so its mark below means something.
    let refused = refused.expect("a refused port 80 is credited");
    assert!(refused.in_part, "{refused:?}");
    assert!(
        refused.requirement_ids.contains(&"V12.2.1".to_owned()),
        "{refused:?}"
    );
    let redirected = plain_credit(redirect(301, "https://example.test/"))
        .expect("a permanent redirect to HTTPS is credited");
    assert_eq!(refused.in_part, redirected.in_part);
}
