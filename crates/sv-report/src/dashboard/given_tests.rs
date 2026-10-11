//! What the build loop was given, as the dashboard shows it: names always, sizes where the record kept
//! them, and nothing for a report from before sizes were kept (backlog 0227, ADR-086).

use super::*;
use serde_json::json;

#[test]
fn the_reader_takes_each_name_with_its_size_where_the_report_kept_one() {
    // The setup: two names handed over, one with a size and one without.
    let build_loop = json!({
        "handed": ["report:report.html", "prompt:review"],
        "handed_sizes": { "report:report.html": 1234 },
    });
    assert_eq!(
        given_of(&build_loop),
        vec![
            ("report:report.html".to_owned(), Some(1234)),
            ("prompt:review".to_owned(), None),
        ]
    );
    // A report from before sizes were kept, and one with no build loop at all, give names or nothing.
    let older = json!({ "handed": ["instructions"] });
    assert_eq!(given_of(&older), vec![("instructions".to_owned(), None)]);
    assert!(given_of(&json!({})).is_empty());
}

#[test]
fn the_section_says_each_size_in_bytes_and_says_when_none_was_kept() {
    let section = given_section(&[
        ("report:report.html".to_owned(), Some(1234)),
        ("prompt:review".to_owned(), None),
    ]);
    assert!(
        section.contains("<h3>What the AI coding tool was given</h3>"),
        "{section}"
    );
    assert!(
        section.contains("<code>report:report.html</code>: 1234 bytes"),
        "{section}"
    );
    assert!(
        section.contains("<code>prompt:review</code>: size not recorded"),
        "{section}"
    );
    // Nothing given, nothing shown.
    assert_eq!(given_section(&[]), "");
}

#[test]
fn a_name_from_the_record_is_escaped_on_the_page() {
    // The setup: a name carrying markup, as a record could hold if it were written by hand.
    let section = given_section(&[("report:<b>x</b>.html".to_owned(), Some(1))]);
    assert!(!section.contains("<b>x</b>"), "{section}");
    assert!(section.contains("&lt;b&gt;"), "{section}");
}
