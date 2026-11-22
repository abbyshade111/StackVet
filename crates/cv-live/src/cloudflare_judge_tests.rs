//! What each Cloudflare setting counts as (backlog 0732; ADR-157, Later, 22 November 2026).

use super::*;
use crate::cloudflare::cloudflare_tests::{Fake, user_token, whole};
use crate::cloudflare::{Zone, read};

fn judged(change: impl FnOnce(Fake) -> Fake) -> (Evidence, Reading) {
    let mut fake = change(whole());
    let reading = read(
        &Zone::parse("stackvet.dev").unwrap(),
        &user_token(),
        &mut fake,
    );
    (judge(&reading), reading)
}

fn with(id: &str, v: &str) -> impl FnOnce(Fake) -> Fake {
    let path = format!("/zones/z1/settings/{id}");
    let body = format!(r#"{{"success":true,"result":{{"id":"{id}","value":{v}}}}}"#);
    move |fake: Fake| fake.ok(&path, &body)
}

fn finds(e: &Evidence, rule: &Rule) -> bool {
    e.findings.iter().any(|f| f.rule_id == rule.id)
}

fn credits(e: &Evidence, rule: &Rule) -> bool {
    e.verified.iter().any(|v| v.check_id == rule.id)
}

#[test]
fn a_zone_set_safely_is_credited_in_part_and_its_cache_rules_are_never_credited() {
    let (e, _) = judged(|f| f);
    assert!(
        e.findings.is_empty(),
        "{:?}",
        e.findings.iter().map(|f| &f.rule_id).collect::<Vec<_>>()
    );
    for rule in [
        &ORIGIN_UNENCRYPTED,
        &ORIGIN_UNCHECKED,
        &OLD_TLS,
        &NO_TLS_13,
        &HTTP_ALLOWED,
        &HSTS,
    ] {
        assert!(credits(&e, rule), "{}", rule.id);
    }
    assert!(
        e.verified
            .iter()
            .all(|v| v.scope.contains("as Cloudflare reports it"))
    );
    assert!(!credits(&e, &CACHE_RULES));
    assert!(e.not_assessed.iter().any(|n| n.rule_id == CACHE_RULES.id));
}

#[test]
fn flexible_or_off_is_plain_http_to_the_app() {
    for mode in [r#""flexible""#, r#""off""#] {
        let (e, _) = judged(with("ssl", mode));
        assert!(finds(&e, &ORIGIN_UNENCRYPTED), "{mode}");
        assert!(!credits(&e, &ORIGIN_UNENCRYPTED));
        assert!(
            !finds(&e, &ORIGIN_UNCHECKED),
            "{mode}: one finding for one fault"
        );
    }
}

#[test]
fn full_encrypts_without_checking_the_certificate() {
    let (e, _) = judged(with("ssl", r#""full""#));
    assert!(credits(&e, &ORIGIN_UNENCRYPTED));
    assert!(finds(&e, &ORIGIN_UNCHECKED));
}

#[test]
fn a_mode_finding_says_it_needs_a_server_behind_cloudflare() {
    // stackvet.dev is served by a Worker alone, where the mode guards nothing: each mode finding says what it depends on.
    for mode in [r#""flexible""#, r#""full""#] {
        let (e, _) = judged(with("ssl", mode));
        let f = e
            .findings
            .iter()
            .find(|f| f.rule_id == ORIGIN_UNENCRYPTED.id || f.rule_id == ORIGIN_UNCHECKED.id)
            .expect("the setup: a mode finding");
        assert!(
            f.description.contains("does not read the DNS records"),
            "{mode}: {}",
            f.description
        );
    }
}

#[test]
fn a_mode_cv_does_not_know_is_not_assessed_never_a_pass() {
    let (e, _) = judged(with("ssl", r#""origin_pull""#));
    for rule in [&ORIGIN_UNENCRYPTED, &ORIGIN_UNCHECKED] {
        assert!(!credits(&e, rule) && !finds(&e, rule));
        assert!(e.not_assessed.iter().any(|n| n.rule_id == rule.id));
    }
}

#[test]
fn an_automatic_mode_is_said_beside_the_mode() {
    let (e, _) = judged(with("ssl_automatic_mode", r#""auto""#));
    assert!(
        e.verified
            .iter()
            .any(|v| v.check_id == ORIGIN_UNENCRYPTED.id && v.scope.contains("may raise"))
    );
}

#[test]
fn tls_older_than_1_2_or_tls_1_3_off_is_a_finding() {
    for v in [r#""1.0""#, r#""1.1""#] {
        let (e, _) = judged(with("min_tls_version", v));
        assert!(finds(&e, &OLD_TLS), "{v}");
    }
    let (e, _) = judged(with("tls_1_3", r#""off""#));
    assert!(finds(&e, &NO_TLS_13));
}

#[test]
fn http_not_redirected_is_a_finding() {
    let (e, _) = judged(with("always_use_https", r#""off""#));
    assert!(finds(&e, &HTTP_ALLOWED));
}

#[test]
fn hsts_short_is_a_finding_off_is_not_assessed_and_subdomains_wait_on_the_level() {
    // Off: Cloudflare adds no header, which says nothing of whether the app sends one (stackvet.dev does).
    let off = r#"{"strict_transport_security":{"enabled":false,"max_age":0,"include_subdomains":false,"preload":false}}"#;
    let (e, _) = judged(with("security_header", off));
    assert!(!finds(&e, &HSTS) && !credits(&e, &HSTS));
    assert!(
        e.not_assessed
            .iter()
            .any(|n| n.rule_id == HSTS.id && n.why.contains("whether the app sends it"))
    );
    let short = r#"{"strict_transport_security":{"enabled":true,"max_age":15552000,"include_subdomains":true,"preload":false}}"#;
    let (e, _) = judged(with("security_header", short));
    assert!(finds(&e, &HSTS));
    let no_sub = r#"{"strict_transport_security":{"enabled":true,"max_age":31536000,"include_subdomains":false,"preload":true}}"#;
    let (e, _) = judged(with("security_header", no_sub));
    assert!(credits(&e, &HSTS));
    assert!(
        e.not_assessed
            .iter()
            .any(|n| n.rule_id == HSTS.id && n.why.contains("level"))
    );
    assert!(
        e.verified
            .iter()
            .any(|v| v.check_id == HSTS.id && v.scope.contains("V3.7.4"))
    );
}

#[test]
fn a_setting_not_read_is_not_assessed() {
    let (e, _) = judged(|f| f.answer("/zones/z1/settings/min_tls_version", 403, "{}"));
    assert!(!credits(&e, &OLD_TLS) && !finds(&e, &OLD_TLS));
    assert!(e.not_assessed.iter().any(|n| n.rule_id == OLD_TLS.id));
}

#[test]
fn a_reading_that_stopped_settles_nothing() {
    let (e, reading) = judged(|f| {
        f.ok(
            "/zones?name=stackvet.dev",
            r#"{"success":true,"result":[]}"#,
        )
    });
    assert!(reading.stopped.is_some());
    assert!(e.findings.is_empty() && e.verified.is_empty());
    assert_eq!(e.not_assessed.len(), RULES.len());
}

#[test]
fn the_token_is_said_before_any_finding_as_not_confirmed_read_only() {
    let (_, reading) = judged(|f| f);
    let said = token_words(&reading);
    assert!(
        said.contains("could not confirm it is read-only") && said.contains("Zone Settings Read"),
        "{said}"
    );
}
