//! What a report says of what the AI coding tool did (backlog 0630, ADR-131): what it is first, a table of what was
//! seen, what could not be looked for, and that it decides nothing.

use crate::next_step_tests::empty_report;
use crate::tool_behavior::{HEADING, Kind, Seen, ToolBehavior, sentences};

fn behavior() -> ToolBehavior {
    ToolBehavior {
        at_commit: Some("0123abcd".into()),
        commits_read: Some("the 2 commits since the last report (made at 89abcdef)".into()),
        tool_commits: 1,
        other_commits: 1,
        seen: vec![Seen {
            kind: Kind::FindingGone,
            what: "\"A [link](https://x)\" (r) at a|b.py:3 was in the last report and is not in this one".into(),
            place: "a|b.py changed in commit abcdef01 \"Tidy up\", by Claude".into(),
        }],
        more: 2,
        not_seen: vec!["packages installed that no lockfile holds".into()],
    }
}

#[test]
fn the_section_says_what_it_is_then_what_was_read_then_what_it_could_not_see() {
    let lines = sentences(&behavior());
    assert!(
        lines[0].starts_with("Information only: nothing here changes a requirement's status"),
        "{lines:?}"
    );
    assert!(
        lines[1].starts_with("Read: the 2 commits since the last report"),
        "{lines:?}"
    );
    assert!(
        lines[1].contains("of which 1 was made by an AI coding tool"),
        "{lines:?}"
    );
    assert!(
        lines
            .iter()
            .any(|l| l.starts_with("A finding goes away for good reasons too")),
        "{lines:?}"
    );
    assert!(
        lines
            .iter()
            .any(|l| l == "2 more lines were seen and not listed here."),
        "{lines:?}"
    );
    assert_eq!(
        lines.last().unwrap(),
        "Not looked for: packages installed that no lockfile holds."
    );
    let quiet = ToolBehavior {
        seen: vec![],
        more: 0,
        ..behavior()
    };
    assert!(
        sentences(&quiet)
            .iter()
            .any(|l| l == "Nothing was seen that is worth a look.")
    );
    assert!(
        !sentences(&quiet)
            .iter()
            .any(|l| l.starts_with("A finding goes away"))
    );
}

#[test]
fn the_section_is_information_and_changes_nothing_else_in_the_report() {
    let mut report = empty_report(Vec::new());
    let before = (
        crate::markdown::security(&report),
        crate::bluf::headline(&report),
        crate::bluf::counted(&report),
        report.counts.applicable,
    );
    assert!(
        !crate::markdown::compliance(&report).contains(HEADING),
        "no section when none was read"
    );
    report.tool_behavior = Some(behavior());
    let md = crate::markdown::compliance(&report);
    assert!(md.contains(&format!("## {HEADING}")), "{md}");
    // What came from the app is inert: no live link, and a bar does not end the cell.
    assert!(md.contains("a\\|b.py:3"), "{md}");
    assert!(!md.contains("[link](https://x)"), "{md}");
    assert!(md.contains("| A finding went away |"), "{md}");
    let html = crate::html::page(&report);
    assert!(html.contains(&format!("<h2>{HEADING}</h2>")), "{html}");
    assert!(html.contains("<td>A finding went away</td>"), "{html}");
    assert!(crate::pages::information(&report).contains(HEADING));
    // It decides nothing: the security page, the headline, the counts, and what applies read as they did.
    assert_eq!(crate::markdown::security(&report), before.0);
    assert_eq!(crate::bluf::headline(&report), before.1);
    assert_eq!(crate::bluf::counted(&report), before.2);
    assert_eq!(report.counts.applicable, before.3);
}
