//! Tests of `signed_in/mod.rs`: an app that limits how often it is asked: a refusal by the limit told from a mistake in
//! stackvet.toml, a limiter waited out, and what a refused sign-in keeps from counting. Moved out of `mod.rs` unchanged
//! (backlog 0324).

use super::fake_app::*;
use super::*;

/// The fake app behind a limiter: the next `times` requests with id `id` are answered `status`,
/// with `Retry-After: retry` when given, instead of reaching the app. Counts what it waited.
struct Limited {
    app: FakeApp,
    id: &'static str,
    status: u16,
    retry: Option<&'static str>,
    times: u32,
    waited: u64,
    limited: u32,
}

impl Http for Limited {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        if (r.id == self.id || self.id == "*") && self.times > 0 {
            self.times -= 1;
            self.limited += 1;
            return Some(ProbeResponse {
                id: r.id.clone(),
                status: self.status,
                headers: self
                    .retry
                    .map(|v| vec![("retry-after".to_owned(), v.to_owned())])
                    .unwrap_or_default(),
                body: "slow down".into(),
            });
        }
        self.app.send(r)
    }
    fn mail(&mut self, to: &str, at_least: usize) -> Option<Vec<String>> {
        self.app.mail(to, at_least)
    }
    fn now(&mut self) -> u64 {
        self.app.now()
    }
    fn wait(&mut self, seconds: u64) {
        self.waited += seconds;
        self.app.wait(seconds);
    }
}

fn limited(id: &'static str, status: u16, retry: Option<&'static str>, times: u32) -> Limited {
    limited_with(Flaws::default(), id, status, retry, times)
}

fn limited_with(
    flaws: Flaws,
    id: &'static str,
    status: u16,
    retry: Option<&'static str>,
    times: u32,
) -> Limited {
    let mut app = FakeApp::new(flaws);
    let acc = accounts();
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let admin = acc.admin.clone().unwrap();
    app.users.insert(admin.user, (admin.password, true));
    Limited {
        app,
        id,
        status,
        retry,
        times,
        waited: 0,
        limited: 0,
    }
}

fn run_limited(limiter: &mut Limited) -> Outcome {
    let mut acc = accounts();
    acc.totp = None;
    run(limiter, &users(), &acc, true, &Default::default())
}

fn credits_private_page(o: &Outcome) -> bool {
    verified_ids(o).contains(&PRIVATE_PAGE.rule_id)
}

/// The not-assessed entry that names the sign-ins the app's limit refused, if there is one.
fn sign_in_limit_gap(o: &Outcome) -> Option<&(String, String)> {
    o.not_assessed
        .iter()
        .find(|(_, why)| why.contains("limit on sign-in attempts refused"))
}

#[test]
fn a_sign_in_the_limit_refuses_is_named_and_not_blamed_on_the_app() {
    // Control: no limiter, no such entry, and the admin page check is credited.
    let mut control = limited("login-admin", 429, Some("5"), 0);
    let o = run_limited(&mut control);
    assert!(sign_in_limit_gap(&o).is_none(), "{:?}", o.not_assessed);
    assert!(verified_ids(&o).contains(&ADMIN_PAGE.rule_id));

    let mut limiter = limited("login-admin", 429, Some("5"), 99);
    let o = run_limited(&mut limiter);
    assert!(limiter.limited >= 2, "the setup: the limiter answered");
    let (ids, why) = sign_in_limit_gap(&o).expect("the refused sign-in is said");
    assert!(why.contains("(login-admin)"), "{why}");
    assert!(why.contains("not the app failing"), "{why}");
    assert!(
        why.contains(&format!("up to {SIGN_INS_IN_A_RUN} times")),
        "{why}"
    );
    // Listed under the requirements whose credit it held back, the admin page's among them.
    for id in ADMIN_PAGE.requirement_ids {
        assert!(ids.split(", ").any(|i| i == *id), "{id} missing from {ids}");
    }
    assert!(o.verified.is_empty(), "the credits are still withheld");
    // One entry, not one per sign-in page or per credit.
    assert_eq!(
        o.not_assessed
            .iter()
            .filter(|(_, why)| why.contains("limit on sign-in attempts"))
            .count(),
        1
    );
}

