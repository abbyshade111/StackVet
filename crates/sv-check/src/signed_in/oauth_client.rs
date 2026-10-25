//! The app's own authorization server, asked through the test client stackvet.toml declares (backlog 0625, part 7;
//! ADR-151).
//!
//! An app other apps sign their users in through is asked what it does when a client sends what it should refuse: a
//! redirect address that is not the registered one (V10.4.1), and a sign-in code sent a second time (V10.4.2). The
//! first test user is signed in, and the client is one the seed registered in the copy of the app the run started,
//! set to skip the consent page. Each question is asked only once the normal case is shown working: the registered
//! address answered with a code, and a code exchanged for a token. A refusal is credited in part, since one client, one
//! person, and a sample of wrong requests are asked.

use super::*;
use sha2::{Digest, Sha256};
use sv_manifest::OauthClientSection;

pub(super) const REDIRECT_NOT_EXACT: Rule = Rule {
    rule_id: "probe.oauth-redirect-not-exact",
    // V10.4.1: redirect addresses checked against the client's registered ones by exact string comparison.
    requirement_ids: &["V10.4.1"],
    cwe: &["CWE-601"],
    impact: "Whoever owns the address the server accepted can have a person's sign-in code, and with it their account \
             in the app that asked, sent to them by a link that starts at the real sign-in page.",
    fix: "Compare the redirect address a client sends with the ones registered for that client, as whole strings, and \
          refuse anything else before signing anyone in; never match by prefix, by host alone, or by pattern.",
};

pub(super) const CODE_REUSED: Rule = Rule {
    rule_id: "probe.oauth-code-reused",
    // V10.4.2: a sign-in code used only once, and the tokens it gave revoked when it comes back.
    requirement_ids: &["V10.4.2"],
    cwe: &["CWE-294"],
    impact: "A sign-in code that leaks after use, from a log, a browser's history, or a referrer, still gets tokens for \
             the person it was issued to.",
    fix: "Mark a code used when it is exchanged, refuse it after that, and revoke the tokens it gave when it comes back \
          (RFC 6749, section 4.1.2).",
};

const REDIRECT: &str = "V10.4.1";
const CODE: &str = "V10.4.2";

/// Where a person's address goes when the server sends the browser on: never a real site.
const ELSEWHERE: &str = "sv-redirect.invalid";

/// The two published details documents, as part 6 asks them (ADR-147).
const DETAILS: &[(&str, &str)] = crate::as_metadata::ASKED;

/// What the server publishes about itself and what the entry says, put together.
struct Server {
    authorize: String,
    token: String,
    introspect: Option<String>,
}

pub(super) fn authorization_server_checks(
    http: &mut dyn Http,
    users: &UsersSection,
    accounts: &Accounts,
    client: Option<&OauthClientSection>,
    out: &mut Outcome,
) {
    let not_assessed = |out: &mut Outcome, why: String| {
        out.not_assessed.push((format!("{REDIRECT}, {CODE}"), why));
    };
    let Some(client) = client else {
        // Said only to an app that answers as an authorization server: to every other app with signed-in pages, a
        // test client means nothing.
        if published(http).is_none() {
            return;
        }
        not_assessed(
            out,
            "stackvet.toml declares no test client for the app's own authorization server, so what it does with a \
             changed redirect address and a sign-in code used twice was not asked. If other apps sign their users in \
             through this one, have the seed register a client in the copy `sv` runs, set to skip the consent page, \
             and add `[stack.run.oauth-client]` with its `id`, `redirect`, and `secret = \"{seed:NAME}\"`."
                .to_owned(),
        );
        return;
    };
    let secret = match client.secret.as_deref() {
        None => None,
        Some(written) if !is_seed_value(written) => {
            // Never repeated: it may be a real key.
            not_assessed(
                out,
                "The test client's `secret` under `[stack.run.oauth-client]` is written into stackvet.toml, which is \
                 committed, so it was not used. Have the seed print it as a `NAME=value` line and write \
                 `secret = \"{seed:NAME}\"`."
                    .to_owned(),
            );
            return;
        }
        Some(cited) => {
            let filled = fill_seed(cited);
            if filled.contains("{seed:") {
                not_assessed(
                    out,
                    format!(
                        "The test client's `secret` is {cited}, and the seed printed no value of that name, so the \
                         client could not be used."
                    ),
                );
                return;
            }
            Some(filled)
        }
    };
    let details = published(http);
    let Some(server) = server(details.as_ref(), client) else {
        not_assessed(
            out,
            "The app published no details about its authorization server at /.well-known/oauth-authorization-server \
             or /.well-known/openid-configuration, and `[stack.run.oauth-client]` gives no `authorize` and `token` \
             paths, so `sv` did not know where to ask."
                .to_owned(),
        );
        return;
    };
    let mut steps = Vec::new();
    let signed = sign_in(http, users, "oauth-client", &accounts.a, &mut steps)
        // Its own line says whether the app took the password.
        .filter(|_| steps.last().is_some_and(|l| l.starts_with("signed in as")))
        .map(|s| s.session);
    out.steps.append(&mut steps);
    let Some(session) = signed else {
        not_assessed(
            out,
            "The first test user could not be signed in for the authorization-server checks, so no sign-in code \
             could be asked for."
                .to_owned(),
        );
        return;
    };
    let verifier = verifier(&accounts.spare);
    let ask = Ask {
        client,
        server: &server,
        session: &session,
        verifier: &verifier,
    };

    // The normal case, first: the registered address answers with a code.
    let first = match ask.code(http, "oauth-authorize", &client.redirect) {
        Ok(code) => code,
        Err(why) => {
            not_assessed(out, why);
            return;
        }
    };
    out.issued.push(first.clone());
    out.steps.push(format!(
        "the authorization server sent the browser back to the test client's redirect address with a sign-in code ({})",
        server.authorize
    ));

    redirect_check(http, &ask, out);
    code_check(http, &ask, secret.as_deref(), &first, out);
}

