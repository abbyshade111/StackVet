//! Tests of `signed_in/mod.rs`: the suite against the fake app: each flaw found by its own rule and no other, a correct
//! app raising nothing, and what the run note says. Moved out of `mod.rs` unchanged (backlog 0324).

use super::fake_app::*;
use super::*;

#[test]
fn one_rule_says_what_an_answer_is() {
    // ADR-021, in one place since 8 October 2026: the limiter's two shapes, a crash, silence,
    // and an answer. The 503 with `Retry-After` is the case one of the seven copies missed.
    let with = |status: u16, headers: &[(&str, &str)]| ProbeResponse {
        id: "q".to_owned(),
        status,
        headers: headers
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        body: String::new(),
    };
    assert_eq!(answer_of(None), Answer::Silent);
    assert_eq!(answer_of(Some(&with(0, &[]))), Answer::Silent);
    assert_eq!(answer_of(Some(&with(500, &[]))), Answer::Crashed(500));
    assert_eq!(answer_of(Some(&with(503, &[]))), Answer::Crashed(503));
    assert_eq!(
        answer_of(Some(&with(503, &[("retry-after", "7")]))),
        Answer::Limited {
            status: 503,
            retry_after: 7
        }
    );
    assert_eq!(
        answer_of(Some(&with(429, &[]))),
        Answer::Limited {
            status: 429,
            retry_after: 5
        }
    );
    let ok = with(403, &[]);
    assert_eq!(answer_of(Some(&ok)), Answer::Answered(&ok));
    assert!(answer_of(Some(&with(429, &[]))).is_limited());
    assert!(answer_of(Some(&with(502, &[]))).is_crash_or_silence());
    assert!(!answer_of(Some(&with(429, &[]))).is_crash_or_silence());
    assert_eq!(crash_of(Some(&with(502, &[]))).as_deref(), Some("502"));
    assert_eq!(crash_of(None).as_deref(), Some("no answer"));
    assert_eq!(crash_of(Some(&with(429, &[]))), None);
}
use std::collections::BTreeMap;

#[test]
fn a_correct_app_raises_nothing_and_every_check_says_what_it_confirmed() {
    let o = run_against(Flaws::default(), &users());
    assert!(o.findings.is_empty(), "{:#?}", o.findings);
    for id in [
        PRIVATE_PAGE.rule_id,
        SESSION_COOKIE.rule_id,
        SESSION_RENEWAL.rule_id,
        ADMIN_PAGE.rule_id,
        ADMIN_ACTION.rule_id,
        OTHER_USERS_DATA.rule_id,
        FORGERY.rule_id,
        LOGOUT.rule_id,
        PRIVATE_PAGE_CACHING.rule_id,
        SIGN_OUT_LINK.rule_id,
        SESSION_TOKEN_UNVERIFIED.rule_id,
    ] {
        assert!(
            verified_ids(&o).contains(&id),
            "{id} was not confirmed: {:?}\n{:?}",
            verified_ids(&o),
            o.not_assessed
        );
    }
    // One sample each (ADR-053, Later): the one private page and the one admin page `users`
    // lists, one sign-in, one session, one cross-site request. Every admin action listed is
    // tried, so that credit stays whole.
    for id in [
        PRIVATE_PAGE.rule_id,
        SESSION_COOKIE.rule_id,
        SESSION_RENEWAL.rule_id,
        ADMIN_PAGE.rule_id,
        FORGERY.rule_id,
        LOGOUT.rule_id,
        PRIVATE_PAGE_CACHING.rule_id,
        SIGN_OUT_LINK.rule_id,
        SESSION_TOKEN_UNVERIFIED.rule_id,
    ] {
        assert!(credited_in_part(&o, id), "{id}: {:#?}", o.verified);
    }
    // One action listed is one sample of the owner's list, so it is in part, as one listed page is (group H).
    assert!(
        credited_in_part(&o, ADMIN_ACTION.rule_id),
        "{:#?}",
        o.verified
    );
}

