//! V10.4.12 (backlog 0625, part 7; ADR-151): only the `response_mode` the client needs. The test client takes its code
//! in the query of its redirect address, the code flow's own mode; a code asked to come back in the address's
//! fragment, or posted to it in a page's form, must not come back that way. Refused, or answered in the query as
//! usual, holds the client to its mode.

use super::*;

pub(in crate::signed_in) const RESPONSE_MODE_NOT_HELD: Rule = Rule {
    rule_id: "probe.oauth-response-mode-not-held",
    // V10.4.12: for a given client, only the `response_mode` it needs allowed.
    requirement_ids: &["V10.4.12"],
    cwe: &["CWE-863"],
    impact: "A sign-in link can choose how the code comes back, and a way the app was never built for (a fragment a \
             page's scripts can read, or a form posted from another page) can hand the code to something other than \
             the app.",
    fix: "Register the `response_mode` each client uses and refuse any other, or take authorization requests only \
          through pushed authorization requests (PAR) or signed request objects (JAR).",
};

const RESPONSE_MODE: &str = "V10.4.12";

/// How an answer sent the code back, when it sent one another way than the query: `(how, code)`.
fn other_way(ask: &Ask, answer: &Option<ProbeResponse>) -> Option<(&'static str, String)> {
    let answer = answer.as_ref()?;
    if (300..400).contains(&answer.status) {
        let to = answer.header("location")?;
        let fragment = to
            .strip_prefix(ask.client.redirect.as_str())?
            .split_once('#')?
            .1;
        let code = fragment
            .split('&')
            .find_map(|pair| pair.strip_prefix("code="))?;
        return Some(("in the redirect address's fragment", code.to_owned()));
    }
    if (200..300).contains(&answer.status) && answer.body.contains(&ask.client.redirect) {
        // A form the page posts to the redirect address, with the code in a field.
        let at = answer
            .body
            .find("name=\"code\"")
            .or_else(|| answer.body.find("name='code'"))?;
        let around = near(&answer.body, at);
        let value = around
            .split("value=")
            .nth(1)
            .and_then(|v| v.trim_start_matches(['"', '\'']).split(['"', '\'']).next())
            .unwrap_or("");
        return Some((
            "in a form the page posts to the redirect address",
            value.to_owned(),
        ));
    }
    None
}

/// The text within 200 bytes either side of `at`, cut where a character starts, so a page in any language reads.
fn near(body: &str, at: usize) -> &str {
    let mut from = at.saturating_sub(200);
    while !body.is_char_boundary(from) {
        from -= 1;
    }
    let mut to = (at + 200).min(body.len());
    while !body.is_char_boundary(to) {
        to += 1;
    }
    &body[from..to]
}

pub(super) fn response_mode_check(http: &mut dyn Http, ask: &Ask, out: &mut Outcome) {
    let mut taken = Vec::new();
    let mut unread = Vec::new();
    let mut ids = Vec::new();
    for mode in ["fragment", "form_post"] {
        let id = format!("oauth-authorize-response-mode-{}", mode.replace('_', "-"));
        let answer = ask.authorize_adding(
            http,
            &id,
            &ask.client.redirect,
            Pkce::S256,
            &format!("&response_mode={mode}"),
        );
        let other = other_way(ask, &answer);
        // Answered in the query after all: the mode was not let through, and the code is blanked like any other.
        let in_query = ask.code_in(&answer);
        if let Some(code) = in_query.clone() {
            out.issued.push(code);
        }
        out.steps.push(format!(
            "asked for a sign-in code with `response_mode={mode}`: {}",
            match (&other, &in_query) {
                (Some((how, _)), _) => format!("it came back {how}"),
                (None, Some(_)) => "it came back in the query, as the client takes it".to_owned(),
                (None, None) => format!("refused ({})", status(&answer)),
            }
        ));
        ids.push(id);
        match other {
            Some((how, code)) => {
                if !code.is_empty() {
                    out.issued.push(code);
                }
                taken.push(format!("`response_mode={mode}` sent the code back {how}"));
            }
            // A crash, or a limiter's answer, is not the server refusing (ADR-021).
            None if in_query.is_none() && !refused(&answer) => unread.push(mode),
            None => {}
        }
    }
    if !taken.is_empty() {
        out.findings.push(finding_on(
            ids,
            &RESPONSE_MODE_NOT_HELD,
            "The authorization server sends a client's sign-in code back in a way the client does not use",
            Severity::Medium,
            format!(
                "The test client `{}` takes its code in the query of {}, and {}.",
                ask.client.id,
                ask.client.redirect,
                taken.join("; and ")
            ),
        ));
        return;
    }
    if !unread.is_empty() {
        out.not_assessed.push((
            RESPONSE_MODE.to_owned(),
            format!(
                "Asked for a sign-in code with `response_mode={}`, the authorization server gave no answer that can be \
                 read as a refusal or a code (it crashed, did not answer, or was limiting requests), so nothing was \
                 concluded.",
                unread.join("` and with `response_mode=")
            ),
        ));
        return;
    }
    // Refused, but only worth anything if the registered address still answers in the query after.
    match ask.code(
        http,
        "oauth-authorize-response-mode-after",
        &ask.client.redirect,
    ) {
        Ok(code) => out.issued.push(code),
        Err(why) => {
            out.not_assessed.push((
                RESPONSE_MODE.to_owned(),
                format!("After the other response modes were refused, the usual one was asked again: {why}"),
            ));
            return;
        }
    }
    out.verified.push(
        crate::Verified::new(
            RESPONSE_MODE_NOT_HELD.rule_id,
            &[RESPONSE_MODE],
            "asked for the test client's sign-in code with `response_mode=fragment` and `response_mode=form_post`, the \
             authorization server refused or answered in the query, as the client takes it, and answered in the query \
             after; one client and two modes were asked"
                .to_owned(),
        )
        .with_evidence(ids)
        .in_part(),
    );
}
