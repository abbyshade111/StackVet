//! A test client `sv` registers itself (backlog 0625, part 7; ADR-151): when stackvet.toml declares none and the
//! server publishes a registration address, the checks run with a client registered there, its secret is blanked with
//! what the server issues, and a registration refused, or a consent page in front of the registered client, is said.

use super::oauth_client::*;
use super::oauth_client_fake::*;
use super::*;

fn undeclared(faults: Faults) -> (Outcome, Server) {
    run_with_client(faults, None)
}

#[test]
fn with_no_test_client_declared_one_is_registered_and_the_checks_run() {
    let (o, server) = undeclared(Faults {
        registers: true,
        ..Default::default()
    });
    // The setup: the registration was made, with `sv`'s own redirect address.
    assert!(server.sent.iter().any(|r| r.id == "oauth-register"));
    assert_eq!(
        server.registered_redirect.as_deref(),
        Some("http://localhost/sv-callback")
    );
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("registered a test client of `sv`'s own")),
        "{:?}",
        o.steps
    );
    // The checks ran with it: the redirect check is credited, and the client is confidential.
    assert!(
        credited(&o, REDIRECT_NOT_EXACT.rule_id).is_some(),
        "{:?}",
        o.not_assessed
    );
    // Its secret and its management token are blanked with what the server issued.
    assert!(o.issued.contains(&secret()));
    assert!(o.issued.contains(&registration_token()));
}

#[test]
fn a_registration_refused_says_why_and_how_to_declare_a_client() {
    let (o, _) = undeclared(Faults {
        registration_needs_token: true,
        ..Default::default()
    });
    let (ids, why) = &o.not_assessed[0];
    assert_eq!(ids, ALL);
    assert!(why.contains("initial access token"), "{why}");
    assert!(why.contains("[stack.run.oauth-client]"), "{why}");
    assert!(o.verified.is_empty() && o.findings.is_empty());
}

#[test]
fn a_registered_client_that_meets_the_consent_page_is_said_to() {
    let (o, server) = undeclared(Faults {
        registers: true,
        consent_for_registered: true,
        ..Default::default()
    });
    assert!(server.sent.iter().any(|r| r.id == "oauth-register"));
    assert!(
        o.verified.is_empty() && o.findings.is_empty(),
        "{:?} {:?}",
        o.verified,
        o.findings
    );
    assert!(
        o.not_assessed.iter().any(|(_, why)| why
            .starts_with("`sv` registered a test client of its own")
            && why.contains("consent page")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn with_no_registration_address_the_note_says_so() {
    let (o, server) = undeclared(Faults::default());
    assert!(!server.sent.iter().any(|r| r.id == "oauth-register"));
    assert!(
        o.not_assessed[0]
            .1
            .contains("publishes no registration address"),
        "{:?}",
        o.not_assessed
    );
}
