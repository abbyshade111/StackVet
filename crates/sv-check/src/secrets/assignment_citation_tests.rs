//! The credential-assignment finding cites what a hard-coded key fails, V13.3.1 and SBD-AC-05, and not
//! V13.2.3 (no default credentials): the rule passes over `admin`, `password`, and every value that reads
//! as a word or has little entropy, so what it finds is never a default credential (backlog 0431).

use super::*;

fn rules() -> SecretRules {
    SecretRules::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/secret-rules.json"),
    )
    .expect("rules load")
}

#[test]
fn a_hard_coded_key_cites_secrets_in_code_and_not_default_credentials() {
    let rules = rules();
    // A random value no vendor's shape describes, built from pieces so this file holds none.
    let value = ["Wq8zR3", "kT5vN1", "pL7xC4"].concat();
    let found = scan_text(&rules, "settings.py", &format!("API_KEY = \"{value}\"\n"));
    // The setup: the assignment rule found it.
    let finding = found
        .iter()
        .find(|f| f.rule_id == ASSIGNMENT_RULE)
        .unwrap_or_else(|| panic!("the setup: the assignment rule found nothing: {found:#?}"));
    for cited in ["V13.3.1", "SBD-AC-05"] {
        assert!(
            finding.requirement_ids.iter().any(|r| r == cited),
            "{finding:#?}"
        );
    }
    assert!(
        finding.requirement_ids.iter().all(|r| r != "V13.2.3"),
        "{:?}",
        finding.requirement_ids
    );
    // And a stock default under a credential's name is not what this rule finds.
    let default = scan_text(&rules, "settings.py", "PASSWORD = \"admin\"\n");
    assert!(
        default.iter().all(|f| f.rule_id != ASSIGNMENT_RULE),
        "{default:#?}"
    );
}
