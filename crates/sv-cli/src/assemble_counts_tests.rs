//! The split of test-file information from the findings (ADR-091, ADR-093).

use super::*;
use sv_check::{Confidence, Location, Severity};

fn finding(rule: &str, file: &str) -> sv_check::Finding {
    sv_check::finding::found(sv_check::Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: format!("found by {rule}"),
        severity: Severity::High,
        confidence: Confidence::Medium,
        location: Location {
            file: file.into(),
            line: 3,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: format!("fp-{rule}-{file}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

#[test]
fn a_counted_test_file_finding_is_a_finding_and_an_uncounted_one_is_information() {
    let fixture = finding("bandit.B105", "tests/test_app.py");
    let app = finding("ast.eval", "app.py");
    assert!(
        fixture.is_test_information(),
        "the fixture is test-file information"
    );
    assert!(
        !app.is_test_information(),
        "the control is a finding in the app"
    );
    let counted = vec!["fp-bandit.B105-tests/test_app.py".to_owned()];
    let (findings, info) = split_test_information(vec![fixture.clone(), app.clone()], &counted);
    assert_eq!(findings.len(), 2, "the counted one moves into the findings");
    assert!(info.is_empty());
    let (findings, info) = split_test_information(vec![fixture], &[]);
    assert!(
        findings.is_empty(),
        "uncounted, it stays out of the findings"
    );
    assert_eq!(info.len(), 1);
}
