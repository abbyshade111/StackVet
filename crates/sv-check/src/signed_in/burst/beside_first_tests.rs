//! A burst's answers are read beside its first, and the first record is shown made before any are counted
//! (backlog 0560, ADR-119): a limit shown on a 200 page, or as a redirect back to the form, is under 400 too.

use super::super::fake_app::*;
use super::tests::{run_adjusted, why_not};
use super::*;

#[test]
fn a_limit_shown_on_a_200_page_is_not_records_going_through() {
    let o = run_adjusted(|a| {
        a.notes_per_minute = Some(10);
        a.notes_limit_answer = Some((200, false));
    });
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("created 11 records through /notes")),
        "{:#?}",
        o.steps
    );
    assert!(
        !rule_ids(&o).contains(&CREATE_UNLIMITED.rule_id),
        "a limit on a 200 page was read as no limit: {:?}",
        o.findings
    );
    // Not a limit the way 429 is either: not credited, and it says what was compared.
    assert!(!verified_ids(&o).contains(&CREATE_UNLIMITED.rule_id));
    assert!(
        why_not(&o).contains("10 were answered as the first was")
            && why_not(&o).contains("200 where the control redirected to /notes/"),
        "{}",
        why_not(&o)
    );
}

#[test]
fn a_limit_answered_with_a_redirect_elsewhere_is_not_records_going_through() {
    let o = run_adjusted(|a| {
        a.notes_per_minute = Some(10);
        a.notes_limit_answer = Some((303, false));
    });
    assert!(
        !rule_ids(&o).contains(&CREATE_UNLIMITED.rule_id),
        "{:?}",
        o.findings
    );
    assert!(
        why_not(&o)
            .contains("sent the browser to nowhere named where the control was sent to /notes/"),
        "{}",
        why_not(&o)
    );
}

#[test]
fn a_burst_whose_first_record_was_never_kept_shows_nothing() {
    let o = run_adjusted(|a| {
        a.flaws.notes_never_kept = true;
    });
    assert!(
        !rule_ids(&o).contains(&CREATE_UNLIMITED.rule_id),
        "eleven answers that kept nothing were read as eleven records: {:?}",
        o.findings
    );
    assert!(
        why_not(&o).contains("its record was not there when read back at /notes/"),
        "{}",
        why_not(&o)
    );
}
