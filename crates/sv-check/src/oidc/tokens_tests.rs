//! Backlog 0500, ADR-127: the access-token audience check (V10.3.1) and the back-channel logout token checks (V10.5.5),
//! against a small app and provider of their own. Each finding and credit is seen given and seen withheld, and each
//! control is seen deciding.

use super::*;
use crate::probes::ProbeResponse;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Clone, Copy)]
struct App {
    /// The private page takes a bearer access token from the provider.
    takes_bearer: bool,
    /// ...and takes one issued for another service as well.
    takes_any_audience: bool,
    /// After refusing one bearer token, refuses every bearer token.
    bearer_breaks: bool,
    /// The back-channel logout address ends sessions for any token at all.
    logout_checks_nothing: bool,
    /// The back-channel logout address ends no session, whatever it is sent.
    logout_does_nothing: bool,
    /// Checks the type and the events, but not the nonce.
    logout_forgets_nonce: bool,
    /// The provider does not mint tokens.
    provider_mints_nothing: bool,
}

#[derive(Default)]
struct Fake {
    app: App,
    next: u32,
    sessions: BTreeSet<String>,
    codes: BTreeSet<String>,
    /// The token the provider minted, by what it is: "self", "other", or a logout variant.
    minted: BTreeMap<String, String>,
    refused_a_bearer: bool,
}

fn answer(status: u16, headers: Vec<(&str, String)>, body: &str) -> ProbeResponse {
    ProbeResponse {
        id: String::new(),
        status,
        headers: headers
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v))
            .collect(),
        body: body.to_owned(),
    }
}

fn header<'a>(r: &'a ProbeRequest, name: &str) -> Option<&'a str> {
    r.headers
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

impl Fake {
    fn id(&mut self) -> String {
        self.next += 1;
        format!("t{}", self.next)
    }

    fn kind_of(&self, token: &str) -> Option<String> {
        self.minted
            .iter()
            .find(|(_, t)| *t == token)
            .map(|(k, _)| k.clone())
    }
}

impl Http for Fake {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        let path = r.path.split('?').next().unwrap_or("").to_owned();
        let sid = header(r, "Cookie")
            .and_then(|c| c.strip_prefix("sid="))
            .map(str::to_owned);
        Some(match path.as_str() {
            "/login/oidc" => {
                let sid = self.id();
                answer(
                    302,
                    vec![
                        (
                            "location",
                            "http://idp:9000/authorize?client_id=client".to_owned(),
                        ),
                        ("set-cookie", format!("sid={sid}; Path=/")),
                    ],
                    "",
                )
            }
            "/callback" => {
                let code = r.path.split("code=").nth(1).unwrap_or("").to_owned();
                match sid {
                    Some(sid) if self.codes.remove(&code) => {
                        self.sessions.insert(sid);
                        answer(302, vec![("location", "/account".to_owned())], "")
                    }
                    _ => answer(400, vec![], ""),
                }
            }
            "/account" => {
                if sid.is_some_and(|s| self.sessions.contains(&s)) {
                    return Some(answer(200, vec![], "your account"));
                }
                let Some(token) =
                    header(r, "Authorization").and_then(|a| a.strip_prefix("Bearer "))
                else {
                    return Some(answer(
                        302,
                        vec![("location", "/login/oidc".to_owned())],
                        "",
                    ));
                };
                let kind = self.kind_of(token);
                let takes = self.app.takes_bearer
                    && !(self.app.bearer_breaks && self.refused_a_bearer)
                    && match kind.as_deref() {
                        Some("self") => true,
                        Some("other") => self.app.takes_any_audience,
                        _ => false,
                    };
                if takes {
                    answer(200, vec![], "your account")
                } else {
                    self.refused_a_bearer = true;
                    answer(401, vec![], "")
                }
            }
            "/backchannel-logout" => {
                let token = r.body_text().trim_start_matches("logout_token=").to_owned();
                let kind = self.kind_of(&token).unwrap_or_default();
                let proper = match kind.as_str() {
                    "good" => true,
                    "with-nonce" => self.app.logout_forgets_nonce,
                    _ => false,
                };
                if !self.app.logout_does_nothing && (proper || self.app.logout_checks_nothing) {
                    self.sessions.clear();
                    answer(200, vec![], "")
                } else {
                    answer(400, vec![], "")
                }
            }
            _ => answer(404, vec![], ""),
        })
    }

    fn provider(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        let path = r.path.split('?').next().unwrap_or("").to_owned();
        let form = r.body_text().into_owned();
        Some(match path.as_str() {
            p if p == idp::MODE => answer(200, vec![], ""),
            "/authorize" => {
                let code = self.id();
                self.codes.insert(code.clone());
                answer(
                    302,
                    vec![("location", format!("http://app/callback?code={code}"))],
                    "",
                )
            }
            p if (p == idp::ACCESS_TOKEN || p == idp::LOGOUT_TOKEN)
                && !self.app.provider_mints_nothing =>
            {
                let kind = form.split('=').nth(1).unwrap_or("").to_owned();
                let token = format!("token-{}-{}", kind, self.id());
                self.minted.insert(kind, token.clone());
                answer(200, vec![], &format!("{{\"token\":\"{token}\"}}"))
            }
            _ => answer(404, vec![], ""),
        })
    }
}

fn section(backchannel: bool) -> OidcSection {
    OidcSection {
        start: "/login/oidc".to_owned(),
        private: "/account".to_owned(),
        backchannel_logout: backchannel.then(|| "/backchannel-logout".to_owned()),
        ..Default::default()
    }
}

