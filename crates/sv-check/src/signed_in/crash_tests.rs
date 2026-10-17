//! Tests of `signed_in/mod.rs`: an app that crashes on some requests: a crash never turning a finding into a pass, and
//! what rests on a request that crashed. Moved out of `mod.rs` unchanged (backlog 0324).

use super::fake_app::*;
use super::tests::with_signup;
use super::*;
use std::collections::BTreeSet;

/// The fake app, answering 500 to every request whose id is `crash` (or nothing at all, with
/// `silent`), and noting every id sent.
struct Crashing {
    app: FakeApp,
    crash: Option<String>,
    silent: bool,
    sent: BTreeSet<String>,
    /// Every request sent, in order and with repeats, as (id, method, path).
    log: Vec<(String, String, String)>,
}

impl Http for Crashing {
    fn send(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        self.sent.insert(r.id.clone());
        self.log
            .push((r.id.clone(), r.method.clone(), r.path.clone()));
        if self.crash.as_deref() == Some(r.id.as_str()) {
            if self.silent {
                return None;
            }
            return Some(ProbeResponse {
                id: r.id.clone(),
                status: 500,
                headers: Vec::new(),
                body: "Internal Server Error".into(),
            });
        }
        self.app.send(r)
    }
    fn send_together(&mut self, rs: &[ProbeRequest]) -> Option<Vec<Option<ProbeResponse>>> {
        for r in rs {
            self.sent.insert(r.id.clone());
            self.log
                .push((r.id.clone(), r.method.clone(), r.path.clone()));
        }
        let mut answers = self.app.send_together(rs)?;
        // One copy of the same instant crashes, as one of a burst can.
        if let Some(i) = rs
            .iter()
            .position(|r| self.crash.as_deref() == Some(r.id.as_str()))
        {
            let r = &rs[i];
            answers[i] = (!self.silent).then(|| ProbeResponse {
                id: r.id.clone(),
                status: 500,
                headers: Vec::new(),
                body: "Internal Server Error".into(),
            });
        }
        Some(answers)
    }
    fn mail(&mut self, to: &str, at_least: usize) -> Option<Vec<String>> {
        self.app.mail(to, at_least)
    }
    fn now(&mut self) -> u64 {
        self.app.now()
    }
    fn wait(&mut self, seconds: u64) {
        self.app.wait(seconds);
    }
    fn model(&mut self, r: &ProbeRequest) -> Option<ProbeResponse> {
        self.app.model(r)
    }
    fn model_address(&mut self) -> Option<String> {
        self.app.model_address()
    }
}

/// One fixture: the app's flaws, what stackvet.toml says, and how the run is made.
struct Scenario {
    name: &'static str,
    flaws: Flaws,
    users: UsersSection,
    seeded: bool,
    policy: sv_manifest::PolicySection,
    slow: bool,
    /// Session limits for the fake app, in seconds: idle, lifetime.
    limits: (Option<u64>, Option<u64>),
    /// How long the fake app's tokens last, when it hands out tokens rather than session ids.
    jwt: Option<u64>,
}

impl Scenario {
    fn new(name: &'static str, flaws: Flaws, users: UsersSection, seeded: bool) -> Self {
        Scenario {
            name,
            flaws,
            users,
            seeded,
            policy: Default::default(),
            slow: false,
            limits: (None, None),
            jwt: None,
        }
    }

    /// One run, crashing on `crash`: the outcome and every request id sent.
    fn run(&self, crash: Option<&str>) -> (Outcome, BTreeSet<String>) {
        self.run_answering(crash, false)
    }

    /// `run`, with the crash answered by nothing at all when `silent`.
    fn run_answering(&self, crash: Option<&str>, silent: bool) -> (Outcome, BTreeSet<String>) {
        let (out, http) = self.run_logged(crash, silent);
        (out, http.sent)
    }

