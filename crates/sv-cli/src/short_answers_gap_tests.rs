//! Backlog 0598 (ADR-021): the answers that came with less body than they said are said once, as one gap, with the
//! app's own log line beside them, and never as findings about what the blank pages lack.

use super::short_answers_gap;
use sv_check::secrets::SecretRules;

fn rules() -> &'static SecretRules {
    SecretRules::shared().expect("the rules sv ships")
}

#[test]
fn nothing_short_is_no_gap() {
    assert!(short_answers_gap(rules(), &[], None).is_none());
}

#[test]
fn short_answers_are_one_gap_with_the_app_s_line() {
    let short = vec![
        "login-page (200, 0 of 5415 bytes)".to_owned(),
        "signup-page (200, 0 of 5620 bytes)".to_owned(),
    ];
    let gap = short_answers_gap(
        rules(),
        &short,
        Some("RuntimeError: Response content longer than Content-Length"),
    )
    .expect("a gap");
    assert_eq!(
        gap.what,
        "2 answers the app sent with less body than they said"
    );
    for part in [
        "login-page (200, 0 of 5415 bytes), signup-page (200, 0 of 5620 bytes)",
        "read as no answer",
        "not assessed",
        "The app's own log says: `RuntimeError: Response content longer than Content-Length`.",
    ] {
        assert!(gap.why.contains(part), "{part}: {}", gap.why);
    }
    assert_eq!(gap.reason, sv_report::GapReason::Partial);
}

#[test]
fn the_app_s_line_is_redacted() {
    // A key-shaped value in the log line, built from pieces, never reaches the report.
    let key = ["sk", "_live_", "4eC39HqLyjWDarjtT1zdp7dc"].concat();
    let gap = short_answers_gap(
        rules(),
        &["home (200, 3 of 90 bytes)".to_owned()],
        Some(&format!("Content-Length mismatch for STRIPE_KEY={key}")),
    )
    .expect("a gap");
    assert!(!gap.why.contains(&key), "{}", gap.why);
    assert!(
        gap.what
            .starts_with("1 answer the app sent with less body than it said")
    );
}
