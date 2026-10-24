//! Backlog 0625, part 6, ADR-147: an authorization server's published details, read from the two answers the run asks
//! for. Weak client authentication, an implicit flow, and `plain` PKCE are findings; only strong methods and only the
//! allowed response types are credited in part; an answer that is not such a document concludes nothing.

use super::*;

fn answer(id: &str, status: u16, body: &str) -> ProbeResponse {
    ProbeResponse {
        id: id.into(),
        status,
        headers: vec![("content-type".into(), "application/json".into())],
        body: body.into(),
    }
}

/// An OpenID provider's details with the given client authentication methods, response types, and PKCE methods.
fn oidc(auth: Option<&str>, types: &str, pkce: &str) -> ProbeResponse {
    let auth = auth
        .map(|a| format!(",\"token_endpoint_auth_methods_supported\":[{a}]"))
        .unwrap_or_default();
    answer(
        "oidc-metadata",
        200,
        &format!(
            "{{\"issuer\":\"http://app\",\"authorization_endpoint\":\"http://app/authorize\",\
             \"response_types_supported\":[{types}],\"code_challenge_methods_supported\":[{pkce}]{auth}}}"
        ),
    )
}

fn found(responses: &[ProbeResponse]) -> Vec<String> {
    let mut ids: Vec<String> = evaluate(responses).into_iter().map(|f| f.rule_id).collect();
    ids.sort();
    ids
}

fn credited(responses: &[ProbeResponse]) -> Vec<(String, bool)> {
    verified(responses, &[])
        .into_iter()
        .map(|v| (v.check_id, v.in_part))
        .collect()
}

const STRONG: &str = "\"private_key_jwt\",\"tls_client_auth\"";

#[test]
fn the_two_addresses_are_asked_read_only() {
    let asked = requests();
    assert_eq!(asked.len(), 2);
    assert!(asked.iter().all(|r| r.method == "GET" && r.body.is_none()));
    assert_eq!(
        asked.iter().map(|r| r.path.as_str()).collect::<Vec<_>>(),
        [
            "/.well-known/oauth-authorization-server",
            "/.well-known/openid-configuration"
        ]
    );
}

#[test]
fn a_provider_offering_only_strong_methods_and_allowed_types_is_credited_in_part() {
    let good = [oidc(
        Some(STRONG),
        "\"code\",\"id_token\",\"id_token code\"",
        "\"S256\"",
    )];
    assert!(found(&good).is_empty(), "{:?}", found(&good));
    assert_eq!(
        credited(&good),
        [(CLIENT_AUTH.to_owned(), true), (IMPLICIT.to_owned(), true)]
    );
    let credits = verified(&good, &[]);
    assert!(
        credits[0].scope.contains("not what each client is set to"),
        "{}",
        credits[0].scope
    );
    assert_eq!(credits[0].evidence, ["oidc-metadata"]);
}

#[test]
fn a_shared_secret_or_no_method_listed_is_a_finding_and_withholds_the_credit() {
    for auth in [
        Some("\"client_secret_basic\""),
        Some("\"private_key_jwt\",\"client_secret_post\""),
        Some("\"none\""),
        // Left out, RFC 8414 means client_secret_basic.
        None,
    ] {
        let details = [oidc(auth, "\"code\"", "\"S256\"")];
        assert_eq!(found(&details), [CLIENT_AUTH], "{auth:?}");
        assert!(
            credited(&details).iter().all(|(id, _)| id != CLIENT_AUTH),
            "{auth:?}"
        );
    }
}

#[test]
fn a_response_type_holding_token_is_the_implicit_flow() {
    for types in [
        "\"code\",\"token\"",
        "\"code id_token token\"",
        "\"id_token token\"",
    ] {
        let details = [oidc(Some(STRONG), types, "\"S256\"")];
        assert_eq!(found(&details), [IMPLICIT], "{types}");
        assert!(
            credited(&details).iter().all(|(id, _)| id != IMPLICIT),
            "{types}"
        );
    }
}

#[test]
fn plain_pkce_is_a_finding_and_never_a_credit() {
    let details = [oidc(Some(STRONG), "\"code\"", "\"S256\",\"plain\"")];
    assert_eq!(found(&details), [PLAIN_PKCE]);
    assert!(
        credited(&[oidc(Some(STRONG), "\"code\"", "\"S256\"")])
            .iter()
            .all(|(id, _)| id != PLAIN_PKCE)
    );
}

#[test]
fn the_oauth_document_is_read_for_client_auth_and_pkce_and_not_for_v10_6_1() {
    let oauth = answer(
        "as-metadata",
        200,
        "{\"issuer\":\"http://app\",\"response_types_supported\":[\"token\"],\
         \"token_endpoint_auth_methods_supported\":[\"client_secret_post\"],\"code_challenge_methods_supported\":[\"plain\"]}",
    );
    // Sorted by id, as `found` gives them.
    assert_eq!(found(&[oauth]), [PLAIN_PKCE, CLIENT_AUTH]);
}

#[test]
fn an_answer_that_is_not_published_details_concludes_nothing() {
    for answers in [
        vec![answer("oidc-metadata", 404, "Not Found")],
        vec![answer(
            "oidc-metadata",
            200,
            "<html><body>Welcome</body></html>",
        )],
        // JSON, but no issuer: not a server's details.
        vec![answer(
            "oidc-metadata",
            200,
            "{\"response_types_supported\":[\"token\"]}",
        )],
        // Cut at what the run keeps, so it does not parse.
        vec![answer(
            "oidc-metadata",
            200,
            "{\"issuer\":\"http://app\",\"response_types_supported\":[\"tok",
        )],
        vec![],
    ] {
        assert!(
            evaluate(&answers).is_empty() && verified(&answers, &[]).is_empty(),
            "{answers:?}"
        );
    }
}