fn bearer(app: App) -> Outcome {
    let mut out = Outcome::default();
    tokens::access_token_audience(
        &mut Fake {
            app,
            ..Default::default()
        },
        &section(false),
        &mut out,
    );
    out
}

fn logout(app: App, backchannel: bool) -> Outcome {
    let mut out = Outcome::default();
    tokens::backchannel_logout(
        &mut Fake {
            app,
            ..Default::default()
        },
        &section(backchannel),
        &mut out,
    );
    out
}

fn found(out: &Outcome, rule: &Rule) -> bool {
    out.findings.iter().any(|f| f.rule_id == rule.rule_id)
}

fn credited(out: &Outcome, rule: &Rule) -> bool {
    out.verified
        .iter()
        .any(|v| v.check_id == rule.rule_id && v.in_part)
}

fn not_assessed(out: &Outcome, id: &str) -> Option<String> {
    out.not_assessed
        .iter()
        .find(|(ids, _)| ids.contains(id))
        .map(|(_, why)| why.clone())
}

#[test]
fn a_token_for_another_service_that_opens_the_page_is_a_finding() {
    let out = bearer(App {
        takes_bearer: true,
        takes_any_audience: true,
        ..Default::default()
    });
    assert!(found(&out, &tokens::RESOURCE_AUD), "{:?}", out.findings);
    assert!(!credited(&out, &tokens::RESOURCE_AUD));
}

#[test]
fn a_token_for_another_service_refused_is_credited_in_part() {
    let out = bearer(App {
        takes_bearer: true,
        ..Default::default()
    });
    assert!(
        credited(&out, &tokens::RESOURCE_AUD),
        "{:?}",
        out.not_assessed
    );
    assert!(!found(&out, &tokens::RESOURCE_AUD));
}

#[test]
fn an_app_that_takes_no_bearer_token_is_not_judged_on_whose_it_is() {
    let out = bearer(App::default());
    assert!(!found(&out, &tokens::RESOURCE_AUD) && !credited(&out, &tokens::RESOURCE_AUD));
    assert!(
        not_assessed(&out, "V10.3.1")
            .unwrap()
            .contains("does not take the provider's access tokens")
    );
}

#[test]
fn a_refusal_followed_by_refusing_its_own_token_is_not_credited() {
    let out = bearer(App {
        takes_bearer: true,
        bearer_breaks: true,
        ..Default::default()
    });
    assert!(!credited(&out, &tokens::RESOURCE_AUD));
    assert!(
        not_assessed(&out, "V10.3.1")
            .unwrap()
            .contains("refused its own")
    );
}

#[test]
fn no_token_from_the_provider_asks_nothing() {
    let out = bearer(App {
        takes_bearer: true,
        provider_mints_nothing: true,
        ..Default::default()
    });
    assert!(out.findings.is_empty() && out.verified.is_empty());
    assert!(not_assessed(&out, "V10.3.1").is_some());
}

#[test]
fn a_logout_address_that_takes_any_token_is_a_finding_naming_each_fault() {
    let out = logout(
        App {
            logout_checks_nothing: true,
            ..Default::default()
        },
        true,
    );
    let f = out
        .findings
        .iter()
        .find(|f| f.rule_id == tokens::LOGOUT_TOKEN.rule_id)
        .expect("a finding");
    for fault in ["`typ`", "`events`", "`nonce`"] {
        assert!(f.description.contains(fault), "{}", f.description);
    }
}

#[test]
fn a_logout_address_that_forgets_the_nonce_is_a_finding_for_that_alone() {
    let out = logout(
        App {
            logout_forgets_nonce: true,
            ..Default::default()
        },
        true,
    );
    let f = out
        .findings
        .iter()
        .find(|f| f.rule_id == tokens::LOGOUT_TOKEN.rule_id)
        .expect("a finding");
    assert!(
        f.description.contains("`nonce`") && !f.description.contains("`typ`"),
        "{}",
        f.description
    );
}

#[test]
fn every_wrong_logout_token_refused_where_a_correct_one_works_is_credited_in_part() {
    let out = logout(App::default(), true);
    assert!(
        credited(&out, &tokens::LOGOUT_TOKEN),
        "{:?}",
        out.not_assessed
    );
    assert!(!found(&out, &tokens::LOGOUT_TOKEN));
}

#[test]
fn a_logout_address_that_ends_nothing_credits_nothing() {
    let out = logout(
        App {
            logout_does_nothing: true,
            ..Default::default()
        },
        true,
    );
    assert!(!credited(&out, &tokens::LOGOUT_TOKEN) && !found(&out, &tokens::LOGOUT_TOKEN));
    assert!(
        not_assessed(&out, "V10.5.5")
            .unwrap()
            .contains("did not end the session")
    );
}

#[test]
fn no_logout_address_named_sends_nothing_and_says_why() {
    let mut fake = Fake::default();
    let mut out = Outcome::default();
    tokens::backchannel_logout(&mut fake, &section(false), &mut out);
    assert!(fake.minted.is_empty(), "no token is minted");
    assert!(
        not_assessed(&out, "V10.5.5")
            .unwrap()
            .contains("backchannel-logout")
    );
}

#[test]
fn the_cross_session_probe_cites_v7_6_2() {
    assert!(CROSS_SESSION.requirement_ids.contains(&"V7.6.2"));
}
