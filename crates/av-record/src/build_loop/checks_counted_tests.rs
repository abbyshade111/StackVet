//! Backlog 0640: a check of the app is a `stackvet_check` call, or `stackvet_write_report`, which checks it in full. A design brief or the preflight comes back with counts
//! too, and the owner's Cursor build was said to have 15 checks for 13, its "first check" a brief made before any code.

use super::*;

fn line(tool: &str, minute: u8, findings: usize) -> String {
    Line {
        time: format!("2026-10-22T22:{minute:02}:00Z"),
        tool: tool.to_owned(),
        counts: Some(LoopCounts {
            findings,
            checked: 0,
            needs_attention: 0,
            not_assessed: 0,
        }),
        ..Line::default()
    }
    .to_json()
}

#[test]
fn briefs_and_the_preflight_are_not_checks_and_the_first_is_the_first_check() {
    let record = [
        line("stackvet_before", 3, 9),
        line("stackvet_before", 4, 9),
        line("stackvet_check", 8, 2),
        line("stackvet_preflight", 9, 7),
        line("stackvet_check", 12, 1),
        line("stackvet_write_report", 20, 0),
    ]
    .join("\n");
    let b = summarize(record.as_bytes());
    assert_eq!(b.calls, 6, "the briefs and the preflight are still calls");
    assert_eq!(
        b.checks, 3,
        "the two stackvet_check calls and the full report are checks"
    );
    assert_eq!(
        b.first_counts.map(|c| c.findings),
        Some(2),
        "the first check is the first stackvet_check"
    );
    assert_eq!(
        b.last_counts.map(|c| c.findings),
        Some(0),
        "the full report is the last check"
    );
}
