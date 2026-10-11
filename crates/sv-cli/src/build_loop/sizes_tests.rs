//! The size of each report file and prompt the record holds, beside its name (ADR-084, decided 11
//! October 2026, backlog 0227): written when a line is, read back when a report reads the record, and
//! refused when it names a thing the line does not hand over.

use super::*;
use std::collections::BTreeMap;

fn handed_line(handed: &[&str], sizes: &[(&str, u64)]) -> Line {
    Line {
        time: "2026-10-11T04:00:00Z".to_owned(),
        tool: "read_report".to_owned(),
        handed: handed.iter().map(|h| (*h).to_owned()).collect(),
        handed_sizes: sizes
            .iter()
            .map(|(name, size)| ((*name).to_owned(), *size))
            .collect::<BTreeMap<_, _>>(),
        ..Line::default()
    }
}

#[test]
fn a_size_is_written_beside_the_name_it_belongs_to_and_read_back() {
    // The setup: the line names a report and a prompt, and sizes both, so both must come back.
    let line = handed_line(
        &["report:report.html", "prompt:review"],
        &[("report:report.html", 1234), ("prompt:review", 56)],
    );
    let written = line.to_json();
    let value: Value = serde_json::from_str(&written).unwrap();
    assert_eq!(value["sizes"]["report:report.html"], 1234, "{written}");
    let read = Line::parse(&written).expect("a line sv wrote reads back");
    assert_eq!(read, line);
}

#[test]
fn a_line_written_before_sizes_reads_with_none_kept() {
    // An older line has names and no sizes: it reads, and keeps no size for any name.
    let older =
        r#"{"time":"2026-10-10T09:00:00Z","tool":"read_report","handed":["report:report.html"]}"#;
    let read = Line::parse(older).expect("an older line still reads");
    assert_eq!(read.handed, vec!["report:report.html".to_owned()]);
    assert!(read.handed_sizes.is_empty(), "{read:?}");
}

#[test]
fn a_size_for_a_name_the_line_does_not_hand_over_makes_the_line_one_sv_did_not_write() {
    // The setup: a size for a report that the line never handed over.
    let forged = r#"{"time":"2026-10-11T04:00:00Z","tool":"read_report","handed":["report:report.html"],"sizes":{"report:other.html":9}}"#;
    assert!(
        Line::parse(forged).is_none(),
        "a size for a name not handed over was kept"
    );
}

#[test]
fn a_size_that_is_not_a_whole_number_makes_the_line_one_sv_did_not_write() {
    let text_size = r#"{"time":"2026-10-11T04:00:00Z","tool":"read_report","handed":["report:report.html"],"sizes":{"report:report.html":"big"}}"#;
    assert!(
        Line::parse(text_size).is_none(),
        "a size that is text was kept"
    );
}
