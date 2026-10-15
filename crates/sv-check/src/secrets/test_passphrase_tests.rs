//! Backlog 0529, the owner's answer (b) of 15 October 2026: a credential assignment in a test file whose value is
//! made of words is test information, not a finding; a random-looking value in a test file still counts, as ADR-091
//! ruled; and the same words in the app's own code count wherever they are.

use super::*;

/// A value with no words in it, made at run time so no key-shaped literal is in the source (CLAUDE.md).
fn random_looking() -> String {
    let alphabet = b"aZ3kQ9pX7mW2vR5tN8";
    let seed = std::process::id() as usize;
    (0..24)
        .map(|i| char::from(alphabet[(seed + i * 7) % alphabet.len()]))
        .collect()
}

fn one(relative: &str, line: &str) -> Finding {
    let found = assignment_findings(relative, line, 1, &(0..usize::MAX));
    assert_eq!(found.len(), 1, "the setup reports the line: {found:?}");
    found.into_iter().next().unwrap()
}

#[test]
fn the_trial_s_test_passphrase_is_information() {
    let f = one(
        "tests/conftest.py",
        r#"PASSWORD = "violet-canoe-marble-ledger""#,
    );
    assert!(f.in_test_code(), "the setup: a tests folder");
    assert!(f.is_test_information(), "{f:?}");
    assert!(
        f.description
            .contains("a test password reused elsewhere is still a risk"),
        "{}",
        f.description
    );
    assert!(
        !f.description.contains("violet"),
        "the value is never in the words"
    );
}

#[test]
fn a_random_looking_value_in_a_test_file_still_counts() {
    let value = random_looking();
    assert!(
        !reads_as_words(&value),
        "the setup: {} characters with no words",
        value.len()
    );
    let f = one("tests/conftest.py", &format!("API_KEY = \"{value}\""));
    assert!(f.in_test_code());
    assert!(
        !f.is_test_information(),
        "ADR-091: a real-looking key in a test file counts"
    );
}

#[test]
fn the_same_passphrase_in_the_app_s_own_code_counts() {
    let f = one(
        "app/settings.py",
        r#"PASSWORD = "violet-canoe-marble-ledger""#,
    );
    assert!(!f.in_test_code());
    assert!(!f.is_test_information());
    assert!(!f.description.contains("test file"), "{}", f.description);
}

#[test]
fn another_tool_on_the_same_line_keeps_it_counted_unless_it_is_bandit_s_own_test_rule() {
    let mut f = one(
        "tests/conftest.py",
        r#"PASSWORD = "violet-canoe-marble-ledger""#,
    );
    f.also_reported_by.push("bandit.B105".into());
    assert!(
        f.is_test_information(),
        "Bandit's B105 is information in test code too"
    );
    f.also_reported_by.push("semgrep.generic.hardcoded".into());
    assert!(!f.is_test_information(), "another tool's finding counts");
}

#[test]
fn what_reads_as_words() {
    for words in [
        "violet-canoe-marble-ledger",
        "TestPassword",
        "correct horse battery staple",
        "test_user_password",
    ] {
        assert!(reads_as_words(words), "{words}");
    }
    for not_words in [
        "hunter2",
        "Qz-Xk-Lm-Pv",
        "kqzvmwpt-xrtqzv",
        "password",
        "sk_live_abc",
        "pass.word!",
    ] {
        assert!(!reads_as_words(not_words), "{not_words}");
    }
}
