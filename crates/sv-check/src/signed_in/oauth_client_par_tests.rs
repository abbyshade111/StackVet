//! Pushed authorization requests (V10.4.13) and sign-outs without a hint (V10.6.2), asked of the app's own
//! authorization server through the test client (backlog 0625, part 7; ADR-151): a code for a request not pushed first,
//! and a sign-out no one confirmed, are found; a server that takes only pushed requests, or asks before signing a
//! person out, is credited in part; and an answer that says neither concludes nothing.

use super::oauth_client::logout::FORCED_LOGOUT;
use super::oauth_client::par::NOT_PUSHED_TAKEN;
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
fn a_code_for_a_request_not_pushed_first_is_found() {
    let (o, _) = with(Faults {
        publishes_par: true,
        ..Default::default()
    });
    let f = found(&o, NOT_PUSHED_TAKEN.rule_id).unwrap_or_else(|| panic!("{:?}", o.steps));
    assert_eq!(f.requirement_ids, ["V10.4.13"]);
    assert!(
        f.description
            .contains("publishes a pushed authorization request address, but does not require it"),
        "{}",
        f.description
    );
}

#[test]
fn a_server_that_takes_only_pushed_requests_is_credited_in_part_and_the_rest_is_said_unasked() {
    let (o, server) = with(Faults {
        requires_par: true,
        ..Default::default()
    });
    // The setup: the pushed request was made, and its `request_uri` taken at the authorization address.
    assert!(server.sent.iter().any(|r| r.id == "oauth-par"));
    assert!(server.sent.iter().any(|r| r.id == "oauth-authorize-pushed"));
    let credit =
        credited(&o, NOT_PUSHED_TAKEN.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(credit.requirement_ids, ["V10.4.13"]);
    assert!(found(&o, NOT_PUSHED_TAKEN.rule_id).is_none());
    // Every other requirement is said to be unasked, with the first request's refusal as the reason.
    assert!(
        o.not_assessed.iter().any(
            |(ids, _)| ids == "V10.4.1, V10.4.2, V10.4.5, V10.4.6, V10.4.10, V10.4.12, V10.6.2"
        ),
        "{:?}",
        o.not_assessed
    );
    // The pushed request's code is blanked like any other.
    for code in server.codes.keys() {
        assert!(o.issued.contains(code), "{code} not listed");
    }
}

#[test]
fn a_crash_on_a_request_not_pushed_first_is_not_a_refusal() {
    let (o, _) = with(Faults {
        crashes_unpushed: true,
        ..Default::default()
    });
    assert!(
        credited(&o, NOT_PUSHED_TAKEN.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        said(&o, "V10.4.13")
            .iter()
            .any(|w| w.contains("not a refusal `sv` can read")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_server_that_refuses_pushed_requests_too_is_not_credited() {
    let (o, server) = with(Faults {
        requires_par: true,
        refuses_pushed_too: true,
        ..Default::default()
    });
    // The setup: the pushed request was made and its `request_uri` sent.
    assert!(server.sent.iter().any(|r| r.id == "oauth-authorize-pushed"));
    assert!(
        credited(&o, NOT_PUSHED_TAKEN.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        said(&o, "V10.4.13")
            .iter()
            .any(|w| w.contains("so the refusal shows nothing")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_refused_request_with_no_pushed_address_to_try_concludes_nothing_about_par() {
    let (o, _) = with(Faults {
        consent: true,
        ..Default::default()
    });
    assert!(credited(&o, NOT_PUSHED_TAKEN.rule_id).is_none());
    assert!(
        said(&o, "V10.4.13")
            .iter()
            .any(|w| w.contains("publishes no pushed authorization request address")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_server_that_asks_before_signing_a_person_out_is_credited_in_part() {
    let (o, server) = with(Faults::default());
    for id in [
        "oidc-logout-no-hint",
        "oidc-logout-forged-hint",
        "oauth-session-after-oidc-logout-forged-hint",
    ] {
        assert!(server.sent.iter().any(|r| r.id == id), "{id} was not sent");
    }
    // The forged hint was sent, token-shaped, and is no real token.
    let forged = server
        .sent
        .iter()
        .find(|r| r.id == "oidc-logout-forged-hint")
        .unwrap();
    assert_eq!(
        forged
            .path
            .split("id_token_hint=")
            .nth(1)
            .map(|h| h.matches('.').count()),
        Some(2)
    );
    let credit =
        credited(&o, FORCED_LOGOUT.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(credit.requirement_ids, ["V10.6.2"]);
    assert!(!server.logged_out);
}

#[test]
fn a_sign_out_no_one_confirmed_is_found() {
    for (faults, said) in [
        (
            Faults {
                logs_out_without_hint: true,
                ..Default::default()
            },
            "with no `id_token_hint`",
        ),
        (
            Faults {
                takes_forged_hint: true,
                ..Default::default()
            },
            "with an `id_token_hint` `sv` made up",
        ),
    ] {
        let (o, server) = with(faults);
        let f = found(&o, FORCED_LOGOUT.rule_id).unwrap_or_else(|| panic!("{said}: {:?}", o.steps));
        assert_eq!(f.requirement_ids, ["V10.6.2"]);
        assert!(f.description.contains(said), "{}", f.description);
        assert!(server.logged_out);
    }
}

#[test]
fn an_answer_that_says_neither_signed_in_nor_out_concludes_nothing() {
    let (o, _) = with(Faults {
        unclear_after_logout: true,
        ..Default::default()
    });
    assert!(
        credited(&o, FORCED_LOGOUT.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(found(&o, FORCED_LOGOUT.rule_id).is_none());
    assert!(
        said(&o, "V10.6.2")
            .iter()
            .any(|w| w.contains("nothing was concluded")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn with_no_sign_out_address_the_sign_out_is_said_unasked() {
    let (o, server) = with(Faults {
        no_end_session: true,
        ..Default::default()
    });
    assert!(!server.sent.iter().any(|r| r.id.starts_with("oidc-logout")));
    assert!(
        said(&o, "V10.6.2")
            .iter()
            .any(|w| w.contains("end_session_endpoint")),
        "{:?}",
        o.not_assessed
    );
}
