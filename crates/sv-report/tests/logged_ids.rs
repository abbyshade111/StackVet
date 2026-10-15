//! Semgrep's rule for credentials written to a log, set apart on a line that logs only ids, decides nothing
//! (backlog 0583; ADR-111, Later): it is listed apart, shown in full with what was logged, and leaves V16.2.5 alone.

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

const RULE: &str = sv_check::logged_ids::RULE;

#[test]
fn a_log_line_logging_only_ids_leaves_v16_2_5_alone_and_says_what_it_logged() {
    let f = frameworks();
    let buckets = Buckets {
        applicable: vec!["V16.2.5".into()],
        ..Default::default()
    };
    let mut set_apart = finding(RULE, &["V16.2.5"]);
    set_apart.outranked = Some(Outranked::LogsOnlyIds {
        names: vec!["user_id".to_owned(), "game.id".to_owned()],
    });
    let report = build(inputs(&f, &buckets, vec![set_apart], &[]));
    let line = report
        .requirements
        .iter()
        .find(|r| r.id == "V16.2.5")
        .expect("V16.2.5 is in the report");
    assert_ne!(line.status, Status::NeedsAttention, "{line:?}");
    // Still there, still marked: the report's own marking of what it outranks did not undo it.
    let kept = report
        .findings
        .iter()
        .find(|x| x.rule_id == RULE)
        .expect("the finding is still listed");
    assert!(
        matches!(kept.outranked, Some(Outranked::LogsOnlyIds { .. })),
        "{:?}",
        kept.outranked
    );
    let markdown = sv_report::markdown::security(&report);
    assert!(
        markdown.contains("it logs only `user_id` and `game.id`, which are each an id"),
        "{markdown}"
    );
    assert!(
        markdown.contains("log lines that log only ids"),
        "{markdown}"
    );
}

#[test]
fn the_same_finding_not_set_apart_still_needs_attention() {
    // The control: without the mark, the finding decides V16.2.5 as every finding does.
    let f = frameworks();
    let buckets = Buckets {
        applicable: vec!["V16.2.5".into()],
        ..Default::default()
    };
    let report = build(inputs(&f, &buckets, vec![finding(RULE, &["V16.2.5"])], &[]));
    assert_eq!(report.requirements[0].status, Status::NeedsAttention);
}