#[test]
fn the_first_sign_in_refused_by_the_limit_is_not_called_a_mistake_in_securevibe_toml() {
    let mut limiter = limited("login-a", 429, Some("5"), 99);
    let o = run_limited(&mut limiter);
    assert!(limiter.limited >= 2, "the setup: the limiter answered");
    assert!(
        !o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("not what stackvet.toml says")),
        "{:?}",
        o.not_assessed
    );
    let (ids, why) = sign_in_limit_gap(&o).expect("the refused sign-in is said");
    assert!(why.contains("(login-a)"), "{why}");
    let mut expected: Vec<&str> = NEEDS_A_SESSION.split(", ").collect();
    expected.sort_unstable();
    assert_eq!(
        ids,
        &expected.join(", "),
        "every check that needed A's session"
    );

    // The same sign-in refused for another reason is still the manifest's to look at.
    let mut refused = limited("login-a", 401, None, 99);
    let o = run_limited(&mut refused);
    assert!(sign_in_limit_gap(&o).is_none(), "{:?}", o.not_assessed);
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("not what stackvet.toml says")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_refused_sign_in_names_only_the_checks_it_kept_from_counting() {
    // The review of 6 October, item 6: B's sign-in refused, nothing credited to withhold, and
    // A's checks named as not run, though they ran on a session the limit let through.
    let b = ["login-b".to_owned()];
    assert_eq!(
        limit_gap_names(&[], &b),
        "The checks that needed that sign-in"
    );
    assert_eq!(limit_gap_names(&["V2.3.4", "V2.3.4"], &b), "V2.3.4");
    // A's own sign-in refused: A's checks are named, with what was withheld.
    let a = ["login-a".to_owned()];
    let named = limit_gap_names(&["V2.3.4"], &a);
    for id in NEEDS_A_SESSION.split(", ").chain(["V2.3.4"]) {
        assert!(named.split(", ").any(|i| i == id), "{id} missing: {named}");
    }
}

#[test]
fn a_limited_page_that_is_not_a_sign_in_is_not_called_one() {
    // The sign-in page itself (GET), a private page, and a record created (POST elsewhere):
    // limited, but not sign-ins.
    for id in ["login-page-a", "private-anonymous", "owned-create"] {
        let mut limiter = limited(id, 429, Some("5"), 99);
        let o = run_limited(&mut limiter);
        assert!(limiter.limited >= 2, "the setup: the limiter answered {id}");
        assert!(
            sign_in_limit_gap(&o).is_none(),
            "{id}: {:?}",
            o.not_assessed
        );
    }
}

