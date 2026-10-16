//! Backlog 0232, part B: the session a sign-in through another service leaves, judged as the form's sign-in is: a new
//! session at sign-in (V7.2.4), and the cookie's HttpOnly and SameSite (V3.3.4, V3.3.2), each finding resting on the
//! provider flow's own requests.

use super::*;
use crate::probes::ProbeResponse;
use std::collections::BTreeSet;

#[derive(Default, Clone, Copy)]
struct App {
    /// The provider's return signs in the session the start page made, and sets no new cookie.
    keeps_session_from_before: bool,
    /// The new session cookie lacks HttpOnly and SameSite.
    bare_cookie: bool,
}

#[derive(Default)]
struct Fake {
    app: App,
    next: u32,
    sessions: BTreeSet<String>,
    codes: BTreeSet<String>,
}

fn answer(status: u16, headers: Vec<(&str, String)>) -> ProbeResponse {
    ProbeResponse {
        id: String::new(),
        status,
        headers: headers
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v))
            .collect(),
        body: String::new(),
    }
}

/// Every cookie value the request carries.
fn values(r: &ProbeRequest) -> Vec<String> {
    r.headers
        .iter()
        .filter(|(k, _)| k == "Cookie")
        .flat_map(|(_, v)| {
            v.split("; ")
                .filter_map(|c| c.split_once('='))
                .map(|(_, v)| v.to_owned())
        })
        .collect()
}

impl Http for Fake {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        self.next += 1;
        let path = r.path.split('?').next().unwrap_or("").to_owned();
        Some(match path.as_str() {
            "/login/oidc" => answer(
                302,
                vec![
                    (
                        "location",
                        "http://idp:9000/authorize?client_id=client".to_owned(),
                    ),
                    ("set-cookie", format!("pre=p{}; Path=/", self.next)),
                ],
            ),
            "/callback" => {
                let code = r.path.split("code=").nth(1).unwrap_or("").to_owned();
                if !self.codes.remove(&code) {
                    return Some(answer(400, vec![]));
                }
                if self.app.keeps_session_from_before {
                    self.sessions.extend(values(r));
                    return Some(answer(302, vec![("location", "/account".to_owned())]));
                }
                let sid = format!("s{}", self.next);
                self.sessions.insert(sid.clone());
                let attrs = if self.app.bare_cookie {
                    "Path=/"
                } else {
                    "HttpOnly; SameSite=Lax; Path=/"
                };
                answer(
                    302,
                    vec![
                        ("location", "/account".to_owned()),
                        ("set-cookie", format!("sid={sid}; {attrs}")),
                    ],
                )
            }
            "/account" => {
                if values(r).iter().any(|v| self.sessions.contains(v)) {
                    answer(200, vec![])
                } else {
                    answer(302, vec![("location", "/login/oidc".to_owned())])
                }
            }
            _ => answer(404, vec![]),
        })
    }

    fn provider(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        let path = r.path.split('?').next().unwrap_or("").to_owned();
        Some(match path.as_str() {
            p if p == idp::MODE => answer(200, vec![]),
            "/authorize" => {
                self.next += 1;
                let code = format!("c{}", self.next);
                self.codes.insert(code.clone());
                answer(
                    302,
                    vec![("location", format!("http://app/callback?code={code}"))],
                )
            }
            _ => answer(404, vec![]),
        })
    }
}

fn run(app: App) -> Outcome {
    let section = OidcSection {
        start: "/login/oidc".to_owned(),
        private: "/account".to_owned(),
        ..Default::default()
    };
    let mut out = Outcome::default();
    session::provider_session(
        &mut Fake {
            app,
            ..Default::default()
        },
        &section,
        &mut out,
    );
    out
}

fn finding<'a>(out: &'a Outcome, rule: &str) -> Option<&'a crate::Finding> {
    out.findings.iter().find(|f| f.rule_id == rule)
}

fn credited(out: &Outcome, rule: &str) -> bool {
    out.verified.iter().any(|v| v.check_id == rule)
}

#[test]
fn a_new_protected_session_at_the_provider_s_return_is_credited() {
    let out = run(App::default());
    assert!(
        credited(&out, "probe.session-not-renewed"),
        "{:?} {:?}",
        out.findings,
        out.not_assessed
    );
    assert!(
        credited(&out, "probe.session-cookie-attributes"),
        "{:?}",
        out.findings
    );
    assert!(out.findings.is_empty(), "{:?}", out.findings);
}

#[test]
fn the_session_from_before_sign_in_kept_is_a_finding_on_the_provider_flow() {
    let out = run(App {
        keeps_session_from_before: true,
        ..Default::default()
    });
    let f = finding(&out, "probe.session-not-renewed").expect("a finding");
    assert_eq!(f.evidence, ["oidc-start-session", "oidc-back-session"]);
    assert!(!credited(&out, "probe.session-not-renewed"));
}

#[test]
fn a_session_cookie_without_its_attributes_is_a_finding_on_the_provider_s_return() {
    let out = run(App {
        bare_cookie: true,
        ..Default::default()
    });
    let f = finding(&out, "probe.session-cookie-attributes").expect("a finding");
    assert_eq!(f.evidence, ["oidc-back-session"]);
    assert!(
        f.description.contains("no HttpOnly") && f.description.contains("no SameSite"),
        "{}",
        f.description
    );
}
