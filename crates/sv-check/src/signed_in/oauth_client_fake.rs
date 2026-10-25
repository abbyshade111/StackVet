//! A fake authorization server for the test-client checks (backlog 0625, part 7; ADR-151), following RFC 6749 and RFC
//! 7636 unless a fault says otherwise, and the helpers their tests share.

use super::fake_app::{accounts, users};
use super::oauth_client::*;
use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// How the fake server goes wrong.
#[derive(Default, Clone, Copy)]
pub(super) struct Faults {
    /// Takes any redirect address that starts with the registered one.
    pub(super) prefix_match: bool,
    /// Takes any redirect address whose host starts with the registered one's.
    pub(super) host_start_match: bool,
    /// Gives tokens for a code already used.
    pub(super) code_twice: bool,
    /// Refuses a code used twice, but leaves the first token active.
    pub(super) keeps_tokens: bool,
    /// Shows a consent page rather than answering with a code.
    pub(super) consent: bool,
    /// Publishes an introspection address.
    pub(super) introspects: bool,
    /// Refuses every sign-in code once a changed redirect address has been asked for.
    pub(super) stops_after_a_change: bool,
    /// Publishes no details about itself, as an app that is no authorization server.
    pub(super) publishes_nothing: bool,
    /// Crashes on a redirect address it does not take, rather than refusing it.
    pub(super) crashes_on_a_change: bool,
    /// Answers a code sent a second time with a limiter's 429.
    pub(super) limits_the_second: bool,
    /// Gives a code when asked with no PKCE challenge.
    pub(super) pkce_optional: bool,
    /// Takes PKCE's `plain` method.
    pub(super) plain_accepted: bool,
    /// Exchanges a code whatever verifier comes with it.
    pub(super) verifier_unchecked: bool,
    /// Takes a refresh token again after it was used.
    pub(super) refresh_reusable: bool,
    /// Refuses a used refresh token, but leaves the newer one working.
    pub(super) keeps_newer_refresh: bool,
    /// Gives no new refresh token when one is used.
    pub(super) no_rotation: bool,
    /// Takes the test client as a public one, with no secret.
    pub(super) public: bool,
    /// Refuses every exchange once a code has come with the wrong verifier.
    pub(super) stops_after_wrong_verifier: bool,
    /// Crashes on an authorization request with no PKCE challenge.
    pub(super) crashes_without_pkce: bool,
    /// Answers a used refresh token sent again with a limiter's 429.
    pub(super) limits_refresh_replay: bool,
    /// Exchanges a confidential client's code with no client authentication.
    pub(super) takes_no_client_auth: bool,
    /// Exchanges a confidential client's code with any secret.
    pub(super) takes_any_secret: bool,
    /// Answers an exchange with no client authentication with a limiter's 429.
    pub(super) limits_unauthenticated: bool,
    /// Refuses every exchange once one has come without the right secret.
    pub(super) stops_after_unauthenticated: bool,
    /// Sends the code back in the redirect address's fragment when asked to.
    pub(super) fragment_mode: bool,
    /// Posts the code to the redirect address from a page's form when asked to.
    pub(super) form_post_mode: bool,
    /// Crashes on an authorization request that names a response mode.
    pub(super) crashes_on_response_mode: bool,
    /// Refuses every authorization request once one has named a response mode.
    pub(super) stops_after_response_mode: bool,
    /// Publishes a pushed authorization request address.
    pub(super) publishes_par: bool,
    /// Takes authorization requests only when pushed first (and publishes the address).
    pub(super) requires_par: bool,
    /// Signs the person out for a sign-out request with no `id_token_hint`.
    pub(super) logs_out_without_hint: bool,
    /// Signs the person out for a sign-out request with any `id_token_hint`.
    pub(super) takes_forged_hint: bool,
    /// Publishes no sign-out address.
    pub(super) no_end_session: bool,
    /// Answers authorization requests with a page once a sign-out request has come.
    pub(super) unclear_after_logout: bool,
    /// Crashes, rather than refusing, on an authorization request not pushed first.
    pub(super) crashes_unpushed: bool,
    /// Refuses even a pushed request's `request_uri` at the authorization address.
    pub(super) refuses_pushed_too: bool,
}

