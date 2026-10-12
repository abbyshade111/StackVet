//! The authorization-failure log check reads a refusal on every listed private page, not the first (backlog 0232,
//! part 5; ADR-053, Later): whole only when two or more pages were refused and each one's refusal was found in the
//! app's output with its status. Each page's marker carries its number before the hyphen, so none is part of another.

use super::*;

const RULE: &str = "probe.authorization-failure-logged";

fn markers(planted: &[(&str, usize)]) -> Markers {
    Markers {
        refused_requests: planted
            .iter()
            .map(|(m, page)| ((*m).to_owned(), 302, *page))
            .collect(),
        ..Default::default()
    }
}

fn credit(o: &LogOutcome) -> Option<&crate::Verified> {
    o.verified.iter().find(|v| v.check_id == RULE)
}

const BOTH: &str = "\
2026-10-12T10:00:03Z GET /account?sv-log-refused-4a91=1 302 anonymous
2026-10-12T10:00:04Z GET /notes?sv-log-refused1-4a91=1 302 anonymous
";

#[test]
fn every_listed_page_refused_and_logged_with_its_status_is_whole() {
    let o = evaluate(
        &markers(&[("sv-log-refused-4a91", 0), ("sv-log-refused1-4a91", 1)]),
        BOTH,
    );
    let credit = credit(&o).expect("credited");
    assert!(!credit.in_part, "{credit:?}");
    assert!(
        credit.scope.starts_with("2 private pages"),
        "{}",
        credit.scope
    );
}

#[test]
fn a_listed_page_whose_refusal_was_not_found_leaves_it_in_part() {
    let only_first = BOTH.lines().next().unwrap();
    let o = evaluate(
        &markers(&[("sv-log-refused-4a91", 0), ("sv-log-refused1-4a91", 1)]),
        only_first,
    );
    let credit = credit(&o).expect("credited for the page that was");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.starts_with("1 private page "),
        "{}",
        credit.scope
    );
}

#[test]
fn one_page_listed_is_one_sample() {
    let o = evaluate(&markers(&[("sv-log-refused-4a91", 0)]), BOTH);
    assert!(credit(&o).expect("credited").in_part);
}

#[test]
fn one_pages_line_does_not_stand_for_another() {
    // Only the second page's line is in the output. The first page's marker is not part of the
    // second's, so the first is not found, and the credit counts one page.
    let second_only = BOTH.lines().nth(1).unwrap();
    let o = evaluate(
        &markers(&[("sv-log-refused-4a91", 0), ("sv-log-refused1-4a91", 1)]),
        second_only,
    );
    let credit = credit(&o).expect("credited for the second page");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.starts_with("1 private page "),
        "{}",
        credit.scope
    );
}

#[test]
fn no_pages_marker_is_part_of_anothers() {
    // Whatever the tag, page `i`'s marker carries `i` before its hyphen.
    for tag in ["4a91", "p1x4a91", "11", "1-4a91"] {
        let marker = |i: usize| {
            let n = if i == 0 { String::new() } else { i.to_string() };
            format!("sv-log-refused{n}-{tag}")
        };
        for a in 0..13 {
            for b in 0..13 {
                if a != b {
                    assert!(
                        !marker(b).contains(&marker(a)),
                        "{} is part of {}",
                        marker(a),
                        marker(b)
                    );
                }
            }
        }
    }
}
