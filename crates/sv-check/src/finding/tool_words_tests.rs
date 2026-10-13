//! When an outside tool reports the same weakness on the same line as `sv`'s own rule, the one finding kept says
//! the tool's rule title, as it says `sv`'s own rules' (backlog 0380): before, only "Also reported by: `semgrep.x`"
//! was left of the tool. Its message is never copied over, since it can quote the value it found (S8).

use super::*;

fn finding(rule: &str, severity: Severity, title: &str, description: &str) -> Finding {
    Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: title.into(),
        severity,
        confidence: Confidence::Medium,
        location: Location {
            file: "app.py".into(),
            line: 12,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: vec!["CWE-89".into()],
        description: description.into(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    }
}

#[test]
fn a_tool_that_agrees_on_a_line_is_heard_by_its_rule_title_and_not_by_its_message() {
    // A message a tool wrote can quote what it found; this one quotes a made-up value, built from pieces.
    let quoted = ["qX7", "pR2", "vN9"].concat();
    let own = finding(
        "ast.sql-built-by-hand",
        Severity::High,
        "A query built by hand",
        "sv's own words",
    );
    let tool = finding(
        "semgrep.python.lang.security.audit.formatted-sql-query",
        Severity::Medium,
        "Detected possible formatted SQL query",
        &format!("query = \"SELECT ... {quoted}\" is built with a format string"),
    );
    let merged = merge_same_place(vec![own, tool]);
    // The setup: the two were one weakness on one line, so they merged, and `sv`'s own was kept.
    assert_eq!(merged.len(), 1, "{merged:?}");
    let kept = &merged[0];
    assert_eq!(kept.rule_id, "ast.sql-built-by-hand");
    assert!(
        kept.also_reported_by
            .iter()
            .any(|r| r.starts_with("semgrep.")),
        "{kept:?}"
    );
    assert!(
        kept.description.contains(
            "The outside tool's rule `semgrep.python.lang.security.audit.formatted-sql-query` reported this line too, \
             as medium: Detected possible formatted SQL query"
        ),
        "the tool is heard by its rule's title: {}",
        kept.description
    );
    assert!(
        !kept.description.contains(&quoted),
        "the tool's message is not copied over"
    );
    assert!(
        kept.description.starts_with("sv's own words"),
        "the kept finding's own words come first"
    );
}

#[test]
fn a_tool_kept_over_another_tool_says_the_other_by_its_title_too() {
    let semgrep = finding("semgrep.python.sqli", Severity::High, "SQL injection", "");
    let bandit = finding(
        "bandit.B608",
        Severity::Medium,
        "Possible SQL injection vector",
        "",
    );
    let merged = merge_same_place(vec![bandit, semgrep]);
    assert_eq!(merged.len(), 1, "{merged:?}");
    assert_eq!(
        merged[0].rule_id, "semgrep.python.sqli",
        "the more severe is kept"
    );
    assert!(
        merged[0]
            .description
            .contains("The outside tool's rule `bandit.B608` reported this line too, as medium: Possible SQL injection vector"),
        "{}",
        merged[0].description
    );
}
