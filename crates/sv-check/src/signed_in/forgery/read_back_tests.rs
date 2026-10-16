//! A create sent as another site would send it is taken only when the record is there when read back, or, when
//! it cannot be read back, when it was answered as the app's own request was (backlog 0560, ADR-119). A form
//! shown again with 200 and "The CSRF token is missing." is under 400 too.

use super::super::fake_app::{FakeApp, Flaws, run_seeded, users};
use super::*;

fn forgery_findings(out: &Outcome) -> Vec<&crate::Finding> {
    out.findings
        .iter()
        .filter(|f| f.rule_id == FORGERY.rule_id)
        .collect()
}

#[test]
fn a_refusal_shown_on_a_200_page_is_not_a_request_taken() {
    let (out, app) = run_seeded(
        FakeApp::new(Flaws {
            forgery_refused_on_200: true,
            ..Default::default()
        }),
        &users(),
    );
    assert!(
        app.notes.iter().any(|(_, t)| t == "sv-probe-private-4c7e"),
        "the first user's own note, the control, was not made"
    );
    assert!(
        forgery_findings(&out).is_empty(),
        "{:?}",
        forgery_findings(&out)
    );
    // A 200 page names no record to read back, and it is not the control's answer: neither taken nor refused.
    assert!(!out.verified.iter().any(|v| v.check_id == FORGERY.rule_id));
    assert!(
        out.not_assessed
            .iter()
            .any(|(ids, why)| ids.contains("V3.5.1")
                && why.contains("200 where the control redirected to /notes/1")),
        "{:?}",
        out.not_assessed
    );
}

#[test]
fn a_refusal_that_redirects_like_a_note_that_was_made_is_read_back() {
    let (out, app) = run_seeded(
        FakeApp::new(Flaws {
            forgery_refused_alike: true,
            ..Default::default()
        }),
        &users(),
    );
    assert!(!app.notes.iter().any(|(_, t)| t == "sv-probe-forged-9b21"));
    assert!(
        forgery_findings(&out).is_empty(),
        "a forged note that was never made was read as taken: {:?}",
        forgery_findings(&out)
    );
    assert!(out.verified.iter().any(|v| v.check_id == FORGERY.rule_id));
}

#[test]
fn a_forged_note_that_was_made_is_found_by_reading_it_back() {
    let (out, _) = run_seeded(
        FakeApp::new(Flaws {
            no_csrf_check: true,
            ..Default::default()
        }),
        &users(),
    );
    let found = forgery_findings(&out);
    assert!(
        found.iter().any(|f| f
            .description
            .contains("and the record it made was there when the signed-in user read /notes/")),
        "{found:?}"
    );
}

#[test]
fn a_read_back_that_could_not_find_the_controls_record_is_not_believed() {
    // A create whose form carries no `{marker}`: reading its record back can never find a marker, so "not
    // there" would read every forged copy as refused. The control's own record not being found that way
    // leaves the answer to be read beside the control's.
    let mut u = users();
    u.creates = vec![sv_manifest::CreateTemplate {
        method: "POST".into(),
        path: "/notes".into(),
        form: [("text", "hello"), ("csrf_token", "{csrf}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        json: Default::default(),
        read: None,
        id_field: None,
        page: false,
    }];
    let (out, _) = run_seeded(
        FakeApp::new(Flaws {
            no_csrf_check: true,
            ..Default::default()
        }),
        &u,
    );
    let found = forgery_findings(&out);
    assert!(
        found.iter().any(|f| f
            .description
            .contains("/notes (it answered 303 as the app's own request did)")),
        "{found:?}"
    );
}

#[test]
fn a_json_api_that_refuses_with_200_and_an_error_takes_nothing() {
    let (out, _) = run_seeded(
        FakeApp::new(Flaws {
            api_refuses_with_200: true,
            api_checks_origin: true,
            ..Default::default()
        }),
        &super::tests::with_json_api(),
    );
    assert!(
        !out.findings
            .iter()
            .any(|f| f.rule_id == SIMPLE_REQUEST.rule_id),
        "a refusal answered 200 with an error was read as taken: {:?}",
        out.findings
    );
    // The answer names no record and is not the control's shape: not judged, and it says what was compared.
    assert!(
        out.not_assessed.iter().any(|(ids, why)| ids == "V3.5.2"
            && why.contains("an answer with the names `error` where the control's had")),
        "{:?}",
        out.not_assessed
    );
}
