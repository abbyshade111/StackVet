//! Backlog 0401: the record keeps each finding's rule and a hash of its file beside its fingerprint, so a report can
//! look for each finding that went again in today's code, and name the ones whose line still reads as it did.

use super::*;
use sv_check::test_support::scratch;
use sv_check::{Confidence, Finding, Location, Severity};

fn finding(rule: &str, file: &str, line: usize) -> Finding {
    Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: "binds every address".into(),
        severity: Severity::Medium,
        confidence: Confidence::Medium,
        location: Location {
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
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    }
}

fn check(time: &str, fingerprints: &[&str], placed: &[(&str, &str, &str)]) -> Line {
    Line {
        time: time.to_owned(),
        tool: "stackvet_check".to_owned(),
        counts: Some(LoopCounts::default()),
        fingerprints: Some(fingerprints.iter().map(|f| (*f).to_owned()).collect()),
        placed: placed
            .iter()
            .map(|(f, r, k)| ((*f).to_owned(), ((*r).to_owned(), (*k).to_owned())))
            .collect(),
        ..Line::default()
    }
}

#[test]
fn where_each_finding_was_reads_back_and_nothing_else_does() {
    let key = sv_check::review::file_key("app.py");
    assert_eq!(key.len(), 16);
    let line = check(
        "1",
        &["v2-61eb1668992f3990"],
        &[("v2-61eb1668992f3990", "bandit.B104", &key)],
    );
    assert_eq!(Line::parse(&line.to_json()), Some(line));
    // A rule's place holding words, a file's name where its hash belongs, or a fingerprint the line does not list:
    // none is a line sv writes.
    for bad in [
        check(
            "1",
            &["v2-61eb1668992f3990"],
            &[("v2-61eb1668992f3990", "no way to report", &key)],
        ),
        check(
            "1",
            &["v2-61eb1668992f3990"],
            &[("v2-61eb1668992f3990", "bandit.B104", "app.py")],
        ),
        check(
            "1",
            &["v2-61eb1668992f3990"],
            &[("v2-0000000000000000", "bandit.B104", &key)],
        ),
    ] {
        assert_eq!(Line::parse(&bad.to_json()), None, "{}", bad.to_json());
    }
}

#[test]
fn only_a_kept_v2_fingerprint_in_a_file_is_placed() {
    let mut in_file = finding("bandit.B104", "app.py", 2);
    in_file.fingerprint = "v2-61eb1668992f3990".into();
    let mut old = finding("bandit.B104", "app.py", 3);
    old.fingerprint = "882dedc677bff5a5".into();
    let mut not_kept = finding("bandit.B104", "app.py", 4);
    not_kept.fingerprint = "v2-1111111111111111".into();
    let kept = vec![
        "882dedc677bff5a5".to_owned(),
        "v2-61eb1668992f3990".to_owned(),
    ];
    let placed = placed_of(&[in_file, old, not_kept], &kept);
    assert_eq!(placed.len(), 1);
    assert_eq!(
        placed.get("v2-61eb1668992f3990"),
        Some(&(
            "bandit.B104".to_owned(),
            sv_check::review::file_key("app.py")
        ))
    );
}

/// The app with `app.py` written first as `before`, checked, then as `after`, checked again.
fn between(before: &str, after: &str, set_aside: &[String]) -> BuildLoop {
    let app = scratch("");
    std::fs::write(app.join("app.py"), before).unwrap();
    let mut found = finding("bandit.B104", "app.py", 2);
    found.fingerprint = sv_check::review::fingerprint(&app, &found);
    assert!(
        found.fingerprint.starts_with("v2-"),
        "the setup: {}",
        found.fingerprint
    );
    let kept = vec![found.fingerprint.clone()];
    let placed = placed_of(std::slice::from_ref(&found), &kept);
    assert_eq!(placed.len(), 1, "the setup: the finding is placed");
    let first = Line {
        placed,
        ..check("1", &[found.fingerprint.as_str()], &[])
    };
    let last = check("2", &[], &[]);
    let record = format!("{}\n{}\n", first.to_json(), last.to_json());
    std::fs::write(app.join("app.py"), after).unwrap();
    let mut summary = summarize(record.as_bytes());
    settle(&mut summary, set_aside);
    look_again(&mut summary, &app, set_aside);
    std::fs::remove_dir_all(&app).ok();
    summary
}

const BEFORE: &str = "import app\nhost = \"0.0.0.0\"\nrun(host)\n";

#[test]
fn a_finding_that_went_behind_a_look_away_comment_is_named() {
    let summary = between(
        BEFORE,
        "import app\nhost = \"0.0.0.0\"  # nosec\nrun(host)\n",
        &[],
    );
    assert_eq!(summary.looked_again, 1);
    assert_eq!(
        summary.went_same,
        vec![sv_report::WentSame {
            rule: "bandit.B104".into(),
            file: "app.py".into(),
            line: 2,
            how: sv_check::review::StillReads::LookAway {
                scanner: "Bandit".into()
            },
        }]
    );
}

#[test]
fn a_finding_whose_line_was_changed_is_looked_for_and_not_named() {
    let summary = between(BEFORE, "import app\nhost = \"127.0.0.1\"\nrun(host)\n", &[]);
    assert_eq!(summary.looked_again, 1);
    assert!(summary.went_same.is_empty());
}

#[test]
fn a_finding_set_aside_as_a_false_alarm_is_not_looked_for() {
    let app = scratch("");
    std::fs::write(app.join("app.py"), BEFORE).unwrap();
    let mut found = finding("bandit.B104", "app.py", 2);
    found.fingerprint = sv_check::review::fingerprint(&app, &found);
    std::fs::remove_dir_all(&app).ok();
    let summary = between(BEFORE, BEFORE, &[found.fingerprint]);
    assert_eq!(summary.looked_again, 0);
    assert!(summary.went_same.is_empty());
}
