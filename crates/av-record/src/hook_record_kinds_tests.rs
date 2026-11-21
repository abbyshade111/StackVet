//! Backlog 0607: the claims about security a turn's end makes, kept by kind and counted, never in the tool's words.

use crate::hook_record::{TESTS_PASS, append, claims_in, line_of, read};
use serde_json::json;
use std::path::Path;

#[test]
fn each_kind_is_told_from_a_sentence_that_only_names_it() {
    for (said, kind) in [
        ("I added rate limiting to the login route.", "rate-limiting"),
        (
            "Login now has a lockout after five tries, implemented in auth.py.",
            "rate-limiting",
        ),
        ("Fixed the XSS in the notes page.", "cross-site-scripting"),
        ("CSRF protection is enabled on every form.", "forgery-token"),
        ("Passwords are now hashed with bcrypt.", "password-storage"),
        (
            "All queries use parameterized queries now.",
            "sql-injection",
        ),
        (
            "Fixed the command injection in the export.",
            "command-injection",
        ),
        (
            "Added an authorization check to the admin routes.",
            "access-control",
        ),
        ("Moved the API key into an environment variable.", "secrets"),
    ] {
        assert!(
            claims_in(said).contains(&kind),
            "{said}: {:?}",
            claims_in(said)
        );
    }
    for said in [
        "Rate limiting is not in place yet.",
        "I'll add CSRF protection next.",
        "You should hash the passwords with bcrypt.",
        "If you want, I could fix the XSS.",
        "The page shows the API key setting.",
        "Next: rate limiting.",
    ] {
        assert!(claims_in(said).is_empty(), "{said}: {:?}", claims_in(said));
    }
}

#[test]
fn a_sentence_can_claim_two_kinds_and_the_tests_too() {
    let kinds = claims_in("Added CSRF tokens and rate limiting. All tests pass.");
    assert!(
        kinds.contains(&"forgery-token")
            && kinds.contains(&"rate-limiting")
            && kinds.contains(&TESTS_PASS),
        "{kinds:?}"
    );
}

#[test]
fn the_record_counts_each_kind_by_turn_and_keeps_none_of_the_words() {
    let dir = std::env::temp_dir().join(format!(
        "sv-hook-kinds-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("record.jsonl");
    let rules = sv_check::secrets::SecretRules::shared().expect("the rules sv ships");
    for (n, said) in [
        "Added rate limiting to sign-in, at /login.",
        "Fixed the XSS. Rate limiting is enforced now.",
        "Nothing else changed.",
    ]
    .iter()
    .enumerate()
    {
        let line = line_of(
            &json!({ "hook_event_name": "Stop", "session_id": "s", "last_assistant_message": said }),
            Path::new("/app"),
            &format!("T{n}"),
            rules,
        )
        .unwrap();
        assert!(
            !line.to_string().contains("/login"),
            "the tool's words are never kept"
        );
        append(&path, line).unwrap();
    }
    let seen = read(&path, None);
    let turns = |kind: &str| seen.said.iter().find(|s| s.kind == kind).map(|s| s.turns);
    assert_eq!(turns("rate-limiting"), Some(2));
    assert_eq!(turns("cross-site-scripting"), Some(1));
    assert_eq!(seen.turns, 3);
    let _ = std::fs::remove_dir_all(&dir);
}
