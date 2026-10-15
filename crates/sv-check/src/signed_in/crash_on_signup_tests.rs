//! Backlog 0528 (the three-tool trial of 14 October 2026): a 500 on `sv`'s long-password sign-up was set aside as a
//! crash, which credits nothing, and never named as the weakness it is. Now it is a low finding under V16.5.3, naming
//! the request by its fields and their lengths, and what the crash would have answered stays withheld.

use super::fake_app::*;
use super::tests::run_signing_up;
use super::*;

fn crash_finding(o: &Outcome) -> Option<&Finding> {
    o.findings
        .iter()
        .find(|f| f.rule_id == CRASHED_ON_ORDINARY.rule_id)
}

#[test]
fn a_crash_on_the_long_password_sign_up_is_a_low_finding_naming_fields_not_values() {
    let o = run_signing_up(Flaws {
        crashes_on_long_password: true,
        ..Default::default()
    });
    let f = crash_finding(&o).expect("the crash on the long-password sign-up is not reported");
    assert_eq!(f.severity, Severity::Low);
    assert_eq!(f.requirement_ids, ["V16.5.3"]);
    assert_eq!(f.evidence, ["signup-long"]);
    assert!(
        f.description.contains("`signup-long` answered 500"),
        "the finding does not name the request"
    );
    assert!(
        f.description.contains("`password` (") && f.description.contains(" characters)"),
        "the finding does not give the password's length"
    );
    // Never a value: the long password is built from the run's spare hex.
    let spare = accounts().spare;
    assert!(
        !f.description.contains(&spare[..11]),
        "the finding shows part of the password"
    );
    // What the crash would have answered stays withheld, as before.
    assert!(
        !o.findings
            .iter()
            .any(|f| f.rule_id == LONG_PASSWORD.rule_id),
        "the long-password finding rests on the crash"
    );
}

#[test]
fn an_app_that_does_not_crash_has_no_such_finding() {
    let o = run_signing_up(Flaws::default());
    assert!(crash_finding(&o).is_none());
}

#[test]
fn a_request_is_named_by_its_fields_and_lengths_for_a_form_or_json() {
    let request = |body: &str| ProbeRequest {
        id: "signup-long".into(),
        method: "POST".into(),
        path: "/signup".into(),
        headers: Vec::new(),
        body: Some(body.as_bytes().to_vec()),
    };
    assert_eq!(
        fields_by_length(&request("email=a%40b.test&password=abc+de")),
        "`email` (8 characters), `password` (6 characters)"
    );
    assert_eq!(
        fields_by_length(&request("{\"email\":\"a@b.test\",\"password\":\"x\"}")),
        "`email` (8 characters), `password` (1 character)"
    );
}
