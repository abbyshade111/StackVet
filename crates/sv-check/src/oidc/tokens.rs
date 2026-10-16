//! Tokens the test provider makes on request, and what the app does with them (backlog 0500, ADR-127).
//!
//! - **V10.3.1:** an access token the provider issued for another service, sent as a bearer token, must not open the
//!   private page. Asked only when one issued to this app does: an app that takes only cookies is no resource server for
//!   these tokens, and is not judged on them.
//! - **V9.2.2:** an ID token the provider issued to this app, sent as a bearer token where an access token opens the
//!   private page, must not open it: an ID token says who signed in, and is no permission to act. Asked, like V10.3.1,
//!   only when an access token issued to this app opens the page.
//! - **V10.5.5:** a back-channel logout token of the wrong type, without the logout `events` claim, or with a `nonce`,
//!   must not end anybody's session. Asked only when stackvet.toml names the address the app takes them at, and
//!   credited only when a correct one does end a session.

use super::*;
use crate::probes::ProbeRequest;

pub(super) const RESOURCE_AUD: Rule = Rule {
    rule_id: "probe.oidc-access-token-audience-not-checked",
    requirement_ids: &["V10.3.1"],
    cwe: &["CWE-287"],
    impact: "An access token the sign-in provider issued for any other service opens the app: whoever runs that service, \
             or anyone who gets one of its tokens, can use the app as the person the token names.",
    fix: "Accept an access token only when it was issued for this app: check the token's `aud` (or, through the \
          provider's introspection, its `client_id`) against the app's own, and refuse every other.",
};

pub(super) const ID_AS_ACCESS: Rule = Rule {
    rule_id: "probe.oidc-id-token-taken-as-access-token",
    requirement_ids: &["V9.2.2"],
    cwe: &["CWE-287"],
    impact: "An ID token opens the app where an access token should be needed. An ID token only tells an app who signed \
             in, and is often kept in the browser where scripts and logs can reach it, so whoever gets hold of one can \
             use the app as that person.",
    fix: "Accept only access tokens as bearer tokens: check the token's type (`typ` of `at+jwt`, RFC 9068) or ask the \
          provider's introspection whether it is an active access token, and refuse an ID token however good its \
          signature and audience.",
};

pub(super) const LOGOUT_TOKEN: Rule = Rule {
    rule_id: "probe.oidc-logout-token-not-checked",
    requirement_ids: &["V10.5.5"],
    cwe: &["CWE-345"],
    impact: "A token that is not a proper logout token ends a person's session, so another kind of token from the same \
             provider (an ID token, say) can be posted to the logout address to sign people out at will.",
    fix: "Before ending any session, check that the logout token's `typ` header is `logout+jwt`, that it carries the \
          `events` claim with the back-channel logout member, and that it carries no `nonce`, as well as its \
          signature, issuer, and audience.",
};

/// Asks the provider for a token, and gives back the token from its `{ "token": ... }`.
fn mint(http: &mut dyn Http, path: &str, form: &str, id: &str) -> Option<String> {
    let request = ProbeRequest {
        id: id.to_owned(),
        method: "POST".to_owned(),
        path: path.to_owned(),
        headers: vec![(
            "Content-Type".to_owned(),
            "application/x-www-form-urlencoded".to_owned(),
        )],
        body: Some(form.to_owned().into()),
    };
    let answer = http.provider(&request).filter(|r| r.status == 200)?;
    serde_json::from_str::<serde_json::Value>(&answer.body)
        .ok()?
        .get("token")?
        .as_str()
        .map(str::to_owned)
}

/// Whether the private page opens for `token` sent as a bearer token, with no cookie.
fn opens_with_bearer(http: &mut dyn Http, section: &OidcSection, token: &str, id: &str) -> bool {
    ok(&http.send(&ProbeRequest {
        id: id.to_owned(),
        method: "GET".to_owned(),
        path: section.private.clone(),
        headers: vec![("Authorization".to_owned(), format!("Bearer {token}"))],
        body: None,
    }))
}

