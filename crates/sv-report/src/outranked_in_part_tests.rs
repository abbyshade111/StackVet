//! An outside tool's finding set aside by a check of the running app (ADR-023, Later, 6 October 2026)
//! only when that check verified the requirement whole (backlog 0303): a credit in part rests, by its
//! own words, on one sample, and says nothing of the route or file the tool's finding is about.

use crate::mark_outranked;
use std::collections::BTreeSet;
use sv_check::finding::Outranked;

fn semgrep_finding(requirement: &str) -> sv_check::Finding {
    sv_check::finding::found(sv_check::Finding {
        evidence: Vec::new(),
        rule_id: "semgrep.python.flask.security.audit.csrf-disabled".into(),
        title: "CSRF protection is turned off".into(),
        severity: sv_check::Severity::High,
        confidence: sv_check::Confidence::Medium,
        location: sv_check::Location {
            file: "app/views/admin.py".into(),
            line: 12,
        },
        secret: None,
        requirement_ids: vec![requirement.into()],
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: "v2-semgrep-admin-12".into(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    })
}

fn outranked(credit: sv_check::Verified) -> Option<Outranked> {
    let mut findings = vec![semgrep_finding("V3.5.1")];
    mark_outranked(
        &mut findings,
        &["V3.5.1".to_owned()],
        &[credit],
        &BTreeSet::new(),
    );
    findings.remove(0).outranked
}

#[test]
fn a_whole_probe_credit_sets_an_outside_finding_aside() {
    // The control: the path this item is about is reached, so its absence below means something.
    let credit = sv_check::Verified::new(
        "probe.cross-site-request-accepted",
        &["V3.5.1"],
        "every write the run tried refused a request from another site".to_owned(),
    );
    assert!(
        matches!(
            outranked(credit),
            Some(Outranked::CheckedWhileRunning { ref check }) if check == "probe.cross-site-request-accepted"
        ),
        "the setup: a whole probe credit outranks the tool's finding"
    );
}

#[test]
fn a_probe_credit_in_part_leaves_an_outside_finding_standing() {
    let credit = sv_check::Verified::new(
        "probe.cross-site-request-accepted",
        &["V3.5.1"],
        "one write refused a request from another site".to_owned(),
    )
    .in_part();
    assert_eq!(outranked(credit), None);
}