/// Whether the entry's secret is a value the seed prints, and nothing else.
fn is_seed_value(written: &str) -> bool {
    written.starts_with("{seed:") && written.ends_with('}') && written.matches('{').count() == 1
}

/// The first of the two published details documents the app answers with: a 200 whose body is a JSON object with an
/// `issuer`, as part 6 reads them (ADR-147).
fn published(http: &mut dyn Http) -> Option<serde_json::Map<String, serde_json::Value>> {
    for (id, path) in DETAILS {
        let answer = http.send(&ProbeRequest {
            id: format!("oauth-client-{id}"),
            method: "GET".into(),
            path: (*path).into(),
            headers: vec![("Accept".into(), "application/json".into())],
            body: None,
        });
        if let Some(serde_json::Value::Object(document)) = answer
            .filter(|r| r.status == 200)
            .and_then(|r| serde_json::from_str(&r.body).ok())
            && document.contains_key("issuer")
        {
            return Some(document);
        }
    }
    None
}

/// Where to ask: the entry's own paths first, the published details where it gives none.
fn server(
    published: Option<&serde_json::Map<String, serde_json::Value>>,
    client: &OauthClientSection,
) -> Option<Server> {
    let field = |name: &str| {
        published
            .and_then(|d| d.get(name))
            .and_then(serde_json::Value::as_str)
            .map(|address| strip_origin(address).to_owned())
    };
    Some(Server {
        authorize: client
            .authorize
            .clone()
            .or_else(|| field("authorization_endpoint"))?,
        token: client.token.clone().or_else(|| field("token_endpoint"))?,
        introspect: field("introspection_endpoint"),
    })
}

/// A PKCE code verifier from the run's random letters: 43 characters, RFC 7636's shortest.
fn verifier(spare: &str) -> String {
    let letters: String = spare.chars().filter(char::is_ascii_alphanumeric).collect();
    format!("{letters:0<43}").chars().take(43).collect()
}

/// RFC 7636's S256 challenge for `verifier`.
fn challenge(verifier: &str) -> String {
    base64(&Sha256::digest(verifier.as_bytes()), true)
}

/// Base64 of `bytes`: URL-safe and unpadded, as PKCE and JWTs write it, or standard with padding, as HTTP Basic does.
fn base64(bytes: &[u8], url_safe: bool) -> String {
    let alphabet: &[u8] = if url_safe {
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
    } else {
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    };
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..=chunk.len() {
            out.push(alphabet[((n >> (18 - 6 * i)) & 63) as usize] as char);
        }
        if !url_safe {
            out.push_str(&"=".repeat(3 - chunk.len()));
        }
    }
    out
}

/// A value written into a query or a form, every character but RFC 3986's unreserved ones encoded.
fn encoded(value: &str) -> String {
    value
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// The value of `name` in the query of `address`, as written.
fn query_value<'a>(address: &'a str, name: &str) -> Option<&'a str> {
    let query = address.split_once('?')?.1.split('#').next()?;
    query
        .split('&')
        .find_map(|pair| pair.strip_prefix(name)?.strip_prefix('='))
}

