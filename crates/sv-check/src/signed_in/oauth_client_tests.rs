//! The app's own authorization server, asked through the test client (backlog 0625, part 7; ADR-151): a redirect
//! address matched by its start, or a code taken twice, is found; refused, with the normal case working, is credited
//! in part; and every way of not getting that far is said, crediting nothing.

use super::fake_app::accounts;
use super::oauth_client::*;
use super::oauth_client_fake::*;
use super::*;
use std::collections::BTreeSet;

#[test]
fn a_server_that_compares_redirect_addresses_whole_and_refuses_a_used_code_is_credited_in_part() {
    let (o, server) = run_with_client(Faults::default(), Some(&client()));
    // The setup worked: a code came back, and it was exchanged.
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("the authorization server sent the browser back")),
        "{:?}",
        o.steps
    );
    assert!(
        server.sent.iter().any(|r| r.id == "oauth-token"),
        "{:?}",
        o.not_assessed
    );
    let redirect =
        credited(&o, REDIRECT_NOT_EXACT.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(redirect.requirement_ids, ["V10.4.1"]);
    assert!(
        redirect
            .scope
            .contains("refused 3 changed redirect addresses"),
        "{}",
        redirect.scope
    );
    let code = credited(&o, CODE_REUSED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(code.requirement_ids, ["V10.4.2"]);
    assert!(
        code.scope.contains("publishes no introspection address"),
        "{}",
        code.scope
    );
    // Only the request not pushed first (V10.4.13), which a server that does not require pushed requests always shows.
    assert!(
        o.findings.iter().all(|f| f.requirement_ids == ["V10.4.13"]),
        "{:?}",
        o.findings
    );
}

#[test]
fn a_redirect_address_matched_by_its_start_is_found() {
    let (o, _) = run_with_client(
        Faults {
            prefix_match: true,
            ..Default::default()
        },
        Some(&client()),
    );
    let f = found(&o, REDIRECT_NOT_EXACT.rule_id).unwrap_or_else(|| panic!("{:?}", o.steps));
    assert_eq!(f.requirement_ids, ["V10.4.1"]);
    assert!(
        f.description.contains("http://localhost/callbacksv"),
        "{}",
        f.description
    );
    assert!(
        f.description.contains("http://localhost/callback?sv=1"),
        "{}",
        f.description
    );
    assert!(credited(&o, REDIRECT_NOT_EXACT.rule_id).is_none());
}

#[test]
fn a_redirect_host_matched_by_its_start_is_found() {
    let (o, _) = run_with_client(
        Faults {
            host_start_match: true,
            ..Default::default()
        },
        Some(&client()),
    );
    let f = found(&o, REDIRECT_NOT_EXACT.rule_id).unwrap_or_else(|| panic!("{:?}", o.steps));
    assert!(
        f.description
            .contains("http://localhost.sv-redirect.invalid/callback"),
        "{}",
        f.description
    );
}

#[test]
fn refusals_from_a_server_that_then_refuses_the_registered_address_too_credit_nothing() {
    let (o, _) = run_with_client(
        Faults {
            stops_after_a_change: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        credited(&o, REDIRECT_NOT_EXACT.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(found(&o, REDIRECT_NOT_EXACT.rule_id).is_none());
    assert!(
        o.not_assessed.iter().any(|(ids, why)| ids == "V10.4.1"
            && why.starts_with("After the changed redirect addresses were refused")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_crash_on_a_changed_redirect_address_is_not_a_refusal() {
    let (o, _) = run_with_client(
        Faults {
            crashes_on_a_change: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        credited(&o, REDIRECT_NOT_EXACT.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        o.not_assessed.iter().any(|(ids, why)| ids == "V10.4.1"
            && why.contains("no answer that can be read as a refusal")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_limiter_on_the_second_exchange_is_not_a_refusal() {
    let (o, _) = run_with_client(
        Faults {
            limits_the_second: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        credited(&o, CODE_REUSED.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(found(&o, CODE_REUSED.rule_id).is_none());
    assert!(
        o.not_assessed
            .iter()
            .any(|(ids, why)| ids == "V10.4.2" && why.contains("answered 429")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_code_exchanged_twice_is_found() {
    let (o, _) = run_with_client(
        Faults {
            code_twice: true,
            ..Default::default()
        },
        Some(&client()),
    );
    let f = found(&o, CODE_REUSED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(f.requirement_ids, ["V10.4.2"]);
    assert_eq!(
        f.title,
        "The authorization server gives tokens for a sign-in code already used"
    );
    assert!(credited(&o, CODE_REUSED.rule_id).is_none());
}

#[test]
fn with_an_introspection_address_the_first_token_is_asked_after_the_code_comes_back() {
    let introspects = Faults {
        introspects: true,
        ..Default::default()
    };
    let (o, server) = run_with_client(introspects, Some(&client()));
    assert!(server.sent.iter().any(|r| r.id == "oauth-introspect"));
    let code = credited(&o, CODE_REUSED.rule_id).unwrap_or_else(|| panic!("{:?}", o.findings));
    assert!(code.scope.contains("no longer active"), "{}", code.scope);

    let (o, _) = run_with_client(
        Faults {
            keeps_tokens: true,
            ..introspects
        },
        Some(&client()),
    );
    let f = found(&o, CODE_REUSED.rule_id).unwrap_or_else(|| panic!("{:?}", o.verified));
    assert_eq!(
        f.title,
        "The authorization server refuses a sign-in code used twice, but leaves its tokens working"
    );
}

#[test]
fn a_consent_page_concludes_nothing_and_says_how_to_skip_it() {
    let (o, _) = run_with_client(
        Faults {
            consent: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        o.verified.is_empty() && o.findings.is_empty(),
        "{:?} {:?}",
        o.verified,
        o.findings
    );
    let (ids, why) = &o.not_assessed[0];
    // The first request's refusal says it for all but V10.4.13, which the server's published details answer.
    assert_eq!(
        ids,
        "V10.4.1, V10.4.2, V10.4.5, V10.4.6, V10.4.10, V10.4.12, V10.6.2"
    );
    assert!(
        why.contains("consent page") && why.contains("skip_authorization=True"),
        "{why}"
    );
}

#[test]
fn without_a_test_client_an_authorization_server_is_told_both_were_unasked() {
    let (o, server) = run_with_client(Faults::default(), None);
    // Only the published details were asked: no sign-in, no code.
    assert!(
        server
            .sent
            .iter()
            .all(|r| r.path.starts_with("/.well-known/")),
        "{:?}",
        server.sent.iter().map(|r| &r.path).collect::<Vec<_>>()
    );
    let (ids, why) = &o.not_assessed[0];
    assert_eq!(ids, ALL);
    assert!(why.contains("[stack.run.oauth-client]"), "{why}");
}

#[test]
fn without_a_test_client_an_app_that_is_no_authorization_server_is_told_nothing() {
    let mut app = Server::new(Faults {
        publishes_nothing: true,
        ..Default::default()
    });
    let mut out = Outcome::default();
    authorization_server_checks(&mut app, &plain_users(), &accounts(), None, &mut out);
    assert!(out.not_assessed.is_empty(), "{:?}", out.not_assessed);
    assert!(out.verified.is_empty() && out.findings.is_empty() && out.steps.is_empty());
}

#[test]
fn a_secret_written_into_stackvet_toml_is_not_used_or_repeated() {
    let written = sv_manifest::OauthClientSection {
        secret: Some(secret()),
        ..client()
    };
    let (o, server) = run_with_client(Faults::default(), Some(&written));
    assert!(server.sent.is_empty());
    let (_, why) = &o.not_assessed[0];
    assert!(why.contains("written into stackvet.toml"), "{why}");
    assert!(!why.contains(&secret()), "{why}");
}

#[test]
fn a_secret_the_seed_did_not_print_is_said_so() {
    let unprinted = sv_manifest::OauthClientSection {
        secret: Some("{seed:OTHER}".into()),
        ..client()
    };
    let (o, server) = run_with_client(Faults::default(), Some(&unprinted));
    assert!(server.sent.is_empty());
    assert!(
        o.not_assessed[0]
            .1
            .contains("printed no value of that name"),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn every_code_and_token_the_server_gave_is_listed_for_blanking() {
    // A public client, so the refresh tokens' check runs too and its tokens are counted.
    let (o, server) = run_with_client(
        Faults {
            public: true,
            ..Default::default()
        },
        Some(&public_client()),
    );
    let issued: BTreeSet<&str> = o.issued.iter().map(String::as_str).collect();
    // The setup: every check got as far as its codes and tokens, refresh tokens among them: five codes (the first, the
    // one after the changed addresses, the wrong verifier's, the one after it, and the refresh check's) and eight tokens.
    assert!(
        server.codes.len() >= 5 && server.active.len() >= 8,
        "{:?}",
        o.not_assessed
    );
    for code in server.codes.keys() {
        assert!(issued.contains(code.as_str()), "{code} not listed");
    }
    for token in server.active.keys() {
        assert!(issued.contains(token.as_str()), "{token} not listed");
    }
    assert_eq!(
        issued.len(),
        server.codes.len() + server.active.len(),
        "{issued:?}"
    );
}
