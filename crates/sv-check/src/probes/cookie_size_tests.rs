//! V3.3.5 (backlog 0507): a cookie whose name and value together pass 4096 bytes is one a browser drops. Found on the
//! pages a stranger sees; credited in part only when a cookie was there to judge, and never beside the finding.

use super::*;

/// A page with every header a careful app sends, and `cookies`.
fn home(cookies: &[String]) -> ProbeResponse {
    let mut headers = vec![
        (
            "content-security-policy".to_owned(),
            "default-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'"
                .to_owned(),
        ),
        ("x-content-type-options".to_owned(), "nosniff".to_owned()),
        ("referrer-policy".to_owned(), "no-referrer".to_owned()),
        (
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        ),
        (
            "cross-origin-opener-policy".to_owned(),
            "same-origin".to_owned(),
        ),
    ];
    headers.extend(cookies.iter().map(|c| ("set-cookie".to_owned(), c.clone())));
    ProbeResponse {
        id: "home".into(),
        status: 200,
        headers,
        body: "<html>hi</html>".into(),
    }
}

/// A cookie called `cart` whose name and value together are `length` bytes, with its attributes after.
fn cart(length: usize) -> String {
    format!(
        "cart={}; HttpOnly; SameSite=Lax; Path=/",
        "x".repeat(length - "cart".len())
    )
}

fn size_finding(page: &ProbeResponse) -> Option<Finding> {
    evaluate(std::slice::from_ref(page))
        .into_iter()
        .find(|f| f.rule_id == COOKIE_SIZE.rule_id)
}

fn size_credit(page: &ProbeResponse) -> Option<crate::Verified> {
    verified(std::slice::from_ref(page))
        .into_iter()
        .find(|v| v.check_id == COOKIE_SIZE.rule_id)
}

#[test]
fn a_cookie_one_byte_over_is_found_and_credits_nothing() {
    let page = home(&[
        cart(COOKIE_MOST + 1),
        "theme=dark; HttpOnly; SameSite=Lax".to_owned(),
    ]);
    let found = size_finding(&page).expect("4097 bytes is found");
    assert_eq!(found.requirement_ids, ["V3.3.5"]);
    assert_eq!(found.severity, Severity::Low);
    assert!(found.description.contains("`cart`") && found.description.contains("4097 bytes"));
    assert!(
        !found.description.contains("`theme`"),
        "{}",
        found.description
    );
    assert_eq!(found.evidence, ["home"]);
    // Found, it is not credited too (ADR-059).
    assert!(size_credit(&page).is_none());
}

#[test]
fn a_cookie_of_exactly_the_limit_is_credited_in_part() {
    let page = home(&[cart(COOKIE_MOST)]);
    assert!(size_finding(&page).is_none());
    let credit = size_credit(&page).expect("4096 bytes is credited");
    assert!(credit.in_part, "{credit:?}");
    assert_eq!(credit.requirement_ids, ["V3.3.5"]);
}

#[test]
fn the_attributes_after_the_value_do_not_count() {
    // A short cookie with a long list of attributes: the browser's limit is on the name and value.
    let long_attributes = format!(
        "session=abc; HttpOnly; SameSite=Lax; Path=/{}",
        "a".repeat(5000)
    );
    assert!(size_finding(&home(&[long_attributes])).is_none());
}

#[test]
fn no_cookie_is_neither_found_nor_credited() {
    let page = home(&[]);
    assert!(size_finding(&page).is_none());
    assert!(
        size_credit(&page).is_none(),
        "no cookie is not a cookie of the right size"
    );
}
