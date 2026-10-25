//! V10.4.10 (backlog 0625, part 7; ADR-151): a confidential client authenticated on its token requests. A code
//! exchanged with no client authentication, and one with the wrong secret, must give nothing; refusals count only
//! when a fresh code, exchanged with the right secret after them, still gives a token. Asked only of a confidential
//! client: a public one has no secret to prove anything with.

use super::*;

pub(in crate::signed_in) const CLIENT_NOT_AUTHENTICATED: Rule = Rule {
    rule_id: "probe.oauth-client-not-authenticated",
    // V10.4.10: a confidential client authenticated for its back-channel requests, token requests among them.
    requirement_ids: &["V10.4.10"],
    cwe: &["CWE-287"],
    impact: "Whoever has a sign-in code meant for the app, from a log or a browser's history, can exchange it for tokens \
             without the app's secret, so the secret protects nothing.",
    fix: "Refuse every token request from a confidential client that does not authenticate as it was registered to \
          (its secret, a signed assertion, or mutual TLS), with `invalid_client`.",
};

const CLIENT_AUTH: &str = "V10.4.10";

/// A secret of the right shape that is not `secret`: its last character changed.
fn wrong(secret: &str) -> String {
    let last = if secret.ends_with('x') { 'y' } else { 'x' };
    format!("{}{last}", &secret[..secret.len().saturating_sub(1)])
}

pub(super) fn client_auth_check(
    http: &mut dyn Http,
    ask: &Ask,
    secret: Option<&str>,
    out: &mut Outcome,
) {
    let not_assessed =
        |out: &mut Outcome, why: String| out.not_assessed.push((CLIENT_AUTH.to_owned(), why));
    let Some(secret) = secret else {
        not_assessed(
            out,
            "The test client is a public one, with no secret, and V10.4.10 is about confidential clients. Declare a \
             confidential client under `[stack.run.oauth-client]`, with `secret = \"{seed:NAME}\"`, to have it asked."
                .to_owned(),
        );
        return;
    };
    let mut taken = Vec::new();
    let mut unread = Vec::new();
    let mut ids = Vec::new();
    let wrong_secret = wrong(secret);
    for (what, sent) in [
        ("with no client authentication", None),
        ("with the wrong secret", Some(wrong_secret.as_str())),
    ] {
        let slug = if sent.is_none() {
            "no-client-auth"
        } else {
            "wrong-secret"
        };
        let code = match ask.code(
            http,
            &format!("oauth-authorize-{slug}"),
            &ask.client.redirect,
        ) {
            Ok(code) => code,
            Err(why) => {
                not_assessed(
                    out,
                    format!("A sign-in code was asked for, to exchange {what}: {why}"),
                );
                return;
            }
        };
        out.issued.push(code.clone());
        let id = format!("oauth-token-{slug}");
        let exchanged = ask.exchange(http, &id, &code, sent);
        out.issued.extend(tokens_in(&exchanged));
        out.steps.push(format!(
            "exchanged a sign-in code {what}: {}",
            if access_token(&exchanged).is_some() {
                "a token came back".to_owned()
            } else {
                format!("refused ({})", status(&exchanged))
            }
        ));
        ids.push(id);
        if access_token(&exchanged).is_some() {
            taken.push(what);
        } else if !refused(&exchanged) {
            // A crash, or a limiter's answer, is not the server refusing (ADR-021).
            unread.push(what);
        }
    }
    if !taken.is_empty() {
        out.findings.push(finding_on(
            ids,
            &CLIENT_NOT_AUTHENTICATED,
            "The authorization server gives tokens to a confidential client that did not prove who it is",
            Severity::High,
            format!(
                "A sign-in code for the confidential test client `{}` was exchanged at {} for an access token {}.",
                ask.client.id,
                ask.server.token,
                taken.join(", and another ")
            ),
        ));
        return;
    }
    if !unread.is_empty() {
        not_assessed(
            out,
            format!(
                "A sign-in code exchanged {} was answered with neither a token nor a refusal (the server crashed, did \
                 not answer, or was limiting requests), so nothing was concluded.",
                unread.join(", and one ")
            ),
        );
        return;
    }
    // Refused, but only worth anything if the right secret still works.
    let after = ask
        .code(
            http,
            "oauth-authorize-client-auth-after",
            &ask.client.redirect,
        )
        .map(|code| {
            out.issued.push(code.clone());
            let exchanged =
                ask.exchange(http, "oauth-token-client-auth-after", &code, Some(secret));
            out.issued.extend(tokens_in(&exchanged));
            access_token(&exchanged).is_some()
        });
    match after {
        Ok(true) => out.verified.push(
            crate::Verified::new(
                CLIENT_NOT_AUTHENTICATED.rule_id,
                &[CLIENT_AUTH],
                "the authorization server refused to exchange a confidential test client's sign-in code with no \
                 client authentication and with the wrong secret, and exchanged one with the right secret after; \
                 token requests were asked, and pushed authorization and revocation requests were not"
                    .to_owned(),
            )
            .with_evidence(ids)
            .in_part(),
        ),
        Ok(false) => not_assessed(
            out,
            "After the exchanges without the right secret were refused, a fresh code exchanged with it gave no token, \
             so the refusals show nothing."
                .to_owned(),
        ),
        Err(why) => not_assessed(
            out,
            format!("After the exchanges without the right secret were refused, a fresh code was asked for: {why}"),
        ),
    }
}
