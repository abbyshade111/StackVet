//! `ast.backup-codes-stored-plain` (backlog 0609, V6.5.2): a field for the backup or recovery codes given them straight
//! from a random generator, with no hashing in the same line. Only ever a finding: codes hashed in another line, or made
//! in a variable and hashed before they are saved, are not seen, and a clean result shows nothing.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.backup-codes-stored-plain";

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

fn check(language: &str, file: &str, cases: &[(&str, bool)]) {
    for (code, expected) in cases {
        assert_eq!(
            reported(language, file, &format!("{code}\n")),
            *expected,
            "{language}: `{code}` should{} be reported",
            if *expected { "" } else { " not" }
        );
    }
}

#[test]
fn the_rule_is_only_ever_a_finding_against_v6_5_2() {
    let rule = rules()
        .rules()
        .find(|r| r.id == RULE)
        .expect("the rule is shipped");
    assert_eq!(rule.requirement_ids, ["V6.5.2"]);
    assert!(rule.findings_only, "a clean result must credit nothing");
}

#[test]
fn codes_saved_straight_from_a_generator_are_found_and_hashed_ones_are_not() {
    check(
        "python",
        "accounts/models.py",
        &[
            (
                "user.backup_codes = [secrets.token_hex(4) for _ in range(10)]",
                true,
            ),
            ("profile.recovery_codes = secrets.token_urlsafe(8)", true),
            (
                "User(email=email, backup_codes=[str(uuid.uuid4()) for _ in range(8)])",
                true,
            ),
            // Hashed in the same line, made elsewhere, or a field of another name: not this.
            (
                "user.backup_codes = [make_password(secrets.token_hex(4)) for _ in range(10)]",
                false,
            ),
            ("user.backup_codes = [hash_code(c) for c in codes]", false),
            ("user.backup_codes = codes", false),
            ("user.api_key = secrets.token_hex(16)", false),
            // Made in a variable, to be shown once and hashed before saving: the safe way, not read.
            (
                "backup_codes = [secrets.token_hex(4) for _ in range(10)]",
                false,
            ),
        ],
    );
    for language in ["javascript", "typescript"] {
        check(
            language,
            "src/account.ts",
            &[
                (
                    "user.backupCodes = codes.map(() => crypto.randomBytes(4).toString('hex'));",
                    true,
                ),
                (
                    "save({ recoveryCodes: Array.from({ length: 10 }, () => randomUUID()) });",
                    true,
                ),
                (
                    "user.backupCodes = await Promise.all(codes.map((c) => bcrypt.hash(c, 10)));",
                    false,
                ),
                ("user.backupCodes = codes;", false),
                ("user.sessionId = crypto.randomUUID();", false),
            ],
        );
    }
}