/// Everything one question needs.
struct Ask<'a> {
    client: &'a OauthClientSection,
    server: &'a Server,
    session: &'a Session,
    verifier: &'a str,
}

impl Ask<'_> {
    /// The authorization request, for `redirect`, with PKCE's S256 challenge.
    fn authorize(&self, http: &mut dyn Http, id: &str, redirect: &str) -> Option<ProbeResponse> {
        let scope = self.client.scope.as_deref().unwrap_or("openid");
        let path = format!(
            "{}{}response_type=code&client_id={}&redirect_uri={}&scope={}&state={id}&code_challenge={}\
             &code_challenge_method=S256",
            self.server.authorize,
            if self.server.authorize.contains('?') {
                '&'
            } else {
                '?'
            },
            encoded(&self.client.id),
            encoded(redirect),
            encoded(scope),
            challenge(self.verifier),
        );
        http.send(&get(id, &path, self.session))
    }

    /// A code for the registered address, or why none came, in words for the report.
    fn code(&self, http: &mut dyn Http, id: &str, redirect: &str) -> Result<String, String> {
        let answer = self.authorize(http, id, redirect);
        let location = answer
            .as_ref()
            .filter(|r| (300..400).contains(&r.status))
            .and_then(|r| r.header("location"));
        match (&answer, location) {
            (None, _) => Err(format!(
                "The authorization address, {}, did not answer, so no sign-in code could be had.",
                self.server.authorize
            )),
            (Some(_), Some(to)) if to.starts_with(redirect) => match query_value(to, "code") {
                Some(code) if query_value(to, "state") == Some(id) => Ok(code.to_owned()),
                Some(_) => Err(format!(
                    "The authorization server sent the browser back to the test client with a code, but not with the \
                     `state` it was sent, so the answer could not be told apart from another; nothing was concluded \
                     from {}.",
                    self.server.authorize
                )),
                None => Err(format!(
                    "The authorization server sent the browser back to the test client's redirect address without a \
                     code (with {}), so nothing more was asked.",
                    query_value(to, "error")
                        .map_or("no error named".to_owned(), |e| format!("`error={e}`"))
                )),
            },
            (Some(r), Some(to)) => Err(format!(
                "Asked for a sign-in code for the test client, the authorization server answered {} and sent the \
                 browser to {}, not to the registered redirect address, so no code was had. If that is the sign-in \
                 page, the first test user's sign-in did not carry over; if it is a consent page, have the seed \
                 register the client to skip it.",
                r.status,
                strip_origin(to).split(['?', '#']).next().unwrap_or(""),
            )),
            (Some(r), None) => Err(format!(
                "Asked for a sign-in code for the test client, the authorization server answered {} with a page \
                 rather than sending the browser back with a code. That is most often a consent page, which `sv` does \
                 not click through: have the seed register the client to skip it (with django-oauth-toolkit, \
                 `skip_authorization=True`).",
                r.status
            )),
        }
    }

    /// The token request for `code`, with the client's secret in HTTP Basic when it has one, as RFC 6749 asks a
    /// server to take it. The record never keeps a request's headers.
    fn exchange(
        &self,
        http: &mut dyn Http,
        id: &str,
        code: &str,
        secret: Option<&str>,
    ) -> Option<ProbeResponse> {
        let mut headers = vec![
            (
                "Content-Type".to_owned(),
                "application/x-www-form-urlencoded".to_owned(),
            ),
            ("Accept".to_owned(), "application/json".to_owned()),
        ];
        if let Some(secret) = secret {
            let pair = format!("{}:{}", encoded(&self.client.id), encoded(secret));
            headers.push((
                "Authorization".to_owned(),
                format!("Basic {}", base64(pair.as_bytes(), false)),
            ));
        }
        http.send(&ProbeRequest {
            id: id.to_owned(),
            method: "POST".to_owned(),
            path: self.server.token.clone(),
            headers,
            body: Some(format!(
                "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&code_verifier={}",
                encoded(code),
                encoded(&self.client.redirect),
                encoded(&self.client.id),
                self.verifier
            )
            .into_bytes()),
        })
    }
}

/// The access token in a token answer, when it is a 2xx with one.
fn access_token(answer: &Option<ProbeResponse>) -> Option<String> {
    let answer = answer.as_ref().filter(|r| (200..300).contains(&r.status))?;
    serde_json::from_str::<serde_json::Value>(&answer.body)
        .ok()?
        .get("access_token")?
        .as_str()
        .map(str::to_owned)
}