#[test]
fn the_run_note_says_what_each_of_these_asked_and_what_came_back() {
    // A second reading of three checks at once, on the surface the owner sees. A check that
    // quietly stopped asking would leave the findings list empty, which looks exactly like an
    // app with nothing wrong; the note is where the two are told apart.
    let correct = run_with_users(Flaws::default(), &with_signup());
    let note = correct.steps.join(" | ");
    assert!(note.contains("session value this check invented"), "{note}");
    assert!(note.contains("breaking the form's own"), "{note}");
    assert!(note.contains("Clear-Site-Data"), "{note}");

    let flawed = run_with_users(
        Flaws {
            session_not_verified: true,
            validation_only_in_browser: true,
            clears_site_data: true,
            ..Default::default()
        },
        &with_signup(),
    );
    let note = flawed.steps.join(" | ");
    assert!(note.contains("every other cookie kept: opened"), "{note}");
    assert!(note.contains("maxlength=40: accepted"), "{note}");
    assert!(note.contains("Clear-Site-Data: yes"), "{note}");
}

#[test]
fn each_flaw_is_found_by_its_own_rule_and_by_no_other() {
    for (flaw, rule) in [
        (
            Flaws {
                private_open: true,
                ..Default::default()
            },
            PRIVATE_PAGE.rule_id,
        ),
        (
            Flaws {
                admin_open: true,
                ..Default::default()
            },
            ADMIN_PAGE.rule_id,
        ),
        (
            Flaws {
                idor: true,
                ..Default::default()
            },
            OTHER_USERS_DATA.rule_id,
        ),
        (
            Flaws {
                records_public: true,
                ..Default::default()
            },
            OTHER_USERS_DATA.rule_id,
        ),
        (
            Flaws {
                owner_from_request: true,
                record_names_owner: true,
                ..Default::default()
            },
            OWNER_FIELD.rule_id,
        ),
        (
            Flaws {
                notes_unescaped: true,
                ..Default::default()
            },
            STORED_HTML.rule_id,
        ),
        (
            Flaws {
                no_csrf_check: true,
                ..Default::default()
            },
            FORGERY.rule_id,
        ),
        (
            Flaws {
                keep_session_at_login: true,
                ..Default::default()
            },
            SESSION_RENEWAL.rule_id,
        ),
        (
            Flaws {
                logout_keeps_session: true,
                ..Default::default()
            },
            LOGOUT.rule_id,
        ),
        (
            Flaws {
                no_httponly: true,
                ..Default::default()
            },
            SESSION_COOKIE.rule_id,
        ),
        (
            Flaws {
                default_admin: true,
                ..Default::default()
            },
            DEFAULT_ACCOUNT.rule_id,
        ),
        (
            Flaws {
                password_in_url: true,
                ..Default::default()
            },
            PASSWORD_IN_URL.rule_id,
        ),
        (
            Flaws {
                short_session_ids: true,
                ..Default::default()
            },
            WEAK_SESSION_ID.rule_id,
        ),
        (
            Flaws {
                password_shown: true,
                ..Default::default()
            },
            UNMASKED_PASSWORD.rule_id,
        ),
        (
            Flaws {
                paste_blocked: true,
                ..Default::default()
            },
            PASTE_BLOCKED.rule_id,
        ),
        (
            Flaws {
                logout_on_get: true,
                ..Default::default()
            },
            SIGN_OUT_ON_GET.rule_id,
        ),
        (
            Flaws {
                private_page_cacheable: true,
                ..Default::default()
            },
            PRIVATE_PAGE_CACHING.rule_id,
        ),
        (
            Flaws {
                record_leaks_fields: true,
                ..Default::default()
            },
            RECORD_LEAKS_FIELDS.rule_id,
        ),
        (
            Flaws {
                no_sign_out_link: true,
                ..Default::default()
            },
            SIGN_OUT_LINK.rule_id,
        ),
        (
            Flaws {
                no_referrer: true,
                refuses_null_origin: true,
                ..Default::default()
            },
            OWN_FORMS_REFUSED.rule_id,
        ),
    ] {
        let o = run_against(flaw, &users());
        let found = rule_ids(&o);
        assert!(
            found.contains(&rule),
            "{rule} not found: {found:?}\n{:?}",
            o.not_assessed
        );
        assert!(
            !verified_ids(&o).contains(&rule),
            "{rule} both found and confirmed"
        );
        let others: Vec<&&str> = found.iter().filter(|r| **r != rule).collect();
        // A private page open to anybody makes the signed-in session unprovable on that page,
        // which is its own not-assessed, not another finding.
        assert!(others.is_empty(), "{rule} also raised {others:?}");
    }
}

