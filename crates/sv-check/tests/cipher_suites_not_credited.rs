//! V12.1.2 asks that only recommended cipher suites are enabled on the live site, the strongest
//! preferred. Semgrep's two cipher rules read a cipher list written into the app's own code, and most
//! apps write none: TLS ends at a proxy or a hosting platform. A clean run of them credited V12.1.2 for
//! any Go or Python app, beside the hand check that says `sv` does not check it (the owner's decision of
//! 6 October 2026, backlog 0147). They are only ever a finding now (backlog 0434; ADR-018, Later).

use std::collections::BTreeSet;
use std::path::PathBuf;
use sv_check::adapters::{self, Adapters};

const CIPHER_RULES: [&str; 2] = [
    "go.lang.security.audit.crypto.tls.tls-with-insecure-cipher",
    "python.lang.security.audit.insecure-transport.ssl.no-set-ciphers.no-set-ciphers",
];

#[test]
fn a_clean_run_of_the_cipher_rules_credits_nothing_and_a_finding_still_names_v12_1_2() {
    let adapters =
        Adapters::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json"))
            .expect("the adapter file loads");
    let semgrep = adapters
        .all()
        .iter()
        .find(|a| a.id == "semgrep")
        .expect("the setup: semgrep is an adapter");
    // The setup: both rules are in the map, and each still says what it finds against.
    for rule in CIPHER_RULES {
        let mapped = semgrep
            .rules
            .get(rule)
            .unwrap_or_else(|| panic!("the setup: {rule} is not in the map"));
        assert_eq!(mapped.findings_against, ["V12.1.2"], "{rule}");
    }
    let loaded: BTreeSet<String> = CIPHER_RULES.iter().map(|r| (*r).to_owned()).collect();
    let credited = adapters::clean_run_evidence(
        semgrep,
        &loaded,
        &["go".to_owned(), "python".to_owned()],
        &["main.go".to_owned(), "app.py".to_owned()],
    );
    assert!(
        !credited.iter().any(|r| r == "V12.1.2"),
        "a clean run credited {credited:?}"
    );
}
