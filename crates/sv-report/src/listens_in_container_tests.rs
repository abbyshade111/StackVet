//! Backlog 0658: re-deciding what a report lists apart keeps the in-container reason that was set when the tools ran,
//! unless the finding is about a requirement the app is not held to, which says more.

use super::*;
use sv_check::finding::{Confidence, Finding, Location, Outranked, Severity};

fn b104(rule: &str) -> Finding {
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
        title: "Possible binding to all interfaces".into(),
        severity: Severity::Medium,
        confidence: Confidence::Medium,
        location: Location {
            file: "app.py".into(),
            line: 25,
        },
        secret: None,
        requirement_ids: vec!["V13.4.6".into()],
        cwe: vec!["CWE-605".into()],
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
    }
}

#[test]
fn the_reason_survives_re_deciding_and_gives_way_to_one_that_says_more() {
    let mut findings = vec![b104("bandit.B104"), b104("bandit.B104")];
    for f in &mut findings {
        f.outranked = Some(Outranked::ListensInContainer);
    }
    findings[1].requirement_ids = vec!["V8.1.1".into()];
    mark_outranked(
        &mut findings,
        &["V13.4.6".to_owned()],
        &[],
        &BTreeSet::new(),
    );
    assert_eq!(
        findings[0].outranked,
        Some(Outranked::ListensInContainer),
        "kept"
    );
    assert_eq!(
        findings[1].outranked,
        Some(Outranked::NotHeldTo),
        "about a requirement not held to"
    );
}