pub(super) const REGISTERED: &str = "http://localhost/callback";
pub(super) const CLIENT: &str = "sv-test";

/// The client's secret, made at run time so no file holds one.
pub(super) fn secret() -> String {
    ["cl", "ient-", "pw-", "7Qx2"].concat()
}

pub(super) struct Server {
    faults: Faults,
    /// Each code given, with its PKCE method and challenge when it had one, and whether it was used.
    pub(super) codes: BTreeMap<String, (Option<(String, String)>, bool)>,
    /// Every token given, and whether each is still active.
    pub(super) active: BTreeMap<String, bool>,
    /// Each refresh token given, with the code it comes from, and whether it was used.
    refresh: BTreeMap<String, (String, bool)>,
    given: usize,
    pub(super) sent: Vec<ProbeRequest>,
    /// A code has come with the wrong verifier.
    wrong_verifier_seen: bool,
    /// An exchange has come without the right secret.
    unauthenticated_seen: bool,
    /// An authorization request has named a response mode.
    mode_seen: bool,
    /// The person's session at the server has ended.
    pub(super) logged_out: bool,
    /// A sign-out request has come.
    logout_seen: bool,
    /// Each pushed request's parameters, by its `request_uri`.
    pushed: BTreeMap<String, BTreeMap<String, String>>,
    /// A changed redirect address has been asked for.
    pub(super) changed: bool,
}

fn answer(id: &str, status: u16, headers: &[(&str, &str)], body: &str) -> Option<ProbeResponse> {
    Some(ProbeResponse {
        id: id.to_owned(),
        status,
        headers: headers
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        body: body.to_owned(),
    })
}

fn decoded(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            out.push(u8::from_str_radix(&value[i + 1..i + 3], 16).unwrap());
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).unwrap()
}

fn params(text: &str) -> BTreeMap<String, String> {
    text.split('&')
        .filter_map(|p| p.split_once('='))
        .map(|(k, v)| (k.to_owned(), decoded(v)))
        .collect()
}

fn header<'a>(request: &'a ProbeRequest, name: &str) -> Option<&'a str> {
    request
        .headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}

fn s256(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::new();
    for chunk in digest.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..=chunk.len() {
            out.push(A[((n >> (18 - 6 * i)) & 63) as usize] as char);
        }
    }
    out
}

impl Server {
    pub(super) fn new(faults: Faults) -> Self {
        Server {
            faults,
            codes: BTreeMap::new(),
            active: BTreeMap::new(),
            refresh: BTreeMap::new(),
            wrong_verifier_seen: false,
            unauthenticated_seen: false,
            mode_seen: false,
            logged_out: false,
            logout_seen: false,
            pushed: BTreeMap::new(),
            given: 0,
            sent: Vec::new(),
            changed: false,
        }
    }

    fn takes(&self, redirect: &str) -> bool {
        redirect == REGISTERED
            || (self.faults.prefix_match && redirect.starts_with(REGISTERED))
            || (self.faults.host_start_match && redirect.starts_with("http://localhost"))
    }

    fn token(&mut self) -> String {
        self.given += 1;
        let token = format!("tok{}{}", self.given, "x".repeat(8));
        self.active.insert(token.clone(), true);
        token
    }

