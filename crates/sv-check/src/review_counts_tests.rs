//! The verdict `counts` (ADR-093): a reviewer's sealed "this test-file information finding is real".

use super::*;
use crate::finding::{Confidence, Location, Severity};

fn finding(rule: &str, file: &str, line: usize) -> Finding {
    crate::finding::found(Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: format!("found by {rule}"),
        severity: Severity::High,
        confidence: Confidence::Medium,
        location: Location {
            file: file.into(),
            line,
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
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

/// This computer's key for the app these tests read, as in the review tests.
fn key() -> crate::seal::AppKey {
    static KEY: std::sync::OnceLock<crate::seal::Key> = std::sync::OnceLock::new();
    KEY.get_or_init(|| crate::seal::Key::random().unwrap())
        .for_app(&crate::seal::App::named_for_tests("app"))
}

fn counts(rule: &str, file: &str, sealed: bool) -> FindingReview {
    let mut e = FindingReview {
        rule: rule.into(),
        file: file.into(),
        fingerprint: format!("fp-{rule}"),
        verdict: COUNTS.into(),
        why: "Read the test, and it builds a real key from the environment, not one written in the file.".into(),
        by: Some("owner".into()),
        on: Some("2026-09-27".into()),
        seal: None,
    };
    if sealed {
        e.seal = Some(
            key().seal(&crate::seal::as_strs(&crate::seal::finding_review_fields(
                &e,
            ))),
        );
    }
    e
}

fn apply_with(entries: &[FindingReview], findings: Vec<Finding>) -> Outcome {
    apply(
        Path::new("/no/app/folder"),
        entries,
        findings,
        Day::parse("2026-09-27").unwrap(),
        &Checker::key(key()),
        &|_, _| Looked::Ran,
    )
}

#[test]
fn counts_is_refused_on_a_finding_that_is_not_test_information() {
    // The control: the same rule in the app's own code is a finding already, so there is nothing to count.
    let out = apply_with(
        &[counts("bandit.B105", "app/settings.py", true)],
        vec![finding("bandit.B105", "app/settings.py", 3)],
    );
    assert!(out.counted_in_test.is_empty());
    assert_eq!(out.findings.len(), 1, "the finding stays where it was");
    assert!(
        out.not_counted.iter().any(|w| w.contains("not")),
        "the refusal says why: {:?}",
        out.not_counted
    );
}

#[test]
fn an_unsealed_counts_is_a_proposal_and_the_information_stays_information() {
    let out = apply_with(
        &[counts("bandit.B105", "tests/test_app.py", false)],
        vec![finding("bandit.B105", "tests/test_app.py", 3)],
    );
    assert!(
        out.counted_in_test.is_empty(),
        "a proposal does not count, so the information list keeps the finding"
    );
}

#[test]
fn a_sealed_counts_on_test_information_is_counted_and_is_not_an_accepted_risk() {
    let f = finding("bandit.B105", "tests/test_app.py", 3);
    assert!(
        f.is_test_information(),
        "the fixture is test-file information (ADR-091)"
    );
    let out = apply_with(&[counts("bandit.B105", "tests/test_app.py", true)], vec![f]);
    assert_eq!(out.counted_in_test, vec!["fp-bandit.B105".to_owned()]);
    assert_eq!(
        out.findings.len(),
        1,
        "a counted finding stays in the findings"
    );
    assert!(
        out.set_aside.is_empty(),
        "it is not an accepted risk, so no set-aside line"
    );
}