    /// `run_answering`, giving back the fake app as the run left it, with every request sent.
    fn run_logged(&self, crash: Option<&str>, silent: bool) -> (Outcome, Crashing) {
        let mut app = FakeApp::new(self.flaws);
        app.idle_limit = self.limits.0;
        app.lifetime_limit = self.limits.1;
        app.jwt_lifetime = self.jwt;
        // A run whose app hands out tokens has the test model's server to name in them.
        app.model_up = self.jwt.is_some();
        let mut acc = accounts();
        if self.seeded {
            for account in [&acc.a, &acc.b] {
                app.users
                    .insert(account.user.clone(), (account.password.clone(), false));
            }
            let admin = acc.admin.clone().unwrap();
            app.users.insert(admin.user, (admin.password, true));
            let totp = acc.totp.clone().unwrap();
            app.users.insert(
                totp.account.user.clone(),
                (totp.account.password.clone(), false),
            );
            app.totp.insert(totp.account.user, totp.secret);
        } else {
            acc.admin = None;
            acc.totp = None;
        }
        let mut http = Crashing {
            app,
            crash: crash.map(str::to_owned),
            silent,
            sent: BTreeSet::new(),
            log: Vec::new(),
        };
        let out = run_with(
            &mut http,
            &self.users,
            &acc,
            self.seeded,
            &self.policy,
            self.slow,
        );
        (out, http)
    }

    /// The run without a crash: what it held back, and every request it sent, for
    /// `turned_by` to crash one at a time. Held back is found at fault, or, for a rule in
    /// `ONLY_CREDITED`, left open: a rule whose check never ran without the crash may be rightly
    /// credited with one, once the crash hides a flaw that stood in its way.
    fn plain(&self) -> (BTreeSet<String>, Vec<String>) {
        let (plain, sent) = self.run(None);
        let credited: BTreeSet<String> =
            plain.verified.iter().map(|v| v.check_id.clone()).collect();
        let mut found: BTreeSet<String> =
            plain.findings.iter().map(|f| f.rule_id.clone()).collect();
        found.extend(
            ONLY_CREDITED
                .iter()
                .filter(|rule| {
                    let ids = rule.requirement_ids.join(", ");
                    !credited.contains(rule.rule_id)
                        && plain.not_assessed.iter().any(|(i, _)| *i == ids)
                })
                .map(|rule| rule.rule_id.to_owned()),
        );
        (found, sent.into_iter().collect())
    }

    /// The run with request `id` crashed: every rule `found` held back that it credited.
    fn turned_by(&self, id: &str, found: &BTreeSet<String>) -> Vec<String> {
        let (crashed, _) = self.run(Some(id));
        crashed
            .verified
            .iter()
            .filter(|credit| found.contains(&credit.check_id))
            .map(|credit| {
                format!(
                    "{}: {} credited when {id} crashed",
                    self.name, credit.check_id
                )
            })
            .collect()
    }
}