#[test]
fn an_app_with_every_flaw_at_once_has_every_one_found() {
    // The second witness for each rule, and a check that no flaw hides another.
    let o = run_against(
        Flaws {
            admin_open: true,
            idor: true,
            no_csrf_check: true,
            logout_keeps_session: true,
            no_httponly: true,
            notes_unescaped: true,
            ..Default::default()
        },
        &users(),
    );
    let mut found = rule_ids(&o);
    found.sort_unstable();
    let mut expected = vec![
        ADMIN_PAGE.rule_id,
        OTHER_USERS_DATA.rule_id,
        FORGERY.rule_id,
        LOGOUT.rule_id,
        SESSION_COOKIE.rule_id,
        STORED_HTML.rule_id,
    ];
    expected.sort_unstable();
    assert_eq!(found, expected, "{:?}", o.not_assessed);

    // The two that change what a session is, together: a page open to all and a session kept
    // across sign-in. Each has to be found beside the other.
    let o = run_against(
        Flaws {
            private_open: true,
            keep_session_at_login: true,
            ..Default::default()
        },
        &users(),
    );
    let found = rule_ids(&o);
    assert!(found.contains(&PRIVATE_PAGE.rule_id), "{found:?}");
    assert!(found.contains(&SESSION_RENEWAL.rule_id), "{found:?}");
}

#[test]
fn a_cookie_set_at_sign_in_is_the_session_only_when_the_page_needs_it() {
    // The review of 1 to 4 October, item 13: an app that keeps its session from before sign-in
    // and sets an unrelated cookie at sign-in was found to take a made-up session (that cookie,
    // altered) and credited for renewing its session.
    let o = run_against(
        Flaws {
            keep_session_at_login: true,
            other_cookie_at_login: true,
            ..Default::default()
        },
        &users(),
    );
    let found = rule_ids(&o);
    assert!(found.contains(&SESSION_RENEWAL.rule_id), "{found:?}");
    assert!(
        !found.contains(&SESSION_TOKEN_UNVERIFIED.rule_id),
        "{found:?}"
    );
    assert!(!verified_ids(&o).contains(&SESSION_RENEWAL.rule_id));
    assert!(!verified_ids(&o).contains(&SESSION_COOKIE.rule_id));
    // The setup: sign-in really did set that cookie, and only it.
    assert!(
        o.steps.iter().any(|s| s.contains("signed in as A")),
        "{:?}",
        o.steps
    );
}

#[test]
fn accounts_that_were_never_made_are_not_assessed_either() {
    // Second shape of the sign-in setup check: the app is fine, the accounts are not there.
    let mut app = FakeApp::new(Flaws::default());
    let o = run(&mut app, &users(), &accounts(), true, &Default::default());
    assert!(o.findings.is_empty(), "{:?}", rule_ids(&o));
    assert!(o.verified.is_empty(), "{:?}", verified_ids(&o));
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("did not open /account")),
        "the reason has to be the sign-in, not whatever check happened to stop next: {:?}",
        o.not_assessed
    );
}

#[test]
fn accounts_can_be_made_through_the_apps_own_sign_up() {
    // No seed: both users sign up the way a visitor would, token and all, and then everything
    // that does not need an admin runs as it does for a seeded app.
    let mut u = users();
    u.seed = None;
    u.admin = Vec::new();
    u.admin_actions = Vec::new();
    u.signup = Some(RequestTemplate {
        method: "POST".into(),
        path: "/signup".into(),
        form: [
            ("email", "{user}"),
            ("password", "{password}"),
            ("csrf_token", "{csrf}"),
        ]
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect(),
        json: BTreeMap::new(),
    });
    let mut app = FakeApp::new(Flaws {
        idor: true,
        ..Default::default()
    });
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    let o = run(&mut app, &u, &acc, false, &Default::default());
    assert!(
        app.users.contains_key(&acc.a.user) && app.users.contains_key(&acc.b.user),
        "both users signed up"
    );
    assert!(
        o.steps.iter().any(|s| s.starts_with("signed up A")),
        "{:?}",
        o.steps
    );
    assert_eq!(
        rule_ids(&o),
        vec![OTHER_USERS_DATA.rule_id],
        "{:?}",
        o.not_assessed
    );
    assert!(verified_ids(&o).contains(&LOGOUT.rule_id));
}