/// Every token in a token answer, for blanking from the record.
fn tokens_in(answer: &Option<ProbeResponse>) -> Vec<String> {
    let Some(value) = answer
        .as_ref()
        .and_then(|r| serde_json::from_str::<serde_json::Value>(&r.body).ok())
    else {
        return Vec::new();
    };
    ["access_token", "refresh_token", "id_token"]
        .iter()
        .filter_map(|name| value.get(*name)?.as_str().map(str::to_owned))
        .collect()
}

/// The registered address changed three ways, as `(what, address)`: longer by a few letters, with a query added, and
/// with its host made the start of a host `sv` names. An exact comparison refuses all three; a prefix match takes the
/// first two, and a host matched by its start takes the third.
fn variants(registered: &str) -> Vec<(&'static str, String)> {
    let (base, query) = registered
        .split_once('?')
        .map_or((registered, None), |(b, q)| (b, Some(q)));
    let with_query = |base: &str| match query {
        Some(q) => format!("{base}?{q}"),
        None => base.to_owned(),
    };
    let mut out = vec![
        (
            "the registered address with letters added to its path",
            with_query(&format!("{base}sv")),
        ),
        (
            "the registered address with a query added",
            match query {
                Some(q) => format!("{base}?{q}&sv=1"),
                None => format!("{base}?sv=1"),
            },
        ),
    ];
    if let Some((scheme, rest)) = base.split_once("://") {
        let (host, path) = rest.split_once('/').map_or((rest, ""), |(h, p)| (h, p));
        let (name, port) = host
            .split_once(':')
            .map_or((host, None), |(n, p)| (n, Some(p)));
        out.push((
            "the registered address's host made the start of another host",
            with_query(&format!(
                "{scheme}://{name}.{ELSEWHERE}{}/{path}",
                port.map(|p| format!(":{p}")).unwrap_or_default()
            )),
        ));
    }
    out
}

