//! A finding set apart as from a rule written for a framework the app does not use decides nothing (backlog
//! 0521; ADR-111, Later): it is listed apart, shown in full with why, and leaves its requirement's status alone.

use std::path::PathBuf;
use sv_check::finding::Outranked;
use sv_check::{Confidence, Finding, Location, Severity, Verified};
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::Buckets;
use sv_report::{Inputs, Status, build};

fn data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

fn frameworks() -> Frameworks {
    Frameworks::load(&data().join("frameworks")).expect("the OWASP data loads")
}

fn finding(rule_id: &str, requirement_ids: &[&str]) -> Finding {
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
        rule_id: rule_id.into(),
        title: "something".into(),
        severity: Severity::High,
        confidence: Confidence::High,
        location: Location {
            file: "app.py".into(),
            line: 1,
        },
        secret: None,
        requirement_ids: requirement_ids.iter().map(|s| (*s).to_owned()).collect(),
        cwe: vec![],
        description: "d".into(),
        impact: "i".into(),
        fix: "f".into(),
    }
}

fn inputs<'a>(
    frameworks: &'a Frameworks,
    buckets: &'a Buckets,
    findings: Vec<Finding>,
    verified: &'a [Verified],
) -> Inputs<'a> {
    Inputs {
        set_aside: Vec::new(),
        test_information: Vec::new(),
        reviews_not_counted: Vec::new(),
        app_name: "Test",
        on_the_internet: false,
        ai_tool: Default::default(),
        look_away: Default::default(),
        target_level: 1,
        generated: None,
        made_by: Default::default(),
        run_note: None,
        run_steps: Vec::new(),
        test_output: None,
        run_status: None,
        coding_rules_cited: Default::default(),
        frameworks,
        buckets,
        claims: &[],
        findings,
        verified,
        gaps: vec![],
        manual_only: Default::default(),
        named_in_tests: Default::default(),
        not_for_tests: Default::default(),
        human: None,
        threats: None,
    }
}

#[test]
fn a_django_rules_finding_on_a_flask_app_leaves_v3_5_1_alone_and_says_why() {
    let f = frameworks();
    let buckets = Buckets {
        applicable: vec!["V3.5.1".into()],
        ..Default::default()
    };
    let mut set_apart = finding(
        "semgrep.python.django.security.django-no-csrf-token.django-no-csrf-token",
        &["V3.5.1"],
    );
    set_apart.outranked = Some(Outranked::OtherFramework {
        framework: "Django".to_owned(),
    });
    let report = build(inputs(&f, &buckets, vec![set_apart], &[]));
    let line = report
        .requirements
        .iter()
        .find(|r| r.id == "V3.5.1")
        .expect("V3.5.1 is in the report");
    assert_ne!(line.status, Status::NeedsAttention, "{line:?}");
    // Still there, still marked: the report's own marking of what it outranks did not undo it.
    let kept = report
        .findings
        .iter()
        .find(|x| x.rule_id.contains("django-no-csrf-token"))
        .expect("the finding is still listed");
    assert_eq!(
        kept.outranked,
        Some(Outranked::OtherFramework {
            framework: "Django".to_owned()
        })
    );
    let markdown = sv_report::markdown::security(&report);
    assert!(
        markdown.contains(
            "From a rule written for Django, and this app's package lists do not name Django"
        ),
        "{markdown}"
    );
}

#[test]
fn the_same_finding_not_set_apart_still_needs_attention() {
    // The control: without the mark, the finding decides V3.5.1 as every finding does.
    let f = frameworks();
    let buckets = Buckets {
        applicable: vec!["V3.5.1".into()],
        ..Default::default()
    };
    let report = build(inputs(
        &f,
        &buckets,
        vec![finding(
            "semgrep.python.django.security.django-no-csrf-token.django-no-csrf-token",
            &["V3.5.1"],
        )],
        &[],
    ));
    assert_eq!(report.requirements[0].status, Status::NeedsAttention);
}
