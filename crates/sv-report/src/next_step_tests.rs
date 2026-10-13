//! Each finding no review answers for names its next step in the reports (backlog 0253, ADR-098).

use crate::{Counts, Report};

pub(crate) fn empty_report(gaps: Vec<crate::Gap>) -> Report {
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
        ai_tool: Default::default(),
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
    }
}

fn finding() -> sv_check::Finding {
    sv_check::finding::found(sv_check::Finding {
        evidence: Vec::new(),
        rule_id: "ast.eval".into(),
        title: "eval of a value the user gives".into(),
        severity: sv_check::Severity::High,
        confidence: sv_check::Confidence::Medium,
        location: sv_check::Location {
            file: "app.py".into(),
            line: 4,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: "v2-ast.eval-4".into(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

#[test]
fn a_finding_no_review_answers_for_names_its_next_step_in_both_reports() {
    let mut report = empty_report(Vec::new());
    report.findings = vec![finding()];
    let md = crate::markdown::security(&report);
    assert!(md.contains("**Next step.** Fix it in the code"), "{md}");
    assert!(md.contains("`sv review`"), "{md}");
    let html = crate::html::page(&report);
    assert!(
        html.contains("<strong>Next step.</strong> Fix it in the code"),
        "the page says it too"
    );
}
