//! An upload is judged against the ordinary file, never by "under 400" alone (backlog 0560, ADR-119): a refusal
//! shown on the upload form with 200 is under 400 too.

use super::super::fake_app::*;
use super::tests::{upload_run_adjusted, with_upload};
use super::*;

fn ids(o: &Outcome, verified: bool) -> Vec<&str> {
    if verified {
        o.verified.iter().map(|v| v.check_id.as_str()).collect()
    } else {
        o.findings.iter().map(|f| f.rule_id.as_str()).collect()
    }
}

fn not_assessed_for<'a>(o: &'a Outcome, id: &str) -> Vec<&'a String> {
    o.not_assessed
        .iter()
        .filter(|(ids, _)| ids == id)
        .map(|(_, why)| why)
        .collect()
}

#[test]
fn a_refusal_on_the_form_with_200_is_settled_by_fetching_the_file_back() {
    let o = run_against(
        Flaws {
            upload_refusals_on_200: true,
            ..Default::default()
        },
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s == "uploaded an ordinary GIF to /upload"),
        "{:?}",
        o.steps
    );
    for rule in [OVERSIZED_FILE.rule_id, CONTENT_MISMATCH.rule_id] {
        assert!(!ids(&o, false).contains(&rule), "{rule}: {:?}", o.findings);
        assert!(
            ids(&o, true).contains(&rule),
            "{rule}: {:?}",
            o.not_assessed
        );
    }
}

#[test]
fn without_serves_at_a_refusal_on_the_form_cannot_be_told_and_says_so() {
    let o = run_against(
        Flaws {
            upload_refusals_on_200: true,
            ..Default::default()
        },
        &with_upload(None, Some(UPLOAD_LIMIT as u64)),
    );
    for (rule, id) in [
        (OVERSIZED_FILE.rule_id, "V5.2.1"),
        (CONTENT_MISMATCH.rule_id, "V5.2.2"),
    ] {
        assert!(!ids(&o, false).contains(&rule), "{rule}: {:?}", o.findings);
        assert!(!ids(&o, true).contains(&rule), "{rule}");
        let why = not_assessed_for(&o, id);
        assert!(
            why.iter()
                .any(|w| w.contains("200 with a form on it, where the control's page had none")),
            "{id}: {why:?}"
        );
    }
}

#[test]
fn a_refusal_answered_like_a_stored_file_is_settled_by_fetching_it_back() {
    let o = run_against(
        Flaws {
            upload_refused_alike: true,
            ..Default::default()
        },
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    for rule in [OVERSIZED_FILE.rule_id, CONTENT_MISMATCH.rule_id] {
        assert!(
            !ids(&o, false).contains(&rule),
            "a file that was never kept was read as accepted: {rule}"
        );
        assert!(
            ids(&o, true).contains(&rule),
            "{rule}: {:?}",
            o.not_assessed
        );
    }
}

#[test]
fn a_file_that_was_kept_is_found_by_fetching_it_back() {
    let o = run_against(
        Flaws {
            oversized_upload_ok: true,
            unchecked_contents_ok: true,
            ..Default::default()
        },
        &with_upload(Some("/files/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    for rule in [OVERSIZED_FILE.rule_id, CONTENT_MISMATCH.rule_id] {
        let f = o
            .findings
            .iter()
            .find(|f| f.rule_id == rule)
            .unwrap_or_else(|| panic!("{rule}: {:?}", o.findings));
        assert!(
            f.description
                .contains("and the file was served back at /files/sv-probe-"),
            "{}",
            f.description
        );
    }
}

#[test]
fn a_quota_shown_on_a_200_page_after_a_refusal_leaves_it_not_assessed() {
    // The ordinary file fills the quota; every file after it is refused on the form with 200. The GIF sent
    // after the oversized one was "under 400", which read as the refusal standing.
    let o = upload_run_adjusted(
        Flaws {
            upload_refusals_on_200: true,
            ..Default::default()
        },
        |a| a.upload_quota = Some(1),
    );
    assert!(!ids(&o, true).contains(&OVERSIZED_FILE.rule_id));
    let why = not_assessed_for(&o, "V5.2.1");
    assert!(
        why.iter()
            .any(|w| w.contains("answered unlike the first ordinary one")),
        "{why:?}"
    );
}

#[test]
fn a_serves_at_that_does_not_find_the_ordinary_file_is_not_trusted_to_say_a_file_is_gone() {
    // A `serves-at` that finds nothing, the ordinary file included: a 404 there says nothing about the files
    // after it, which the app kept.
    let o = run_against(
        Flaws {
            oversized_upload_ok: true,
            unchecked_contents_ok: true,
            ..Default::default()
        },
        &with_upload(Some("/nowhere/{name}"), Some(UPLOAD_LIMIT as u64)),
    );
    for rule in [OVERSIZED_FILE.rule_id, CONTENT_MISMATCH.rule_id] {
        assert!(ids(&o, false).contains(&rule), "{rule}: {:?}", o.verified);
        assert!(!ids(&o, true).contains(&rule), "{rule}");
    }
}
