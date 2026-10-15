//! The made-up session tried on every private page the owner lists, not one (backlog 0232, part 2; ADR-053, Later):
//! V7.2.1 asks that all session-token verification happen on the server.

use super::*;

/// An app that opens the request ids it lists, refuses the rest, and records what it was sent.
struct Scripted {
    open: Vec<&'static str>,
    sent: Vec<ProbeRequest>,
}

impl Http for Scripted {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.sent.push(request.clone());
        Some(ProbeResponse {
            id: request.id.clone(),
            status: if self.open.contains(&request.id.as_str()) {
                200
            } else {
                403
            },
            headers: Vec::new(),
            body: String::new(),
        })
    }
}

/// Signed in with a session cookie `sid` set at sign-in.
fn signed_in() -> SignedIn {
    let mut session = Session::default();
    session
        .cookies
        .push(("sid".into(), "0123456789abcdef0123".into()));
    SignedIn {
        session,
        set_at_login: vec![Cookie {
            name: "sid".into(),
            value: String::new(),
            http_only: true,
            same_site: None,
            secure: false,
            path: None,
        }],
        before_login: Vec::new(),
        landed: String::new(),
        answer: String::new(),
        limited: false,
    }
}

fn private() -> Vec<String> {
    ["/account", "/notes", "/notes/{id}", "/settings"]
        .iter()
        .map(|p| (*p).to_owned())
        .collect()
}

fn run(open: Vec<&'static str>) -> (Outcome, Scripted) {
    let mut app = Scripted {
        open,
        sent: Vec::new(),
    };
    let mut out = Outcome::default();
    invented_session_check(
        &mut app,
        &signed_in(),
        Some("/account"),
        &private(),
        Some(true),
        &mut out,
    );
    (out, app)
}

fn credit(out: &Outcome) -> Option<&crate::Verified> {
    out.verified
        .iter()
        .find(|v| v.check_id == SESSION_TOKEN_UNVERIFIED.rule_id)
}

#[test]
fn every_listed_page_is_tried_and_all_refused_is_a_whole_credit() {
    // Each page opens with the real session; none opens with the made-up one.
    let (out, app) = run(vec![
        "invented-session-control",
        "invented-session-control-1",
        "invented-session-control-2",
    ]);
    // The setup: each page was asked both ways, the one with a placeholder left out.
    let asked: Vec<&str> = app.sent.iter().map(|r| r.path.as_str()).collect();
    assert_eq!(
        asked,
        [
            "/account",
            "/account",
            "/notes",
            "/notes",
            "/settings",
            "/settings"
        ]
    );
    let credit = credit(&out).expect("credited");
    assert!(!credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("/account, /notes, /settings"),
        "{}",
        credit.scope
    );
}

#[test]
fn a_made_up_session_that_opens_one_listed_page_is_found_naming_it() {
    let (out, _) = run(vec![
        "invented-session-control",
        "invented-session-control-1",
        "invented-session-1",
        "invented-session-control-2",
    ]);
    let found = out
        .findings
        .iter()
        .find(|f| f.rule_id == SESSION_TOKEN_UNVERIFIED.rule_id)
        .expect("the finding");
    assert_eq!(found.evidence, ["invented-session-1"]);
    assert!(
        found.description.contains("/notes still opened"),
        "{}",
        found.description
    );
    assert!(credit(&out).is_none(), "credited beside its own finding");
}

#[test]
fn a_page_the_real_session_did_not_open_leaves_the_credit_in_part() {
    let (out, _) = run(vec![
        "invented-session-control",
        "invented-session-control-2",
    ]);
    let credit = credit(&out).expect("credited for the pages it could try");
    assert!(credit.in_part, "{credit:?}");
    assert!(
        credit.scope.contains("/account, /settings"),
        "{}",
        credit.scope
    );
}

#[test]
fn one_page_listed_is_one_sample() {
    let mut app = Scripted {
        open: vec!["invented-session-control"],
        sent: Vec::new(),
    };
    let mut out = Outcome::default();
    invented_session_check(
        &mut app,
        &signed_in(),
        Some("/account"),
        &["/account".to_owned()],
        Some(true),
        &mut out,
    );
    assert_eq!(app.sent.len(), 2, "the page asked once each way");
    assert!(credit(&out).expect("credited").in_part);
}

#[test]
fn a_run_tries_the_made_up_session_on_each_page_stackvet_toml_lists() {
    // Through the whole signed-in run: the pages reach the check from the manifest, not only from a test's call.
    // `/nowhere` does not open with the real session either, so it is said and the credit stays in part.
    use super::super::fake_app::{FakeApp, Flaws, run_seeded, users_full};
    let mut users = users_full();
    users.private = vec!["/account".into(), "/nowhere".into()];
    let (out, _) = run_seeded(FakeApp::new(Flaws::default()), &users);
    assert!(
        out.steps.iter().any(|s| s
            == "asked for /nowhere with the real session, before making one up: refused"),
        "{:#?}",
        out.steps
    );
    let credit = credit(&out).expect("credited for /account");
    assert!(credit.in_part, "{credit:?}");
}
