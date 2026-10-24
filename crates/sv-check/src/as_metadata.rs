//! What an authorization server says about itself in its published details (backlog 0625, part 6; ADR-147).
//!
//! An app that other applications send their users to, to sign in, publishes what it accepts: RFC 8414's
//! `/.well-known/oauth-authorization-server`, and OpenID Connect's `/.well-known/openid-configuration`. Two read-only
//! requests to the copy of the app `sv` started ask for both. An answer that is not a JSON object with an `issuer`
//! concludes nothing, so an app that is not an authorization server is told nothing.
//!
//! What the details say is what the server says it accepts, not what each client is set to or what the server does
//! when asked, so a credit here is in part, and only V10.4.16 and V10.6.1 credit: listing `plain` PKCE shows V10.4.6
//! failing, and not listing it does not show PKCE required.

use crate::finding::{Confidence, Finding, Location, Severity};
use crate::probes::{ProbeRequest, ProbeResponse};
use crate::verified::Verified;
use serde_json::Value;

/// The two addresses asked, as `(answer id, path)`.
pub const ASKED: &[(&str, &str)] = &[
    ("as-metadata", "/.well-known/oauth-authorization-server"),
    ("oidc-metadata", "/.well-known/openid-configuration"),
];

pub const CLIENT_AUTH: &str = "probe.authorization-server-weak-client-auth";
pub const IMPLICIT: &str = "probe.authorization-server-implicit-flow";
pub const PLAIN_PKCE: &str = "probe.authorization-server-plain-pkce";

/// Client authentication methods built on a shared secret, or none: what V10.4.16 asks a server not to accept.
const WEAK_AUTH: &[&str] = &[
    "client_secret_basic",
    "client_secret_post",
    "client_secret_jwt",
    "none",
];
/// The response types V10.6.1 allows an OpenID provider, each as its words in any order.
const ALLOWED_TYPES: &[&[&str]] = &[&["code"], &["id_token"], &["code", "id_token"]];

pub fn requests() -> Vec<ProbeRequest> {
    ASKED
        .iter()
        .map(|(id, path)| ProbeRequest {
            id: (*id).into(),
            method: "GET".into(),
            path: (*path).into(),
            headers: vec![("Accept".into(), "application/json".into())],
            body: None,
        })
        .collect()
}

/// Each published document the app gave, as `(answer id, document)`: a 200 whose body is a JSON object with an
/// `issuer`. A body cut at what the run keeps does not parse, and is not read.
fn documents(responses: &[ProbeResponse]) -> Vec<(&str, serde_json::Map<String, Value>)> {
    ASKED
        .iter()
        .filter_map(|(id, _)| {
            let answer = responses.iter().find(|r| r.id == *id && r.status == 200)?;
            let Value::Object(document) = serde_json::from_str(&answer.body).ok()? else {
                return None;
            };
            document.contains_key("issuer").then_some((*id, document))
        })
        .collect()
}

/// The strings of a list field; `None` when the field is left out or is not a list of strings.
fn strings<'a>(document: &'a serde_json::Map<String, Value>, field: &str) -> Option<Vec<&'a str>> {
    document
        .get(field)?
        .as_array()?
        .iter()
        .map(Value::as_str)
        .collect()
}

/// The client authentication methods the token address takes: RFC 8414 means `client_secret_basic` when it is left out.
fn auth_methods(document: &serde_json::Map<String, Value>) -> Vec<&str> {
    strings(document, "token_endpoint_auth_methods_supported")
        .unwrap_or_else(|| vec!["client_secret_basic"])
}

/// The response types an OpenID provider lists that V10.6.1 does not allow.
fn not_allowed_types<'a>(types: &[&'a str]) -> Vec<&'a str> {
    types
        .iter()
        .copied()
        .filter(|t| {
            let mut words: Vec<&str> = t.split_whitespace().collect();
            words.sort_unstable();
            !ALLOWED_TYPES.iter().any(|allowed| {
                let mut allowed = allowed.to_vec();
                allowed.sort_unstable();
                allowed == words
            })
        })
        .collect()
}

fn path_of(id: &str) -> &'static str {
    ASKED
        .iter()
        .find(|(a, _)| *a == id)
        .map(|(_, p)| *p)
        .unwrap_or("")
}

fn finding(
    rule_id: &str,
    title: &str,
    requirement: &str,
    id: &str,
    description: String,
    impact: &str,
    fix: &str,
) -> Finding {
    let mut finding = crate::finding::found(Finding {
        evidence: vec![id.to_owned()],
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: rule_id.into(),
        title: title.into(),
        severity: Severity::Medium,
        confidence: Confidence::High,
        location: Location::running_app(),
        secret: None,
        requirement_ids: vec![requirement.into()],
        cwe: vec!["CWE-287".into()],
        description,
        impact: impact.into(),
        fix: fix.into(),
    });
    if rule_id == IMPLICIT {
        finding.cwe = vec!["CWE-522".into()];
    }
    finding
}

