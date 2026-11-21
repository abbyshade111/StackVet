//! V10.5.3 against a fake harness whose copy of the app reads the fake provider's published details as it starts,
//! as a real app does (backlog 0620). Each test says what the copy does with details that name another issuer.

use super::*;
use crate::probes::{ProbeRequest, ProbeResponse};
use crate::running::Liveness;
use crate::signed_in::{Accounts, Http};
use std::cell::RefCell;

/// What the fake copy of the app does with the provider's published details.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum App {
    /// Reads them, and refuses to sign anybody in when their issuer is not the one it was given.
    Refuses,
    /// Reads them, and will not start when their issuer is not the one it was given.
    WillNotStart,
    /// Reads them, and signs people in whatever their issuer says.
    Takes,
    /// Never reads them: it was given the provider's addresses directly.
    NeverReads,
    /// Reads them and refuses, and signs nobody in even when they are right.
    Broken,
}

#[derive(Default)]
struct State {
    mode: String,
    served_wrong: u64,
    /// Whether a copy is running, and whether the details it read named another issuer.
    running: Option<bool>,
    signed_in: Vec<String>,
    next: u32,
    starts: u32,
    removed: u32,
    provider_ignores_modes: bool,
}

struct Harness {
    app: App,
    state: RefCell<State>,
    /// The harness cannot start a copy at all.
    no_copies: bool,
}

struct Way<'h>(&'h Harness);

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

fn cookie(r: &ProbeRequest) -> Option<String> {
    r.headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("cookie"))
        .and_then(|(_, v)| v.split("; ").find_map(|c| c.strip_prefix("sid=")))
        .map(str::to_owned)
}

impl Http for Way<'_> {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        let mut state = self.0.state.borrow_mut();
        let wrong = state.running?;
        let path = r.path.split('?').next().unwrap_or("");
        let refuses = match self.0.app {
            App::Takes | App::NeverReads => false,
            App::Refuses | App::WillNotStart => wrong,
            App::Broken => true,
        };
        Some(match path {
            "/login/oidc" => answer(
                302,
                vec![(
                    "location",
                    "http://idp:9000/authorize?client_id=c&redirect_uri=http://app/callback&state=s".to_owned(),
                )],
                "",
            ),
            "/callback" if refuses => answer(401, vec![], ""),
            "/callback" => {
                state.next += 1;
                let sid = format!("s{}", state.next);
                state.signed_in.push(sid.clone());
                answer(
                    302,
                    vec![
                        ("location", "/account".to_owned()),
                        ("set-cookie", format!("sid={sid}; Path=/")),
                    ],
                    "",
                )
            }
            "/account" if cookie(r).is_some_and(|s| state.signed_in.contains(&s)) => answer(200, vec![], ""),
            "/account" => answer(302, vec![("location", "/login/oidc".to_owned())], ""),
            _ => answer(404, vec![], ""),
        })
    }

    fn provider(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        let mut state = self.0.state.borrow_mut();
        let path = r.path.split('?').next().unwrap_or("");
        Some(match (r.method.as_str(), path) {
            ("POST", idp::MODE) => {
                if state.provider_ignores_modes {
                    return Some(answer(500, vec![], ""));
                }
                let body = r
                    .body
                    .as_ref()
                    .map(|b| String::from_utf8_lossy(b).into_owned());
                state.mode = body
                    .unwrap_or_default()
                    .strip_prefix("mode=")
                    .unwrap_or("")
                    .to_owned();
                state.served_wrong = 0;
                answer(200, vec![], "{}")
            }
            ("GET", idp::METADATA) => answer(
                200,
                vec![],
                &format!("{{\"served_wrong\":{}}}", state.served_wrong),
            ),
            ("GET", "/authorize") => answer(
                302,
                vec![("location", "http://app/callback?code=c1&state=s".to_owned())],
                "",
            ),
            _ => answer(404, vec![], ""),
        })
    }
}

impl Services for Harness {
    fn http<'s>(&'s self, target: Target, with: With) -> Box<dyn Http + 's> {
        assert_eq!(target, Target::Again, "only the copy is asked");
        assert!(with.provider, "the copy's way has the provider");
        Box::new(Way(self))
    }
    fn model_canary(&self) -> Option<String> {
        None
    }
    fn app_log(&self) -> String {
        String::new()
    }
    fn seed(
        &self,
        _: Target,
        _: &str,
        _: &Accounts,
    ) -> Result<std::collections::BTreeMap<String, String>, String> {
        Ok(Default::default())
    }
    fn start_switched_off(&self, _: &str) -> bool {
        false
    }
    fn remove_switched_off(&self) {}
    fn start_again(&self) -> bool {
        if self.no_copies {
            return false;
        }
        let mut state = self.state.borrow_mut();
        state.starts += 1;
        // The copy reads the published details as it starts, as a real one does.
        let wrong = state.mode == idp::WRONG_METADATA_ISS;
        if self.app != App::NeverReads && wrong {
            state.served_wrong += 1;
        }
        if self.app == App::WillNotStart && wrong {
            return false;
        }
        state.running = Some(wrong);
        true
    }
    fn remove_again(&self) {
        let mut state = self.state.borrow_mut();
        state.running = None;
        state.removed += 1;
    }
    fn liveness(&self, after: &str) -> Liveness {
        Liveness {
            after: after.to_owned(),
            status: "running".to_owned(),
            restarts: 0,
            exit_code: 0,
            out_of_memory: false,
            answered: true,
        }
    }
}

