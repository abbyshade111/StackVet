//! Low-severity findings grouped by rule in the reports, and listed in full beside them (backlog 0250, ADR-095).

use crate::{Counts, Report};

fn empty_report(gaps: Vec<crate::Gap>) -> Report {
    Report {
        level_why: None,
        baseline: None,
        build_loop: None,
        seen: None,
        code_lines: Default::default(),
        sbom: None,
        timings: Vec::new(),
        app_name: "test".into(),
        target_level: 1,
        generated: None,
        sv: Default::default(),
        run_record: None,
        manifest_file: crate::default_manifest_file(),
        run_note: None,
        run_steps: Vec::new(),
        test_output: None,
        run_status: None,
        ai_process: Default::default(),
        counts: Counts::default(),
        requirements: vec![],
        excluded: vec![],
        undecided: vec![],
        claims: vec![],
        findings: vec![],
        set_aside: Vec::new(),
        test_information: Vec::new(),
        reviews_not_counted: Vec::new(),
        out_of_scope: vec![],
        checklist_above_level: vec![],
        tests_to_write: vec![],
        only_you_can_check: Vec::new(),
        before_going_live: Vec::new(),
        on_the_internet: false,
        hook_record: None,
        tool_behavior: None,
        checklist_done: Default::default(),
        ai_tool: Default::default(),
        look_away: Default::default(),
        questions_for_you: Vec::new(),
        no_instructions_yet: 0,
        named_not_credited: vec![],
        not_for_tests: 0,
        threats: Vec::new(),
        threat_parts: Vec::new(),
        threat_atlas_release: None,
        satisfied_elsewhere: vec![],
        gaps,
        examined: Vec::new(),
        could_not_run: Vec::new(),
        partly_read: Vec::new(),
        not_run_this_time: None,
        could_settle: Default::default(),
        plain: Default::default(),
        paper: Default::default(),
        fetched: None,
    }
}

fn finding(rule: &str, file: &str, line: usize, severity: sv_check::Severity) -> sv_check::Finding {
    sv_check::finding::found(sv_check::Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: format!("{rule} at line {line}"),
        severity,
        confidence: sv_check::Confidence::Medium,
        location: sv_check::Location {
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
        fingerprint: format!("v2-{rule}-{file}-{line}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

fn report_with_low() -> Report {
    use sv_check::Severity::{High, Low};
    let mut report = empty_report(Vec::new());
    report.findings = vec![
        finding("ast.eval", "app.py", 4, High),
        finding("semgrep.missing-csrf", "app.py", 10, Low),
        finding("semgrep.missing-csrf", "views.py", 20, Low),
        finding("semgrep.missing-csrf", "app.py", 30, Low),
        finding("bandit.B311", "util.py", 7, Low),
    ];
    report
}

#[test]
fn low_findings_are_grouped_by_rule_most_first_and_the_rest_are_kept_whole() {
    let report = report_with_low();
    let (rest, low) = crate::split_low(&report, report.findings.iter().collect());
    assert_eq!(rest.len(), 1, "the high one is kept as it was");
    assert_eq!(rest[0].rule_id, "ast.eval");
    assert_eq!(low.len(), 2);
    assert_eq!(low[0].rule, "semgrep.missing-csrf");
    assert_eq!(low[0].count, 3);
    assert_eq!(
        low[0].files,
        vec!["app.py".to_owned(), "views.py".to_owned()]
    );
    assert_eq!(low[1].rule, "bandit.B311");
    assert_eq!(low[1].count, 1);
    let total: usize = low.iter().map(|g| g.count).sum();
    assert_eq!(total, 4, "every low finding is in a group: none is dropped");
}

#[test]
fn the_reports_show_one_line_per_rule_and_the_file_beside_them_has_every_one() {
    let report = report_with_low();
    let md = crate::markdown::security(&report);
    assert!(md.contains("### 4 low-severity findings, by rule"), "{md}");
    assert!(md.contains("3 times, in `app.py`, `views.py`"), "{md}");
    assert!(
        !md.contains("semgrep.missing-csrf at line 20"),
        "a grouped finding is not shown in full on the page"
    );
    assert!(
        md.contains("ast.eval at line 4"),
        "the high one is shown in full"
    );
    let html = crate::html::page(&report);
    assert!(
        html.contains("4 low-severity findings, by rule"),
        "the html page groups them too"
    );
    let full = crate::markdown::low_findings(&report);
    for line in [10, 20, 30] {
        assert!(
            full.contains(&format!("semgrep.missing-csrf at line {line}")),
            "line {line} is in full in the file"
        );
    }
    assert!(full.contains("bandit.B311 at line 7"));
    assert!(
        !full.contains("ast.eval at line 4"),
        "the file holds the low ones only"
    );
}

#[test]
fn with_no_low_findings_the_reports_are_as_before_and_the_file_says_none() {
    let mut report = empty_report(Vec::new());
    report.findings = vec![finding("ast.eval", "app.py", 4, sv_check::Severity::High)];
    assert!(!crate::markdown::security(&report).contains("low-severity finding"));
    assert!(crate::markdown::low_findings(&report).contains("There are none"));
}

#[test]
fn a_low_finding_about_a_key_is_never_grouped() {
    let mut key = finding("secrets.generic", "config.py", 3, sv_check::Severity::Low);
    // A placeholder built from pieces, so the file holds nothing that looks like a key.
    key.secret = Some(sv_check::Secret::redact(
        &["example", "-placeholder", "-value"].concat(),
    ));
    let mut report = empty_report(Vec::new());
    report.findings = vec![
        key,
        finding("bandit.B311", "util.py", 7, sv_check::Severity::Low),
    ];
    let (rest, low) = crate::split_low(&report, report.findings.iter().collect());
    assert_eq!(rest.len(), 1, "the key stays in full");
    assert_eq!(rest[0].rule_id, "secrets.generic");
    assert_eq!(low.len(), 1, "the other low one is grouped");
}
