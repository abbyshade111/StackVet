//! Backlog 0537, a first case of sweep 3 (0561, ADR-119): the sign-in page asked for twice, as two strangers. An
//! anti-forgery token that is one value for everybody (the Antigravity app of the trial of 14 October 2026 makes
//! one per process) is found; one token for each visit credits nothing.

use super::super::fake_app::*;
use super::*;

fn shared(o: &Outcome) -> Option<&Finding> {
    o.findings
        .iter()
        .find(|f| f.rule_id == TOKEN_SHARED.rule_id)
}

#[test]
fn one_token_for_every_stranger_is_found() {
    let o = run_against(
        Flaws {
            csrf_token_shared: true,
            ..Default::default()
        },
        &users(),
    );
    let found = shared(&o).unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert_eq!(found.requirement_ids, ["V3.5.1"]);
    assert_eq!(found.evidence, ["csrf-stranger-1", "csrf-stranger-2"]);
    // Never the token itself: its first characters and its length.
    assert!(!found.description.contains(CSRF), "{}", found.description);
    assert!(
        found
            .description
            .contains(&format!("{} characters", CSRF.len())),
        "{}",
        found.description
    );
}

#[test]
fn a_token_for_each_visit_is_neither_found_nor_credited() {
    let o = run_against(Flaws::default(), &users());
    assert!(shared(&o).is_none(), "{:?}", o.findings);
    assert!(
        !o.verified
            .iter()
            .any(|v| v.check_id == TOKEN_SHARED.rule_id),
        "{:?}",
        o.verified
    );
    // The setup: both pages were read and a token found on each.
    assert!(
        o.steps
            .iter()
            .any(|s| s.ends_with("a different anti-forgery token each time")),
        "{:#?}",
        o.steps
    );
}

#[test]
fn a_page_with_no_token_says_so_and_finds_nothing() {
    /// The fake app with the token field taken out of every sign-in page.
    struct NoToken(FakeApp);
    impl Http for NoToken {
        fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
            let mut answer = self.0.send(r)?;
            if r.id.starts_with("csrf-stranger-") {
                answer.body = answer.body.replace("csrf_token", "something_else");
            }
            Some(answer)
        }
        fn now(&mut self) -> u64 {
            self.0.now()
        }
        fn wait(&mut self, seconds: u64) {
            self.0.wait(seconds);
        }
    }
    let mut http = NoToken(FakeApp::new(Flaws {
        csrf_token_shared: true,
        ..Default::default()
    }));
    let mut out = Outcome::default();
    shared_token_check(&mut http, &users(), &mut out);
    assert!(shared(&out).is_none(), "{:?}", out.findings);
    assert!(
        out.steps
            .iter()
            .any(|s| s.contains("no anti-forgery token")),
        "{:?}",
        out.steps
    );
}