/// V10.4.1: each changed address asked for, and the registered one asked again after.
fn redirect_check(http: &mut dyn Http, ask: &Ask, out: &mut Outcome) {
    let mut taken = Vec::new();
    let mut ids = Vec::new();
    for (i, (what, address)) in variants(&ask.client.redirect).into_iter().enumerate() {
        let id = format!("oauth-authorize-changed-{}", i + 1);
        let answer = ask.authorize(http, &id, &address);
        let sent_there = answer
            .as_ref()
            .filter(|r| (300..400).contains(&r.status))
            .and_then(|r| r.header("location"))
            .is_some_and(|to| to.starts_with(&address));
        out.steps.push(format!(
            "asked for a sign-in code with {what}: {}",
            if sent_there {
                "the server sent the browser there".to_owned()
            } else {
                format!("refused ({})", status(&answer))
            }
        ));
        if let Some(code) = answer
            .as_ref()
            .and_then(|r| r.header("location"))
            .and_then(|to| query_value(to, "code"))
        {
            out.issued.push(code.to_owned());
        }
        ids.push(id);
        if sent_there {
            taken.push((what, address));
        }
    }
    if !taken.is_empty() {
        out.findings.push(finding_on(
            ids,
            &REDIRECT_NOT_EXACT,
            "The authorization server sends sign-in codes to redirect addresses that were never registered",
            Severity::High,
            format!(
                "Asked to sign the first test user in for the test client `{}`, whose registered redirect address is \
                 {}, the server sent the browser on with a code to {}.",
                ask.client.id,
                ask.client.redirect,
                taken
                    .iter()
                    .map(|(what, address)| format!("{address} ({what})"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
        return;
    }
    // Refused, but only worth anything if the registered address still answers: a server that refuses everything
    // shows nothing.
    match ask.code(http, "oauth-authorize-again", &ask.client.redirect) {
        Ok(code) => {
            out.issued.push(code);
            out.verified.push(
                crate::Verified::new(
                    REDIRECT_NOT_EXACT.rule_id,
                    &[REDIRECT],
                    format!(
                        "the authorization server refused {} changed redirect addresses for one test client, and \
                         answered the registered one with a code before and after; other clients, and other changes, \
                         were not asked",
                        ids.len()
                    ),
                )
                .with_evidence(ids)
                .in_part(),
            );
        }
        Err(why) => out.not_assessed.push((
            REDIRECT.to_owned(),
            format!("After the changed redirect addresses were refused, the registered one was asked again: {why}"),
        )),
    }
}

/// V10.4.2: the first code exchanged, then sent again; and, where the server publishes an introspection address,
/// whether the token the first exchange gave still works.
fn code_check(http: &mut dyn Http, ask: &Ask, secret: Option<&str>, code: &str, out: &mut Outcome) {
    let first = ask.exchange(http, "oauth-token", code, secret);
    out.issued.extend(tokens_in(&first));
    let Some(token) = access_token(&first) else {
        out.not_assessed.push((
            CODE.to_owned(),
            format!(
                "The sign-in code was sent to the token address, {}, with the PKCE verifier{}, and was not exchanged for \
                 an access token ({}), so whether it can be used twice was not asked.",
                ask.server.token,
                if secret.is_some() { " and the client's secret" } else { "" },
                status(&first)
            ),
        ));
        return;
    };
    let again = ask.exchange(http, "oauth-token-again", code, secret);
    out.issued.extend(tokens_in(&again));
    out.steps.push(format!(
        "exchanged the sign-in code for an access token ({}), then sent the same code again ({})",
        status(&first),
        status(&again)
    ));
    let ids = vec!["oauth-token".to_owned(), "oauth-token-again".to_owned()];
    if access_token(&again).is_some() {
        out.findings.push(finding_on(
            ids,
            &CODE_REUSED,
            "The authorization server gives tokens for a sign-in code already used",
            Severity::High,
            format!(
                "A sign-in code for the test client `{}` was exchanged at {} for an access token, and the same code, \
                 sent again, was exchanged for another ({}).",
                ask.client.id,
                ask.server.token,
                status(&again)
            ),
        ));
        return;
    }
    if !again
        .as_ref()
        .is_some_and(|r| (400..500).contains(&r.status))
    {
        out.not_assessed.push((
            CODE.to_owned(),
            format!(
                "The sign-in code, sent a second time, was answered {}, neither a token nor a refusal, so nothing was \
                 concluded.",
                status(&again)
            ),
        ));
        return;
    }
    // Refused. Whether the token the first exchange gave was revoked, where the server says how to ask.
    let revoked = ask
        .server
        .introspect
        .as_ref()
        .zip(secret)
        .map(|(path, secret)| still_active(http, ask, path, secret, &token));
    match revoked {
        Some(Some(true)) => out.findings.push(finding_on(
            [ids, vec!["oauth-introspect".to_owned()]].concat(),
            &CODE_REUSED,
            "The authorization server refuses a sign-in code used twice, but leaves its tokens working",
            Severity::Medium,
            format!(
                "A sign-in code for the test client `{}`, sent a second time, was refused ({}), and the access token \
                 its first exchange gave was still active when asked at {}. V10.4.2 asks that the tokens a code gave \
                 be revoked when it comes back.",
                ask.client.id,
                status(&again),
                ask.server.introspect.as_deref().unwrap_or("")
            ),
        )),
        Some(Some(false)) => out.verified.push(
            crate::Verified::new(
                CODE_REUSED.rule_id,
                &[CODE],
                "the authorization server refused a sign-in code sent a second time, and the access token its first \
                 exchange gave was no longer active; one client and one code were asked"
                    .to_owned(),
            )
            .with_evidence([ids, vec!["oauth-introspect".to_owned()]].concat())
            .in_part(),
        ),
        _ => out.verified.push(
            crate::Verified::new(
                CODE_REUSED.rule_id,
                &[CODE],
                format!(
                    "the authorization server refused a sign-in code sent a second time; whether it revoked the tokens \
                     the code gave was not asked, since {}",
                    if secret.is_none() {
                        "the test client is a public one, which an introspection address does not answer"
                    } else {
                        "the server publishes no introspection address"
                    }
                ),
            )
            .with_evidence(ids)
            .in_part(),
        ),
    }
}

/// RFC 7662's answer for `token`: `Some(true)` active, `Some(false)` not, `None` when it gave no answer to read.
fn still_active(
    http: &mut dyn Http,
    ask: &Ask,
    path: &str,
    secret: &str,
    token: &str,
) -> Option<bool> {
    let pair = format!("{}:{}", encoded(&ask.client.id), encoded(secret));
    let answer = http
        .send(&ProbeRequest {
            id: "oauth-introspect".to_owned(),
            method: "POST".to_owned(),
            path: path.to_owned(),
            headers: vec![
                (
                    "Content-Type".to_owned(),
                    "application/x-www-form-urlencoded".to_owned(),
                ),
                ("Accept".to_owned(), "application/json".to_owned()),
                (
                    "Authorization".to_owned(),
                    format!("Basic {}", base64(pair.as_bytes(), false)),
                ),
            ],
            body: Some(format!("token={}", encoded(token)).into_bytes()),
        })
        .filter(|r| r.status == 200)?;
    serde_json::from_str::<serde_json::Value>(&answer.body)
        .ok()?
        .get("active")?
        .as_bool()
}
