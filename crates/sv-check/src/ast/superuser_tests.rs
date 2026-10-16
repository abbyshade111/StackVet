//! `ast.database-as-superuser` (backlog 0500, V13.2.2): a connection address written into the code that signs in to
//! the database as its built-in administrator. Only ever a finding: an address kept in a setting is not read.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.database-as-superuser";

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
fn the_rule_is_only_ever_a_finding_against_v13_2_2() {
    let rule = rules()
        .rules()
        .find(|r| r.id == RULE)
        .expect("the rule is shipped");
    assert_eq!(rule.requirement_ids, ["V13.2.2"]);
    assert!(rule.findings_only, "a clean result must credit nothing");
}

#[test]
fn an_administrator_in_a_connection_address_is_found_and_an_app_account_is_not() {
    for (language, file, wrap) in [
        ("python", "settings.py", "DB = {code}\n"),
        ("javascript", "db.js", "const db = {code};\n"),
        ("typescript", "db.ts", "const db: string = {code};\n"),
    ] {
        check(
            language,
            file,
            |code| wrap.replace("{code}", code),
            &[
                ("'postgresql://postgres:pw@db:5432/notes'", true),
                ("'postgres://postgres@localhost/notes'", true),
                ("'mysql://root:pw@db/notes'", true),
                ("'mysql+pymysql://root@db/notes'", true),
                ("'mssql://sa:pw@db/notes'", true),
                ("'mongodb+srv://admin:pw@cluster0.example.net/notes'", true),
                ("'postgresql://notes_app:pw@db/notes'", false),
                ("'postgresql://db/notes'", false),
                ("'mysql://rooted:pw@db/notes'", false),
                ("'https://root@example.com'", false),
            ],
        );
    }
}
