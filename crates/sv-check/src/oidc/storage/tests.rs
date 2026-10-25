//! Backlog 0629: the browser's storage after signing out of a sign-in through the test provider. The session the
//! provider's sign-in opens is what the browser is handed, the sign-out control looked for is the one leading to
//! `[stack.run.oidc] logout`, and the credit and the finding name this way in.

use super::*;
use crate::browser::{Action, Job, KEPT_AFTER_SIGN_OUT};
use crate::probes::ProbeResponse;
use serde_json::{Value, json};

/// What the browser's storage does on signing out: "cleared", "kept", or "absent" (no browser).
#[derive(Default)]
struct Fake {
    storage: &'static str,
    next: u32,
    code: Option<String>,
    sid: Option<String>,
    jobs: Vec<Job>,
    /// The provider's mode, as last set, and what it was each time the browser was given a job.
    mode: String,
    modes_in_jobs: Vec<String>,
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

impl Http for Fake {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        self.next += 1;
        let path = r.path.split('?').next().unwrap_or("").to_owned();
        Some(match path.as_str() {
            "/login/oidc" => answer(
                302,
                vec![(
                    "location",
                    "http://idp:9000/authorize?client_id=client".to_owned(),
                )],
            ),
            "/callback" => {
                let code = r.path.split("code=").nth(1).unwrap_or("").to_owned();
                if self.code.take().as_deref() != Some(code.as_str()) {
                    return Some(answer(400, vec![]));
                }
                let sid = format!("s{}", self.next);
                self.sid = Some(sid.clone());
                answer(
                    302,
                    vec![
                        ("location", "/account".to_owned()),
                        ("set-cookie", format!("sid={sid}; HttpOnly; Path=/")),
                    ],
                )
            }
            _ => answer(404, vec![]),
        })
    }

    fn provider(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        let path = r.path.split('?').next().unwrap_or("").to_owned();
        Some(match path.as_str() {
            p if p == idp::MODE => {
                let body = r
                    .body
                    .as_deref()
                    .map(String::from_utf8_lossy)
                    .unwrap_or_default();
                self.mode = body.strip_prefix("mode=").unwrap_or("").to_owned();
                answer(200, vec![])
            }
            "/authorize" => {
                self.next += 1;
                let code = format!("c{}", self.next);
                self.code = Some(code.clone());
                answer(
                    302,
                    vec![("location", format!("http://app/callback?code={code}"))],
                )
            }
            _ => answer(404, vec![]),
        })
    }

    /// The real browser: signed in only by the cookie the provider's sign-in set, and signed out by the control.
    fn browser(&mut self, job: &Job) -> Option<Vec<Value>> {
        self.jobs.push(job.clone());
        self.modes_in_jobs.push(self.mode.clone());
        if self.storage == "absent" {
            return None;
        }
        let mut signed = false;
        let mut looks = 0;
        Some(
            job.actions
                .iter()
                .map(|action| match action {
                    Action::SetCookies(c) => {
                        signed = c.iter().any(|c| Some(&c.value) == self.sid.as_ref());
                        json!({ "refused": [] })
                    }
                    Action::Goto(p) if signed && p == "/account" => {
                        json!({ "status": 200, "path": p })
                    }
                    Action::Goto(p) if p == "/" => json!({ "status": 200, "path": "/" }),
                    Action::Goto(_) => json!({ "status": 200, "path": "/login/oidc" }),
                    Action::Act(_) => {
                        signed = false;
                        json!({ "found": true, "after": { "status": 200, "path": "/" } })
                    }
                    Action::Eval(_) => {
                        looks += 1;
                        let mut local = vec!["theme"];
                        if looks == 2 || (looks == 3 && self.storage == "kept") {
                            local.push("token");
                        }
                        json!({ "value": { "local": local, "session": [], "indexeddb": [] } })
                    }
                    _ => json!({}),
                })
                .collect(),
        )
    }
}

fn section(logout: bool) -> OidcSection {
    OidcSection {
        start: "/login/oidc".to_owned(),
        private: "/account".to_owned(),
        logout: logout.then(|| sv_manifest::RequestTemplate {
            method: "POST".into(),
            path: "/auth/logout".into(),
            form: Default::default(),
            json: Default::default(),
        }),
        ..Default::default()
    }
}

fn run(storage: &'static str, logout: bool) -> (Outcome, Fake) {
    let mut fake = Fake {
        storage,
        ..Default::default()
    };
    let mut out = Outcome::default();
    browser_storage(&mut fake, &section(logout), &mut out);
    (out, fake)
}

#[test]
fn storage_emptied_after_signing_out_of_the_providers_sign_in_is_credited_in_part_and_named() {
    let (o, fake) = run("cleared", true);
    // The setup: the browser was handed the session the provider's sign-in set, and looked for the `logout` control.
    let job = fake.jobs.first().expect("the browser was given a job");
    assert_eq!(job.actions[0], Action::Goto("/".into()));
    assert!(
        job.actions.iter().any(|a| matches!(a, Action::SetCookies(c) if c.iter().any(|c| Some(&c.value) == fake.sid.as_ref()))),
        "{job:?}"
    );
    assert!(
        job.actions
            .iter()
            .any(|a| matches!(a, Action::Act(js) if js.contains("/auth/logout"))),
        "{job:?}"
    );
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == KEPT_AFTER_SIGN_OUT.rule_id)
        .unwrap_or_else(|| panic!("{:?} {:?}", o.not_assessed, o.steps));
    assert!(credit.in_part);
    // The provider refused sign-ins while the browser worked, so the person could not be signed straight back in,
    // and was ordinary again after.
    assert_eq!(fake.modes_in_jobs, [idp::REFUSE]);
    assert_eq!(fake.mode, idp::NORMAL);
    assert!(
        credit
            .scope
            .starts_with("after a sign-in through the test provider, "),
        "{}",
        credit.scope
    );
}

#[test]
fn storage_kept_after_signing_out_of_the_providers_sign_in_is_found_and_named() {
    let (o, _) = run("kept", true);
    let f = o
        .findings
        .iter()
        .find(|f| f.rule_id == KEPT_AFTER_SIGN_OUT.rule_id)
        .unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert!(
        f.description
            .contains("after a sign-in through the test provider, the app kept"),
        "{}",
        f.description
    );
}

#[test]
fn without_logout_or_without_a_browser_it_is_said_not_assessed() {
    let (o, fake) = run("cleared", false);
    assert!(fake.jobs.is_empty());
    assert!(
        o.not_assessed
            .iter()
            .any(|(ids, why)| ids == "V14.3.1" && why.contains("has no `logout`")),
        "{:?}",
        o.not_assessed
    );
    let (o, _) = run("absent", true);
    assert!(o.verified.is_empty() && o.findings.is_empty());
    assert!(
        o.not_assessed.iter().any(|(ids, why)| ids == "V14.3.1"
            && why.starts_with("In a real browser, after a sign-in through the test provider")),
        "{:?}",
        o.not_assessed
    );
}
