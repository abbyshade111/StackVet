//! V10.4.6 (backlog 0625, part 7; ADR-151): PKCE required. A code asked for with no challenge, and with the `plain`
//! method, must not come back, and a code exchanged with the wrong verifier must not give a token. Asked with codes of
//! its own, and credited only when a fresh code, exchanged with the right verifier after them, still gives one.

use super::*;

pub(in crate::signed_in) const PKCE_NOT_REQUIRED: Rule = Rule {
    rule_id: "probe.oauth-pkce-not-required",
    // V10.4.6: a valid `code_challenge` required, `plain` refused, and the `code_verifier` checked.
    requirement_ids: &["V10.4.6"],
    cwe: &["CWE-345"],
    impact: "A sign-in code caught on its way back to the app, by another app on the person's phone or from a log, can \
             be exchanged for tokens by whoever caught it, since nothing ties the code to the client that asked for it.",
    fix: "Require a `code_challenge` with `code_challenge_method=S256` on every authorization request, refuse `plain`, \
          and refuse a token request whose `code_verifier` does not match the challenge (RFC 7636).",
};

const PKCE: &str = "V10.4.6";

/// A verifier of the right shape that is not `verifier`: its last letter changed.
fn wrong(verifier: &str) -> String {
    let last = if verifier.ends_with('A') { 'B' } else { 'A' };
    format!("{}{last}", &verifier[..verifier.len() - 1])
}

pub(super) fn pkce_check(http: &mut dyn Http, ask: &Ask, secret: Option<&str>, out: &mut Outcome) {
    let mut taken = Vec::new();
    let mut unread = Vec::new();
    let mut ids = Vec::new();
    for (id, what, pkce) in [
        (
            "oauth-authorize-no-pkce",
            "with no PKCE challenge",
            Pkce::Left,
        ),
        (
            "oauth-authorize-plain-pkce",
            "with PKCE's `plain` method",
            Pkce::Plain,
        ),
    ] {
        let answer = ask.authorize_with(http, id, &ask.client.redirect, pkce);
        let code = ask.code_in(&answer);
        out.steps.push(format!(
            "asked for a sign-in code {what}: {}",
            if code.is_some() {
                "a code came back".to_owned()
            } else {
                format!("refused ({})", status(&answer))
            }
        ));
        ids.push(id.to_owned());
        match code {
            Some(code) => {
                out.issued.push(code);
                taken.push(format!("a code was given when asked for {what}"));
            }
            // A crash, or a limiter's answer, is not the server refusing (ADR-021).
            None if !refused(&answer) => unread.push(format!("asked for {what}")),
            None => {}
        }
    }
    let answer = ask.authorize(
        http,
        "oauth-authorize-for-wrong-verifier",
        &ask.client.redirect,
    );
    ids.push("oauth-authorize-for-wrong-verifier".to_owned());
    match ask.code_in(&answer) {
        Some(code) => {
            out.issued.push(code.clone());
            let exchanged = ask.exchange_with(
                http,
                "oauth-token-wrong-verifier",
                &code,
                secret,
                &wrong(ask.verifier),
            );
            out.issued.extend(tokens_in(&exchanged));
            ids.push("oauth-token-wrong-verifier".to_owned());
            out.steps.push(format!(
                "exchanged a sign-in code with the wrong PKCE verifier: {}",
                if access_token(&exchanged).is_some() {
                    "a token came back".to_owned()
                } else {
                    format!("refused ({})", status(&exchanged))
                }
            ));
            if access_token(&exchanged).is_some() {
                taken.push("a code was exchanged for a token with the wrong verifier".to_owned());
            } else if !refused(&exchanged) {
                unread.push("exchanged with the wrong verifier".to_owned());
            }
        }
        None => unread
            .push("exchanged with the wrong verifier, since no code came to exchange".to_owned()),
    }

    if !taken.is_empty() {
        out.findings.push(finding_on(
            ids,
            &PKCE_NOT_REQUIRED,
            "The authorization server does not hold sign-in codes to PKCE",
            Severity::Medium,
            format!(
                "For the test client `{}`: {}.",
                ask.client.id,
                taken.join("; and ")
            ),
        ));
        return;
    }
    if !unread.is_empty() {
        out.not_assessed.push((
            PKCE.to_owned(),
            format!(
                "Asked for a sign-in code without PKCE or with its `plain` method, and to exchange one with the wrong \
                 verifier, the authorization server gave no answer that can be read as a refusal when {} (it \
                 crashed, did not answer, or was limiting requests), so nothing was concluded.",
                unread.join(", and when ")
            ),
        ));
        return;
    }
    // Refused, but only worth anything if a code exchanged with the right verifier still gives a token.
    let after = ask
        .code(http, "oauth-authorize-pkce-after", &ask.client.redirect)
        .map(|code| {
            out.issued.push(code.clone());
            let exchanged = ask.exchange(http, "oauth-token-pkce-after", &code, secret);
            out.issued.extend(tokens_in(&exchanged));
            access_token(&exchanged).is_some()
        });
    match after {
        Ok(true) => out.verified.push(
            crate::Verified::new(
                PKCE_NOT_REQUIRED.rule_id,
                &[PKCE],
                "the authorization server refused a sign-in code asked for with no PKCE challenge and with the \
                 `plain` method, and refused to exchange one with the wrong verifier, and a code exchanged with the \
                 right one gave a token after; one client was asked"
                    .to_owned(),
            )
            .with_evidence(ids)
            .in_part(),
        ),
        Ok(false) => out.not_assessed.push((
            PKCE.to_owned(),
            "After the wrong PKCE requests were refused, a fresh code exchanged with the right verifier gave no token, \
             so the refusals show nothing."
                .to_owned(),
        )),
        Err(why) => out.not_assessed.push((
            PKCE.to_owned(),
            format!("After the wrong PKCE requests were refused, a fresh code was asked for: {why}"),
        )),
    }
}