    fn authorize(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        if header(request, "cookie") != Some("session=s1") || self.logged_out {
            return answer(&request.id, 302, &[("location", "/login")], "");
        }
        if self.faults.unclear_after_logout && self.logout_seen {
            return answer(&request.id, 200, &[], "<p>Something went wrong.</p>");
        }
        let mut query = params(request.path.split_once('?')?.1);
        if let Some(uri) = query.get("request_uri").cloned() {
            if self.faults.refuses_pushed_too {
                return answer(&request.id, 400, &[], "invalid request_uri");
            }
            let Some(pushed) = self.pushed.get(&uri).cloned() else {
                return answer(&request.id, 400, &[], "invalid request_uri");
            };
            query = pushed;
        } else if self.faults.crashes_unpushed {
            return answer(&request.id, 500, &[], "Internal Server Error");
        } else if self.faults.requires_par {
            return answer(
                &request.id,
                400,
                &[],
                "pushed authorization request required",
            );
        }
        let redirect = query.get("redirect_uri")?.clone();
        self.changed |= redirect != REGISTERED;
        if self.faults.stops_after_a_change && self.changed {
            return answer(&request.id, 400, &[], "temporarily unavailable");
        }
        if query.get("client_id").map(String::as_str) != Some(CLIENT) || !self.takes(&redirect) {
            if self.faults.crashes_on_a_change {
                return answer(&request.id, 500, &[], "Internal Server Error");
            }
            return answer(&request.id, 400, &[], "invalid redirect_uri");
        }
        if self.faults.consent {
            return answer(
                &request.id,
                200,
                &[],
                "<form>Allow sv-test to see your account?</form>",
            );
        }
        let state = query.get("state")?;
        let pkce = match (
            query.get("code_challenge"),
            query.get("code_challenge_method").map(String::as_str),
        ) {
            (Some(c), Some("S256")) => Some(("S256".to_owned(), c.clone())),
            (Some(c), Some("plain")) if self.faults.plain_accepted => {
                Some(("plain".to_owned(), c.clone()))
            }
            (None, _) if self.faults.pkce_optional => None,
            (None, _) if self.faults.crashes_without_pkce => {
                return answer(&request.id, 500, &[], "Internal Server Error");
            }
            // RFC 7636: the error goes back to the client.
            _ => {
                let to = format!("{redirect}?error=invalid_request&state={state}");
                return answer(&request.id, 302, &[("location", &to)], "");
            }
        };
        let mode = query.get("response_mode").map(String::as_str);
        if mode.is_some() {
            self.mode_seen = true;
            if self.faults.crashes_on_response_mode {
                return answer(&request.id, 500, &[], "Internal Server Error");
            }
        } else if self.faults.stops_after_response_mode && self.mode_seen {
            return answer(&request.id, 400, &[], "temporarily unavailable");
        }
        self.given += 1;
        let code = format!("code{}{}", self.given, "c".repeat(8));
        self.codes.insert(code.clone(), (pkce, false));
        match mode {
            Some("fragment") if self.faults.fragment_mode => {
                let to = format!("{redirect}#code={code}&state={state}");
                answer(&request.id, 302, &[("location", &to)], "")
            }
            Some("form_post") if self.faults.form_post_mode => answer(
                &request.id,
                200,
                &[],
                &format!(
                    r#"<form method="post" action="{redirect}"><input type="hidden" name="code" value="{code}"><input type="hidden" name="state" value="{state}"></form>"#
                ),
            ),
            _ => {
                let to = format!(
                    "{redirect}{}code={code}&state={state}",
                    if redirect.contains('?') { '&' } else { '?' },
                );
                answer(&request.id, 302, &[("location", &to)], "")
            }
        }
    }