/// Runs `job` on every item, a thread per core each taking the next item as it finishes (backlog
/// 0271). The sweeps used to give each thread a fixed share of one scenario's requests and wait
/// for the slowest share before the next scenario; one list of every scenario's crashed runs,
/// slowest scenario first, keeps every core busy to the end. Which runs are made, and what each
/// is judged on, is the same either way; the results are sorted, so a failure reads the same.
fn each_at_once<T: Sync>(items: &[T], job: impl Fn(&T) -> Vec<String> + Sync) -> Vec<String> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let mut out: Vec<String> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads.min(items.len()).max(1))
            .map(|_| {
                scope.spawn(|| {
                    let mut out = Vec::new();
                    while let Some(item) =
                        items.get(next.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
                    {
                        out.extend(job(item));
                    }
                    out
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    });
    out.sort();
    out
}

/// The rules that are never a finding, only credited or left open: for these, held back is
/// left open (not assessed under the rule's own requirements alone) by the run without a
/// crash. Listed here rather than read from `RESTS_ON_A_REFUSAL`, so a rule missing from that
/// list is still tried.
const ONLY_CREDITED: &[&Rule] = &[&CHANGE_ENDS_SESSIONS, &CHANGE_NOTIFIED];

#[test]
fn a_run_signs_in_no_more_often_than_the_spec_says() {
    // Every sign-in, by what it is sent to rather than its id: the ids vary (`log-marker-…`,
    // `redirect-login-…`), and a repeat under the same id is a second attempt to an app's limit.
    let mut most = 0;
    let mut guesses_seen = false;
    for scenario in scenarios() {
        let (_, http) = scenario.run_logged(None, false);
        let login = scenario
            .users
            .login
            .as_ref()
            .expect("signs in")
            .path
            .clone();
        let login = login.split('?').next().unwrap_or_default();
        let sign_ins: Vec<&str> = http
            .log
            .iter()
            .filter(|(_, method, path)| method != "GET" && path.split('?').next() == Some(login))
            .map(|(id, _, _)| id.as_str())
            .collect();
        // The password-guessing check's wrong passwords are meant to meet the limit, and come
        // last of all the sign-ins, so a limit that stops them takes nothing else with it. The
        // one exception is the check on whether a failed sign-in tells accounts apart
        // (`reveal-`), whose three wrong passwords come after, and which leaves its answers
        // uncompared when a limit refuses them.
        let first_guess = sign_ins.iter().position(|id| id.starts_with("guess-"));
        if let Some(first) = first_guess {
            guesses_seen = true;
            let after: Vec<&&str> = sign_ins[first..]
                .iter()
                .skip_while(|id| id.starts_with("guess-"))
                .collect();
            assert!(
                after.iter().all(|id| id.starts_with("reveal-")),
                "{}: a sign-in after the guesses: {sign_ins:?}",
                scenario.name
            );
        }
        let counted = first_guess.unwrap_or(sign_ins.len());
        assert!(
            counted <= SIGN_INS_IN_A_RUN,
            "{}: {counted} sign-ins, more than the {SIGN_INS_IN_A_RUN} the spec states",
            scenario.name
        );
        most = most.max(counted);
    }
    // The setup: the sign-ins are really counted, and the guesses really set apart.
    assert!(
        most >= 40,
        "only {most} sign-ins counted in the busiest run"
    );
    assert!(guesses_seen, "no run made the guessing check's attempts");
    // The spec gives the builder the same number.
    assert!(
        sv_manifest::spec::STARTER_MANIFEST.contains(&format!("up to {SIGN_INS_IN_A_RUN} times")),
        "the spec does not state the number"
    );
}

/// The users with an upload form the app serves files back from, for the upload scenarios.
fn upload_users() -> UsersSection {
    let mut u = users();
    u.upload = vec![sv_manifest::UploadSection {
        path: "/upload".into(),
        field: "file".into(),
        form: [("csrf_token".to_owned(), "{csrf}".to_owned())].into(),
        serves_at: Some("/files/{name}".into()),
        max_bytes: Some(UPLOAD_LIMIT as u64),
        unpacks_archives: Some(vec![
            sv_manifest::ArchiveFormat::Zip,
            sv_manifest::ArchiveFormat::Gzip,
        ]),
        max_unpacked_bytes: Some(ARCHIVE_UNPACK_LIMIT),
        max_files: Some(ARCHIVE_FILE_LIMIT),
        accepts: None,
    }];
    u
}

fn scenarios() -> Vec<Scenario> {
    // Sign-up and seeding both: the password rules need the app's own sign-up, and the role
    // check needs a seeded admin to compare with.
    let mut signed_up = with_signup();
    signed_up.seed = Some("seed".into());
    signed_up.admin = vec!["/admin".into()];
    let mut sockets_and_files = users();
    // Two private WebSocket paths, so each is held to the crash guard by its own ids (ADR-118).
    sockets_and_files.private_websockets = vec!["/ws".into(), "/chat".into()];
    sockets_and_files.upload = vec![sv_manifest::UploadSection {
        path: "/upload".into(),
        field: "file".into(),
        form: [("csrf_token".to_owned(), "{csrf}".to_owned())].into(),
        serves_at: Some("/files/{name}".into()),
        max_bytes: Some(UPLOAD_LIMIT as u64),
        unpacks_archives: Some(vec![
            sv_manifest::ArchiveFormat::Zip,
            sv_manifest::ArchiveFormat::Gzip,
        ]),
        max_unpacked_bytes: Some(ARCHIVE_UNPACK_LIMIT),
        max_files: Some(ARCHIVE_FILE_LIMIT),
        accepts: None,
    }];
    vec![
        // ADR-055: a list that holds the first common word and not the other two.
        Scenario::new(
            "a short common-password list",
            Flaws {
                common_list_short: true,
                ..Default::default()
            },
            {
                let mut u = with_signup();
                u.seed = Some("seed".into());
                u
            },
            true,
        ),
        // Ending sessions as asked (backlog 0492): a crash at an ended session's page must not read as it ended,
        // so neither credit stands on one.
        Scenario::new(
            "sessions ended as asked",
            Flaws::default(),
            super::tests::with_end_sessions(users()),
            true,
        ),
        // Ending sessions (backlog 0492): a crash at the other session's page after the wrong password, or at an
        // ended session's page, must not read as the session ended.
        Scenario::new(
            "sessions ended without the password, and an admin's request ignored",
            Flaws {
                end_sessions_without_password: true,
                admin_end_sessions_does_nothing: true,
                ..Default::default()
            },
            super::tests::with_end_sessions(users()),
            true,
        ),
        // A member's page, served sandboxed and then as part of the app (backlog 0535): a crash at the save or
        // at the fetch must not read as the page served safely, or as served unsafely.
        Scenario::new(
            "a member's page served sandboxed",
            Flaws::default(),
            super::tests::with_page(users(), Some("/pages/{id}/frame")),
            true,
        ),
        Scenario::new(
            "a member's page served as part of the app",
            Flaws {
                pages_unsandboxed: true,
                ..Default::default()
            },
            super::tests::with_page(users(), Some("/pages/{id}/frame")),
            true,
        ),
        // The phone, two-factor, and recovery changes, each made without the password (ADR-105):
        // a crash at a wrong-password change, or at the page that shows its value, must not
        // read as that change refused.
        Scenario::new(
            "the account changes made without the password",
            Flaws {
                phone_change_without_password: true,
                two_factor_change_without_password: true,
                recovery_change_without_password: true,
                ..Default::default()
            },
            with_account_changes(with_signup()),
            false,
        ),
        // A second create, from `creates`, taken from another site (backlog 0232, part 3): a
        // crash at `forged-create-1` must not read as that request refused.
        Scenario::new(
            "a second create taken from another site",
            Flaws {
                comments_take_foreign: true,
                ..Default::default()
            },
            {
                let mut u = users();
                u.creates = vec![sv_manifest::CreateTemplate {
                    method: "POST".into(),
                    path: "/comments".into(),
                    form: [("text".to_owned(), "{marker}".to_owned())].into(),
                    json: Default::default(),
                    read: None,
                    id_field: None,
                    page: false,
                }];
                u
            },
            true,
        ),
        // Each of ADR-053's three on its own, so that crashing any one request is tried
        // against a run where that finding is the only one standing between it and a pass.
        Scenario::new(
            "another user's record listed",
            Flaws {
                list_shows_others: true,
                ..Default::default()
            },
            users_full(),
            true,
        ),
        Scenario::new(
            "another user's record changed",
            Flaws {
                idor_update: true,
                ..Default::default()
            },
            users_full(),
            true,
        ),
        Scenario::new(
            "another user's record deleted",
            Flaws {
                idor_delete: true,
                ..Default::default()
            },
            users_full(),
            true,
        ),
        Scenario::new(
            "seeded",
            Flaws {
                private_open: true,
                admin_open: true,
                idor: true,
                no_csrf_check: true,
                logout_keeps_session: true,
                flow_unguarded: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "two-factor codes twice",
            Flaws {
                totp_reusable: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario {
            policy: sv_manifest::PolicySection {
                context_words: vec!["Acme Notes".into()],
                failed_sign_ins: Some(3),
                within_minutes: Some(15),
                failed_codes: Some(3),
                requests_per_minute: Some(5),
                ..Default::default()
            },
            ..Scenario::new(
                "signed up",
                Flaws {
                    short_password_ok: true,
                    common_password_ok: true,
                    breached_password_ok: true,
                    context_word_ok: true,
                    case_folded: true,
                    change_without_current: true,
                    email_change_without_password: true,
                    booking_races: true,
                    code_guessing_unlimited: true,
                    signup_trusts_role: true,
                    email_change_trusts_role: true,
                    ..Default::default()
                },
                signed_up,
                true,
            )
        },
        Scenario::new(
            "signed up, emailed codes",
            Flaws {
                code_reusable: true,
                code_unbound: true,
                change_keeps_old: true,
                change_keeps_sessions: true,
                change_sends_no_email: true,
                deletion_keeps_sessions: true,
                ..Default::default()
            },
            with_signup(),
            false,
        ),
        Scenario::new(
            "a private WebSocket, uploads, and any session cookie",
            Flaws {
                ws_open: true,
                oversized_upload_ok: true,
                unchecked_contents_ok: true,
                svg_scripts_kept: true,
                no_malware_scan: true,
                upload_path_traversal: true,
                archive_size_unchecked: true,
                archive_files_unchecked: true,
                session_not_verified: true,
                ..Default::default()
            },
            sockets_and_files,
            true,
        ),
        Scenario {
            jwt: Some(30),
            ..Scenario::new(
                "the app's own tokens, neither signature nor expiry checked, and the key they \
                     name fetched",
                Flaws {
                    jwt_signature_ignored: true,
                    jwt_expiry_ignored: true,
                    jwt_key_source_followed: true,
                    ..Default::default()
                },
                users(),
                true,
            )
        },
        Scenario {
            policy: sv_manifest::PolicySection {
                idle_timeout_minutes: Some(15),
                session_lifetime_minutes: Some(60),
                ..Default::default()
            },
            slow: true,
            ..Scenario::new(
                "slow, sessions and codes that never end, and two-factor codes of any age",
                Flaws {
                    totp_any_age: true,
                    code_long_lived: true,
                    ..Default::default()
                },
                users(),
                true,
            )
        },
        // ADR-082, backlog 229 part 1: one scenario per rule the credit names. Each turns on a
        // flaw the checks already find on their own (see `each_flaw_is_found_by_its_own_rule`),
        // so the sweep can crash each request of a run where that rule is the one at fault.
        Scenario::new(
            "an announcement any signed-in user can post",
            Flaws {
                admin_action_open: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "a JSON create the app also takes as a form",
            Flaws {
                api_takes_forms: true,
                ..Default::default()
            },
            {
                let mut u = users();
                u.owned = Some(sv_manifest::OwnedSection {
                    create: RequestTemplate {
                        method: "POST".into(),
                        path: "/api/notes".into(),
                        form: BTreeMap::new(),
                        json: [("text".to_owned(), "{marker}".to_owned())].into(),
                    },
                    read: Some("/api/notes/{id}".into()),
                    id_field: None,
                    list: None,
                    update: None,
                    delete: None,
                });
                u
            },
            true,
        ),
        Scenario::new(
            "a password of lowercase letters refused",
            Flaws {
                composition_rules: true,
                ..Default::default()
            },
            with_signup(),
            false,
        ),
        Scenario::new(
            "a password longer than 64 characters refused",
            Flaws {
                longest_64: true,
                ..Default::default()
            },
            with_signup(),
            false,
        ),
        Scenario::new(
            "a reset address with a header typed in it",
            Flaws {
                reset_mails_typed_address: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "private pages without the browser's headers",
            Flaws {
                private_page_no_headers: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "private pages a browser may keep",
            Flaws {
                private_page_cacheable: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "no way to sign out on the private pages",
            Flaws {
                no_sign_out_link: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "the same session for every sign-in",
            Flaws {
                same_session_id: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "a session cookie without HttpOnly",
            Flaws {
                no_httponly: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "a session kept across sign-in",
            Flaws {
                keep_session_at_login: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        Scenario::new(
            "a password field that shows what is typed",
            Flaws {
                password_shown: true,
                ..Default::default()
            },
            users(),
            true,
        ),
        // One file of each set, the last, at fault alone (backlog 0232, part 4): a crash at its
        // request must not read as that file served safely.
        Scenario::new(
            "an uploaded .php5 that runs",
            Flaws {
                runs_uploaded_php5: true,
                ..Default::default()
            },
            upload_users(),
            true,
        ),
        Scenario::new(
            "an uploaded .xhtml that renders",
            Flaws {
                renders_uploaded_xhtml: true,
                ..Default::default()
            },
            upload_users(),
            true,
        ),
        Scenario::new(
            "an uploaded .php that runs",
            Flaws {
                runs_uploaded_code: true,
                ..Default::default()
            },
            upload_users(),
            true,
        ),
        Scenario::new(
            "an uploaded .html that renders",
            Flaws {
                renders_uploaded_pages: true,
                ..Default::default()
            },
            upload_users(),
            true,
        ),
        Scenario::new(
            "an uploaded file's name written into its header raw",
            Flaws {
                download_name_raw: true,
                ..Default::default()
            },
            upload_users(),
            true,
        ),
    ]
}

#[test]
fn a_private_page_that_crashes_for_a_stranger_is_not_credited_as_refused() {
    // The control: the correct app credits the private page.
    let plain = Scenario::new("plain", Flaws::default(), users(), true);
    let (o, _) = plain.run(None);
    assert!(
        verified_ids(&o).contains(&PRIVATE_PAGE.rule_id),
        "{:?}",
        o.steps
    );

    // The same app crashing on that one request: not credited, and the owner is told why.
    let (o, _) = plain.run(Some("private-anonymous"));
    assert!(!verified_ids(&o).contains(&PRIVATE_PAGE.rule_id));
    let (ids, why) = o
        .not_assessed
        .iter()
        .find(|(_, why)| why.contains(PRIVATE_PAGE.rule_id))
        .unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert!(ids.contains("V8.2.1"), "{ids}");
    assert!(why.contains("private-anonymous (500)"), "{why}");
    assert!(why.contains("crashed or did not answer"), "{why}");
    // Everything that did not rest on that request is still credited.
    assert!(
        verified_ids(&o).contains(&ADMIN_PAGE.rule_id),
        "{:?}",
        verified_ids(&o)
    );

    // No answer at all is not a refusal either.
    let (o, _) = plain.run_answering(Some("private-anonymous"), true);
    assert!(!verified_ids(&o).contains(&PRIVATE_PAGE.rule_id));
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.contains("private-anonymous (no answer)")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_page_that_crashes_after_a_plain_sign_out_link_is_not_reported_as_signed_out() {
    let correct = Scenario::new("correct", Flaws::default(), users(), true);
    let (o, sent) = correct.run(Some("private-after-get-logout"));
    assert!(
        sent.contains("private-after-get-logout"),
        "the setup reaches the check"
    );
    assert!(!rule_ids(&o).contains(&SIGN_OUT_ON_GET.rule_id));
    let (_, why) = o
        .not_assessed
        .iter()
        .find(|(_, why)| why.contains(SIGN_OUT_ON_GET.rule_id))
        .unwrap_or_else(|| panic!("{:?}", o.not_assessed));
    assert!(why.contains("private-after-get-logout (500)"), "{why}");
    assert!(why.contains("neither reported nor credited"), "{why}");

    // The app that really does sign out on a plain link is still reported, crash or no crash
    // elsewhere.
    let flawed = Scenario::new(
        "signs out on a plain link",
        Flaws {
            logout_on_get: true,
            ..Default::default()
        },
        users(),
        true,
    );
    let (o, _) = flawed.run(Some("private-anonymous"));
    assert!(
        rule_ids(&o).contains(&SIGN_OUT_ON_GET.rule_id),
        "{:?}",
        o.steps
    );
}

#[test]
fn a_crash_on_a_correct_app_raises_no_finding() {
    // The other direction: a correct app, each request crashed in turn. Any finding that
    // appears was raised by the crash, whether or not `RAISED_ON_A_REFUSAL` lists it.
    let mut signed_up = with_signup();
    signed_up.seed = Some("seed".into());
    signed_up.admin = vec!["/admin".into()];
    let mut sockets_and_files = users();
    // Two private WebSocket paths, so each is held to the crash guard by its own ids (ADR-118).
    sockets_and_files.private_websockets = vec!["/ws".into(), "/chat".into()];
    sockets_and_files.upload = vec![sv_manifest::UploadSection {
        path: "/upload".into(),
        field: "file".into(),
        form: [("csrf_token".to_owned(), "{csrf}".to_owned())].into(),
        serves_at: Some("/files/{name}".into()),
        max_bytes: Some(UPLOAD_LIMIT as u64),
        unpacks_archives: Some(vec![
            sv_manifest::ArchiveFormat::Zip,
            sv_manifest::ArchiveFormat::Gzip,
        ]),
        max_unpacked_bytes: Some(ARCHIVE_UNPACK_LIMIT),
        max_files: Some(ARCHIVE_FILE_LIMIT),
        accepts: None,
    }];
    let scenarios = [
        Scenario {
            policy: sv_manifest::PolicySection {
                context_words: vec!["Acme Notes".into()],
                failed_sign_ins: Some(3),
                within_minutes: Some(15),
                failed_codes: Some(3),
                ..Default::default()
            },
            ..Scenario::new(
                "signed up, with a limit on wrong passwords",
                Flaws {
                    locks_out_after: Some(3),
                    ..Default::default()
                },
                signed_up,
                true,
            )
        },
        Scenario::new(
            "a private WebSocket and uploads",
            Flaws::default(),
            sockets_and_files,
            true,
        ),
        Scenario {
            policy: sv_manifest::PolicySection {
                idle_timeout_minutes: Some(15),
                session_lifetime_minutes: Some(60),
                ..Default::default()
            },
            slow: true,
            limits: (Some(15 * 60), Some(60 * 60)),
            ..Scenario::new(
                "slow, with sessions that end",
                Flaws::default(),
                users(),
                true,
            )
        },
    ];
    let mut jobs: Vec<(&Scenario, String, std::time::Duration)> = Vec::new();
    for scenario in &scenarios {
        let started = std::time::Instant::now();
        let (plain, sent) = scenario.run(None);
        let took = started.elapsed();
        // The setup is a correct app: nothing to find before anything crashes.
        assert!(
            plain.findings.is_empty(),
            "{}: {:?}",
            scenario.name,
            plain
                .findings
                .iter()
                .map(|f| &f.rule_id)
                .collect::<Vec<_>>()
        );
        jobs.extend(sent.into_iter().map(|id| (scenario, id, took)));
    }
    jobs.sort_by_key(|(_, _, took)| std::cmp::Reverse(*took));
    let raised = each_at_once(&jobs, |(scenario, id, _)| {
        let (crashed, _) = scenario.run(Some(id));
        crashed
            .findings
            .iter()
            // The one finding a crash is meant to raise (backlog 0528, ADR-021, Later): the crash itself, on an
            // ordinary sign-up, resting on that sign-up alone.
            .filter(|f| {
                !(f.rule_id == CRASHED_ON_ORDINARY.rule_id
                    && is_ordinary_signup(id)
                    && f.evidence == [id.clone()])
            })
            .map(|f| format!("{}: {} raised when {id} crashed", scenario.name, f.rule_id))
            .collect()
    });
    assert!(raised.is_empty(), "{raised:#?}");
}

/// Every name in `RESTS_ON_A_REFUSAL` is an answer some scenario's run really sends, so a
/// misspelt name cannot leave a credit unnamed while the test still passes (ADR-082, 229 part 1).
#[test]
fn every_name_in_the_table_is_an_answer_some_run_sends() {
    let mut sent = BTreeSet::new();
    for scenario in scenarios() {
        sent.extend(scenario.run(None).1);
    }
    let missing: Vec<String> = RESTS_ON_A_REFUSAL
        .iter()
        .flat_map(|(rule, requests)| {
            requests
                .iter()
                .filter(|r| !sent.iter().any(|id| one_of(id, &[r])))
                .map(move |r| format!("{rule}: {r}"))
        })
        .collect();
    assert!(missing.is_empty(), "names no run sends: {missing:#?}");
}

#[test]
fn a_crash_never_turns_a_finding_into_a_pass() {
    let scenarios = scenarios();
    // Each scenario's run without a crash, timed, so the slowest scenarios' crashed runs start
    // first and none is left running alone at the end.
    let plain: Vec<(BTreeSet<String>, Vec<String>, std::time::Duration)> = scenarios
        .iter()
        .map(|s| {
            let started = std::time::Instant::now();
            let (found, sent) = s.plain();
            (found, sent, started.elapsed())
        })
        .collect();
    let mut jobs: Vec<(usize, &String)> = plain
        .iter()
        .enumerate()
        .flat_map(|(i, (_, sent, _))| sent.iter().map(move |id| (i, id)))
        .collect();
    jobs.sort_by_key(|(i, _)| std::cmp::Reverse(plain[*i].2));
    let turned = each_at_once(&jobs, |(i, id)| scenarios[*i].turned_by(id, &plain[*i].0));
    let reached: BTreeSet<String> = plain
        .iter()
        .flat_map(|(found, _, _)| found.clone())
        .collect();
    assert!(turned.is_empty(), "{turned:#?}");
    // The sweep is only as good as the findings it starts from: a rule listed here that no
    // scenario found at fault was never tried, so it is named rather than passed quietly.
    let missed: Vec<&str> = RESTS_ON_A_REFUSAL
        .iter()
        .map(|(rule, _)| *rule)
        .filter(|rule| !reached.contains(*rule))
        .collect();
    assert!(
        missed.is_empty(),
        "no scenario found these at fault: {missed:?}"
    );
}
