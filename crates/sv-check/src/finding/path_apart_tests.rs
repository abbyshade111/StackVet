//! Backlog 0653, ADR-138: a path rule finding is worth a look only where the request-path rule reads the language and
//! nothing else reported the line; a merge of the two keeps the request rule's finding.

use super::*;

fn path_finding(rule: &str, file: &str, severity: Severity, confidence: Confidence) -> Finding {
    Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: rule.into(),
        title: format!("found by {rule}"),
        severity,
        confidence,
        location: Location {
            file: file.into(),
            line: 7,
        },
        secret: None,
        requirement_ids: vec!["V5.3.2".into()],
        cwe: vec!["CWE-22".into()],
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    }
}

#[test]
fn a_path_from_a_value_is_worth_a_look_only_where_a_requests_input_is_read() {
    let value = |file: &str| path_finding(PATH_FROM_VALUE, file, Severity::Medium, Confidence::Low);
    for file in [
        "src/main.rs",
        "app.py",
        "web/server.ts",
        "main.go",
        "run.sh",
    ] {
        let f = value(file);
        assert!(f.worth_a_look() && f.apart(), "{file}: listed apart");
    }
    // Where the request-path rule reads nothing, a request's input is not looked for, so the line stays a thing to fix.
    for file in [
        "src/files.c",
        "lib/app.dart",
        "Sources/App.swift",
        "page.html",
    ] {
        assert!(!value(file).worth_a_look(), "{file}: still a thing to fix");
    }
    // Another tool agreeing about the line makes it more than worth a look.
    let mut agreed = value("app.py");
    agreed.also_reported_by = vec!["semgrep.python.lang.security.audit.path-traversal".into()];
    assert!(!agreed.worth_a_look());
    // The request-path rule's own finding is a thing to fix.
    assert!(
        !path_finding(
            PATH_FROM_REQUEST,
            "app.py",
            Severity::High,
            Confidence::Medium
        )
        .worth_a_look()
    );
}

#[test]
fn both_rules_on_one_line_are_one_finding_the_request_rules() {
    let merged = merge_same_place(vec![
        path_finding(PATH_FROM_VALUE, "app.py", Severity::Medium, Confidence::Low),
        path_finding(
            PATH_FROM_REQUEST,
            "app.py",
            Severity::High,
            Confidence::Medium,
        ),
    ]);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].rule_id, PATH_FROM_REQUEST);
    assert!(
        merged[0]
            .also_reported_by
            .iter()
            .any(|r| r == PATH_FROM_VALUE)
    );
    assert!(!merged[0].worth_a_look() && !merged[0].apart());
}
