//! V3.4.6 is met by the Content-Security-Policy's `frame-ancestors`; `X-Frame-Options` alone, which the requirement
//! calls obsolete and not to be relied upon, is short of it and said so (backlog 0264; ADR-047, Later).

use super::*;

fn page(policy: &str, frame_options: Option<&str>) -> ProbeResponse {
    let mut headers = vec![
        ("content-security-policy".to_owned(), policy.to_owned()),
        ("x-content-type-options".to_owned(), "nosniff".to_owned()),
        ("referrer-policy".to_owned(), "same-origin".to_owned()),
        (
            "cross-origin-opener-policy".to_owned(),
            "same-origin".to_owned(),
        ),
        (
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        ),
    ];
    if let Some(v) = frame_options {
        headers.push(("x-frame-options".to_owned(), v.to_owned()));
    }
    ProbeResponse {
        id: "home".into(),
        status: 200,
        headers,
        body: "<html>a page</html>".into(),
    }
}

const NO_ANCESTORS: &str =
    "default-src 'self'; object-src 'none'; base-uri 'none'; report-uri /csp";

#[test]
fn x_frame_options_alone_is_short_of_v3_4_6_and_said_as_obsolete() {
    let answer = page(NO_ANCESTORS, Some("DENY"));
    let found = evaluate(std::slice::from_ref(&answer));
    let headers = found
        .iter()
        .find(|f| f.rule_id == SECURITY_HEADERS.rule_id)
        .expect("X-Frame-Options alone is short of V3.4.6");
    assert!(
        headers.description.contains("frame-ancestors"),
        "{}",
        headers.description
    );
    assert!(
        headers.description.contains("obsolete"),
        "{}",
        headers.description
    );
    assert!(
        !verified(std::slice::from_ref(&answer))
            .iter()
            .any(|c| c.check_id == SECURITY_HEADERS.rule_id),
        "credited beside its own finding"
    );
}

#[test]
fn frame_ancestors_meets_it_with_or_without_x_frame_options() {
    for frame_options in [None, Some("DENY")] {
        let answer = page(
            &format!("{NO_ANCESTORS}; frame-ancestors 'none'"),
            frame_options,
        );
        assert!(
            !evaluate(std::slice::from_ref(&answer))
                .iter()
                .any(|f| f.rule_id == SECURITY_HEADERS.rule_id),
            "{frame_options:?}"
        );
    }
}

#[test]
fn neither_is_said_as_before() {
    let found = evaluate(&[page(NO_ANCESTORS, None)]);
    let headers = found
        .iter()
        .find(|f| f.rule_id == SECURITY_HEADERS.rule_id)
        .expect("no rule against framing");
    assert!(
        headers
            .description
            .contains("shown inside somebody else's page"),
        "{}",
        headers.description
    );
    assert!(
        !headers.description.contains("obsolete"),
        "{}",
        headers.description
    );
}
