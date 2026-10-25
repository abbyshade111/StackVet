//! PKCE (V10.4.6) and refresh tokens used twice (V10.4.5), asked of the app's own authorization server through the test
//! client (backlog 0625, part 7; ADR-151): each way of letting it through is found, a server that holds to both is
//! credited in part, and an answer that is no refusal, or a client the requirement is not about, concludes nothing.

use super::oauth_client::*;
use super::oauth_client_fake::*;
use super::*;

fn public(faults: Faults) -> Outcome {
    run_with_client(
        Faults {
            public: true,
            ..faults
        },
        Some(&public_client()),
    )
    .0
}

fn said<'a>(o: &'a Outcome, id: &str) -> Vec<&'a str> {
    o.not_assessed
        .iter()
        .filter(|(ids, _)| ids == id)
        .map(|(_, why)| why.as_str())
        .collect()
}

#[test]
fn a_server_that_holds_codes_to_pkce_is_credited_in_part() {
    let (o, server) = run_with_client(Faults::default(), Some(&client()));
    // The setup: the three wrong requests were sent, and a code exchanged after them.
    for id in [
        "oauth-authorize-no-pkce",
        "oauth-authorize-plain-pkce",
        "oauth-token-wrong-verifier",
        "oauth-token-pkce-after",
    ] {
        assert!(server.sent.iter().any(|r| r.id == id), "{id} was not sent");
    }
    let pkce =
        credited(&o, PKCE_NOT_REQUIRED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(pkce.requirement_ids, ["V10.4.6"]);
    assert!(found(&o, PKCE_NOT_REQUIRED.rule_id).is_none());
}

#[test]
fn each_way_round_pkce_is_found() {
    for (faults, said) in [
        (
            Faults {
                pkce_optional: true,
                ..Default::default()
            },
            "a code was given when asked for with no PKCE challenge",
        ),
        (
            Faults {
                plain_accepted: true,
                ..Default::default()
            },
            "a code was given when asked for with PKCE's `plain` method",
        ),
        (
            Faults {
                verifier_unchecked: true,
                ..Default::default()
            },
            "a code was exchanged for a token with the wrong verifier",
        ),
    ] {
        let (o, _) = run_with_client(faults, Some(&client()));
        let f =
            found(&o, PKCE_NOT_REQUIRED.rule_id).unwrap_or_else(|| panic!("{said}: {:?}", o.steps));
        assert_eq!(f.requirement_ids, ["V10.4.6"]);
        assert!(f.description.contains(said), "{}", f.description);
        assert!(credited(&o, PKCE_NOT_REQUIRED.rule_id).is_none());
    }
}

#[test]
fn refusals_from_a_server_that_then_exchanges_no_code_credit_no_pkce() {
    let (o, server) = run_with_client(
        Faults {
            stops_after_wrong_verifier: true,
            ..Default::default()
        },
        Some(&client()),
    );
    // The setup: the exchange after the wrong requests was sent.
    assert!(server.sent.iter().any(|r| r.id == "oauth-token-pkce-after"));
    assert!(
        credited(&o, PKCE_NOT_REQUIRED.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        said(&o, "V10.4.6")
            .iter()
            .any(|w| w.starts_with("After the wrong PKCE requests were refused")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_crash_on_a_request_without_pkce_is_not_a_refusal() {
    let (o, _) = run_with_client(
        Faults {
            crashes_without_pkce: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        credited(&o, PKCE_NOT_REQUIRED.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        said(&o, "V10.4.6")
            .iter()
            .any(|w| w.contains("no answer that can be read as a refusal")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_limiter_on_a_refresh_token_sent_again_is_not_a_refusal() {
    let o = public(Faults {
        limits_refresh_replay: true,
        ..Default::default()
    });
    assert!(
        credited(&o, REFRESH_REPLAYED.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(found(&o, REFRESH_REPLAYED.rule_id).is_none());
    assert!(
        said(&o, "V10.4.5")
            .iter()
            .any(|w| w.contains("answered 429")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_rotating_server_that_revokes_the_sign_in_on_replay_is_credited_in_part() {
    let o = public(Faults::default());
    let refresh =
        credited(&o, REFRESH_REPLAYED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(refresh.requirement_ids, ["V10.4.5"]);
    assert!(
        refresh.scope.contains("refused the newer one too"),
        "{}",
        refresh.scope
    );
}

#[test]
fn a_refresh_token_taken_twice_is_found() {
    let o = public(Faults {
        refresh_reusable: true,
        ..Default::default()
    });
    let f = found(&o, REFRESH_REPLAYED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(
        f.title,
        "The authorization server takes a refresh token again after it was used"
    );
}

#[test]
fn a_newer_refresh_token_left_working_after_a_replay_is_found() {
    let o = public(Faults {
        keeps_newer_refresh: true,
        ..Default::default()
    });
    let f = found(&o, REFRESH_REPLAYED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(
        f.title,
        "The authorization server refuses a used refresh token, but leaves the newer one working"
    );
}

#[test]
fn without_rotation_a_refused_replay_is_credited_without_the_revocation() {
    let o = public(Faults {
        no_rotation: true,
        ..Default::default()
    });
    let refresh =
        credited(&o, REFRESH_REPLAYED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert!(refresh.scope.contains("was not asked"), "{}", refresh.scope);
}

#[test]
fn a_confidential_client_is_not_asked_about_refresh_tokens() {
    let (o, server) = run_with_client(Faults::default(), Some(&client()));
    assert!(!server.sent.iter().any(|r| r.id == "oauth-refresh"));
    assert!(credited(&o, REFRESH_REPLAYED.rule_id).is_none());
    assert!(
        said(&o, "V10.4.5")
            .iter()
            .any(|w| w.contains("public clients")),
        "{:?}",
        o.not_assessed
    );
}