/// V10.3.1: an access token for another service, sent where one for this app opens the private page.
pub(super) fn access_token_audience(http: &mut dyn Http, section: &OidcSection, out: &mut Outcome) {
    let not_assessed =
        |out: &mut Outcome, why: String| out.not_assessed.push(("V10.3.1".to_owned(), why));
    let Some(own) = mint(
        http,
        idp::ACCESS_TOKEN,
        "aud=self",
        "oidc-access-token-self",
    ) else {
        not_assessed(
            out,
            "The test provider did not hand over an access token, so nothing was sent as a bearer token.".to_owned(),
        );
        return;
    };
    if !opens_with_bearer(http, section, &own, "oidc-bearer-own") {
        not_assessed(
            out,
            format!(
                "{} did not open for an access token the provider issued to this app, sent as a bearer token, so the \
                 app does not take the provider's access tokens there and is not judged on whose they are. Whether it \
                 takes them anywhere else was not asked.",
                section.private
            ),
        );
        return;
    }
    let Some(other) = mint(
        http,
        idp::ACCESS_TOKEN,
        "aud=other",
        "oidc-access-token-other",
    ) else {
        not_assessed(
            out,
            "The test provider did not hand over an access token for another service.".to_owned(),
        );
        return;
    };
    let took_other = opens_with_bearer(http, section, &other, "oidc-bearer-other");
    out.steps.push(format!(
        "sent the provider's access tokens as bearer tokens to {}: one for this app opened it; one for another \
         service {}",
        section.private,
        if took_other { "opened it too" } else { "was refused" }
    ));
    if took_other {
        out.findings.push(finding(
            &RESOURCE_AUD,
            "An access token issued for another service opens the app",
            Severity::High,
            format!(
                "Sent as a bearer token with no cookie, an access token the test provider issued for another service \
                 (`aud` and `client_id` not this app's) opened {}, as one issued to this app did.",
                section.private
            ),
        ));
    } else if opens_with_bearer(http, section, &own, "oidc-bearer-own-again") {
        out.verified.push(
            crate::Verified::new(
                RESOURCE_AUD.rule_id,
                RESOURCE_AUD.requirement_ids,
                format!(
                    "an access token issued for another service, sent as a bearer token to {}, refused, where one \
                     issued to this app opened it before and after",
                    section.private
                ),
            )
            // One page, and one other audience.
            .in_part(),
        );
    } else {
        not_assessed(
            out,
            "The app refused an access token for another service, but then refused its own as well, so the refusal \
             may not have been about whose token it was."
                .to_owned(),
        );
    }
    crate::verified::unless_credited(RESOURCE_AUD.rule_id, &out.verified);
}

/// V9.2.2: an ID token issued to this app, sent where an access token issued to it opens the private page.
pub(super) fn id_token_as_access(http: &mut dyn Http, section: &OidcSection, out: &mut Outcome) {
    let not_assessed =
        |out: &mut Outcome, why: String| out.not_assessed.push(("V9.2.2".to_owned(), why));
    let Some(access) = mint(
        http,
        idp::ACCESS_TOKEN,
        "aud=self",
        "oidc-id-check-access-token",
    ) else {
        not_assessed(
            out,
            "The test provider did not hand over an access token, so nothing was sent as a bearer token.".to_owned(),
        );
        return;
    };
    if !opens_with_bearer(http, section, &access, "oidc-id-check-access-opens") {
        not_assessed(
            out,
            format!(
                "{} did not open for an access token the provider issued to this app, sent as a bearer token, so the \
                 app does not take bearer tokens there, and whether it would take an ID token as one was not asked.",
                section.private
            ),
        );
        return;
    }
    let Some(id_token) = mint(http, idp::ID_TOKEN, "kind=id", "oidc-id-token") else {
        not_assessed(
            out,
            "The test provider did not hand over an ID token.".to_owned(),
        );
        return;
    };
    let took = opens_with_bearer(http, section, &id_token, "oidc-bearer-id-token");
    out.steps.push(format!(
        "sent tokens issued to this app as bearer tokens to {}: an access token opened it; an ID token {}",
        section.private,
        if took { "opened it too" } else { "was refused" }
    ));
    if took {
        out.findings.push(finding(
            &ID_AS_ACCESS,
            "An ID token is taken as an access token",
            Severity::High,
            format!(
                "Sent as a bearer token with no cookie, an ID token the test provider issued to this app opened {}, as \
                 an access token did. Its signature, issuer, and audience were all good: what was wrong is its kind.",
                section.private
            ),
        ));
    } else if opens_with_bearer(http, section, &access, "oidc-id-check-access-again") {
        out.verified.push(
            crate::Verified::new(
                ID_AS_ACCESS.rule_id,
                ID_AS_ACCESS.requirement_ids,
                format!(
                    "an ID token issued to this app, sent as a bearer token to {}, refused, where an access token \
                     opened it before and after",
                    section.private
                ),
            )
            // One page, and one wrong kind of token: V9.2.2 asks about every token the app takes.
            .in_part(),
        );
    } else {
        not_assessed(
            out,
            "The app refused the ID token, but then refused its access token as well, so the refusal may not have \
             been about the kind of token."
                .to_owned(),
        );
    }
    crate::verified::unless_credited(ID_AS_ACCESS.rule_id, &out.verified);
}

