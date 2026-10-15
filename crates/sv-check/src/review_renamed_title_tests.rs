//! A finding renamed after people recorded decisions about it (backlog 0530): an accepted risk recorded under the
//! pinning finding's old title still counts for the same finding under its new one.

use super::*;

/// This computer's key for the app these tests read, as in the review tests.
fn key() -> crate::seal::AppKey {
    static KEY: std::sync::OnceLock<crate::seal::Key> = std::sync::OnceLock::new();
    KEY.get_or_init(|| crate::seal::Key::random().unwrap())
        .for_app(&crate::seal::App::named_for_tests("app"))
}

fn accepted(fingerprint: String) -> FindingReview {
    let mut e = FindingReview {
        rule: "config.versions-pinned".into(),
        file: "requirements.txt".into(),
        fingerprint,
        verdict: ACCEPTED_RISK.into(),
        why: "The app is rebuilt by hand from this file and checked each time before it runs."
            .into(),
        by: Some("owner".into()),
        on: Some("2026-10-10".into()),
        seal: None,
    };
    e.seal = Some(
        key().seal(&crate::seal::as_strs(&crate::seal::finding_review_fields(
            &e,
        ))),
    );
    e
}

/// The pinning finding a pinned `requirements.txt` with no lockfile gets today, fingerprinted as a run does.
fn pinning_finding(app: &Path) -> Vec<Finding> {
    std::fs::write(app.join("requirements.txt"), "flask==3.0.0\n").unwrap();
    let mut findings: Vec<Finding> = crate::config::check_dir(app)
        .findings
        .into_iter()
        .filter(|f| f.rule_id == "config.versions-pinned")
        .collect();
    // The setup must have worked: one finding, under its new title.
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(
        findings[0].title,
        "No Python lockfile beside `requirements.txt`"
    );
    fill_fingerprints(app, &mut findings);
    findings
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-renamed-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn apply_with(app: &Path, entries: &[FindingReview], findings: Vec<Finding>) -> Outcome {
    apply(
        app,
        entries,
        findings,
        Day::parse("2026-10-15").unwrap(),
        &Checker::key(key()),
        &|_, _| Looked::Ran,
    )
}

#[test]
fn an_accepted_risk_recorded_under_the_old_title_still_counts() {
    let dir = scratch("old-title");
    let findings = pinning_finding(&dir);
    let old = named(
        "config.versions-pinned",
        "requirements.txt",
        "Python does not pin the versions it installs",
    );
    assert_ne!(findings[0].fingerprint, old, "the title did change");
    let out = apply_with(&dir, &[accepted(old)], findings);
    assert!(out.not_counted.is_empty(), "{:?}", out.not_counted);
    // An accepted risk still counts as a finding; it is set aside with its reason beside it.
    assert_eq!(out.findings.len(), 1);
    assert_eq!(out.set_aside.len(), 1);
    assert_eq!(out.set_aside[0].verdict, ACCEPTED_RISK);
}

#[test]
fn an_accepted_risk_under_the_new_title_counts_too() {
    let dir = scratch("new-title");
    let findings = pinning_finding(&dir);
    let new = findings[0].fingerprint.clone();
    let out = apply_with(&dir, &[accepted(new)], findings);
    assert!(out.not_counted.is_empty(), "{:?}", out.not_counted);
    assert_eq!(out.set_aside.len(), 1);
}

#[test]
fn an_accepted_risk_under_another_title_does_not_count() {
    // The control: the old title's fingerprint counts because the finding carries it, not because any entry on the
    // file does.
    let dir = scratch("other-title");
    let findings = pinning_finding(&dir);
    let other = named(
        "config.versions-pinned",
        "requirements.txt",
        "Node does not pin the versions it installs",
    );
    let out = apply_with(&dir, &[accepted(other)], findings);
    assert_eq!(out.findings.len(), 1);
    assert!(out.set_aside.is_empty());
    assert_eq!(out.not_counted.len(), 1, "{:?}", out.not_counted);
}
