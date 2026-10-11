//! The record's chain of hashes (backlog 0239): each line is chained to the one before it, and a report says whether the
//! chain holds, and at which line it first breaks. Option 3 of the owner's choice of 10 October 2026: one record, with
//! its chain, rather than a second copy.

use super::*;
use std::path::PathBuf;

fn app(tag: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("sv-build-loop-chain-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n[app]\nname = \"t\"\n",
    )
    .unwrap();
    dir
}

fn line(time: &str, tool: &str) -> Line {
    Line {
        time: time.to_owned(),
        tool: tool.to_owned(),
        ..Line::default()
    }
}

fn record_file(dir: &Path) -> PathBuf {
    sv_scan::ecosystems::default_report_dir_in(dir).join(sv_scan::ecosystems::BUILD_LOOP_RECORD)
}

/// Three calls, written through the record as the server writes them.
fn three_calls(dir: &Path) {
    for (time, tool) in [("1", "a"), ("2", "b"), ("3", "c")] {
        record(dir, &line(time, tool)).unwrap();
    }
}

#[test]
fn a_record_written_through_the_chain_holds_and_a_report_says_so() {
    let dir = app("holds");
    three_calls(&dir);
    let text = std::fs::read_to_string(record_file(&dir)).unwrap();
    // The setup: three lines, each carrying a chain.
    assert_eq!(text.lines().count(), 3, "{text}");
    assert!(text.lines().all(|l| l.contains("\"chain\":")), "{text}");
    let summary = read(&dir);
    assert_eq!(summary.chained, 3, "{summary:?}");
    assert_eq!(summary.chain_broken_at, None, "{summary:?}");
    assert!(summary.chain_head.is_some(), "{summary:?}");
}

#[test]
fn an_edited_line_breaks_the_chain_at_that_line() {
    let dir = app("edited");
    three_calls(&dir);
    let text = std::fs::read_to_string(record_file(&dir)).unwrap();
    // The setup: the line to change is really there.
    assert!(text.contains("\"tool\":\"b\""), "{text}");
    std::fs::write(
        record_file(&dir),
        text.replace("\"tool\":\"b\"", "\"tool\":\"x\""),
    )
    .unwrap();
    let summary = read(&dir);
    assert_eq!(summary.chain_broken_at, Some(2), "{summary:?}");
    assert_eq!(summary.chained, 1, "{summary:?}");
}

#[test]
fn a_removed_line_breaks_the_chain_at_the_line_after_the_gap() {
    let dir = app("removed");
    three_calls(&dir);
    let text = std::fs::read_to_string(record_file(&dir)).unwrap();
    let kept: Vec<&str> = text
        .lines()
        .enumerate()
        .filter(|(n, _)| *n != 1)
        .map(|(_, l)| l)
        .collect();
    std::fs::write(record_file(&dir), kept.join("\n") + "\n").unwrap();
    let summary = read(&dir);
    // The third line, now the second, was chained to the line that was removed.
    assert_eq!(summary.chain_broken_at, Some(2), "{summary:?}");
    assert_eq!(summary.chained, 1, "{summary:?}");
}

#[test]
fn lines_written_before_the_chain_are_not_counted_and_do_not_break_it() {
    let dir = app("before");
    // Two lines as the record held them before the chain: no `chain` key.
    let old = format!(
        "{}\n{}\n",
        line("1", "a").to_json(),
        line("2", "b").to_json()
    );
    std::fs::create_dir_all(record_file(&dir).parent().unwrap()).unwrap();
    std::fs::write(record_file(&dir), old).unwrap();
    record(&dir, &line("3", "c")).unwrap();
    let summary = read(&dir);
    assert_eq!(summary.calls, 3, "{summary:?}");
    assert_eq!(summary.chained, 1, "{summary:?}");
    assert_eq!(summary.chain_broken_at, None, "{summary:?}");
}
