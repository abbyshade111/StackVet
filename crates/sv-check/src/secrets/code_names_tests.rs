//! Backlog 0531: a code people are given to get in, under an invite, access, or join name, is reported at medium, judged
//! by its length rather than how random it reads; a name that only ends in `CODE` is not one.

use super::*;

fn found(line: &str) -> Vec<Finding> {
    assignment_findings("app.py", line, 1, &(0..usize::MAX))
}

#[test]
fn the_trial_s_invite_code_is_reported_at_medium() {
    let line = r#"INVITE_FOR_STACKVET = "stackvet-test-invite""#;
    // The setup: the value reads below the randomness bar the other credentials are held to.
    assert!(shannon_entropy("stackvet-test-invite") < 3.5);
    let found = found(line);
    assert_eq!(found.len(), 1, "{found:?}");
    let f = &found[0];
    assert_eq!(f.rule_id, ASSIGNMENT_RULE);
    assert_eq!(
        (f.severity, f.confidence),
        (Severity::Medium, Confidence::Medium)
    );
    assert!(
        f.title
            .starts_with("An invite or access code is written into the code"),
        "{}",
        f.title
    );
    assert!(f.fix.contains("seed script"), "{}", f.fix);
    assert!(
        !f.title.contains("stackvet-test-invite"),
        "the value is never in the words"
    );
}

#[test]
fn each_kind_of_code_name_is_one() {
    for name in [
        "INVITE_CODE",
        "inviteCode",
        "ACCESS_CODE",
        "accessCode",
        "JOIN_CODE",
        "joincode",
        "FAMILY_INVITE",
    ] {
        assert!(
            !found(&format!(r#"{name} = "lantern-harbor-9""#)).is_empty(),
            "{name}"
        );
    }
}

#[test]
fn what_is_not_an_invite_code_is_not_reported_as_one() {
    // The controls: names that only end in a code, a value too short to be one, a placeholder, and a URL.
    for line in [
        r#"ERROR_CODE = "lantern-harbor-9""#,
        r#"COUNTRY_CODE = "lantern-harbor-9""#,
        r#"INVITE_CODE = "1234""#,
        r#"INVITE_CODE = "your-invite-code""#,
        r#"INVITE_URL = "https://example.org/join""#,
    ] {
        assert!(found(line).is_empty(), "{line}: {:?}", found(line));
    }
    // And a real credential under a credential's name keeps its own grade.
    let key = ["Zq7v", "X2mLp9", "wRt4kYb8n"].concat();
    let api = found(&format!(r#"API_KEY = "{key}""#));
    assert_eq!(
        api.first().map(|f| f.severity),
        Some(Severity::High),
        "{api:?}"
    );
}
