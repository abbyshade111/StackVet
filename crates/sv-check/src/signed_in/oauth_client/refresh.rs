//! V10.4.5 (backlog 0625, part 7; ADR-151): a refresh token, once used, refused when it comes back, and the newer one
//! it was traded for revoked with it. Asked only of a public client, which V10.4.5 is written for: a confidential
//! client proves who sends its refresh token with its secret.

use super::*;

pub(in crate::signed_in) const REFRESH_REPLAYED: Rule = Rule {
    rule_id: "probe.oauth-refresh-token-replayed",
    // V10.4.5: refresh token replay mitigated for public clients; with rotation, a used token invalidated, and every
    // refresh token of that authorization revoked when a used one comes back.
    requirement_ids: &["V10.4.5"],
    cwe: &["CWE-294"],
    impact: "A refresh token copied from a person's phone or browser keeps giving whoever has it new access tokens for \
             that person, for as long as the refresh token lives, even after the real app has used it.",
    fix: "Bind refresh tokens to the client that holds them (DPoP or mutual TLS), or rotate them: give a new refresh \
          token with each use, refuse a used one, and revoke every refresh token of that sign-in when a used one comes \
          back.",
};

const REFRESH: &str = "V10.4.5";

/// The refresh token in a token answer, when it is a 2xx with one.
fn refresh_token(answer: &Option<ProbeResponse>) -> Option<String> {
    let answer = answer.as_ref().filter(|r| (200..300).contains(&r.status))?;
    serde_json::from_str::<serde_json::Value>(&answer.body)
        .ok()?
        .get("refresh_token")?
        .as_str()
        .map(str::to_owned)
}

impl Ask<'_> {
    /// The refresh request for `token`, as a public client sends it: its id, and no secret.
    fn refresh(&self, http: &mut dyn Http, id: &str, token: &str) -> Option<ProbeResponse> {
        self.token_request(
            http,
            id,
            &format!(
                "grant_type=refresh_token&refresh_token={}&client_id={}",
                encoded(token),
                encoded(&self.client.id)
            ),
            None,
        )
    }
}

pub(super) fn refresh_check(
    http: &mut dyn Http,
    ask: &Ask,
    secret: Option<&str>,
    out: &mut Outcome,
) {
    let not_assessed =
        |out: &mut Outcome, why: String| out.not_assessed.push((REFRESH.to_owned(), why));
    if secret.is_some() {
        not_assessed(
            out,
            "The test client is a confidential one, with a secret, and V10.4.5 is about public clients, which have \
             nothing to prove who sends a refresh token. Declare a public client under `[stack.run.oauth-client]`, \
             with no `secret`, to have it asked."
                .to_owned(),
        );
        return;
    }
    let code = match ask.code(http, "oauth-authorize-for-refresh", &ask.client.redirect) {
        Ok(code) => code,
        Err(why) => {
            not_assessed(
                out,
                format!("A sign-in code was asked for, to have a refresh token: {why}"),
            );
            return;
        }
    };
    out.issued.push(code.clone());
    let exchanged = ask.exchange(http, "oauth-token-for-refresh", &code, None);
    out.issued.extend(tokens_in(&exchanged));
    let Some(first) = refresh_token(&exchanged) else {
        not_assessed(
            out,
            format!(
                "The sign-in code was exchanged at {} ({}) with no refresh token in the answer, so there was none to \
                 send twice. If the server gives one only for a scope such as `offline_access`, name it with `scope` \
                 under `[stack.run.oauth-client]`.",
                ask.server.token,
                status(&exchanged)
            ),
        );
        return;
    };
    let used = ask.refresh(http, "oauth-refresh", &first);
    out.issued.extend(tokens_in(&used));
    if access_token(&used).is_none() {
        not_assessed(
            out,
            format!(
                "The refresh token was sent to {} and gave no access token ({}), so whether it can be used twice was \
                 not asked.",
                ask.server.token,
                status(&used)
            ),
        );
        return;
    }
    let newer = refresh_token(&used).filter(|t| *t != first);
    let again = ask.refresh(http, "oauth-refresh-again", &first);
    out.issued.extend(tokens_in(&again));
    out.steps.push(format!(
        "used a refresh token ({}{}), then sent it again ({})",
        status(&used),
        if newer.is_some() {
            ", with a new one given"
        } else {
            ", with no new one given"
        },
        status(&again)
    ));
    let mut ids = vec![
        "oauth-token-for-refresh".to_owned(),
        "oauth-refresh".to_owned(),
        "oauth-refresh-again".to_owned(),
    ];
    if access_token(&again).is_some() {
        out.findings.push(finding_on(
            ids,
            &REFRESH_REPLAYED,
            "The authorization server takes a refresh token again after it was used",
            Severity::High,
            format!(
                "A refresh token for the public test client `{}` was used at {}, and the same refresh token, sent \
                 again with no proof of who sent it, gave another access token ({}).",
                ask.client.id,
                ask.server.token,
                status(&again)
            ),
        ));
        return;
    }
    // A crash, or a limiter's answer, is not the server refusing (ADR-021).
    if !refused(&again) {
        not_assessed(
            out,
            format!(
                "The used refresh token, sent again, was answered {}, neither a token nor a refusal, so nothing was \
                 concluded.",
                status(&again)
            ),
        );
        return;
    }
    let Some(newer) = newer else {
        out.verified.push(
            crate::Verified::new(
                REFRESH_REPLAYED.rule_id,
                &[REFRESH],
                "the authorization server refused a refresh token sent a second time; it gave no new refresh token with \
                 the first use, so whether it revokes a sign-in's other refresh tokens was not asked"
                    .to_owned(),
            )
            .with_evidence(ids)
            .in_part(),
        );
        return;
    };
    // Rotated: the newer token must have been revoked when the used one came back.
    let kept = ask.refresh(http, "oauth-refresh-newer", &newer);
    out.issued.extend(tokens_in(&kept));
    ids.push("oauth-refresh-newer".to_owned());
    if access_token(&kept).is_some() {
        out.findings.push(finding_on(
            ids,
            &REFRESH_REPLAYED,
            "The authorization server refuses a used refresh token, but leaves the newer one working",
            Severity::Medium,
            format!(
                "For the public test client `{}`, a used refresh token sent again was refused ({}), and the newer \
                 refresh token its first use gave still gave an access token afterwards ({}). V10.4.5 asks that every \
                 refresh token of that sign-in be revoked when a used one comes back.",
                ask.client.id,
                status(&again),
                status(&kept)
            ),
        ));
    } else if refused(&kept) {
        out.verified.push(
            crate::Verified::new(
                REFRESH_REPLAYED.rule_id,
                &[REFRESH],
                "the authorization server gave a new refresh token with each use, refused a used one sent again, and \
                 then refused the newer one too; one public client and one sign-in were asked"
                    .to_owned(),
            )
            .with_evidence(ids)
            .in_part(),
        );
    } else {
        not_assessed(
            out,
            format!(
                "After the used refresh token was refused, the newer one was answered {}, neither a token nor a \
                 refusal, so whether the server revoked it was not concluded.",
                status(&kept)
            ),
        );
    }
}