fn harness(app: App) -> Harness {
    Harness {
        app,
        state: RefCell::new(State::default()),
        no_copies: false,
    }
}

fn section() -> OidcSection {
    OidcSection {
        start: "/login/oidc".into(),
        private: "/account".into(),
        ..Default::default()
    }
}

fn asked(h: &Harness, before: Outcome) -> Outcome {
    let mut out = before;
    wrong_metadata(h, &section(), &Cell::new(0), &mut out);
    out
}

fn said(out: &Outcome) -> Vec<&str> {
    out.not_assessed
        .iter()
        .filter(|(ids, _)| ids == "V10.5.3")
        .map(|(_, why)| why.as_str())
        .collect()
}

fn credited(out: &Outcome) -> bool {
    out.verified
        .iter()
        .any(|v| v.check_id == WRONG_METADATA.rule_id)
}

fn found(out: &Outcome) -> bool {
    out.findings
        .iter()
        .any(|f| f.rule_id == WRONG_METADATA.rule_id)
}

#[test]
fn an_app_that_signs_in_with_details_naming_another_issuer_is_a_finding() {
    let h = harness(App::Takes);
    let out = asked(&h, Outcome::default());
    assert!(found(&out), "{out:?}");
    let f = out
        .findings
        .iter()
        .find(|f| f.rule_id == WRONG_METADATA.rule_id)
        .unwrap();
    assert_eq!(f.severity, Severity::Low);
    assert!(
        f.description.contains(idp::OTHER_ISSUER),
        "{}",
        f.description
    );
    assert!(!credited(&out), "{out:?}");
    // One copy started and removed, and the provider put right afterwards.
    let state = h.state.borrow();
    assert_eq!((state.starts, state.removed), (1, 1));
    assert_eq!(state.mode, idp::NORMAL);
}

#[test]
fn an_app_that_refuses_them_is_credited_in_part_once_a_copy_with_them_right_signs_in() {
    for app in [App::Refuses, App::WillNotStart] {
        let h = harness(app);
        let out = asked(&h, Outcome::default());
        assert!(credited(&out), "{app:?}: {out:?}");
        let credit = out
            .verified
            .iter()
            .find(|v| v.check_id == WRONG_METADATA.rule_id)
            .unwrap();
        assert!(credit.in_part, "{app:?}: {credit:?}");
        assert!(!found(&out), "{app:?}: {out:?}");
        assert!(said(&out).is_empty(), "{app:?}: {:?}", said(&out));
        let state = h.state.borrow();
        // The wrong copy, then the control: two starts, each removed.
        assert_eq!((state.starts, state.removed), (2, 2), "{app:?}");
        assert_eq!(state.mode, idp::NORMAL, "{app:?}");
    }
}

#[test]
fn a_refusal_is_not_credited_when_a_copy_with_the_details_right_does_not_sign_in_either() {
    let h = harness(App::Broken);
    let out = asked(&h, Outcome::default());
    assert!(!credited(&out), "{out:?}");
    assert!(!found(&out), "{out:?}");
    assert!(
        said(&out)
            .iter()
            .any(|w| w.contains("did not sign in either")),
        "{:?}",
        said(&out)
    );
}

#[test]
fn an_app_that_never_reads_the_details_is_not_assessed_even_when_it_signs_in() {
    let h = harness(App::NeverReads);
    let out = asked(&h, Outcome::default());
    assert!(!found(&out), "{out:?}");
    assert!(!credited(&out), "{out:?}");
    assert!(
        said(&out)
            .iter()
            .any(|w| w.contains("never asked for them")),
        "{:?}",
        said(&out)
    );
    // No control start: there is nothing for one to show.
    assert_eq!(h.state.borrow().starts, 1);
}

#[test]
fn a_harness_that_cannot_start_a_copy_credits_nothing() {
    let mut h = harness(App::Refuses);
    h.no_copies = true;
    let out = asked(&h, Outcome::default());
    assert!(!credited(&out) && !found(&out), "{out:?}");
    assert!(!said(&out).is_empty(), "{out:?}");
}

#[test]
fn a_provider_that_takes_no_instructions_starts_no_copy() {
    let h = harness(App::Takes);
    h.state.borrow_mut().provider_ignores_modes = true;
    let out = asked(&h, Outcome::default());
    assert!(!credited(&out) && !found(&out), "{out:?}");
    assert!(!said(&out).is_empty(), "{out:?}");
    assert_eq!(h.state.borrow().starts, 0);
}

#[test]
fn nothing_is_started_when_the_ordinary_sign_in_did_not_work_in_the_first_copy() {
    let h = harness(App::Takes);
    let before = Outcome {
        not_assessed: vec![(
            IDS.to_owned(),
            "An ordinary sign-in could not be made.".to_owned(),
        )],
        ..Default::default()
    };
    let out = asked(&h, before);
    assert!(!found(&out), "{out:?}");
    assert_eq!(h.state.borrow().starts, 0);
    // V10.5.3 is said not assessed by the first copy's own reason, since it is in IDS.
    assert!(IDS.split(", ").any(|id| id == "V10.5.3"));
}
