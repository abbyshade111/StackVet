//! The always-true and always-false conditions are read beside the unmodified address, never only beside each
//! other (backlog 0560, ADR-119): a database reading the condition answers the always-true one as a page of the
//! same kind as the address unchanged, and under an `OR` the always-false one too.

use super::super::fake_app::*;
use super::tests::{plain, run_twisted, sql_findings, sql_users};
use super::*;

/// The steps that say what was found at each spot.
fn spot_steps(o: &Outcome) -> Vec<&String> {
    o.steps
        .iter()
        .filter(|s| s.starts_with("added an always-true"))
        .collect()
}

const FORM: &str = "<form><input name=q> Please search again.</form>";

#[test]
fn a_finding_names_the_unmodified_address_it_was_read_beside() {
    let (o, _) = plain(
        Flaws {
            sql_in_search: true,
            ..Default::default()
        },
        &sql_users(),
    );
    let found = sql_findings(&o);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0]
            .evidence
            .first()
            .is_some_and(|id| id.ends_with("-plain")),
        "{:?}",
        found[0].evidence
    );
    assert!(
        found[0]
            .description
            .contains("and both were answered as the unchanged address was"),
        "{}",
        found[0].description
    );
}

#[test]
fn an_always_true_unlike_the_unmodified_address_is_not_the_database() {
    // Every always-true answered with a form the page never shows: the pair differs, but not as a database
    // reading the condition would make it differ.
    let (o, _) = run_twisted(Flaws::default(), &sql_users(), |r, mut answer, _| {
        if r.id.contains("-true-") {
            answer.body = FORM.to_owned();
        }
        Some(answer)
    });
    assert!(sql_findings(&o).is_empty(), "{:?}", sql_findings(&o));
    // The search is the spot whose always-true is a success in this app; a record's address with a condition
    // added is no record at all, so nothing is told apart there.
    let steps = spot_steps(&o);
    let search = steps
        .iter()
        .find(|s| s.contains("`q` in /search"))
        .expect("the search was asked");
    assert!(
        search.contains("not judged") && search.contains("the always-true one was unlike"),
        "{search}"
    );
}

#[test]
fn an_either_or_is_read_by_its_always_false_one() {
    // Only the either-or form's always-false answered with the form: its always-true is the page as it was,
    // so reading the wrong one of the pair would make this a finding.
    let (o, _) = run_twisted(Flaws::default(), &sql_users(), |r, mut answer, _| {
        if r.id.contains("-false-") && r.path.contains("%20OR%20") {
            answer.body = FORM.to_owned();
        }
        Some(answer)
    });
    assert!(sql_findings(&o).is_empty(), "{:?}", sql_findings(&o));
    assert!(
        spot_steps(&o)
            .iter()
            .any(|s| s.contains("not judged") && s.contains("the always-false one")),
        "{:?}",
        spot_steps(&o)
    );
}

#[test]
fn an_address_not_a_success_unchanged_is_not_asked_with_conditions() {
    let (o, sent) = run_twisted(
        Flaws {
            sql_in_search: true,
            sql_in_record: true,
            ..Default::default()
        },
        &sql_users(),
        |r, mut answer, _| {
            if r.id.ends_with("-plain") {
                answer.status = 404;
            }
            Some(answer)
        },
    );
    assert!(sql_findings(&o).is_empty(), "{:?}", sql_findings(&o));
    // The setup: each spot was asked unchanged, and nothing more.
    assert_eq!(
        sent.iter()
            .filter(|(id, _, _)| id.starts_with("sql-"))
            .count(),
        3,
        "{sent:?}"
    );
    assert_eq!(
        o.steps
            .iter()
            .filter(|s| s.contains("unchanged: not a success"))
            .count(),
        3,
        "{:?}",
        o.steps
    );
}
