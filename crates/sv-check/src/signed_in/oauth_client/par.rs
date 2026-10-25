//! V10.4.13 (backlog 0625, part 7; ADR-151): the code grant only through pushed authorization requests (RFC 9126). A
//! code given for an authorization request sent straight to the authorization address is a finding. A server that
//! refuses that request, and gives a code for one pushed first to its PAR address, is credited in part.

use super::*;

pub(in crate::signed_in) const NOT_PUSHED_TAKEN: Rule = Rule {
    rule_id: "probe.oauth-code-without-par",
    // V10.4.13: grant type `code` always used together with pushed authorization requests.
    requirement_ids: &["V10.4.13"],
    cwe: &["CWE-345"],
    impact: "An authorization request is read from the sign-in link itself, where anyone who makes the link can change \
             what it asks for, rather than from what the client sent the server directly and authenticated.",
    fix: "Require pushed authorization requests for every client (`require_pushed_authorization_requests`), and \
          refuse an authorization request that carries its parameters in the address rather than a `request_uri`.",
};

const PAR: &str = "V10.4.13";

/// The first request was answered with a code, and it was not pushed first.
pub(super) fn not_pushed_taken(ask: &Ask, out: &mut Outcome) {
    out.findings.push(finding_on(
        vec!["oauth-authorize".to_owned()],
        &NOT_PUSHED_TAKEN,
        "The authorization server gives sign-in codes for requests not pushed to it first",
        Severity::Low,
        format!(
            "The test client's authorization request, sent straight to {} with its parameters in the address, was \
             answered with a sign-in code.{}",
            ask.server.authorize,
            if ask.server.par.is_some() {
                " The server publishes a pushed authorization request address, but does not require it."
            } else {
                ""
            }
        ),
    ));
}

/// The request id `sv` gives as `state` to a pushed request, read back with its code.
const PUSHED: &str = "oauth-authorize-pushed";

/// The first request was not answered with a code: asked again, and, when it is refused and the server has a PAR
/// address, asked through that.
pub(super) fn when_refused(
    http: &mut dyn Http,
    ask: &Ask,
    secret: Option<&str>,
    out: &mut Outcome,
) {
    let not_assessed =
        |out: &mut Outcome, why: String| out.not_assessed.push((PAR.to_owned(), why));
    let Some(par) = &ask.server.par else {
        not_assessed(
            out,
            "The first authorization request was not answered with a code, and the server publishes no pushed \
             authorization request address, so whether it takes codes only through pushed requests was not asked."
                .to_owned(),
        );
        return;
    };
    let straight = ask.authorize(http, "oauth-authorize-without-par", &ask.client.redirect);
    if let Some(code) = ask.code_in(&straight) {
        out.issued.push(code);
    }
    // A crash, or a limiter's answer, is not the server refusing (ADR-021).
    if !refused(&straight) {
        not_assessed(
            out,
            format!(
                "The authorization request sent straight to {} was answered {}, not a refusal `sv` can read, so \
                 nothing was concluded.",
                ask.server.authorize,
                status(&straight)
            ),
        );
        return;
    }
    let pushed = ask.post_form(
        http,
        "oauth-par",
        par,
        &format!(
            "response_type=code&client_id={}&redirect_uri={}&scope={}&state={PUSHED}&code_challenge={}\
             &code_challenge_method=S256",
            encoded(&ask.client.id),
            encoded(&ask.client.redirect),
            encoded(ask.client.scope.as_deref().unwrap_or("openid")),
            challenge(ask.verifier),
        ),
        secret,
    );
    let request_uri = pushed
        .as_ref()
        .filter(|r| (200..300).contains(&r.status))
        .and_then(|r| serde_json::from_str::<serde_json::Value>(&r.body).ok())
        .and_then(|v| v.get("request_uri")?.as_str().map(str::to_owned));
    let Some(request_uri) = request_uri else {
        not_assessed(
            out,
            format!(
                "The server refused the authorization request sent straight to it, and its pushed authorization \
                 request address, {par}, answered {} with no `request_uri`, so no code could be had either way.",
                status(&pushed)
            ),
        );
        return;
    };
    let path = format!(
        "{}{}client_id={}&request_uri={}",
        ask.server.authorize,
        if ask.server.authorize.contains('?') {
            '&'
        } else {
            '?'
        },
        encoded(&ask.client.id),
        encoded(&request_uri)
    );
    let answer = http.send(&get(PUSHED, &path, ask.session));
    match ask.code_in(&answer) {
        Some(code) => {
            out.issued.push(code);
            out.steps.push(format!(
                "an authorization request sent straight to the server was refused ({}), and one pushed first to {par} \
                 was answered with a code",
                status(&straight)
            ));
            out.verified.push(
                crate::Verified::new(
                    NOT_PUSHED_TAKEN.rule_id,
                    &[PAR],
                    format!(
                        "the authorization server refused the test client's authorization request sent straight to it, \
                         and gave a code for one pushed first to {par}; one client was asked, and the other checks, \
                         which send requests straight to the server, were not asked of it"
                    ),
                )
                .with_evidence(vec![
                    "oauth-authorize-without-par".to_owned(),
                    "oauth-par".to_owned(),
                    PUSHED.to_owned(),
                ])
                .in_part(),
            );
        }
        None => not_assessed(
            out,
            format!(
                "The server refused the authorization request sent straight to it, and the one pushed first to {par} \
                 was answered {} without a code either, so the refusal shows nothing.",
                status(&answer)
            ),
        ),
    }
}
