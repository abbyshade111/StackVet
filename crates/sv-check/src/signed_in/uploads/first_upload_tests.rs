//! The first ordinary file is the gate every upload check rests on, so it is not taken as kept on "under 400"
//! alone when its answer is the upload form again (backlog 0560, ADR-119).

use super::super::fake_app::*;
use super::tests::with_upload;
use super::*;

fn gate_closed(o: &Outcome) -> Option<&String> {
    o.not_assessed
        .iter()
        .find(|(ids, _)| ids == "V5.2.1, V5.2.2, V5.3.1, V3.2.1")
        .map(|(_, why)| why)
}

fn nothing_judged(o: &Outcome) {
    for rule in [OVERSIZED_FILE.rule_id, CONTENT_MISMATCH.rule_id] {
        assert!(
            !o.findings.iter().any(|f| f.rule_id == rule),
            "{rule}: {:?}",
            o.findings
        );
        assert!(
            !o.verified.iter().any(|v| v.check_id == rule),
            "{rule}: {:?}",
            o.verified
        );
    }
}

#[test]
fn an_ordinary_file_answered_with_the_form_again_is_not_taken_as_kept() {
    let o = run_against(
        Flaws {
            uploads_off_on_200: true,
            ..Default::default()
        },
        &with_upload(None, Some(UPLOAD_LIMIT as u64)),
    );
    let why = gate_closed(&o).expect("the upload questions are not assessed");
    assert!(why.contains("answered 200 with the upload form"), "{why}");
    assert!(why.contains("`serves-at`"), "{why}");
    nothing_judged(&o);
}

#[test]
fn with_serves_at_that_finds_nothing_the_form_again_still_settles_nothing() {
    let o = run_against(
        Flaws {
            uploads_off_on_200: true,
            ..Default::default()
        },
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    let why = gate_closed(&o).expect("the upload questions are not assessed");
    assert!(why.contains("did not find it"), "{why}");
    nothing_judged(&o);
}

#[test]
fn a_kept_file_answered_with_the_form_again_is_judged_once_it_is_fetched_back() {
    // The control: an app that keeps the file and shows the form again is judged as before, because fetching
    // the ordinary file back found it.
    let o = run_against(
        Flaws {
            uploads_kept_answer_form: true,
            ..Default::default()
        },
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    assert!(gate_closed(&o).is_none(), "{:?}", o.not_assessed);
    for rule in [OVERSIZED_FILE.rule_id, CONTENT_MISMATCH.rule_id] {
        assert!(
            o.verified.iter().any(|v| v.check_id == rule),
            "{rule}: {:?} {:?}",
            o.findings,
            o.not_assessed
        );
    }
}

#[test]
fn a_kept_file_answered_with_the_form_again_without_serves_at_says_what_would_settle_it() {
    let o = run_against(
        Flaws {
            uploads_kept_answer_form: true,
            ..Default::default()
        },
        &with_upload(None, Some(UPLOAD_LIMIT as u64)),
    );
    let why = gate_closed(&o).expect("nothing shows the file was kept");
    assert!(why.contains("`serves-at`"), "{why}");
    nothing_judged(&o);
}
