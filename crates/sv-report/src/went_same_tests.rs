//! What the reports say of the findings that went while their line still reads as it did (backlog 0401): each is
//! named with its rule, file, line, and how, in compliance.md, security.md, and report.html; with none, that they
//! were looked for; with none looked for, nothing.

use crate::next_step_tests::empty_report;
use crate::{BuildLoop, WentSame};
use sv_check::review::StillReads;

fn with(looked_again: usize, went_same: Vec<WentSame>) -> crate::Report {
    let mut report = empty_report(Vec::new());
    report.build_loop = Some(BuildLoop {
        calls: 2,
        checks: 2,
        looked_again,
        went_same,
        ..BuildLoop::default()
    });
    report
}

#[test]
fn each_finding_that_went_on_an_unchanged_line_is_named_in_every_report() {
    let report = with(
        2,
        vec![
            WentSame {
                rule: "bandit.B104".into(),
                file: "app.py".into(),
                line: 2,
                how: StillReads::LookAway {
                    scanner: "Bandit".into(),
                },
            },
            WentSame {
                rule: "config.debug-mode".into(),
                file: "settings.py".into(),
                line: 9,
                how: StillReads::Unchanged,
            },
        ],
    );
    let line = crate::went_same_line(&report).expect("a line to say");
    assert!(line.starts_with("Look at these: 2 findings were"), "{line}");
    assert!(
        line.contains(
            "bandit.B104 at `app.py` line 2 (a comment there now tells Bandit to look away)"
        ),
        "{line}"
    );
    assert!(
        line.contains("config.debug-mode at `settings.py` line 9 (the line is unchanged"),
        "{line}"
    );
    assert!(
        line.contains("A line rewritten to do the same thing is not caught"),
        "{line}"
    );
    for (name, text) in [
        ("compliance.md", crate::markdown::compliance(&report)),
        ("security.md", crate::markdown::security(&report)),
        ("report.html", crate::html::page(&report)),
    ] {
        assert!(text.contains("tells Bandit to look away"), "{name}");
        assert!(text.contains("settings.py"), "{name}");
    }
}

#[test]
fn with_none_it_says_they_were_looked_for_and_with_none_looked_for_nothing() {
    let line = crate::went_same_line(&with(1, Vec::new())).expect("a line to say");
    assert!(line.contains("1 was looked for again"), "{line}");
    assert!(
        line.contains("none still sits on a line that reads as it did"),
        "{line}"
    );
    assert_eq!(crate::went_same_line(&with(0, Vec::new())), None);
}
