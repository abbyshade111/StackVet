//! `pass` as a whole word of a name says it holds a password (backlog 0315): `DB_PASS`,
//! `SMTP_PASS`, `dbPass`. A word that only holds the letters, `BYPASS` or `PASSPORT`, does not.

use super::*;

fn rules() -> SecretRules {
    SecretRules::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/secret-rules.json"),
    )
    .expect("rules load")
}

/// A random value no vendor's shape describes, built from pieces so this file holds none.
fn value() -> String {
    ["Wq8zR3", "kT5vN1", "pL7xC4"].concat()
}

#[test]
fn a_password_under_a_name_ending_in_pass_is_found_and_masked() {
    let rules = rules();
    let value = value();
    for (relative, text) in [
        ("config.yml", format!("smtp:\n  SMTP_PASS: \"{value}\"\n")),
        ("app.js", format!("const dbPass = \"{value}\";\n")),
        ("settings.py", format!("REDIS_PASS = \"{value}\"\n")),
    ] {
        let found = scan_text(&rules, relative, &text);
        assert_eq!(found.len(), 1, "{relative}: {found:#?}");
        let (out, n) = redact_text(&rules, &text);
        assert!(!out.contains(&value) && n == 1, "{relative}: {out}");
    }
}

#[test]
fn a_dot_env_line_ending_in_pass_is_masked_where_sv_repeats_it() {
    // `.env` is where credentials belong, so the scan reports no assignment in it, `DB_PASSWORD`
    // included; a line of it repeated elsewhere (a log tail, a tool's output) is masked.
    let rules = rules();
    let value = value();
    for name in ["DB_PASS", "DB_PASSWORD"] {
        let text = format!("{name}={value}\n");
        assert!(scan_text(&rules, ".env", &text).is_empty(), "{name}");
        let (out, n) = redact_text(&rules, &text);
        assert!(!out.contains(&value) && n == 1, "{name}: {out}");
    }
}

#[test]
fn a_name_that_only_holds_the_letters_pass_is_not_a_password() {
    let rules = rules();
    let value = value();
    for name in [
        "BYPASS_MODE",
        "PASSPORT_NUMBER",
        "COMPASS_HEADING",
        "passengerCount",
    ] {
        let text = format!("{name}={value}\n");
        let found = scan_text(&rules, ".env", &text);
        assert!(found.is_empty(), "{name}: {found:#?}");
        let (out, n) = redact_text(&rules, &text);
        assert_eq!((out.as_str(), n), (text.as_str(), 0), "{name}");
    }
}
