//! The app's own authorization server, asked through the test client (backlog 0625, part 7; ADR-151): a redirect
//! address matched by its start, or a code taken twice, is found; refused, with the normal case working, is credited
//! in part; and every way of not getting that far is said, crediting nothing.

use super::fake_app::{accounts, users};
use super::oauth_client::*;
use super::*;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

/// How the fake server goes wrong.
#[derive(Default, Clone, Copy)]
struct Faults {
    /// Takes any redirect address that starts with the registered one.
    prefix_match: bool,
    /// Takes any redirect address whose host starts with the registered one's.
    host_start_match: bool,
    /// Gives tokens for a code already used.
    code_twice: bool,
    /// Refuses a code used twice, but leaves the first token active.
    keeps_tokens: bool,
    /// Shows a consent page rather than answering with a code.
    consent: bool,
    /// Publishes an introspection address.
    introspects: bool,
    /// Refuses every sign-in code once a changed redirect address has been asked for.
    stops_after_a_change: bool,
    /// Publishes no details about itself, as an app that is no authorization server.
    publishes_nothing: bool,
    /// Crashes on a redirect address it does not take, rather than refusing it.
    crashes_on_a_change: bool,
    /// Answers a code sent a second time with a limiter's 429.
    limits_the_second: bool,
}

const REGISTERED: &str = "http://localhost/callback";
const CLIENT: &str = "sv-test";

/// The client's secret, made at run time so no file holds one.
fn secret() -> String {
    ["cl", "ient-", "pw-", "7Qx2"].concat()
}