/// One fresh sign-in, a logout token of `variant` posted to `address`, and whether the session it made was then ended.
/// `None` when the sign-in or the token could not be made.
fn ended_by(
    http: &mut dyn Http,
    section: &OidcSection,
    address: &str,
    variant: &str,
) -> Option<bool> {
    let session = session_for(http, section, idp::NORMAL, &format!("logout-{variant}"))
        .ok()
        .flatten()?;
    let token = mint(
        http,
        idp::LOGOUT_TOKEN,
        &format!("variant={variant}"),
        &format!("oidc-logout-token-{variant}"),
    )?;
    http.send(&ProbeRequest {
        id: format!("oidc-backchannel-{variant}"),
        method: "POST".to_owned(),
        path: address.to_owned(),
        headers: vec![(
            "Content-Type".to_owned(),
            "application/x-www-form-urlencoded".to_owned(),
        )],
        body: Some(format!("logout_token={token}").into()),
    });
    Some(!ok(&http.send(&get(
        &format!("oidc-after-logout-{variant}"),
        &section.private,
        &session,
    ))))
}

/// What each wrong logout token is, as the report says it.
fn wrong_in(variant: &str) -> &'static str {
    match variant {
        "wrong-typ" => "a `typ` of `JWT` rather than `logout+jwt`",
        "no-events" => "no `events` claim",
        _ => "a `nonce` claim",
    }
}

/// V10.5.5: three logout tokens each wrong in one way, then a correct one as the control.
pub(super) fn backchannel_logout(http: &mut dyn Http, section: &OidcSection, out: &mut Outcome) {
    let Some(address) = section.backchannel_logout.clone() else {
        out.not_assessed.push((
            "V10.5.5".to_owned(),
            "stackvet.toml names no back-channel logout address under `[stack.run.oidc]` (`backchannel-logout`), so no \
             logout token was sent. An app that does not take them has nothing to check."
                .to_owned(),
        ));
        return;
    };
    let mut ended = Vec::new();
    let mut refused = Vec::new();
    let mut unmade = Vec::new();
    for variant in idp::LOGOUT_WRONG {
        match ended_by(http, section, &address, variant) {
            Some(true) => ended.push(variant),
            Some(false) => refused.push(variant),
            None => unmade.push(variant),
        }
    }
    let control = ended_by(http, section, &address, "good");
    out.steps.push(format!(
        "posted logout tokens to {address}: ended a session with {}; refused {}; a correct one {}",
        if ended.is_empty() {
            "none".to_owned()
        } else {
            ended
                .iter()
                .map(|v| wrong_in(v))
                .collect::<Vec<_>>()
                .join(", ")
        },
        if refused.is_empty() {
            "none".to_owned()
        } else {
            refused
                .iter()
                .map(|v| wrong_in(v))
                .collect::<Vec<_>>()
                .join(", ")
        },
        match control {
            Some(true) => "ended one",
            Some(false) => "did not end one",
            None => "could not be sent",
        }
    ));
    if !ended.is_empty() {
        out.findings.push(finding(
            &LOGOUT_TOKEN,
            "A logout token that is not a proper one ends a session",
            Severity::Medium,
            format!(
                "A session was ended by a logout token posted to {address} with {}.",
                ended
                    .iter()
                    .map(|v| wrong_in(v))
                    .collect::<Vec<_>>()
                    .join(", and one with ")
            ),
        ));
    } else if control == Some(true) && unmade.is_empty() {
        out.verified.push(
            crate::Verified::new(
                LOGOUT_TOKEN.rule_id,
                LOGOUT_TOKEN.requirement_ids,
                format!(
                    "logout tokens posted to {address} with the wrong `typ`, with no `events` claim, and with a \
                     `nonce`, each refused, where a correct one ended the session"
                ),
            )
            // The three checks the requirement says the client must make, once each, and nothing of the short expiry
            // it recommends.
            .in_part(),
        );
    } else {
        out.not_assessed.push((
            "V10.5.5".to_owned(),
            if control == Some(true) {
                "Not every wrong logout token could be made and sent through a fresh sign-in.".to_owned()
            } else {
                format!(
                    "A correct logout token posted to {address} did not end the session, so the wrong ones not ending \
                     it shows nothing about the checks."
                )
            },
        ));
    }
    crate::verified::unless_credited(LOGOUT_TOKEN.rule_id, &out.verified);
}

#[cfg(test)]
#[path = "tokens_tests.rs"]
mod tests;
