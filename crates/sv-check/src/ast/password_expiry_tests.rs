//! `ast.password-expiry-enforced` (backlog 0609, V6.2.10): a setting whose name says passwords expire after a time.
//! Only ever a finding, and low confidence: the name is what is read, not whether the app acts on it.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.password-expiry-enforced";

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether the rule reports `source`, read as `file`. A file the parser could not read, or a query that did not
/// compile, fails the test rather than reading as clean.
fn reported(language: &str, file: &str, source: &str) -> bool {
    let read = read_file(rules(), language, file, source);
    assert!(!read.parse_error, "{file} did not parse: {source}");
    assert!(
        read.broken.is_empty(),
        "a query did not compile: {:?}",
        read.broken
    );
    read.findings.iter().any(|f| f.rule_id == RULE)
}

fn check(language: &str, file: &str, wrap: impl Fn(&str) -> String, cases: &[(&str, bool)]) {
    for (code, expected) in cases {
        assert_eq!(
            reported(language, file, &wrap(code)),
            *expected,
            "{language}: `{code}` should{} be reported",
            if *expected { "" } else { " not" }
        );
    }
}

#[test]
fn the_rule_is_only_ever_a_finding_against_v6_2_10() {
    let rule = rules()
        .rules()
        .find(|r| r.id == RULE)
        .expect("the rule is shipped");
    assert_eq!(rule.requirement_ids, ["V6.2.10"]);
    assert!(rule.findings_only, "a clean result must credit nothing");
}

#[test]
fn an_expiry_setting_is_found_and_other_password_settings_are_not() {
    check(
        "python",
        "settings.py",
        |code| format!("{code}\n"),
        &[
            ("PASSWORD_EXPIRY_DAYS = 90", true),
            ("PASSWORD_MAX_AGE = 60 * 60 * 24 * 90", true),
            ("user.force_password_change = True", true),
            ("policy(password_rotation_days=30)", true),
            ("PASSWORD_MIN_LENGTH = 12", false),
            ("password_changed_at = now()", false),
            ("PASSWORD_RESET_TIMEOUT = 3600", false),
        ],
    );
    for (language, file) in [("javascript", "policy.js"), ("typescript", "policy.ts")] {
        check(
            language,
            file,
            |code| format!("{code}\n"),
            &[
                ("const passwordExpiryDays = 90;", true),
                ("const policy = { passwordMaxAge: 90 };", true),
                ("user.mustPasswordChange = true;", true),
                ("const passwordMinLength = 12;", false),
                ("const policy = { passwordHistory: 5 };", false),
            ],
        );
    }
}

#[test]
fn a_rust_expiry_setting_is_found_and_other_password_settings_are_not() {
    // Backlog 0661: a constant, a static, a `let`, a field set where a struct is made, and one assigned.
    check(
        "rust",
        "src/policy.rs",
        |code| format!("{code}\n"),
        &[
            ("const PASSWORD_EXPIRY_DAYS: u32 = 90;", true),
            (
                "static PASSWORD_MAX_AGE: Duration = Duration::from_secs(90 * 24 * 3600);",
                true,
            ),
            (
                "fn policy() -> Policy { Policy { password_rotation_days: 30, min_length: 12 } }",
                true,
            ),
            (
                "fn sign_in(user: &mut User) { user.force_password_change = true; }",
                true,
            ),
            ("fn f() { let password_lifetime = days(90); }", true),
            ("const PASSWORD_MIN_LENGTH: usize = 12;", false),
            (
                "fn f(user: &mut User) { user.password_changed_at = now(); }",
                false,
            ),
            ("const PASSWORD_RESET_TIMEOUT: u64 = 3600;", false),
            ("struct Policy { password_expiry_days: u32 }", false),
        ],
    );
}