    fn exchange(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let basic = format!(
            "Basic {}",
            // `sv-test:client-pw-7Qx2`, in base64.
            base64_of(&format!("{CLIENT}:{}", secret()))
        );
        // A public client sends no secret; a confidential one must send the right one.
        match header(request, "authorization") {
            Some(sent) if sent == basic => {}
            Some(_) if self.faults.takes_any_secret => {}
            None if self.faults.public || self.faults.takes_no_client_auth => {}
            None if self.faults.limits_unauthenticated => {
                return answer(&request.id, 429, &[("retry-after", "1")], "");
            }
            _ => {
                self.unauthenticated_seen = true;
                return answer(&request.id, 401, &[], r#"{"error":"invalid_client"}"#);
            }
        }
        if self.faults.stops_after_unauthenticated && self.unauthenticated_seen {
            return answer(&request.id, 400, &[], r#"{"error":"invalid_grant"}"#);
        }
        let form = params(std::str::from_utf8(request.body.as_deref()?).ok()?);
        if form.get("grant_type").map(String::as_str) == Some("refresh_token") {
            return self.refreshed(&request.id, form.get("refresh_token")?);
        }
        let code = form.get("code")?.clone();
        let Some((pkce, used)) = self.codes.get(&code).cloned() else {
            return answer(&request.id, 400, &[], r#"{"error":"invalid_grant"}"#);
        };
        let verifier = form.get("code_verifier").map_or("", String::as_str);
        let verified = self.faults.verifier_unchecked
            || match &pkce {
                None => true,
                Some((method, challenge)) if method == "plain" => verifier == challenge,
                Some((_, challenge)) => s256(verifier) == *challenge,
            };
        if !verified {
            self.wrong_verifier_seen = true;
            return answer(&request.id, 400, &[], r#"{"error":"invalid_grant"}"#);
        }
        if self.faults.stops_after_wrong_verifier && self.wrong_verifier_seen {
            return answer(&request.id, 400, &[], r#"{"error":"invalid_grant"}"#);
        }
        if used && self.faults.limits_the_second {
            return answer(&request.id, 429, &[("retry-after", "1")], "");
        }
        if used && !self.faults.code_twice {
            if !self.faults.keeps_tokens {
                self.active.values_mut().for_each(|a| *a = false);
            }
            return answer(&request.id, 400, &[], r#"{"error":"invalid_grant"}"#);
        }
        self.codes.insert(code.clone(), (pkce, true));
        let access = self.token();
        let refresh = self.token();
        self.refresh.insert(refresh.clone(), (code, false));
        answer(
            &request.id,
            200,
            &[],
            &format!(
                r#"{{"access_token":"{access}","refresh_token":"{refresh}","token_type":"Bearer"}}"#
            ),
        )
    }

    /// RFC 6749's refresh grant, with rotation unless `no_rotation`, and a used token's sign-in revoked when it comes
    /// back unless `keeps_newer_refresh`.
    fn refreshed(&mut self, id: &str, token: &str) -> Option<ProbeResponse> {
        let refused = || answer(id, 400, &[], r#"{"error":"invalid_grant"}"#);
        let Some((code, used)) = self.refresh.get(token).cloned() else {
            return refused();
        };
        if !self.active.get(token).copied().unwrap_or(false) {
            return refused();
        }
        if used && self.faults.limits_refresh_replay {
            return answer(id, 429, &[("retry-after", "1")], "");
        }
        if used && !self.faults.refresh_reusable {
            if !self.faults.keeps_newer_refresh {
                for (t, (c, _)) in &self.refresh {
                    if *c == code {
                        self.active.insert(t.clone(), false);
                    }
                }
            }
            return refused();
        }
        self.refresh.insert(token.to_owned(), (code.clone(), true));
        let access = self.token();
        if self.faults.no_rotation {
            return answer(id, 200, &[], &format!(r#"{{"access_token":"{access}"}}"#));
        }
        let newer = self.token();
        self.refresh.insert(newer.clone(), (code, false));
        answer(
            id,
            200,
            &[],
            &format!(r#"{{"access_token":"{access}","refresh_token":"{newer}"}}"#),
        )
    }

    /// RFC 9126: the request's parameters kept, and a `request_uri` for them given back.
    fn push(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let basic = format!("Basic {}", base64_of(&format!("{CLIENT}:{}", secret())));
        match header(request, "authorization") {
            Some(sent) if sent == basic => {}
            None if self.faults.public => {}
            _ => return answer(&request.id, 401, &[], r#"{"error":"invalid_client"}"#),
        }
        let form = params(std::str::from_utf8(request.body.as_deref()?).ok()?);
        let uri = format!(
            "urn:ietf:params:oauth:request_uri:{}",
            self.pushed.len() + 1
        );
        self.pushed.insert(uri.clone(), form);
        answer(
            &request.id,
            201,
            &[],
            &format!(r#"{{"request_uri":"{uri}","expires_in":60}}"#),
        )
    }

    /// RP-Initiated Logout: with no hint, a page asking the person; with a hint, the hint checked, which no hint
    /// `sv` makes passes.
    fn logout(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.logout_seen = true;
        let hint = request
            .path
            .split_once('?')
            .map(|(_, q)| params(q))
            .and_then(|q| q.get("id_token_hint").cloned());
        let ends = match hint {
            None => self.faults.logs_out_without_hint,
            Some(_) => self.faults.takes_forged_hint,
        };
        if ends {
            self.logged_out = true;
            return answer(&request.id, 302, &[("location", "/")], "");
        }
        match hint {
            None => answer(&request.id, 200, &[], "<form method=post>Sign out?</form>"),
            Some(_) => answer(&request.id, 400, &[], "invalid id_token_hint"),
        }
    }

    fn introspect(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let form = params(std::str::from_utf8(request.body.as_deref()?).ok()?);
        let active = self
            .active
            .get(form.get("token")?)
            .copied()
            .unwrap_or(false);
        answer(&request.id, 200, &[], &format!(r#"{{"active":{active}}}"#))
    }
}

fn base64_of(text: &str) -> String {
    const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in text.as_bytes().chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..=chunk.len() {
            out.push(A[((n >> (18 - 6 * i)) & 63) as usize] as char);
        }
        out.push_str(&"=".repeat(3 - chunk.len()));
    }
    out
}

impl Http for Server {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.sent.push(request.clone());
        let path = request.path.split('?').next().unwrap_or("");
        match (request.method.as_str(), path) {
            ("GET", "/login") => answer(&request.id, 200, &[], "<form method=post></form>"),
            ("POST", "/login") => answer(
                &request.id,
                303,
                &[
                    ("location", "/"),
                    ("set-cookie", "session=s1; Path=/; HttpOnly"),
                ],
                "",
            ),
            ("GET", "/.well-known/openid-configuration") if !self.faults.publishes_nothing => {
                let mut introspection = if self.faults.introspects {
                    r#","introspection_endpoint":"http://app:8000/o/introspect""#
                } else {
                    ""
                }
                .to_owned();
                if self.faults.publishes_par
                    || self.faults.requires_par
                    || self.faults.crashes_unpushed
                {
                    introspection.push_str(
                        r#","pushed_authorization_request_endpoint":"http://app:8000/o/par""#,
                    );
                }
                if !self.faults.no_end_session {
                    introspection.push_str(r#","end_session_endpoint":"http://app:8000/o/logout""#);
                }
                answer(
                    &request.id,
                    200,
                    &[],
                    &format!(
                        r#"{{"issuer":"http://app:8000","authorization_endpoint":"http://app:8000/o/authorize","token_endpoint":"http://app:8000/o/token"{introspection}}}"#
                    ),
                )
            }
            ("GET", "/o/authorize") => self.authorize(request),
            ("POST", "/o/token") => self.exchange(request),
            ("POST", "/o/introspect") => self.introspect(request),
            ("POST", "/o/par") => self.push(request),
            ("GET", "/o/logout") => self.logout(request),
            _ => answer(&request.id, 404, &[], ""),
        }
    }
}

pub(super) fn client() -> sv_manifest::OauthClientSection {
    sv_manifest::OauthClientSection {
        id: CLIENT.into(),
        redirect: REGISTERED.into(),
        secret: Some("{seed:CLIENT_SECRET}".into()),
        ..Default::default()
    }
}

/// The test client as a public one, with no secret.
pub(super) fn public_client() -> sv_manifest::OauthClientSection {
    sv_manifest::OauthClientSection {
        secret: None,
        ..client()
    }
}

pub(super) fn plain_users() -> UsersSection {
    let mut u = users();
    u.login = Some(RequestTemplate {
        method: "POST".into(),
        path: "/login".into(),
        form: [("email", "{user}"), ("password", "{password}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        json: BTreeMap::new(),
    });
    u
}

/// The checks against a server with `faults`, the seed having printed the client's secret.
pub(super) fn run_with_client(
    faults: Faults,
    client: Option<&sv_manifest::OauthClientSection>,
) -> (Outcome, Server) {
    let values: BTreeMap<String, String> = [("CLIENT_SECRET".to_owned(), secret())].into();
    let _seed = SeedValues::set(&values);
    let mut server = Server::new(faults);
    let mut out = Outcome::default();
    authorization_server_checks(&mut server, &plain_users(), &accounts(), client, &mut out);
    (out, server)
}

pub(super) fn credited<'a>(o: &'a Outcome, rule: &str) -> Option<&'a crate::Verified> {
    o.verified.iter().find(|v| v.check_id == rule)
}

pub(super) fn found<'a>(o: &'a Outcome, rule: &str) -> Option<&'a Finding> {
    o.findings.iter().find(|f| f.rule_id == rule)
}
