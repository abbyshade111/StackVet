//! A wrong-password finding says when the start command sets something that looks like a sign-in limit (backlog 0386).
//! Seven of the nine such findings in the second loop trial were about a limit raised for the copy `sv` ran alone.

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
        description: "Wrong passwords could be tried without limit.".into(),
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

#[test]
fn the_wrong_password_finding_names_a_limit_set_for_the_run() {
    let mut findings = vec![
        finding("probe.failed-sign-ins-unlimited"),
        finding("probe.session-cookie-attributes"),
    ];
    limits_set_for_the_run(
        &mut findings,
        "CLUB_LOGIN_ATTEMPTS=100 CLUB_SECURE_COOKIES=0 python app.py",
    );
    let said = &findings[0].description;
    assert!(
        said.starts_with("Wrong passwords could be tried without limit. "),
        "{said}"
    );
    assert!(
        said.contains("`CLUB_LOGIN_ATTEMPTS`") && said.contains("this finding is about that copy"),
        "{said}"
    );
    // Only the wrong-password finding, and only the setting that looks like a limit.
    assert!(!said.contains("COOKIES"), "{said}");
    assert_eq!(
        findings[1].description,
        "Wrong passwords could be tried without limit."
    );
}

#[test]
fn with_no_limit_set_the_finding_is_as_it_was() {
    let mut findings = vec![finding("probe.failed-sign-ins-unlimited")];
    limits_set_for_the_run(&mut findings, "CLUB_SECURE_COOKIES=0 python app.py");
    limits_set_for_the_run(&mut findings, "python app.py");
    assert_eq!(
        findings[0].description,
        "Wrong passwords could be tried without limit."
    );
}