#[test]
fn a_json_sign_in_with_a_bearer_token_is_used_and_its_cookie_checks_are_not_claimed() {
    // An API that answers sign-in with a token in JSON. The signed-in checks run on the token;
    // the cookie checks have no cookie to look at, and say so rather than passing.
    let mut u = users();
    u.login = Some(RequestTemplate {
        method: "POST".into(),
        path: "/api/login".into(),
        form: BTreeMap::new(),
        json: [("email", "{user}"), ("password", "{password}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
    });
    u.token_field = Some("token".into());
    u.logout = None;
    u.owned = None;
    let o = run_against(
        Flaws {
            admin_open: true,
            ..Default::default()
        },
        &u,
    );
    assert!(
        verified_ids(&o).contains(&PRIVATE_PAGE.rule_id),
        "{:?}",
        o.not_assessed
    );
    assert_eq!(rule_ids(&o), vec![ADMIN_PAGE.rule_id]);
    assert!(!verified_ids(&o).contains(&SESSION_COOKIE.rule_id));
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("token rather than a cookie")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_sign_in_that_does_not_work_is_not_assessed_rather_than_passed() {
    // The setup check. Every refusal below would read as a pass if the session never worked.
    let o = run_against(
        Flaws {
            broken_login: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(o.findings.is_empty(), "{:?}", rule_ids(&o));
    assert!(
        o.verified.is_empty(),
        "nothing can be confirmed: {:?}",
        verified_ids(&o)
    );
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("did not open /account")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_manifest_that_cannot_be_used_says_why_and_asks_nothing() {
    let mut u = users();
    u.login = None;
    let mut app = FakeApp::new(Flaws::default());
    let o = run(&mut app, &u, &accounts(), true, &Default::default());
    assert!(o.findings.is_empty() && o.verified.is_empty());
    assert!(
        o.not_assessed[0].1.contains("`login` is not set"),
        "{:?}",
        o.not_assessed
    );
    assert_eq!(app.next, 0, "no request was made");
}

#[test]
fn what_is_not_listed_is_named_as_not_assessed() {
    let mut u = users();
    u.logout = None;
    u.owned = None;
    u.admin = Vec::new();
    u.admin_actions = Vec::new();
    let o = run_against(Flaws::default(), &u);
    // Each id named in one of the lists, whatever else that list names beside it: since
    // 8 October 2026 the sign-out line names V14.3.1 with V7.4.1, and the owned line V3.5.2.
    let said: Vec<&str> = o
        .not_assessed
        .iter()
        .flat_map(|(ids, _)| ids.split(", "))
        .collect();
    for id in ["V7.4.1", "V8.2.2", "V3.5.1", "V8.2.1", "V3.3.1"] {
        assert!(said.contains(&id), "{id} not named: {said:?}");
    }
}

#[test]
fn the_token_is_found_however_the_page_offers_it() {
    let page = |body: &str| ProbeResponse {
        id: String::new(),
        status: 200,
        headers: vec![],
        body: body.into(),
    };
    let none = Session::default();
    assert_eq!(
        csrf_token(
            &page("<input type=hidden name=\"csrf_token\" value=\"t1\">"),
            &none
        )
        .as_deref(),
        Some("t1")
    );
    assert_eq!(
        csrf_token(
            &page("<input value='t2' name='csrfmiddlewaretoken'>"),
            &none
        )
        .as_deref(),
        Some("t2")
    );
    assert_eq!(
        csrf_token(&page("<meta name=\"csrf-token\" content=\"t3\">"), &none).as_deref(),
        Some("t3")
    );
    let with_cookie = Session {
        cookies: vec![("XSRF-TOKEN".into(), "t4".into())],
        ..Default::default()
    };
    assert_eq!(
        csrf_token(&page("<p>nothing</p>"), &with_cookie).as_deref(),
        Some("t4")
    );
    assert_eq!(
        csrf_token(&page("<input name=\"email\" value=\"x\">"), &none),
        None
    );
    // Unquoted, which is valid HTML and how the first real app this met wrote its form.
    assert_eq!(
        csrf_token(&page("<input type=hidden name=csrf_token value=t5>"), &none).as_deref(),
        Some("t5")
    );
    // A field whose name only contains the word is not the token.
    assert_eq!(
        csrf_token(&page("<input name=not_csrf_token value=x>"), &none),
        None
    );
}

/// `users` with a `creates` entry that saves a member's page, read back where the fake app serves it (backlog
/// 0535).
pub(super) fn with_page(mut users: UsersSection, read: Option<&str>) -> UsersSection {
    users.creates.push(sv_manifest::CreateTemplate {
        method: "POST".into(),
        path: "/pages".into(),
        form: [("html", "{marker}"), ("csrf_token", "{csrf}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
        json: BTreeMap::new(),
        read: read.map(str::to_owned),
        id_field: None,
        page: true,
    });
    users
}

/// `users` with both session-ending requests the fake app answers (backlog 0492).
pub(super) fn with_end_sessions(mut users: UsersSection) -> UsersSection {
    let form = |fields: &[(&str, &str)]| {
        fields
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    };
    users.end_other_sessions = Some(RequestTemplate {
        method: "POST".into(),
        path: "/account/sessions/end".into(),
        form: form(&[("password", "{password}"), ("csrf_token", "{csrf}")]),
        json: BTreeMap::new(),
    });
    users.admin_end_sessions = Some(RequestTemplate {
        method: "POST".into(),
        path: "/admin/users/{user}/sessions/end".into(),
        form: form(&[("csrf_token", "{csrf}")]),
        json: BTreeMap::new(),
    });
    users
}

/// The suite with no `seed`: every account, the test passwords' included, goes through sign-up.
pub(super) fn with_signup() -> UsersSection {
    let mut u = users();
    u.seed = None;
    u.admin = Vec::new();
    u.admin_actions = Vec::new();
    u.signup = Some(RequestTemplate {
        method: "POST".into(),
        path: "/signup".into(),
        form: [
            ("email", "{user}"),
            ("password", "{password}"),
            ("csrf_token", "{csrf}"),
        ]
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect(),
        json: BTreeMap::new(),
    });
    u
}

pub(super) fn run_signing_up(flaws: Flaws) -> Outcome {
    run_signing_up_with(flaws, &with_words(&["Acme Notes"]))
}

pub(super) fn run_signing_up_with(flaws: Flaws, policy: &sv_manifest::PolicySection) -> Outcome {
    let mut app = FakeApp::new(flaws);
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    run(&mut app, &with_signup(), &acc, false, policy)
}

/// A policy listing these context-specific words, as an owner would write them.
pub(super) fn with_words(words: &[&str]) -> sv_manifest::PolicySection {
    sv_manifest::PolicySection {
        context_words: words.iter().map(|w| (*w).to_owned()).collect(),
        ..Default::default()
    }
}

#[test]
fn seed_and_sign_up_together_ask_both_the_admin_and_the_password_questions() {
    // `seed` makes the admin; `signup` is only used for the passwords, never for the test users.
    let mut u = with_signup();
    u.seed = Some("seed".into());
    u.admin = vec!["/admin".into()];
    let o = run_against(Flaws::default(), &u);
    assert!(o.findings.is_empty(), "{:#?}", o.findings);
    for id in [
        ADMIN_PAGE.rule_id,
        SHORT_PASSWORD.rule_id,
        COMMON_PASSWORD.rule_id,
    ] {
        assert!(verified_ids(&o).contains(&id), "{id}: {:?}", o.steps);
    }
}

pub(super) fn run_with(flaws: Flaws, policy: &sv_manifest::PolicySection) -> Outcome {
    run_keeping_app(flaws, policy).0
}

/// The outcome and the app, for the assertions that are about what the app was *sent* rather
/// than about what the report says.
pub(super) fn run_keeping_app(
    flaws: Flaws,
    policy: &sv_manifest::PolicySection,
) -> (Outcome, FakeApp, Accounts) {
    let mut app = FakeApp::new(flaws);
    let mut acc = accounts();
    acc.admin = None;
    acc.totp = None;
    let out = run(&mut app, &with_signup(), &acc, false, policy);
    (out, app, acc)
}

/// The seeded fixture, with a policy: the other way into every emailed-code check, so none of
/// them rests on the sign-up fixture alone.
pub(super) fn seeded_with(flaws: Flaws, policy: &sv_manifest::PolicySection) -> Outcome {
    let mut app = FakeApp::new(flaws);
    let acc = accounts();
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let admin = acc.admin.clone().unwrap();
    app.users.insert(admin.user, (admin.password, true));
    run(&mut app, &users(), &acc, true, policy)
}

pub(super) fn timeouts(idle: Option<u32>, lifetime: Option<u32>) -> sv_manifest::PolicySection {
    sv_manifest::PolicySection {
        idle_timeout_minutes: idle,
        session_lifetime_minutes: lifetime,
        ..Default::default()
    }
}

// --------------------------------------------------------------------------------------------
// A private WebSocket's own session

const WS_RULES: [&str; 2] = [WS_WITHOUT_SESSION.rule_id, WS_AFTER_SIGN_OUT.rule_id];

pub(super) fn ws_run(flaws: Flaws) -> Outcome {
    let mut u = users();
    u.private_websocket = Some("/ws".into());
    run_against(flaws, &u)
}

pub(super) fn ws_findings(o: &Outcome) -> Vec<&str> {
    rule_ids(o)
        .into_iter()
        .filter(|id| WS_RULES.contains(id))
        .collect()
}

pub(super) fn bearer_ws_users() -> UsersSection {
    let mut u = users();
    u.login = Some(RequestTemplate {
        method: "POST".into(),
        path: "/api/login".into(),
        form: BTreeMap::new(),
        json: [("email", "{user}"), ("password", "{password}")]
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect(),
    });
    u.token_field = Some("token".into());
    u.logout = None;
    u.owned = None;
    u.private_websocket = Some("/ws".into());
    u
}

/// The fake app, with a browser that answers only the storage job: signed in through the form,
/// and the password kept in `localStorage`.
struct KeepsPassword {
    app: FakeApp,
    stored: String,
}

impl Http for KeepsPassword {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        self.app.send(r)
    }
    fn now(&mut self) -> u64 {
        self.app.now()
    }
    fn wait(&mut self, seconds: u64) {
        self.app.wait(seconds);
    }
    fn browser(&mut self, job: &crate::browser::Job) -> Option<Vec<serde_json::Value>> {
        use crate::browser::Action;
        use serde_json::json;
        let signs_in = job
            .actions
            .iter()
            .any(|a| matches!(a, Action::Act(e) if e.contains("input[type=password]")));
        if !signs_in {
            return None;
        }
        let store = |pairs: serde_json::Value| json!({ "value": { "local": pairs, "session": [], "indexeddb": [], "cookie": "" } });
        Some(vec![
            json!({ "status": 200, "path": "/login" }),
            store(json!([])),
            json!({ "found": true, "after": { "status": 200, "path": "/" } }),
            json!({ "status": 200, "path": "/account" }),
            store(json!([["pw", self.stored]])),
        ])
    }
}

#[test]
fn the_suite_asks_the_browser_what_the_app_keeps_after_signing_in() {
    let mut u = users();
    u.browser = Some(Default::default());
    let acc = accounts();

    // No browser to start: said, never passed.
    let mut app = FakeApp::new(Flaws::default());
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let o = run(&mut app, &u, &acc, false, &Default::default());
    assert!(
        o.not_assessed
            .iter()
            .any(|(ids, why)| ids == "V10.1.1, V14.3.3" && why.contains("could not be started")),
        "{:?}",
        o.not_assessed
    );

    // A browser that finds the password kept: the finding reaches the outcome.
    let mut app = FakeApp::new(Flaws::default());
    for account in [&acc.a, &acc.b] {
        app.users
            .insert(account.user.clone(), (account.password.clone(), false));
    }
    let mut http = KeepsPassword {
        app,
        stored: acc.a.password.clone(),
    };
    let o = run(&mut http, &u, &acc, false, &Default::default());
    assert!(
        rule_ids(&o).contains(&crate::browser_storage::PASSWORD_IN_STORAGE.rule_id),
        "{:?}",
        o.steps
    );
}
