//! A setting the start command loosens the copy with earns that copy no credit for the rules it bears on (backlog 0564,
//! ADR-114, Later): the credit goes, a gap names the setting, and a finding under those rules says it is about the copy.

use super::*;
use sv_check::{Confidence, Location, Severity};

fn finding(rule: &str) -> sv_check::Finding {
    sv_check::finding::found(sv_check::Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: format!("found by {rule}"),
        severity: Severity::High,
        confidence: Confidence::Medium,
        location: Location {
            file: "app.py".into(),
            line: 0,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: "Seen.".into(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: format!("fp-{rule}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

fn credit(rule: &str, ids: &[&str]) -> sv_check::Verified {
    sv_check::Verified::new(rule, ids, "what the run saw".into())
}

#[test]
fn a_credit_the_loosened_setting_bears_on_is_withheld_and_said() {
    let mut findings = vec![finding("probe.cookie-attributes")];
    let mut verified = vec![
        credit("probe.session-cookie-attributes", &["V3.3.1"]),
        credit("probe.cross-site-request-accepted", &["V3.5.1"]),
    ];
    let gaps = loosened_for_the_run(
        &mut findings,
        &mut verified,
        "SESSION_COOKIE_SECURE=False python app.py",
    );
    // The cookie credit goes; the forgery credit, which the setting does not bear on, stays.
    let left: Vec<&str> = verified.iter().map(|v| v.check_id.as_str()).collect();
    assert_eq!(left, ["probe.cross-site-request-accepted"]);
    assert_eq!(gaps.len(), 1, "{gaps:?}");
    assert!(
        gaps[0].why.contains("`SESSION_COOKIE_SECURE=False`"),
        "{}",
        gaps[0].why
    );
    assert_eq!(gaps[0].requirements, ["V3.3.1"]);
    assert!(
        findings[0].description.contains("this is about that copy"),
        "{}",
        findings[0].description
    );
}

#[test]
fn a_setting_that_only_looks_like_a_sign_in_limit_withholds_nothing() {
    // Raising a limit on every sign-in for the run is what ADR-114 asks of the copy.
    let mut verified = vec![credit("probe.failed-sign-ins-unlimited", &["V6.3.1"])];
    let gaps = loosened_for_the_run(
        &mut [],
        &mut verified,
        "CLUB_LOGIN_ATTEMPTS=100 python app.py",
    );
    assert!(gaps.is_empty(), "{gaps:?}");
    assert_eq!(verified.len(), 1);
}

#[test]
fn a_value_that_could_be_a_key_is_never_shown() {
    let mut verified = vec![credit("probe.cross-site-request-accepted", &["V3.5.1"])];
    // Built from pieces, so no key-shaped value sits in the source.
    let value = ["sk", "-live-", "0123456789abcdef"].concat();
    let start = format!("DISABLE_CSRF={value} python app.py");
    let gaps = loosened_for_the_run(&mut [], &mut verified, &start);
    assert_eq!(gaps.len(), 1, "{gaps:?}");
    assert!(gaps[0].why.contains("`DISABLE_CSRF`"), "{}", gaps[0].why);
    assert!(!gaps[0].why.contains(&value), "{}", gaps[0].why);
}

#[test]
fn every_rule_the_table_bears_on_is_one_of_svs() {
    let reach = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reach.json"),
    )
    .expect("data/reach.json");
    for rule in sv_run::start_settings::rules_borne_on() {
        assert!(
            reach.contains(&format!("\"{rule}\"")),
            "{rule} is not a rule of sv's"
        );
    }
}
