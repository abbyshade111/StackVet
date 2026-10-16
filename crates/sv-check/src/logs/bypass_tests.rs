//! Backlog 0499 (V16.3.3): a refused bypass attempt looked for in the app's own output, between the two marked requests
//! around it. Found is checked in part; not found is not assessed, never a finding.

use super::*;

const RULE: &str = "probe.bypass-attempt-logged";

fn markers(status: u16) -> Markers {
    Markers {
        bypass: Some((
            Window {
                open: "sv-log-before-bypass-forged".to_owned(),
                close: "sv-log-after-bypass-forged".to_owned(),
                login_path: "/notes".to_owned(),
            },
            status,
        )),
        ..Default::default()
    }
}

fn log(between: &str) -> String {
    format!(
        "GET /sv-log-before-bypass-forged 404\n{between}\nGET /sv-log-after-bypass-forged 404\nPOST /notes 403\n"
    )
}

fn credit(o: &LogOutcome) -> Option<&crate::Verified> {
    o.verified.iter().find(|v| v.check_id == RULE)
}

fn not_assessed(o: &LogOutcome) -> bool {
    o.not_assessed.iter().any(|(ids, _)| ids == "V16.3.3")
}

#[test]
fn a_line_naming_the_attempt_is_credited_in_part_and_kept() {
    for line in [
        "WARNING csrf token missing for POST",
        r#"{"event":"csrf_failed","status":403}"#,
        "request refused: origin not allowed",
        "The CSRF token is missing.",
    ] {
        let o = evaluate(&markers(403), &log(line));
        let v = credit(&o).unwrap_or_else(|| panic!("`{line}` should be read as the attempt"));
        assert!(v.in_part, "one kind of bypass is only ever in part");
        assert!(o.lines.iter().any(|k| k.line == line), "the line is kept");
        assert!(!not_assessed(&o));
        assert!(o.findings.is_empty(), "never a finding");
    }
}

#[test]
fn the_access_log_line_of_the_refused_request_counts_only_with_a_refusal_status() {
    let o = evaluate(&markers(403), &log("POST /notes 403"));
    assert!(
        credit(&o).is_some(),
        "the request's path with its refusal status"
    );

    // The same form shown again with 200 is a refusal the access log cannot tell from a success.
    let o = evaluate(&markers(200), &log("POST /notes 200"));
    assert!(credit(&o).is_none());
    assert!(not_assessed(&o));

    // A refusal status on another path, or this path with another status, is not this request.
    let o = evaluate(&markers(403), &log("POST /admin 403"));
    assert!(credit(&o).is_none());
    let o = evaluate(&markers(403), &log("POST /notes 201"));
    assert!(credit(&o).is_none());
}

#[test]
fn a_path_that_only_looks_like_the_event_is_not_read_as_it() {
    let o = evaluate(&markers(403), &log("GET /csrf-demo/forgery 200"));
    assert!(credit(&o).is_none());
    assert!(not_assessed(&o));
}

#[test]
fn a_line_outside_the_window_does_not_count() {
    let text = "csrf token missing\nGET /sv-log-before-bypass-forged 404\nGET /sv-log-after-bypass-forged 404\n";
    let o = evaluate(&markers(403), text);
    assert!(credit(&o).is_none());
    assert!(not_assessed(&o));
}

#[test]
fn nothing_planted_says_nothing_of_v16_3_3() {
    let o = evaluate(&Markers::default(), &log("csrf token missing"));
    assert!(credit(&o).is_none());
    assert!(!not_assessed(&o));

    // No output at all: not assessed when planted, silent when not.
    let o = evaluate(&markers(403), "");
    assert!(not_assessed(&o));
    let o = evaluate(&Markers::default(), "");
    assert!(!not_assessed(&o));
}