struct Server {
    faults: Faults,
    /// Each code given, with its PKCE challenge, and whether it was used.
    codes: BTreeMap<String, (String, bool)>,
    /// The tokens given for each code, and whether each is still active.
    active: BTreeMap<String, bool>,
    given: usize,
    sent: Vec<ProbeRequest>,
    /// A changed redirect address has been asked for.
    changed: bool,
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
    fn new(faults: Faults) -> Self {
        Server {
            faults,
            codes: BTreeMap::new(),
            active: BTreeMap::new(),
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
        if header(request, "cookie") != Some("session=s1") {
            return answer(&request.id, 302, &[("location", "/login")], "");
        }
        let query = params(request.path.split_once('?')?.1);
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
        self.given += 1;
        let code = format!("code{}{}", self.given, "c".repeat(8));
        self.codes
            .insert(code.clone(), (query.get("code_challenge")?.clone(), false));
        let to = format!(
            "{redirect}{}code={code}&state={}",
            if redirect.contains('?') { '&' } else { '?' },
            query.get("state")?
        );
        answer(&request.id, 302, &[("location", &to)], "")
    }

    fn exchange(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let basic = format!(
            "Basic {}",
            // `sv-test:client-pw-7Qx2`, in base64.
            base64_of(&format!("{CLIENT}:{}", secret()))
        );
        if header(request, "authorization") != Some(basic.as_str()) {
            return answer(&request.id, 401, &[], r#"{"error":"invalid_client"}"#);
        }
        let form = params(std::str::from_utf8(request.body.as_deref()?).ok()?);
        let code = form.get("code")?.clone();
        let Some((challenge, used)) = self.codes.get(&code).cloned() else {
            return answer(&request.id, 400, &[], r#"{"error":"invalid_grant"}"#);
        };
        if s256(form.get("code_verifier")?) != challenge {
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
        self.codes.insert(code, (challenge, true));
        let access = self.token();
        let refresh = self.token();
        answer(
            &request.id,
            200,
            &[],
            &format!(
                r#"{{"access_token":"{access}","refresh_token":"{refresh}","token_type":"Bearer"}}"#
            ),
        )
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
                let introspection = if self.faults.introspects {
                    r#","introspection_endpoint":"http://app:8000/o/introspect""#
                } else {
                    ""
                };
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
            _ => answer(&request.id, 404, &[], ""),
        }
    }
}

fn client() -> sv_manifest::OauthClientSection {
    sv_manifest::OauthClientSection {
        id: CLIENT.into(),
        redirect: REGISTERED.into(),
        secret: Some("{seed:CLIENT_SECRET}".into()),
        ..Default::default()
    }
}

fn plain_users() -> UsersSection {
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
fn run_with_client(
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

fn credited<'a>(o: &'a Outcome, rule: &str) -> Option<&'a crate::Verified> {
    o.verified.iter().find(|v| v.check_id == rule)
}

fn found<'a>(o: &'a Outcome, rule: &str) -> Option<&'a Finding> {
    o.findings.iter().find(|f| f.rule_id == rule)
}

#[test]
fn a_server_that_compares_redirect_addresses_whole_and_refuses_a_used_code_is_credited_in_part() {
    let (o, server) = run_with_client(Faults::default(), Some(&client()));
    // The setup worked: a code came back, and it was exchanged.
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("the authorization server sent the browser back")),
        "{:?}",
        o.steps
    );
    assert!(
        server.sent.iter().any(|r| r.id == "oauth-token"),
        "{:?}",
        o.not_assessed
    );
    let redirect =
        credited(&o, REDIRECT_NOT_EXACT.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(redirect.requirement_ids, ["V10.4.1"]);
    assert!(
        redirect
            .scope
            .contains("refused 3 changed redirect addresses"),
        "{}",
        redirect.scope
    );
    let code = credited(&o, CODE_REUSED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(code.requirement_ids, ["V10.4.2"]);
    assert!(
        code.scope.contains("publishes no introspection address"),
        "{}",
        code.scope
    );
    assert!(o.findings.is_empty(), "{:?}", o.findings);
}

#[test]
fn a_redirect_address_matched_by_its_start_is_found() {
    let (o, _) = run_with_client(
        Faults {
            prefix_match: true,
            ..Default::default()
        },
        Some(&client()),
    );
    let f = found(&o, REDIRECT_NOT_EXACT.rule_id).unwrap_or_else(|| panic!("{:?}", o.steps));
    assert_eq!(f.requirement_ids, ["V10.4.1"]);
    assert!(
        f.description.contains("http://localhost/callbacksv"),
        "{}",
        f.description
    );
    assert!(
        f.description.contains("http://localhost/callback?sv=1"),
        "{}",
        f.description
    );
    assert!(credited(&o, REDIRECT_NOT_EXACT.rule_id).is_none());
}

#[test]
fn a_redirect_host_matched_by_its_start_is_found() {
    let (o, _) = run_with_client(
        Faults {
            host_start_match: true,
            ..Default::default()
        },
        Some(&client()),
    );
    let f = found(&o, REDIRECT_NOT_EXACT.rule_id).unwrap_or_else(|| panic!("{:?}", o.steps));
    assert!(
        f.description
            .contains("http://localhost.sv-redirect.invalid/callback"),
        "{}",
        f.description
    );
}

#[test]
fn refusals_from_a_server_that_then_refuses_the_registered_address_too_credit_nothing() {
    let (o, _) = run_with_client(
        Faults {
            stops_after_a_change: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        credited(&o, REDIRECT_NOT_EXACT.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(found(&o, REDIRECT_NOT_EXACT.rule_id).is_none());
    assert!(
        o.not_assessed.iter().any(|(ids, why)| ids == "V10.4.1"
            && why.starts_with("After the changed redirect addresses were refused")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_crash_on_a_changed_redirect_address_is_not_a_refusal() {
    let (o, _) = run_with_client(
        Faults {
            crashes_on_a_change: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        credited(&o, REDIRECT_NOT_EXACT.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(
        o.not_assessed.iter().any(|(ids, why)| ids == "V10.4.1"
            && why.contains("no answer that can be read as a refusal")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_limiter_on_the_second_exchange_is_not_a_refusal() {
    let (o, _) = run_with_client(
        Faults {
            limits_the_second: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        credited(&o, CODE_REUSED.rule_id).is_none(),
        "{:?}",
        o.verified
    );
    assert!(found(&o, CODE_REUSED.rule_id).is_none());
    assert!(
        o.not_assessed
            .iter()
            .any(|(ids, why)| ids == "V10.4.2" && why.contains("answered 429")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_code_exchanged_twice_is_found() {
    let (o, _) = run_with_client(
        Faults {
            code_twice: true,
            ..Default::default()
        },
        Some(&client()),
    );
    let f = found(&o, CODE_REUSED.rule_id).unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert_eq!(f.requirement_ids, ["V10.4.2"]);
    assert_eq!(
        f.title,
        "The authorization server gives tokens for a sign-in code already used"
    );
    assert!(credited(&o, CODE_REUSED.rule_id).is_none());
}

#[test]
fn with_an_introspection_address_the_first_token_is_asked_after_the_code_comes_back() {
    let introspects = Faults {
        introspects: true,
        ..Default::default()
    };
    let (o, server) = run_with_client(introspects, Some(&client()));
    assert!(server.sent.iter().any(|r| r.id == "oauth-introspect"));
    let code = credited(&o, CODE_REUSED.rule_id).unwrap_or_else(|| panic!("{:?}", o.findings));
    assert!(code.scope.contains("no longer active"), "{}", code.scope);

    let (o, _) = run_with_client(
        Faults {
            keeps_tokens: true,
            ..introspects
        },
        Some(&client()),
    );
    let f = found(&o, CODE_REUSED.rule_id).unwrap_or_else(|| panic!("{:?}", o.verified));
    assert_eq!(
        f.title,
        "The authorization server refuses a sign-in code used twice, but leaves its tokens working"
    );
}

#[test]
fn a_consent_page_concludes_nothing_and_says_how_to_skip_it() {
    let (o, _) = run_with_client(
        Faults {
            consent: true,
            ..Default::default()
        },
        Some(&client()),
    );
    assert!(
        o.verified.is_empty() && o.findings.is_empty(),
        "{:?} {:?}",
        o.verified,
        o.findings
    );
    let (ids, why) = &o.not_assessed[0];
    assert_eq!(ids, "V10.4.1, V10.4.2");
    assert!(
        why.contains("consent page") && why.contains("skip_authorization=True"),
        "{why}"
    );
}

#[test]
fn without_a_test_client_an_authorization_server_is_told_both_were_unasked() {
    let (o, server) = run_with_client(Faults::default(), None);
    // Only the published details were asked: no sign-in, no code.
    assert!(
        server
            .sent
            .iter()
            .all(|r| r.path.starts_with("/.well-known/")),
        "{:?}",
        server.sent.iter().map(|r| &r.path).collect::<Vec<_>>()
    );
    let (ids, why) = &o.not_assessed[0];
    assert_eq!(ids, "V10.4.1, V10.4.2");
    assert!(why.contains("[stack.run.oauth-client]"), "{why}");
}

#[test]
fn without_a_test_client_an_app_that_is_no_authorization_server_is_told_nothing() {
    let mut app = Server::new(Faults {
        publishes_nothing: true,
        ..Default::default()
    });
    let mut out = Outcome::default();
    authorization_server_checks(&mut app, &plain_users(), &accounts(), None, &mut out);
    assert!(out.not_assessed.is_empty(), "{:?}", out.not_assessed);
    assert!(out.verified.is_empty() && out.findings.is_empty() && out.steps.is_empty());
}

#[test]
fn a_secret_written_into_stackvet_toml_is_not_used_or_repeated() {
    let written = sv_manifest::OauthClientSection {
        secret: Some(secret()),
        ..client()
    };
    let (o, server) = run_with_client(Faults::default(), Some(&written));
    assert!(server.sent.is_empty());
    let (_, why) = &o.not_assessed[0];
    assert!(why.contains("written into stackvet.toml"), "{why}");
    assert!(!why.contains(&secret()), "{why}");
}

#[test]
fn a_secret_the_seed_did_not_print_is_said_so() {
    let unprinted = sv_manifest::OauthClientSection {
        secret: Some("{seed:OTHER}".into()),
        ..client()
    };
    let (o, server) = run_with_client(Faults::default(), Some(&unprinted));
    assert!(server.sent.is_empty());
    assert!(
        o.not_assessed[0]
            .1
            .contains("printed no value of that name"),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn every_code_and_token_the_server_gave_is_listed_for_blanking() {
    let (o, server) = run_with_client(Faults::default(), Some(&client()));
    let issued: BTreeSet<&str> = o.issued.iter().map(String::as_str).collect();
    // Two codes (the first, and the one asked after the changed addresses) and the first exchange's two tokens.
    assert_eq!(issued.len(), 4, "{issued:?}");
    for code in server.codes.keys() {
        assert!(issued.contains(code.as_str()), "{code} not listed");
    }
    for token in server.active.keys() {
        assert!(issued.contains(token.as_str()), "{token} not listed");
    }
}
