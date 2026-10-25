//! V10.6.2 (backlog 0625, part 7; ADR-151): no forced sign-out. A request to the server's sign-out address with no
//! `id_token_hint`, and one with a hint `sv` made up, are what another site can send a person's browser; neither may
//! end the person's session at the server without asking them. Run last: it may end that session.

use super::*;

pub(in crate::signed_in) const FORCED_LOGOUT: Rule = Rule {
    rule_id: "probe.oidc-logout-without-hint",
    // V10.6.2: forced logout mitigated, by the person's confirmation or a checked `id_token_hint`.
    requirement_ids: &["V10.6.2"],
    cwe: &["CWE-352"],
    impact: "Any page a person visits can sign them out of every app that signs in through this one, again and again, \
             by sending their browser to the sign-out address.",
    fix: "Ask the person to confirm a sign-out that comes without a valid `id_token_hint`, and refuse a hint that is \
          not a token the server issued (OpenID Connect RP-Initiated Logout, section 2).",
};

const LOGOUT: &str = "V10.6.2";

/// Whether the person is still signed in at the server: `Some(true)` when it answers the test client with a code,
/// `Some(false)` when it sends the browser elsewhere or back without one (to its sign-in page, or with
/// `error=login_required`), `None` when the answer says neither.
fn signed_in(http: &mut dyn Http, ask: &Ask, id: &str, out: &mut Outcome) -> Option<bool> {
    let answer = ask.authorize(http, id, &ask.client.redirect);
    if let Some(code) = ask.code_in(&answer) {
        out.issued.push(code);
        return Some(true);
    }
    answer
        .as_ref()
        .filter(|r| (300..400).contains(&r.status) && r.header("location").is_some())
        .map(|_| false)
}

/// A token-shaped hint no server issued, made at run time: a header, a body naming nobody, and no real signature.
fn forged(client: &str) -> String {
    let part = |json: &str| base64(json.as_bytes(), true);
    [
        part(r#"{"alg":"RS256","typ":"JWT"}"#),
        part(&format!(
            r#"{{"iss":"sv","sub":"sv-not-a-person","aud":"{client}"}}"#
        )),
        part("sv-not-a-signature"),
    ]
    .join(".")
}

pub(super) fn logout_check(http: &mut dyn Http, ask: &Ask, out: &mut Outcome) {
    let not_assessed =
        |out: &mut Outcome, why: String| out.not_assessed.push((LOGOUT.to_owned(), why));
    let Some(end) = &ask.server.end_session else {
        not_assessed(
            out,
            "The server publishes no sign-out address for clients (`end_session_endpoint`), so whether another site \
             can sign a person out through it was not asked."
                .to_owned(),
        );
        return;
    };
    if signed_in(http, ask, "oauth-session-before-logout", out) != Some(true) {
        not_assessed(
            out,
            "Before the sign-out requests, the first test user could not be shown signed in at the server, so whether \
             they end the session could not be told."
                .to_owned(),
        );
        return;
    }
    let mut ids = vec!["oauth-session-before-logout".to_owned()];
    let joined = if end.contains('?') { '&' } else { '?' };
    for (what, id, path) in [
        (
            "with no `id_token_hint`",
            "oidc-logout-no-hint",
            end.clone(),
        ),
        (
            "with an `id_token_hint` `sv` made up",
            "oidc-logout-forged-hint",
            format!("{end}{joined}id_token_hint={}", forged(&ask.client.id)),
        ),
    ] {
        let sent = http.send(&get(id, &path, ask.session));
        let after = format!("oauth-session-after-{id}");
        let still = signed_in(http, ask, &after, out);
        out.steps.push(format!(
            "sent the first test user's browser to the sign-out address {what} ({}): {}",
            status(&sent),
            match still {
                Some(true) => "still signed in",
                Some(false) => "signed out",
                None => "could not tell",
            }
        ));
        ids.push(id.to_owned());
        ids.push(after);
        match still {
            Some(true) => {}
            Some(false) => {
                out.findings.push(finding_on(
                    ids,
                    &FORCED_LOGOUT,
                    "The sign-in server signs a person out for any page that sends them to its sign-out address",
                    Severity::Low,
                    format!(
                        "The first test user's browser, sent to {end} {what}, was signed out at the server without \
                         being asked: the test client's next authorization request was no longer answered with a \
                         code."
                    ),
                ));
                return;
            }
            None => {
                not_assessed(
                    out,
                    format!(
                        "After the sign-out request {what}, the server's answer to the next authorization request said \
                         neither that the first test user was signed in nor that they were not, so nothing was \
                         concluded."
                    ),
                );
                return;
            }
        }
    }
    out.verified.push(
        crate::Verified::new(
            FORCED_LOGOUT.rule_id,
            &[LOGOUT],
            "the first test user, sent to the server's sign-out address with no `id_token_hint` and with one `sv` made \
             up, was still signed in after each; a sign-out with a valid hint, and other ways in, were not asked"
                .to_owned(),
        )
        .with_evidence(ids)
        .in_part(),
    );
}
