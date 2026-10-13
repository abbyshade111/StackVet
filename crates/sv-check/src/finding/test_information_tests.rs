//! Which findings are information in test code rather than findings (ADR-091): the four Bandit rules, in test
//! files, that no other tool also reported. Everything else keeps counting, and the secrets rules are never on
//! the list.

use super::*;

fn finding(rule: &str, file: &str, marked: bool) -> Finding {
    Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: format!("found by {rule}"),
        severity: Severity::Low,
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
        fingerprint: format!("fp-{rule}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: marked,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    }
}

#[test]
fn bandit_asserts_and_password_strings_in_a_tests_folder_are_information() {
    for rule in ["bandit.B101", "bandit.B105", "bandit.B106", "bandit.B107"] {
        let f = finding(rule, "tests/test_security.py", false);
        assert!(f.in_test_code(), "{rule}: the file is test code");
        assert!(f.is_test_information(), "{rule} in a tests folder");
    }
}

#[test]
fn the_same_rules_in_app_code_still_count() {
    for rule in ["bandit.B101", "bandit.B105"] {
        let f = finding(rule, "app.py", false);
        assert!(!f.in_test_code(), "{rule}: app.py is app code");
        assert!(!f.is_test_information(), "{rule} in app code is a finding");
    }
}

#[test]
fn a_file_the_manifest_marks_as_test_code_is_information_too() {
    let f = finding("bandit.B101", "app.py", true);
    assert!(f.is_test_information(), "a test file marked by the report");
}

#[test]
fn a_secret_rule_in_a_test_file_still_counts() {
    // The secrets rules are not on the list: a real-looking key in a test file is still a finding.
    for rule in ["semgrep.generic.secrets.aws-key", "gosec.G101"] {
        let f = finding(rule, "tests/test_security.py", false);
        assert!(f.in_test_code(), "{rule}: the file is test code");
        assert!(!f.is_test_information(), "{rule} must count in a test file");
    }
}

#[test]
fn a_rule_another_tool_also_reported_still_counts() {
    let mut f = finding("bandit.B105", "tests/test_security.py", false);
    f.also_reported_by.push("semgrep.generic.hardcoded".into());
    assert!(
        !f.is_test_information(),
        "a finding another tool also made counts"
    );
}