pub fn evaluate(responses: &[ProbeResponse]) -> Vec<Finding> {
    let mut out = Vec::new();
    for (id, document) in documents(responses) {
        let path = path_of(id);
        let weak: Vec<&str> = auth_methods(&document)
            .into_iter()
            .filter(|m| WEAK_AUTH.contains(m))
            .collect();
        if !weak.is_empty() && !out.iter().any(|f: &Finding| f.rule_id == CLIENT_AUTH) {
            let left_out = !document.contains_key("token_endpoint_auth_methods_supported");
            out.push(finding(
                CLIENT_AUTH,
                "The authorization server accepts clients that sign in with a shared secret, or with nothing",
                "V10.4.16",
                id,
                format!(
                    "The app's published details at {path} {} for the token address: {}. These rest on a secret a \
                     client sends, or on none, rather than on a key only the client holds.",
                    if left_out {
                        "do not list the client authentication methods, which RFC 8414 reads as `client_secret_basic`"
                    } else {
                        "list"
                    },
                    weak.iter().map(|m| format!("`{m}`")).collect::<Vec<_>>().join(", ")
                ),
                "A client secret that leaks, from an app's code or a log, lets whoever has it ask for tokens as that \
                 client, and a client with no authentication can be anyone.",
                "Require `private_key_jwt` or mutual TLS (`tls_client_auth`, `self_signed_tls_client_auth`) for every \
                 client, and list only those in `token_endpoint_auth_methods_supported`.",
            ));
        }
        if id == "oidc-metadata"
            && let Some(types) = strings(&document, "response_types_supported")
        {
            let bad = not_allowed_types(&types);
            if !bad.is_empty() {
                out.push(finding(
                    IMPLICIT,
                    "The OpenID provider offers a sign-in that hands tokens straight to the browser",
                    "V10.6.1",
                    id,
                    format!(
                        "The app's published details at {path} list response types V10.6.1 does not allow: {}. A \
                         `token` among them is the implicit flow.",
                        bad.iter().map(|t| format!("`{t}`")).collect::<Vec<_>>().join(", ")
                    ),
                    "In the implicit flow the access token comes back in the page's address, where the browser's \
                     history, other scripts on the page, and the next site's referrer can all read it.",
                    "Offer only `code` (with PKCE), and `id_token` or `code id_token` where they are needed; take \
                     every type holding `token` out of the provider's settings.",
                ));
            }
        }
        if strings(&document, "code_challenge_methods_supported")
            .is_some_and(|m| m.contains(&"plain"))
            && !out.iter().any(|f: &Finding| f.rule_id == PLAIN_PKCE)
        {
            out.push(finding(
                PLAIN_PKCE,
                "The authorization server accepts PKCE's plain method",
                "V10.4.6",
                id,
                format!(
                    "The app's published details at {path} list `plain` among the PKCE methods it accepts, which V10.4.6 \
                     says the server must refuse."
                ),
                "With `plain`, the value that proves a sign-in code is the client's own is sent as it is, so whoever \
                 sees the start of the sign-in can use the code.",
                "Accept only `S256` for `code_challenge_method`, and list only it in `code_challenge_methods_supported`.",
            ));
        }
    }
    out
}

pub fn verified(responses: &[ProbeResponse], already: &[Verified]) -> Vec<Verified> {
    let documents = documents(responses);
    let mut out = Vec::new();
    let strong: Vec<&str> = documents
        .iter()
        .filter(|(_, d)| d.contains_key("token_endpoint_auth_methods_supported"))
        .flat_map(|(_, d)| auth_methods(d))
        .collect();
    let all_strong = !documents.is_empty()
        && documents
            .iter()
            .all(|(_, d)| auth_methods(d).iter().all(|m| !WEAK_AUTH.contains(m)));
    if all_strong && !strong.is_empty() {
        out.push(
            Verified::new(
                CLIENT_AUTH,
                &["V10.4.16"],
                format!(
                    "the authorization server's published details list only {} for the token address: what it says it \
                     accepts, not what each client is set to",
                    strong.iter().map(|m| format!("`{m}`")).collect::<Vec<_>>().join(", ")
                ),
            )
            .with_evidence(documents.iter().map(|(id, _)| (*id).to_owned()).collect())
            .in_part(),
        );
    }
    if let Some((id, document)) = documents.iter().find(|(id, _)| *id == "oidc-metadata")
        && let Some(types) = strings(document, "response_types_supported")
        && !types.is_empty()
        && not_allowed_types(&types).is_empty()
    {
        out.push(
            Verified::new(
                IMPLICIT,
                &["V10.6.1"],
                format!(
                    "the OpenID provider's published details list only {} as response types: what it says it offers, \
                     not what it does when asked for another",
                    types.iter().map(|t| format!("`{t}`")).collect::<Vec<_>>().join(", ")
                ),
            )
            .with_evidence(vec![(*id).to_owned()])
            .in_part(),
        );
    }
    let mut together = already.to_vec();
    together.extend(out.iter().cloned());
    crate::verified::unless_credited(CLIENT_AUTH, &together);
    crate::verified::unless_credited(IMPLICIT, &together);
    out
}

#[cfg(test)]
mod tests;
