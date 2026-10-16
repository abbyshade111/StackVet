//! What a report says of the hook record (backlog 0566, ADR-122): what it is first, what it saw, and never a credit.

use crate::hook::{COMMAND, Earlier, HookSeen, sentences};
use crate::next_step_tests::empty_report;

fn seen() -> HookSeen {
    HookSeen {
        commands: 3,
        writes: 1,
        turns: 2,
        first: Some("2026-10-15T10:00:00Z".into()),
        last: Some("2026-10-15T11:00:00Z".into()),
        tests: 1,
        last_test: Some(("2026-10-15T10:30:00Z".into(), "npm test `x` [a](b)".into())),
        last_test_failed: false,
        written_after_last_test: 1,
        chained: 6,
        earlier: Some(Earlier::NotKept { lines: 4 }),
        ..Default::default()
    }
}

#[test]
fn the_section_says_what_the_record_is_before_what_it_saw() {
    let seen = seen();
    let (lines, command) = sentences(&seen, &[]);
    assert!(
        lines[0].contains("It credits nothing and counts toward nothing"),
        "{lines:?}"
    );
    assert!(
        lines[0].contains("could have changed the record"),
        "{lines:?}"
    );
    assert_eq!(
        lines[1],
        "It saw 3 commands, 1 file write, and 2 turns ended from 2026-10-15T10:00:00Z to 2026-10-15T11:00:00Z."
    );
    assert!(
        lines[2].contains(COMMAND) && lines[2].contains("finished without an error"),
        "{lines:?}"
    );
    assert_eq!(command, Some("npm test `x` [a](b)"));
    assert!(
        lines[2].ends_with("1 file was written after it."),
        "{lines:?}"
    );
    assert!(
        lines
            .iter()
            .any(|l| l.contains("no longer passes through it")),
        "{lines:?}"
    );
}

#[test]
fn nothing_read_is_said_with_how_to_install_the_hook() {
    let (lines, _) = sentences(
        &HookSeen {
            not_read: Some("there is no hook record".into()),
            ..Default::default()
        },
        &[],
    );
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(lines[1].contains("`sv connect claude --hook`"), "{lines:?}");
}

#[test]
fn both_reports_show_the_command_as_code_and_neither_counts_it() {
    let mut report = empty_report(Vec::new());
    let before = (
        crate::markdown::compliance(&report),
        report.counts.applicable,
    );
    report.hook_record = Some(seen());
    let md = crate::markdown::compliance(&report);
    assert!(
        md.contains("## What the hook saw your AI coding tool do"),
        "{md}"
    );
    assert!(
        md.contains("``npm test `x` [a](b)``"),
        "the command is a code span its backticks cannot close: {md}"
    );
    assert!(!md.contains(COMMAND));
    let html = crate::html::page(&report);
    assert!(html.contains("<code>npm test `x` [a](b)</code>"), "{html}");
    assert!(!html.contains('\u{0}'));
    assert_eq!(report.counts.applicable, before.1);
    assert!(
        !before.0.contains("What the hook saw"),
        "no section without the person's consent"
    );
}
