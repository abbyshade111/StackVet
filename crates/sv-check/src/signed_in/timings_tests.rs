//! Backlog 0526: each named check of a suite is timed, and every wait for the app's limit on requests is said and
//! timed, the ones the limiter then let through as well. In the three-tool trial of 14 October 2026 a suite took
//! 129 s, its requests 7 s, and nothing said where the rest went.

use super::*;

/// An app whose limiter answers each page's first request with 429 and a wait of `retry` seconds, then lets it
/// through; the waits are counted, not slept.
struct LimitsOnce {
    seen: std::collections::BTreeSet<String>,
    retry: u64,
    waited: u64,
}

impl Http for LimitsOnce {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let first = self.seen.insert(request.id.clone());
        Some(ProbeResponse {
            id: request.id.clone(),
            status: if first { 429 } else { 200 },
            headers: if first {
                vec![("retry-after".to_owned(), self.retry.to_string())]
            } else {
                Vec::new()
            },
            body: String::new(),
        })
    }

    fn wait(&mut self, seconds: u64) {
        self.waited += seconds;
    }
}

#[test]
fn waits_the_limiter_let_through_are_said_and_timed() {
    let mut app = LimitsOnce {
        seen: Default::default(),
        retry: 7,
        waited: 0,
    };
    let mut out = Outcome::default();
    {
        let mut patient = Patient::new(&mut app);
        for id in ["page-1", "page-2"] {
            let answer = patient.send(&get(id, "/", &Session::default()));
            assert_eq!(
                answer.map(|a| a.status),
                Some(200),
                "the setup: the wait let {id} through"
            );
        }
        patient.settle(&mut out);
    }
    assert_eq!(app.waited, 14, "the setup: both were waited out");
    assert!(
        out.steps.iter().any(|s| s
            == "waited 14 s in all for the app's limit on requests, 2 times, as its answers asked, before asking \
                again: page-1 (7 s), page-2 (7 s)"),
        "{:?}",
        out.steps
    );
    assert_eq!(
        out.timings,
        [("waiting for the app's limit on requests".to_owned(), 14_000)]
    );
    assert!(
        !out.steps.iter().any(|s| s.contains("still answering")),
        "the limiter let them through: {:?}",
        out.steps
    );
}

#[test]
fn no_wait_says_nothing_of_waiting() {
    let mut app = LimitsOnce {
        seen: ["page-1".to_owned()].into(),
        retry: 7,
        waited: 0,
    };
    let mut out = Outcome::default();
    {
        let mut patient = Patient::new(&mut app);
        patient.send(&get("page-1", "/", &Session::default()));
        patient.settle(&mut out);
    }
    assert!(out.timings.is_empty() && out.steps.is_empty(), "{out:?}");
}

#[test]
fn each_named_check_of_a_run_is_timed_in_order() {
    use crate::signed_in::fake_app::{Flaws, run_against, users};
    let o = run_against(Flaws::default(), &users());
    let named: Vec<&str> = o.timings.iter().map(|(what, _)| what.as_str()).collect();
    // The setup: the run got past signing in, or no check ran to be timed.
    assert!(
        o.steps
            .iter()
            .any(|s| s.starts_with("signed in as A and opened")),
        "{:?}",
        o.steps
    );
    for check in ["`owned_checks`", "`password_checks`", "`brute_force_check`"] {
        assert!(
            named.iter().any(|n| *n == format!("the check {check}")),
            "{check} is not timed: {named:?}"
        );
    }
    let at = |check: &str| named.iter().position(|n| n.ends_with(check)).unwrap();
    assert!(
        at("`owned_checks`") < at("`brute_force_check`"),
        "{named:?}"
    );
}
