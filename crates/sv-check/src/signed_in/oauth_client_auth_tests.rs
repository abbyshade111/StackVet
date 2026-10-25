//! Client authentication (V10.4.10) and response mode (V10.4.12), asked of the app's own authorization server through
//! the test client (backlog 0625, part 7; ADR-151): each way of letting a request through is found, a server that holds
//! to both is credited in part, and an answer that is no refusal, a server that then stops answering, or a client the
//! requirement is not about, concludes nothing.

use super::oauth_client::client_auth::CLIENT_NOT_AUTHENTICATED;
use super::oauth_client::response_mode::RESPONSE_MODE_NOT_HELD;
use super::oauth_client_fake::*;
use super::*;

fn with(faults: Faults) -> (Outcome, Server) {
    run_with_client(faults, Some(&client()))
}

fn said<'a>(o: &'a Outcome, id: &str) -> Vec<&'a str> {
    o.not_assessed
        .iter()
        .filter(|(ids, _)| ids == id)
        .map(|(_, why)| why.as_str())
        .collect()
}

#[test]
fn a_server_that_authenticates_a_confidential_client_is_credited_in_part() {
    let (o, server) = with(Faults::default());
    // The setup: both exchanges without the right secret were sent, and one with it after.
    for id in [
        "oauth-token-no-client-auth",
        "oauth-token-wrong-secret",
        "oauth-token-client-auth-after",
    ] {
        assert!(server.sent.iter().any(|r| r.id == id), "{id} was not sent");
    }
    let credit = credited(&o, CLIENT_NOT_AUTHENTICATED.rule_id)
        .unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(credit.requirement_ids, ["V10.4.10"]);
    assert!(
        credit.scope.contains("revocation requests were not"),
        "{}",
        credit.scope
    );
}

#[test]
fn a_code_exchanged_without_the_right_secret_is_found() {
    for (faults, said) in [
        (
            Faults {
                takes_no_client_auth: true,
                ..Default::default()
            },
            "with no client authentication",
        ),
        (
            Faults {
                takes_any_secret: true,
                ..Default::default()
            },
            "with the wrong secret",
        ),
    ] {
        let (o, _) = with(faults);
        let f = found(&o, CLIENT_NOT_AUTHENTICATED.rule_id)
            .unwrap_or_else(|| panic!("{said}: {:?}", o.steps));
        assert_eq!(f.requirement_ids, ["V10.4.10"]);
        assert!(f.description.contains(said), "{}", f.description);
    }
}

#[test]
fn refusals_from_a_server_that_then_exchanges_nothing_credit_no_client_authentication() {
    let (o, server) = with(Faults {
        stops_after_unauthenticated: true,
        ..Default::default()
    });
    assert!(
        server
            .sent
            .iter()
            .any(|r| r.id == "oauth-token-client-auth-after")
    );
    assert!(
        credited(&o, CLIENT_NOT_AUTHENTICATED.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        said(&o, "V10.4.10")
            .iter()
            .any(|w| w.starts_with("After the exchanges without the right secret")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_limiter_on_an_unauthenticated_exchange_is_not_a_refusal() {
    let (o, _) = with(Faults {
        limits_unauthenticated: true,
        ..Default::default()
    });
    assert!(
        credited(&o, CLIENT_NOT_AUTHENTICATED.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(found(&o, CLIENT_NOT_AUTHENTICATED.rule_id).is_none());
    assert!(
        said(&o, "V10.4.10")
            .iter()
            .any(|w| w.contains("neither a token nor a refusal")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_public_client_is_not_asked_about_client_authentication() {
    let (o, server) = run_with_client(
        Faults {
            public: true,
            ..Default::default()
        },
        Some(&public_client()),
    );
    assert!(
        !server
            .sent
            .iter()
            .any(|r| r.id == "oauth-token-no-client-auth")
    );
    assert!(
        said(&o, "V10.4.10")
            .iter()
            .any(|w| w.contains("confidential clients")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_server_that_keeps_the_client_to_the_query_is_credited_in_part() {
    let (o, server) = with(Faults::default());
    for id in [
        "oauth-authorize-response-mode-fragment",
        "oauth-authorize-response-mode-form-post",
        "oauth-authorize-response-mode-after",
    ] {
        assert!(server.sent.iter().any(|r| r.id == id), "{id} was not sent");
    }
    let credit = credited(&o, RESPONSE_MODE_NOT_HELD.rule_id)
        .unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(credit.requirement_ids, ["V10.4.12"]);
}

#[test]
fn a_code_sent_back_another_way_is_found_and_listed_for_blanking() {
    for (faults, said) in [
        (
            Faults {
                fragment_mode: true,
                ..Default::default()
            },
            "in the redirect address's fragment",
        ),
        (
            Faults {
                form_post_mode: true,
                ..Default::default()
            },
            "in a form the page posts to the redirect address",
        ),
    ] {
        let (o, server) = with(faults);
        let f = found(&o, RESPONSE_MODE_NOT_HELD.rule_id)
            .unwrap_or_else(|| panic!("{said}: {:?}", o.steps));
        assert_eq!(f.requirement_ids, ["V10.4.12"]);
        assert!(f.description.contains(said), "{}", f.description);
        // The code that came back that way is blanked like the others.
        for code in server.codes.keys() {
            assert!(o.issued.contains(code), "{said}: {code} not listed");
        }
    }
}

#[test]
fn a_crash_on_another_response_mode_is_not_a_refusal() {
    let (o, _) = with(Faults {
        crashes_on_response_mode: true,
        ..Default::default()
    });
    assert!(
        credited(&o, RESPONSE_MODE_NOT_HELD.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        said(&o, "V10.4.12")
            .iter()
            .any(|w| w.contains("no answer that can be read as a refusal")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn refusals_from_a_server_that_then_refuses_every_code_credit_no_response_mode() {
    let (o, _) = with(Faults {
        stops_after_response_mode: true,
        ..Default::default()
    });
    assert!(
        credited(&o, RESPONSE_MODE_NOT_HELD.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        said(&o, "V10.4.12")
            .iter()
            .any(|w| w.starts_with("After the other response modes were refused")),
        "{:?}",
        o.not_assessed
    );
}
