//! Sweep 3 (backlog 0561, 0575, ADR-119): the create request refused without a token is sent once more with a token
//! a stranger was handed on the sign-in page. An app that checks a token is one it made, but not that it is the
//! session's own, refuses the first and takes this.

use super::super::fake_app::*;
use super::*;

fn sent(app: &FakeApp, id: &str) -> bool {
    app.clock_log.iter().any(|(i, _)| i == id)
}

#[test]
fn a_token_taken_from_any_session_is_found() {
    let (o, app) = run_against_keeping(
        Flaws {
            csrf_token_any_session: true,
            ..Default::default()
        },
        &users(),
    );
    // The setup: the stranger's page was asked for, and the forged create sent with its token.
    assert!(sent(&app, "csrf-other-session") && sent(&app, "forged-create-other-token"));
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == FORGERY.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        found.evidence.iter().any(|e| e.ends_with("-other-token")),
        "{:?}",
        found.evidence
    );
    assert!(
        found
            .description
            .contains("with a token from another session"),
        "{}",
        found.description
    );
    assert!(!o.verified.iter().any(|v| v.check_id == FORGERY.rule_id));
}

#[test]
fn a_token_tied_to_its_session_is_refused_and_the_credit_says_so() {
    let (o, app) = run_against_keeping(Flaws::default(), &users());
    assert!(sent(&app, "forged-create-other-token"));
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == FORGERY.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        credit
            .scope
            .contains("a token a stranger was handed on the sign-in page, refused too"),
        "{}",
        credit.scope
    );
}

/// The scripted app, with the stranger's sign-in page answered without its token.
struct NoStrangerToken(FakeApp);

impl Http for NoStrangerToken {
    fn now(&mut self) -> u64 {
        self.0.now()
    }
    fn wait(&mut self, seconds: u64) {
        self.0.wait(seconds)
    }
    fn mail(&mut self, to: &str, at_least: usize) -> Option<Vec<String>> {
        self.0.mail(to, at_least)
    }
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        let answer = self.0.send(r)?;
        if r.id == "csrf-other-session" {
            return Some(ProbeResponse {
                headers: Vec::new(),
                body: "<form method=post><input name=email></form>".to_owned(),
                ..answer
            });
        }
        Some(answer)
    }
    fn model(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        self.0.model(r)
    }
    fn model_address(&mut self) -> Option<String> {
        self.0.model_address()
    }
    fn browser(&mut self, job: &crate::browser::Job) -> Option<Vec<serde_json::Value>> {
        self.0.browser(job)
    }
    fn send_together(&mut self, rs: &[ProbeRequest]) -> Option<Vec<Option<ProbeResponse>>> {
        self.0.send_together(rs)
    }
}

#[test]
fn without_a_stranger_token_the_credit_is_in_part_and_says_so() {
    let acc = accounts();
    let mut app = FakeApp::new(Flaws::default());
    app.users
        .insert(acc.a.user.clone(), (acc.a.password.clone(), false));
    app.users
        .insert(acc.b.user.clone(), (acc.b.password.clone(), false));
    let admin = acc.admin.clone().unwrap();
    app.users.insert(admin.user, (admin.password, true));
    let mut http = NoStrangerToken(app);
    let o = super::super::run(&mut http, &users(), &acc, true, &Default::default());
    assert!(!sent(&http.0, "forged-create-other-token"));
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == FORGERY.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(credit.in_part, "{}", credit.scope);
    assert!(
        credit
            .scope
            .contains("whether a token from another session is refused was not shown"),
        "{}",
        credit.scope
    );
}
