//! The markdown report lists every test-file information finding in full (backlog 0260, ADR-091).

use super::*;
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
    }
}

fn info(rule: &str, file: &str, line: usize) -> sv_check::Finding {
    sv_check::finding::found(sv_check::Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: format!("{rule} in a test"),
        severity: sv_check::Severity::Low,
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
        fingerprint: format!("v2-{rule}-{line}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

#[test]
fn each_test_file_information_finding_is_listed_with_its_rule_place_and_fingerprint() {
    let mut report = empty_report(Vec::new());
    report.test_information = vec![
        info("bandit.B101", "tests/test_app.py", 12),
        info("bandit.B105", "tests/test_app.py", 30),
    ];
    let page = security(&report);
    assert!(
        page.contains("## 2 things in test code, information only"),
        "{page}"
    );
    for (rule, line, fp) in [
        ("bandit.B101", 12, "v2-bandit.B101-12"),
        ("bandit.B105", 30, "v2-bandit.B105-30"),
    ] {
        assert!(
            page.contains(rule) && page.contains(&format!("line {line}")) && page.contains(fp),
            "{rule} is listed in full"
        );
    }
    assert!(
        page.contains("`counts`"),
        "the page says how to make a real one count"
    );
}

#[test]
fn the_section_is_left_out_when_there_is_no_test_file_information() {
    let page = security(&empty_report(Vec::new()));
    assert!(!page.contains("in test code, information only"));
}

#[test]
fn the_list_follows_the_findings_when_there_are_findings_too() {
    let mut report = empty_report(Vec::new());
    let mut app = info("ast.eval", "app.py", 4);
    app.title = "eval of a value the user gives".into();
    // High, so it is shown in full; a low one would be grouped by rule (ADR-095).
    app.severity = sv_check::Severity::High;
    report.findings = vec![app];
    report.test_information = vec![info("bandit.B101", "tests/test_app.py", 12)];
    let page = security(&report);
    let found = page
        .find("eval of a value the user gives")
        .expect("the finding is on the page");
    let listed = page
        .find("## 1 thing in test code, information only")
        .expect("the information is listed when there are findings too");
    assert!(listed > found, "the information comes after the findings");
}