#[test]
fn a_limiter_that_answers_once_is_waited_out_and_the_refusal_is_the_apps() {
    let mut control = limited("private-anonymous", 429, Some("7"), 0);
    assert!(
        credits_private_page(&run_limited(&mut control)),
        "the setup: with no limiter the private page is credited"
    );
    let mut limiter = limited("private-anonymous", 429, Some("7"), 1);
    let o = run_limited(&mut limiter);
    assert_eq!(limiter.limited, 1, "the setup: the limiter answered");
    assert_eq!(limiter.waited, 7, "waited as long as the app asked");
    assert!(credits_private_page(&o), "{:?}", o.not_assessed);
    assert!(
        !o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("rate limiter")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_limiter_that_keeps_answering_credits_nothing_in_the_run() {
    let mut limiter = limited("private-anonymous", 429, None, 99);
    let o = run_limited(&mut limiter);
    assert_eq!(
        limiter.limited, 2,
        "asked twice: once, and once after waiting"
    );
    assert_eq!(limiter.waited, 5, "no Retry-After: five seconds");
    assert!(!credits_private_page(&o), "a refusal nobody made, credited");
    assert!(o.verified.is_empty(), "{:?}", verified_ids(&o));
    let (ids, why) = o
        .not_assessed
        .iter()
        .find(|(_, why)| why.contains(PRIVATE_PAGE.rule_id))
        .expect("the withdrawn credit says why");
    assert!(ids.contains("V8.2.1"), "{ids}");
    assert!(why.contains("private-anonymous (429)"), "{why}");
}

#[test]
fn a_503_is_a_limiter_only_when_it_says_when_to_come_back() {
    let mut with_retry = limited("private-anonymous", 503, Some("3"), 1);
    let o = run_limited(&mut with_retry);
    assert_eq!(with_retry.waited, 3);
    assert!(credits_private_page(&o));
    // Without Retry-After a 503 is the app failing, not a limiter, and is not waited out.
    let mut without = limited("private-anonymous", 503, None, 1);
    run_limited(&mut without);
    assert_eq!(without.waited, 0);
}

#[test]
fn a_limiter_does_not_make_a_sign_out_out_of_a_plain_page_visit() {
    // The app keeps the session on a GET to /logout. A limiter answering the check's next look
    // at the private page made it read as signed out.
    let mut once = limited("private-after-get-logout", 429, Some("2"), 1);
    let o = run_limited(&mut once);
    assert_eq!(
        once.limited, 1,
        "the setup: the limiter answered that request"
    );
    assert!(
        !rule_ids(&o).contains(&SIGN_OUT_ON_GET.rule_id),
        "a limiter's 429 read as a sign-out"
    );
    // Still answering after the wait: the finding stands, and the run says what it may rest on.
    let mut always = limited("private-after-get-logout", 429, Some("2"), 99);
    let o = run_limited(&mut always);
    assert!(rule_ids(&o).contains(&SIGN_OUT_ON_GET.rule_id));
    assert!(o.verified.is_empty(), "credited: {:?}", verified_ids(&o));
    assert!(
        o.not_assessed
            .iter()
            .any(|(ids, why)| ids.contains("V3.5.3") && why.contains("may be the limiter's")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_limiter_asking_for_an_hour_is_waited_a_minute() {
    // An app may ask for a long wait; the run waits at most a minute, once, and then says what
    // it could not tell rather than stalling the whole report.
    let mut limiter = limited("private-anonymous", 429, Some("3600"), 1);
    let o = run_limited(&mut limiter);
    assert_eq!(limiter.waited, 60);
    assert!(credits_private_page(&o));
    let limited_for = |value: &str| {
        rate_limited(&ProbeResponse {
            id: String::new(),
            status: 429,
            headers: vec![("retry-after".into(), value.into())],
            body: String::new(),
        })
    };
    assert_eq!(limited_for("12"), Some(12));
    assert_eq!(
        limited_for("Wed, 21 Oct 2026 07:28:00 GMT"),
        Some(5),
        "a date: five seconds"
    );
}

#[test]
fn a_real_finding_is_kept_through_a_persistent_limit_and_flagged() {
    // An app with a default admin account, behind a limiter that never lets the stranger's look
    // at the private page through. The default account is found by signing in, which the
    // limiter does not touch: the finding stays, and the run says it may rest on the limiter.
    let flaws = Flaws {
        default_admin: true,
        ..Default::default()
    };
    let mut limiter = limited_with(flaws, "private-anonymous", 429, Some("1"), 99);
    let o = run_limited(&mut limiter);
    assert!(
        rule_ids(&o).contains(&DEFAULT_ACCOUNT.rule_id),
        "{:?}",
        rule_ids(&o)
    );
    assert!(o.verified.is_empty(), "credited: {:?}", verified_ids(&o));
    assert!(
        o.not_assessed
            .iter()
            .any(|(ids, why)| ids.contains("V6.3.2") && why.contains("may be the limiter's")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn what_counts_as_a_limiter_and_how_long_it_is_waited() {
    let answer = |status: u16, retry: Option<&str>| {
        rate_limited(&ProbeResponse {
            id: String::new(),
            status,
            headers: retry
                .map(|v| vec![("retry-after".to_owned(), v.to_owned())])
                .unwrap_or_default(),
            body: String::new(),
        })
    };
    assert_eq!(answer(429, Some("12")), Some(12));
    assert_eq!(answer(429, Some("3600")), Some(60), "at most a minute");
    assert_eq!(answer(429, None), Some(5));
    assert_eq!(answer(503, Some("3")), Some(3));
    assert_eq!(
        answer(503, None),
        None,
        "a 503 that names no wait is the app failing"
    );
    assert_eq!(answer(500, Some("3")), None);
    assert_eq!(answer(401, Some("3")), None, "a refusal is an answer");
    assert_eq!(answer(200, None), None);
}

#[test]
fn a_guess_is_never_waited_out() {
    // The guessing checks send wrong passwords and codes to see the limiter answer. Waiting
    // would change what they measure and send one guess more than they count.
    let mut inner = limited("guess-3", 429, Some("30"), 99);
    let mut patient = Patient::new(&mut inner);
    let request = get("guess-3", "/login", &Session::default());
    let answer = patient.send(&request).expect("an answer");
    assert_eq!(answer.status, 429);
    assert!(patient.still_limited.is_empty());
    drop(patient);
    assert_eq!((inner.limited, inner.waited), (1, 0));
}

#[test]
fn a_limiter_answering_everything_is_waited_for_five_minutes_in_all() {
    // Ten requests, each answered by the limiter asking for a minute: five are waited for,
    // and the rest are recorded as the limiter's without waiting.
    let mut inner = limited("*", 429, Some("60"), 1000);
    let mut patient = Patient::new(&mut inner);
    for n in 0..10 {
        patient.send(&get(&format!("page-{n}"), "/", &Session::default()));
    }
    assert_eq!(
        patient.still_limited.len(),
        10,
        "{:?}",
        patient.still_limited
    );
    assert!(
        patient.still_limited[9].contains("not waited for"),
        "{:?}",
        patient.still_limited
    );
    drop(patient);
    assert_eq!(inner.waited, MOST_WAITING);
    assert_eq!(inner.limited, 15, "five asked twice, five once");
}

#[test]
fn an_anonymous_answer_is_waited_for_and_one_still_limited_is_left_out() {
    let requests = crate::probes::requests("/");
    let home = requests
        .iter()
        .find(|r| r.id == "home")
        .expect("the probes ask for the home page");

    // The limiter says no once: waited out, and the app's own answer is what comes back.
    let mut once = limited("home", 429, Some("2"), 1);
    let (answers, left_out) = ask_anonymously(&mut once, std::slice::from_ref(home));
    assert!(left_out.is_empty(), "{left_out:?}");
    assert_eq!(answers.len(), 1);
    assert_ne!(answers[0].status, 429);
    assert_eq!(once.waited, 2);

    // The limiter never lets go: the answer is left out rather than judged as the app's.
    let mut always = limited("home", 429, Some("2"), 99);
    let (answers, left_out) = ask_anonymously(&mut always, &requests);
    assert!(
        answers.iter().all(|a| a.id != "home"),
        "the limiter's page was kept: {answers:?}"
    );
    assert_eq!(left_out, ["home (429)"]);
    // Every other question still got its answer.
    assert_eq!(answers.len(), requests.len() - 1);

    // Why the limiter's page is left out rather than judged: read as the app's, a 429 page
    // without headers is a security-headers finding, and two 429s on `/.git` are a credit for
    // the folder not being exposed.
    let page = |id: &str| ProbeResponse {
        id: id.to_owned(),
        status: 429,
        headers: vec![("retry-after".to_owned(), "2".to_owned())],
        body: "slow down".into(),
    };
    let judged = [page("home"), page("git-head"), page("git-config")];
    assert!(
        crate::probes::evaluate(&judged)
            .iter()
            .any(|f| f.rule_id == "probe.security-headers"),
        "the false alarm this prevents"
    );
    assert!(
        crate::probes::verified(&judged)
            .iter()
            .any(|v| v.check_id == "probe.source-control-exposed"),
        "the false pass this prevents"
    );
}
