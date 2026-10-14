//! Backlog 0401: a finding no longer found, whose line still reads as it did, is found again by its fingerprint: as
//! written, and with a comment that tells a scanner to look away taken off. A line that was changed is not.

use crate::finding::{Confidence, Finding, Location, Severity};
use crate::review::{StillReads, fingerprint, still_reads};
use crate::test_support::scratch;

fn finding(file: &str, line: usize) -> Finding {
    crate::finding::found(Finding {
        evidence: Vec::new(),
        rule_id: "bandit.B104".into(),
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
    })
}

/// The fingerprint of line 2 of `app.py` as `before` holds it, and the app with `app.py` written as `after`.
fn went(before: &str, after: &str) -> Option<(usize, StillReads)> {
    let app = scratch("");
    std::fs::write(app.join("app.py"), before).unwrap();
    let fp = fingerprint(&app, &finding("app.py", 2));
    assert!(fp.starts_with("v2-"), "the setup: {fp}");
    std::fs::write(app.join("app.py"), after).unwrap();
    let found = still_reads(&app, "bandit.B104", "app.py", &fp);
    std::fs::remove_dir_all(&app).ok();
    found
}

const BEFORE: &str = "import app\nhost = \"0.0.0.0\"\nrun(host)\n";

#[test]
fn a_line_that_reads_the_same_is_found_where_it_is_now() {
    let moved = "import app\n\n# a new comment\nhost = \"0.0.0.0\"\nrun(host)\n";
    assert_eq!(went(BEFORE, moved), Some((4, StillReads::Unchanged)));
}

#[test]
fn a_line_with_a_look_away_comment_added_reads_the_same_once_it_is_taken_off() {
    let hidden = "import app\nhost = \"0.0.0.0\"  # nosec B104\nrun(host)\n";
    assert_eq!(
        went(BEFORE, hidden),
        Some((
            2,
            StillReads::LookAway {
                scanner: "Bandit".into()
            }
        ))
    );
    // And a comment on the line above, the line itself untouched.
    let above = "import app\n# nosec\nhost = \"0.0.0.0\"\nrun(host)\n";
    assert_eq!(
        went(BEFORE, above),
        Some((
            3,
            StillReads::LookAway {
                scanner: "Bandit".into()
            }
        ))
    );
}

#[test]
fn a_line_that_was_changed_or_removed_is_not_found() {
    // Rewritten to do the same thing: not caught, and the function says so.
    assert_eq!(went(BEFORE, "import app\nhost = \"\"\nrun(host)\n"), None);
    assert_eq!(went(BEFORE, "import app\nrun(\"127.0.0.1\")\n"), None);
}
