//! What a signed-in user can do, asked of the running app.
//!
//! The probes in `probes.rs` sign in as nobody, so authorization, sessions and request forgery were
//! always *not assessed*. This asks as two ordinary users, A and B — and an admin, when the app's
//! `seed` command makes one — using what `[stack.run.users]` in stackvet.toml says about how to sign
//! up, sign in and out, and which pages and records belong to whom.
//!
//! # Every check establishes its own setup first
//!
//! A test whose setup can fail quietly is worse than no test: B being refused A's note proves nothing
//! if A could not read it either, and "logout ended the session" proves nothing if the session never
//! worked. So each check first shows the thing it relies on — A's session reaches a private page, A
//! can read what A created, the admin can open the admin page — and when it cannot, the check reports
//! *not assessed* with the reason, never a pass.
//!
//! # Judgment here, requests elsewhere
//!
//! Like `probes.rs`, this decides what to ask and how to read the answers; it talks to the app only
//! through [`Http`]. `sv-run` supplies one that speaks from inside the network fence, and the tests
//! supply a scripted app with each flaw switchable, which is how every rule here is shown to fire.
//!
//! # Where each check lives
//!
//! This file holds what every check shares: sessions and cookies, anti-forgery tokens, requests
//! filled from stackvet.toml's templates, signing in and up, and `run_with`, which calls each
//! area's checks in turn. The rules each check can raise are in `rules.rs`. The checks themselves:
//!
//! - `signin.rs`: wrong-password limits (V6.3.1), `X-Forwarded-For`, default accounts, a password in
//!   an address, and signing out.
//! - `sessions.rs`: session cookies, ids, and timeouts, invented sessions, private pages and their
//!   caching, `Clear-Site-Data`, the fields a record gives back, and a private WebSocket's session
//!   and origin.
//! - `passwords.rs`: the password rules at sign-up (short, common, breached, context words, altered,
//!   long), the password field, changing a password, hints, and deleting an account.
//! - `reset.rs`: a forgotten-password reset, followed through the email it sends.
//! - `codes.rs`: signing in with an emailed code, and what the three email flows share for finding a
//!   code in an email.
//! - `limit.rs`: how the answers to wrong attempts past a stated limit are read, shared by the
//!   wrong-password (`signin.rs`) and wrong-code (`codes.rs`) checks.
//! - `activation.rs`: the activation code emailed at sign-up.
//! - `totp.rs`: two-factor codes from an authenticator app.
//! - `admin.rs`: the admin page and admin actions, a role given at sign-up, and records that belong
//!   to someone else.
//! - `forgery.rs`: cross-site request forgery, `Origin: null`, and forms another site can send
//!   without a preflight. The WebSocket origin check's tests are here too, beside the other origin
//!   tests; the check itself is in `sessions.rs`.
//! - `flows.rs`: steps of a multi-step flow taken out of order.
//! - `uploads.rs`: uploads and downloads.
//!
//! Each file's tests sit beside its checks. The tests here are the ones that exercise several areas
//! at once, and `fake_app.rs` is the scripted app they all drive.

use crate::finding::{Confidence, Finding, Location, Severity};
use crate::probes::{ProbeRequest, ProbeResponse};
use regex::Regex;
use std::collections::BTreeMap;
use std::sync::LazyLock;
use sv_manifest::{RequestTemplate, UploadSection, UsersSection};

mod account_changes;
mod activation;
mod admin;
mod archives;
mod burst;
mod business_limit;
mod codes;
mod control;
mod cross_site;
mod email_role;
mod flows;
mod forgery;
mod input_limits;
mod limit;
mod oauth_client;
mod once;
mod owner_field;
mod passwords;
pub mod recording;
mod redirects;
mod refused;
mod reset;
mod rules;
mod saved_page;
mod sessions;
mod sessions_end;
mod signin;
mod sql;
mod stored_markup;
mod tokens;
mod totp;
pub mod unset_keys;
mod uploads;
use activation::*;
use admin::*;
use burst::*;
use business_limit::*;
use codes::*;
use email_role::*;
use flows::*;
use forgery::*;
use input_limits::*;
use once::*;
use owner_field::*;
use passwords::*;
use redirects::*;
use reset::*;
use rules::*;
pub(crate) use rules::{LOGOUT, Rule, finding, finding_on};
use saved_page::*;
use sessions::*;
use sessions_end::*;
use signin::*;
use sql::*;
use stored_markup::*;
use tokens::*;
use totp::*;
use uploads::*;

mod cookies;
mod each_upload;
pub(crate) use cookies::Session;
use cookies::*;

mod form_tokens;
use form_tokens::*;

mod requests;
use requests::*;
pub(crate) use requests::{
    SeedValues, ShortNames, create_owned, get, ok, seed_names, send_filled, short_name, status,
    wants_short_names,
};

mod answers;
pub(crate) use answers::rate_limited;
use answers::*;
pub use answers::{Answer, answer_of};

mod patient;
pub use patient::{MOST_WAITING, Patient, ask_anonymously, ask_anonymously_within};

/// Something that can put a request to the running app and bring back its answer.
pub trait Http {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse>;

    /// Sends these requests at once, each over its own connection, all started together rather
    /// than one after another, and gives back each answer in the order given. A request may be
    /// given more than once, to send copies of it. `None` when this way of reaching the app cannot
    /// send at the same instant: requests sent one after another cannot show a race, so a check
    /// that needs one is then not assessed.
    fn send_together(&mut self, _requests: &[ProbeRequest]) -> Option<Vec<Option<ProbeResponse>>> {
        None
    }

    /// Sends these requests one after another, each waiting for its answer before the next starts,
    /// as `send` would, but handed to the app's side in one go, and gives back each answer in the
    /// order given. The app sees what it would see from `send` called for each in turn; the run
    /// pays for one way into the fence rather than one each. `None` when this way of reaching the
    /// app cannot, and the caller sends them one by one.
    fn send_in_turn(&mut self, _requests: &[ProbeRequest]) -> Option<Vec<Option<ProbeResponse>>> {
        None
    }

    /// The emails the app has sent to this address during the run, oldest first, as text: `None`
    /// when the run has no mail sink to read them from. Waits a little for there to be at least
    /// `at_least` of them, since an app may send its mail after it has answered.
    fn mail(&mut self, _to: &str, _at_least: usize) -> Option<Vec<String>> {
        None
    }

    /// Now, in seconds since 1970, by the clock the app is also reading. A test's fake app keeps
    /// its own, so a check that has to wait for the next time step does not really wait.
    fn now(&mut self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs())
    }

    /// Waits this many seconds, by that same clock.
    fn wait(&mut self, seconds: u64) {
        std::thread::sleep(std::time::Duration::from_secs(seconds));
    }

    /// A line for the person watching a long wait, such as `--slow`'s, so a run that is waiting does not look stuck
    /// (backlog 0247). Nothing by default: a test's fake app has no one watching, and the real run prints it.
    fn say(&mut self, _line: &str) {}

    /// The prompt `sv` itself wrote for the question it is about to send as `id` (backlog 0623, ADR-082, Later). Only the
    /// AI suite calls this, with the string it composed, before any field the app's page fills in: never a request's
    /// body or headers. The recorder keeps it beside the answer; a wrapper passes it on; anything else lets it go.
    fn prompt(&mut self, _id: &str, _prompt: &str) {}

    /// Sends a request to the run's test OpenID Connect provider rather than to the app: `None`
    /// when the run has no provider. The path is the provider's own, query included.
    fn provider(&mut self, _request: &ProbeRequest) -> Option<ProbeResponse> {
        None
    }

    /// Has the run's headless browser do what `job` says, and gives back one answer per action:
    /// `None` when the run has no browser, or it did not finish. See `browser.rs`.
    fn browser(&mut self, _job: &crate::browser::Job) -> Option<Vec<serde_json::Value>> {
        None
    }

    /// Sends a request to the run's test model rather than to the app: `None` when the run has no
    /// test model. See `ai.rs`.
    fn model(&mut self, _request: &ProbeRequest) -> Option<ProbeResponse> {
        None
    }

    /// The test model's server as the app reaches it on the fenced network, such as
    /// `http://sv-1-model:9100`: `None` when the run has no test model that answered. A wrapper
    /// must pass this on, or the checks behind it read as having no test server.
    fn model_address(&mut self) -> Option<String> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub user: String,
    pub password: String,
}

/// The accounts the run made: two ordinary users, and an admin when `seed` was asked for one.
#[derive(Debug, Clone)]
pub struct Accounts {
    pub a: Account,
    pub b: Account,
    pub admin: Option<Account>,
    /// Random hex, at least 32 characters, for the passwords the password checks sign up with.
    /// Made with the accounts so every run's are different and none can be guessed from the code.
    pub spare: String,
    /// A third account with two-factor sign-in, when stackvet.toml has a `totp` entry and a
    /// `seed` to enroll it. Never A or B: they have to keep signing in with a password alone.
    pub totp: Option<TotpAccount>,
    /// A secret for the admin's own two-factor sign-in, made with the accounts when there is an
    /// admin, a `totp` entry and a `seed`. `seed` is given it in base32 as `SV_ADMIN_TOTP_SECRET`
    /// and may enroll the admin with it; when the admin's sign-in then stops at the code step, the
    /// code is worked out from it. Held like the two-factor account's: never written anywhere but
    /// the app's container.
    pub admin_totp_secret: Option<Vec<u8>>,
    /// What `seed` printed as `NAME=value` lines, for the manifest's `{seed:NAME}` (backlog 0536, ADR-124): a value the
    /// app's own sign-up needs, such as an invite code, made inside the fence at run time. Held like the passwords:
    /// filled into the requests that cite it and taken out of everything the run writes down.
    pub seed_values: BTreeMap<String, String>,
}

/// An account `seed` enrolled in two-factor sign-in with a secret this run made.
#[derive(Debug, Clone)]
pub struct TotpAccount {
    pub account: Account,
    /// The raw secret. `seed` is given it in base32 as `SV_TOTP_SECRET`.
    pub secret: Vec<u8>,
}

/// What asking as a signed-in user showed.
#[derive(Debug, Default, Clone)]
pub struct Outcome {
    pub findings: Vec<Finding>,
    pub verified: Vec<crate::Verified>,
    /// What could not be asked, and why, in the words the report uses: requirement ids, reason.
    pub not_assessed: Vec<(String, String)>,
    /// What happened, in order, for the run note: "signed in as A", "B was refused A's record".
    pub steps: Vec<String>,
    /// The strings the probes planted for the log check to look for afterwards. See `logs.rs`.
    pub log_markers: crate::logs::Markers,
    /// Every question this suite asked the app and what it answered, in order (ADR-082, part 1).
    pub exchanges: Vec<recording::Recorded>,
    /// The lines of the app's output the log checks read, each with what for (ADR-082).
    pub log_lines: Vec<crate::logs::KeptLine>,
    /// The last lines of the app's output when the log was read, at most `crate::logs::TAIL`.
    pub log_tail: Vec<String>,
    /// How long each named check took, and all the waiting for the app's limit on requests, in milliseconds, in the
    /// order run (backlog 0526): inside the suite, so counted in no total, and named so a slow check, or a limiter
    /// waited out, is told from a slow app.
    pub timings: Vec<(String, u64)>,
    /// The field names in the readable part of the first account's sign-in token, when it is a JWT (backlog 0497).
    /// Names only, never the values.
    pub token_fields: Option<crate::personal_fields::TokenFields>,
    /// The other sites the signed-in pages tried to reach in a real browser, which the fence stopped, by host name
    /// (backlog 0496). `None` when the browser did not list them; compared with `[stack] external-apis` by
    /// `crate::browser::undeclared_outside_services`.
    pub outside_hosts: Option<Vec<String>>,
    /// The sign-in codes and tokens the app's own authorization server issued to the test client (ADR-151), blanked
    /// from everything the run writes down once the suite is done (`script::keep_out`), as the seed's values are.
    pub issued: Vec<String>,
}

/// An origin the app has certainly never heard of, for the forgery check.
const STRANGER: &str = "https://sv-probe-stranger.invalid";

// ------------------------------------------------------------------------------------------------
// The suite

/// A user signed in, with what the sign-in showed.
pub(crate) struct SignedIn {
    pub(crate) session: Session,
    /// Cookies set by the sign-in response itself: the session cookies.
    set_at_login: Vec<Cookie>,
    /// Cookies the app had given before sign-in.
    before_login: Vec<(String, String)>,
    /// Where the sign-in answer left the browser, for a reason when the session turns out not to
    /// be signed in: "answered 200", or "sent the browser to /login/2fa".
    pub(crate) landed: String,
    /// The sign-in's answer in a sentence: its status, the fields sent (names only), and the words the page showed,
    /// for the reason when the session turns out not to be signed in (backlog 0527).
    pub(crate) answer: String,
    /// Whether the sign-in was answered by the app's rate limiter, after the wait [`Patient`] gives
    /// it: the limit on sign-in attempts refused `sv`, so nothing about the session says anything
    /// about the app's checks or stackvet.toml.
    pub(crate) limited: bool,
}

/// The session checks (V3.3.2, V3.3.4, V7.2.4) of a sign-in made some other way than the form, through another service
/// (backlog 0232, part B): `before_login` the cookies the app gave before it, `back` the answer that finished it, and
/// `evidence` the two requests a finding rests on.
pub(crate) fn provider_session_checks(
    http: &mut dyn Http,
    session: Session,
    before_login: Vec<(String, String)>,
    back: &ProbeResponse,
    works: bool,
    (private, evidence): (&str, [&str; 2]),
    out: &mut Outcome,
) {
    let a = SignedIn {
        session,
        set_at_login: set_cookies(back),
        before_login,
        landed: String::new(),
        answer: String::new(),
        limited: false,
    };
    // Which cookie carries the session, shown as the form's run shows it: the private page asked without the cookies
    // the sign-in set.
    let carried = sign_in_cookies_carry_session(http, &a, works.then_some(private));
    session_checks(&a, works, carried, evidence, out);
}

pub(crate) fn sign_in(
    http: &mut dyn Http,
    users: &UsersSection,
    who: &str,
    account: &Account,
    steps: &mut Vec<String>,
) -> Option<SignedIn> {
    let login = users.login.as_ref()?;
    let mut session = Session::default();
    // The sign-in page first, as a browser would: this is where a token and a pre-login session
    // cookie come from.
    let mut csrf = None;
    if let Some(page) = http.send(&get(&format!("login-page-{who}"), &login.path, &session)) {
        session.absorb(&page);
        csrf = csrf_token(&page, &session);
    }
    let before_login = session.cookies.clone();
    let values = Values {
        user: &account.user,
        password: &account.password,
        csrf,
        ..Default::default()
    };
    let (response, set_at_login) = send_template(
        http,
        &format!("login-{who}"),
        login,
        &values,
        &mut session,
        &[],
    );
    let response = response?;
    let token = if values.csrf.is_some() {
        ", with the page's anti-forgery token"
    } else {
        ""
    };
    // Only a yes or no is taken from the password (backlog 0527): a page that repeats it is not quoted.
    let repeats_password = refused::repeats_password(&response, account);
    let answer = refused::answer(login, &response, repeats_password);
    if response.status >= 400 {
        // Refused: why, in the page's words, is the one thing that lets the person fix it (backlog 0527).
        // A crash is not a refusal (ADR-021): 500 and up "failed".
        steps.push(format!(
            "signing in as {} {} ({}{token}): {}",
            who.to_uppercase(),
            if response.status >= 500 {
                "failed"
            } else {
                "was refused"
            },
            response.status,
            refused::sent_and_said(login, &response, repeats_password)
        ));
    } else {
        steps.push(format!(
            "signed in as {} ({}{token})",
            who.to_uppercase(),
            response.status,
        ));
    }
    if let Some(field) = &users.token_field {
        session.bearer = serde_json::from_str::<serde_json::Value>(&response.body)
            .ok()
            .and_then(|v| v.get(field).and_then(|t| t.as_str()).map(str::to_owned));
    }
    let landed = match response.header("location") {
        Some(to) if (300..400).contains(&response.status) => {
            // The path alone: what follows `?` may be a token of the app's own.
            let path = strip_origin(to);
            format!(
                "sent the browser to {}",
                path.split(['?', '#']).next().unwrap_or(path)
            )
        }
        _ => format!("answered {}", response.status),
    };
    Some(SignedIn {
        session,
        set_at_login,
        before_login,
        landed,
        answer,
        limited: rate_limited(&response).is_some(),
    })
}

/// A signed in again after the session timeouts were waited out (`sv run --slow`), shown to open
/// `confirm` as the first sign-in did, when there is one to open. `None`, with the checks that
/// needed A's session left not assessed and why, when the app did not answer or the new session
/// did not open the page: the run goes no further, as when the first sign-in fails.
fn sign_in_again(
    http: &mut dyn Http,
    users: &UsersSection,
    a: &Account,
    confirm: Option<&str>,
    out: &mut Outcome,
) -> Option<SignedIn> {
    let refused = |out: &mut Outcome, what: String| {
        out.not_assessed.push((
            "V8.2.1, V8.2.2, V7.2.4, V7.4.1, V3.5.1, V3.3.2, V3.3.4, V14.3.2, V7.4.4".to_owned(),
            format!(
                "After waiting out the session timeouts, the first test user's earlier session had \
                 sat unused for the whole wait, so it was signed in again; {what}. The checks that \
                 needed a working session were not run, rather than run with one the app may have \
                 ended."
            ),
        ));
    };
    let Some(fresh) = sign_in(http, users, "a-after-wait", a, &mut out.steps) else {
        refused(out, "the app did not answer that sign-in".to_owned());
        return None;
    };
    if let Some(path) = confirm {
        let response = http.send(&get("private-a-after-wait", path, &fresh.session));
        out.steps.push(format!(
            "signed in as A again after the wait and opened {path} ({})",
            status(&response)
        ));
        if !ok(&response) {
            refused(
                out,
                format!(
                    "the new session did not open {path} ({})",
                    status(&response)
                ),
            );
            return None;
        }
    }
    Some(fresh)
}

/// How many times one run signs in, at most, before the password-guessing check's wrong passwords
/// at the very end: the number the spec gives a builder, so an app's own limit on sign-in attempts
/// can be set to allow it in the copy `sv` runs. The busiest of the scripted runs signs in 61 times
/// (`a_run_signs_in_no_more_often_than_the_spec_says`). It was 60 until the setup sweep (backlog 0559,
/// ADR-119): a check that makes an account of its own signs in as it once to show it was made.
pub const SIGN_INS_IN_A_RUN: usize = 62;

/// The requirements whose checks need the first test user signed in.
const NEEDS_A_SESSION: &str =
    "V8.2.1, V8.2.2, V7.2.4, V7.4.1, V3.5.1, V3.3.2, V3.3.4, V14.3.2, V7.4.4, V2.2.1, V2.3.2";

/// What the report says when the app's limit on sign-in attempts refused `sv`'s own sign-ins
/// (`refused`, as request ids): the limit doing its job, not the app failing the checks it blocked,
/// and what to change so the next run gets through.
fn sign_in_limit_says(refused: &[String]) -> String {
    format!(
        "The app's limit on sign-in attempts refused `sv`'s own sign-in ({}) even after `sv` \
         waited as the app asked. That is the limit working, not the app failing these checks, and \
         not a mistake in stackvet.toml: the checks that needed that sign-in were not run, or \
         not credited. `sv` signs in up to {SIGN_INS_IN_A_RUN} times in one run, all from one \
         address, before the password-guessing check's wrong passwords at the end. Keep the copy \
         of the app `sv` runs stopping wrong passwords at the number `failed-sign-ins` declares, \
         as the real app does. If the app also has a limit that counts every sign-in, right \
         passwords too, let the copy `sv` runs allow that many of those (for example, through a \
         setting only the test copy is started with), keep the real limit everywhere else, and \
         run again.",
        refused.join(", ")
    )
}

/// What the sign-in limit's gap is listed under: the requirements whose credit was withheld, and the
/// first user's checks only when it was the first user's sign-in that was refused, since any other's
/// left those running on a session the limit let through (the review of 6 October, item 6). With no
/// requirement to name, it says so in words.
fn limit_gap_names(withheld: &[&str], refused: &[String]) -> String {
    let mut ids: Vec<&str> = withheld.to_vec();
    if refused.iter().any(|id| id == "login-a") {
        ids.extend(NEEDS_A_SESSION.split(", "));
    }
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        "The checks that needed that sign-in".to_owned()
    } else {
        ids.join(", ")
    }
}

/// Everything the signed-in probes can ask, given what stackvet.toml says.
///
/// `seeded` says whether `seed` already made the accounts; when it did not, they are made through the
/// app's own sign-up.
pub fn run(
    http: &mut dyn Http,
    users: &UsersSection,
    accounts: &Accounts,
    seeded: bool,
    policy: &sv_manifest::PolicySection,
) -> Outcome {
    run_with(http, users, accounts, seeded, policy, false)
}

/// `run`, and with `slow` also the checks that have to wait: the session timeouts the owner states,
/// waited out (`sv run --slow`).
///
/// Every request goes through [`Patient`], which waits out a rate limiter's answer once. When the
/// limiter was still answering after that, no refusal in the run can be told from the limiter's, so
/// nothing the run would have credited is: each credit becomes not assessed, naming the requests the
/// limiter kept answering. Findings stay, since hiding a real one is the worse fault, and the run
/// says which of them could rest on the limiter's refusal rather than the app's.
pub fn run_with(
    http: &mut dyn Http,
    users: &UsersSection,
    accounts: &Accounts,
    seeded: bool,
    policy: &sv_manifest::PolicySection,
    slow: bool,
) -> Outcome {
    run_within(
        http,
        users,
        accounts,
        seeded,
        (policy, slow),
        &std::cell::Cell::new(0),
        None,
    )
}

/// The connection the checks are handed, counting what is asked of the app through it: requests,
/// requests sent together, and pages a browser opens. What a mock provider or the mailbox is asked
/// is not counted, since neither is the app.
struct Counted<'a> {
    inner: &'a mut dyn Http,
    asked: usize,
}

impl Http for Counted<'_> {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.asked += 1;
        self.inner.send(request)
    }

    fn send_together(&mut self, requests: &[ProbeRequest]) -> Option<Vec<Option<ProbeResponse>>> {
        self.asked += requests.len();
        self.inner.send_together(requests)
    }

    fn mail(&mut self, to: &str, at_least: usize) -> Option<Vec<String>> {
        self.inner.mail(to, at_least)
    }

    fn now(&mut self) -> u64 {
        self.inner.now()
    }

    fn wait(&mut self, seconds: u64) {
        self.inner.wait(seconds)
    }

    fn say(&mut self, line: &str) {
        self.inner.say(line)
    }

    fn prompt(&mut self, id: &str, prompt: &str) {
        self.inner.prompt(id, prompt)
    }

    fn provider(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.inner.provider(request)
    }

    fn browser(&mut self, job: &crate::browser::Job) -> Option<Vec<serde_json::Value>> {
        self.asked += 1;
        self.inner.browser(job)
    }

    fn model(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.inner.model(request)
    }

    fn model_address(&mut self) -> Option<String> {
        self.inner.model_address()
    }
}

/// What `out` held before a check ran, and what had been asked of the app: where each list ended.
struct Said {
    asked: usize,
    findings: usize,
    verified: usize,
    not_assessed: usize,
}

impl Said {
    fn mark(out: &Outcome, asked: usize) -> Self {
        Said {
            asked,
            findings: out.findings.len(),
            verified: out.verified.len(),
            not_assessed: out.not_assessed.len(),
        }
    }

    /// The requirement ids the check named since `self`: found, credited, or not assessed.
    fn since(&self, out: &Outcome) -> std::collections::BTreeSet<String> {
        let mut ids = std::collections::BTreeSet::new();
        for f in &out.findings[self.findings..] {
            ids.extend(f.requirement_ids.iter().cloned());
        }
        for v in &out.verified[self.verified..] {
            ids.extend(v.requirement_ids.iter().cloned());
        }
        for (list, _) in &out.not_assessed[self.not_assessed..] {
            ids.extend(list.split(',').map(|id| id.trim().to_owned()));
        }
        ids.retain(|id| !id.is_empty());
        ids
    }

    /// Holds a check that speaks once it has asked: when it asked the app anything and named none
    /// of `speaks_to`, those are not assessed, "asked and never answered", rather than left out of
    /// the report; and sv-check's own tests stop there, so the check is fixed rather than
    /// covered for.
    fn held(&self, out: &mut Outcome, check: &str, speaks_to: &[&str], asked: usize) {
        if asked == self.asked {
            return;
        }
        let named = self.since(out);
        if speaks_to.iter().any(|id| named.contains(*id)) {
            return;
        }
        // sv-check's own tests stop here, against the fake app, where every check's every path is
        // known. Not any debug build: `sv` built for the workspace's tests runs real apps in
        // Docker, and there a silent check is said in the report rather than ending the run.
        if cfg!(test) {
            panic!(
                "`{check}` asked the app {} time(s) and returned without naming any of \
                 {speaks_to:?}: a finding, a credit, or a not-assessed entry with its reason",
                asked - self.asked
            );
        }
        out.not_assessed.push((
            speaks_to.join(", "),
            format!(
                "A check asked the app about this and returned without saying what it found \
                 (`{check}`), so nothing about it is known from this run."
            ),
        ));
    }
}

/// Runs one check of the suite that speaks once it has asked, and holds it to that
/// (`Said::held`): the architecture assessment of 8 October 2026, item 8, which found four checks
/// that returned without a word. `speaks_to` is every requirement the check names.
macro_rules! asked {
    ($out:ident, $http:ident, $check:literal, $speaks_to:expr, $call:expr) => {{
        let said = Said::mark(&$out, $http.asked);
        // Timed, so the report can say which check a slow suite spent its time in (backlog 0526).
        let started = std::time::Instant::now();
        let answer = $call;
        $out.timings.push((
            format!("the check `{}`", $check),
            u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        ));
        said.held(&mut $out, $check, $speaks_to, $http.asked);
        answer
    }};
}

/// Runs one check that may ask and then say nothing, as its answer: its rules are findings only
/// (`tools/coverage.py`, the findings-only list), or it credits only what it can show, so saying
/// nothing means nothing was found and nothing shown. Marked, so every check in the suite is one
/// kind or the other (`check_guard_tests`).
macro_rules! quiet {
    ($call:expr) => {
        $call
    };
}

/// `run_with`, with the waiting for a limiter counted against `spent`, the run's one budget.
pub fn run_within(
    http: &mut dyn Http,
    users: &UsersSection,
    accounts: &Accounts,
    seeded: bool,
    (policy, slow): (&sv_manifest::PolicySection, bool),
    spent: &std::cell::Cell<u64>,
    oauth_client: Option<&sv_manifest::OauthClientSection>,
) -> Outcome {
    // A user name apart from the address (backlog 0599): short names for the whole run, on this thread.
    let _short = ShortNames::set(users.signup.as_ref().is_some_and(wants_short_names));
    // The seed's values (backlog 0536): cited as `{seed:NAME}`, filled on this thread only.
    let _seed = SeedValues::set(&accounts.seed_values);
    let mut patient = Patient::within(http, spent);
    patient.login_path = users
        .login
        .as_ref()
        .map(|login| login.path.split('?').next().unwrap_or_default().to_owned());
    let mut out = run_checks(
        &mut patient,
        users,
        accounts,
        seeded,
        (policy, slow),
        oauth_client,
    );
    patient.settle(&mut out);
    out
}

/// What a limiter that kept answering after the wait means for `out` (ADR-021): said once; the
/// sign-ins it refused named; every credit not assessed, since a refusal may have been the
/// limiter's; the findings kept, with a note.
/// The waits for the app's limit on requests, said once in the steps and kept as one timing (backlog 0526): in the
/// three-tool trial of 14 October 2026 a suite took 129 s, its requests 7 s, and nothing said where the rest went.
fn settle_waits(waits: &[(String, u64)], out: &mut Outcome) {
    if waits.is_empty() {
        return;
    }
    let seconds: u64 = waits.iter().map(|(_, s)| s).sum();
    out.steps.push(format!(
        "waited {seconds} s in all for the app's limit on requests, {} time{}, as its answers asked, before asking \
         again: {}",
        waits.len(),
        if waits.len() == 1 { "" } else { "s" },
        waits
            .iter()
            .map(|(id, s)| format!("{id} ({s} s)"))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.timings.push((
        "waiting for the app's limit on requests".to_owned(),
        seconds.saturating_mul(1000),
    ));
}

fn settle_limited(patient: &mut Patient<'_>, out: &mut Outcome) {
    if patient.still_limited.is_empty() {
        return;
    }
    let limited = patient.still_limited.join(", ");
    out.steps.push(format!(
        "the app's rate limiter was still answering after waiting as it asked: {limited}"
    ));
    // The sign-ins the limit refused, said once and plainly, under every requirement whose check
    // they kept from running or from being credited.
    let refused_sign_ins = std::mem::take(&mut patient.limited_sign_ins);
    if !refused_sign_ins.is_empty() {
        let withheld: Vec<&str> = out
            .verified
            .iter()
            .flat_map(|v| v.requirement_ids.iter().map(String::as_str))
            .collect();
        out.not_assessed.push((
            limit_gap_names(&withheld, &refused_sign_ins),
            sign_in_limit_says(&refused_sign_ins),
        ));
    }
    for credit in std::mem::take(&mut out.verified) {
        out.not_assessed.push((
            credit.requirement_ids.join(", "),
            format!(
                "`{}` would have been credited ({}), but the app's rate limiter was still refusing \
                 requests after `sv` waited as it asked ({limited}), so a refusal here may be the \
                 limiter's rather than the app's own check. Run again with the limiter relaxed for \
                 the test run, or with a longer wait between requests.",
                credit.check_id, credit.scope
            ),
        ));
    }
    if !out.findings.is_empty() {
        let mut ids: Vec<&str> = out
            .findings
            .iter()
            .flat_map(|f| f.requirement_ids.iter().map(String::as_str))
            .collect();
        ids.sort_unstable();
        ids.dedup();
        out.not_assessed.push((
            ids.join(", "),
            format!(
                "The app's rate limiter was still refusing requests after `sv` waited as it asked \
                 ({limited}). The findings above are kept, but one that rests on the app refusing \
                 something may be the limiter's refusal: read each against its evidence."
            ),
        ));
    }
}

/// The passes the signed-in checks credit because the app refused something, and the requests
/// whose refusal earns each: an id, or the start of one when it ends in `-`. A server error (5xx),
/// or no answer at all, is read as "not 2xx" and so as refused wherever a refusal is read, which
/// credited a page that crashes for a stranger as refused to somebody not signed in. A crash is
/// not an answer to the question, so a pass here is withheld when any of its requests crashed.
///
/// `a_crash_never_turns_a_finding_into_a_pass` holds it: it runs the scripted app with its flaws
/// switched on, crashes each request a run sends, one at a time, and fails when a rule held back
/// without the crash is credited with it. It found requests this list first missed, of
/// five kinds. It can only try a rule some scenario holds back (finds at fault, or, for a rule
/// that is only ever credited, leaves open), and fails when
/// a rule listed here is not; a new check that credits a refusal needs its flaw added to a
/// scenario there.
const RESTS_ON_A_REFUSAL: &[(&str, &[&str])] = &[
    (PRIVATE_PAGE.rule_id, &["private-anonymous"]),
    // Ending sessions (backlog 0492): the wrong password refused, and the ended session refused after.
    (
        SESSIONS_END_WITHOUT_PASSWORD.rule_id,
        &["end-sessions-wrong", "private-end-2-after"],
    ),
    (
        ADMIN_END_SESSIONS_IGNORED.rule_id,
        &["private-end-target-after"],
    ),
    (ADMIN_PAGE.rule_id, &["login-a", "admin-a-", "admin-admin-"]),
    (ROLE_FIELD.rule_id, &["role-admin-"]),
    (
        EMAIL_ROLE_FIELD.rule_id,
        &["email-role-admin-", "email-role-change"],
    ),
    (
        OTHER_USERS_DATA.rule_id,
        &[
            "login-b",
            "owned-b",
            "owned-anonymous",
            // ADR-053: the lists the second user opens, the change and delete they send, and the
            // first user's read-back after each. A crashed change or delete is not one refused.
            "owned-b-list-",
            "owned-b-update",
            "owned-b-delete",
            "owned-a-after-",
            // Backlog 0560: the first user's read just before the delete; a crashed one leaves a delete not
            // judged rather than refused.
            "owned-a-before-delete",
            // ADR-053, Later: the first user's own request at a second record, which shows the
            // request reaches a route; a crashed control is not a control that worked.
            "owned-control-",
        ],
    ),
    // The `owned` create, and each `creates` entry forged after it (backlog 0232, part 3).
    (FORGERY.rule_id, &["forged-create", "forged-create-"]),
    // Each flow's fresh sign-ins as B, the first flow's `login-flow-b<i>` and each later one's
    // `login-flow-<k>-b<i>` (ADR-118).
    (STEP_SKIPPED.rule_id, &["login-flow-"]),
    (
        EMAIL_CODE_UNBOUND.rule_id,
        &["email-code-use-crossed", "email-code-private-crossed"],
    ),
    (
        EMAIL_CODE_REUSABLE.rule_id,
        &["email-code-use-again", "email-code-private-again"],
    ),
    (
        EMAIL_CODE_LONG_LIVED.rule_id,
        &["email-code-use-late", "email-code-private-late"],
    ),
    (
        EMAIL_CODE_GUESSING.rule_id,
        &[
            "email-code-use-guess-",
            "email-code-use-after-guesses",
            "email-code-private-after-guesses",
        ],
    ),
    (NO_BRUTE_FORCE_LIMIT.rule_id, &["guess-"]),
    (
        BREACHED_PASSWORD.rule_id,
        &["signup-breached", "login-breached", "private-breached"],
    ),
    (
        SHORT_PASSWORD.rule_id,
        &["signup-short", "login-short", "private-short"],
    ),
    (
        COMMON_PASSWORD.rule_id,
        &[
            "signup-common",
            "login-common",
            "private-common",
            // ADR-055: the two more common words.
            "signup-common-b",
            "login-common-b",
            "private-common-b",
            "signup-common-c",
            "login-common-c",
            "private-common-c",
        ],
    ),
    (
        CONTEXT_WORD_PASSWORD.rule_id,
        &["signup-context", "login-context", "private-context"],
    ),
    (
        ALTERED_PASSWORD.rule_id,
        &["login-case", "private-case", "login-cut", "private-cut"],
    ),
    (CHANGE_PASSWORD.rule_id, &["login-old", "private-old"]),
    (
        CHANGE_WITHOUT_CURRENT.rule_id,
        &[
            "change-password-wrong-current",
            "login-wrong-current",
            "login-changed",
            "private-changed",
        ],
    ),
    (CHANGE_ENDS_SESSIONS.rule_id, &["bystander-after"]),
    (DONE_TWICE.rule_id, &["once-"]),
    (CREATE_UNLIMITED.rule_id, &["burst-"]),
    (
        EMAIL_CHANGE_WITHOUT_PASSWORD.rule_id,
        &[
            "change-email-wrong-password",
            "login-email-wrong-password",
            "login-email-moved-wrong",
            "private-email-moved-wrong",
        ],
    ),
    (
        PHONE_CHANGE_WITHOUT_PASSWORD.rule_id,
        &["change-phone-wrong-password", "shows-phone-wrong-password"],
    ),
    (
        TWO_FACTOR_CHANGE_WITHOUT_PASSWORD.rule_id,
        &[
            "change-two-factor-wrong-password",
            "shows-two-factor-wrong-password",
        ],
    ),
    (
        RECOVERY_CHANGE_WITHOUT_PASSWORD.rule_id,
        &[
            "change-recovery-wrong-password",
            "shows-recovery-wrong-password",
        ],
    ),
    (
        SESSIONS_SURVIVE_DELETION.rule_id,
        &["delete-after", "login-deleted", "private-deleted"],
    ),
    (NO_IDLE_TIMEOUT.rule_id, &["timeout-idle-after"]),
    (
        NO_SESSION_LIFETIME.rule_id,
        &["timeout-busy-after-lifetime"],
    ),
    (SESSION_TOKEN_UNVERIFIED.rule_id, &["invented-session"]),
    (APP_TOKEN_UNSIGNED.rule_id, &["token-altered"]),
    (APP_TOKEN_ALG_NONE.rule_id, &["token-alg-none"]),
    (APP_TOKEN_EXPIRED.rule_id, &["token-expired"]),
    // Each private WebSocket path's handshakes, the first path's ids and each later one's with `-<k>`
    // after them (ADR-118). The signed-in handshake is each path's control: with several paths, one
    // whose control crashed would otherwise go unjudged while another path's refusal credits it.
    (
        WS_WITHOUT_SESSION.rule_id,
        &[
            "websocket-signed-in",
            "websocket-signed-in-",
            "websocket-no-session",
            "websocket-no-session-",
            "websocket-invented-session",
            "websocket-invented-session-",
        ],
    ),
    (
        WS_FOREIGN_ORIGIN.rule_id,
        &[
            "websocket-signed-in",
            "websocket-signed-in-",
            "websocket-foreign-origin",
            "websocket-foreign-origin-",
        ],
    ),
    (LOGOUT.rule_id, &["after-logout"]),
    (
        TOTP_OLD_CODE.rule_id,
        &["login-1", "totp-1", "totp-confirm-1"],
    ),
    (
        TOTP_REUSED.rule_id,
        &["login-3-", "totp-3-", "totp-confirm-3-"],
    ),
    (OVERSIZED_FILE.rule_id, &["upload-oversized"]),
    (CONTENT_MISMATCH.rule_id, &["upload-mismatched"]),
    (UPLOAD_SVG_SCRIPT.rule_id, &["upload-svg"]),
    (UPLOAD_PATH_TRAVERSAL.rule_id, &["upload-traversal"]),
    (UPLOAD_NOT_SCANNED.rule_id, &["upload-eicar"]),
    (ARCHIVE_UNCHECKED.rule_id, &["upload-archive-"]),
    // Credits that rest on a refusal or an answer, named by the requests the check read (ADR-082,
    // backlog 229 part 1). A credit that reads its answers from a helper names every request it made.
    (ADMIN_ACTION.rule_id, &["admin-action-"]),
    (SIMPLE_REQUEST.rule_id, &["simple-request-"]),
    (
        COMPOSITION_RULES.rule_id,
        &["signup-lower", "login-lower", "private-lower"],
    ),
    (
        LONG_PASSWORD.rule_id,
        &["signup-long", "login-long", "private-long"],
    ),
    (UNMASKED_PASSWORD.rule_id, &["password-field-"]),
    (CHANGE_NOTIFIED.rule_id, &["change-password-right-current"]),
    (MAIL_HEADER_INJECTED.rule_id, &["reset-header-"]),
    (PRIVATE_PAGE_CACHING.rule_id, &["private-page-headers-"]),
    (PRIVATE_PAGE_HEADERS.rule_id, &["private-page-headers-"]),
    (SIGN_OUT_LINK.rule_id, &["private-page-headers-"]),
    (
        STATIC_SESSION.rule_id,
        &[
            "login-a",
            "login-a-again",
            "login-page-a-again",
            "static-again",
        ],
    ),
    (SESSION_COOKIE.rule_id, &["login-a"]),
    (SESSION_RENEWAL.rule_id, &["login-page-a", "login-a"]),
    (DOWNLOAD_NAME_INJECTED.rule_id, &["download-hostile"]),
    (UPLOAD_EXECUTED.rule_id, &["upload-code", "upload-code-"]),
    (
        UPLOAD_RENDERED.rule_id,
        &["upload-page-file", "upload-page-file-"],
    ),
];

/// The findings the signed-in checks raise because the app refused something, or answered two
/// requests differently, and the requests whose answer raises each. A crash on one of them reads
/// as the app refusing: a page that fails after signing out by a plain link reads as the session
/// ended, a sign-up with a lowercase or a long password that fails reads as the password refused,
/// a reset for nobody that fails reads as answered differently from one for a real account, and a
/// guess that fails may never have been counted by the app's limit. So a finding here is moved to
/// not assessed when any of its requests crashed.
///
/// `a_crash_on_a_correct_app_raises_no_finding` holds it: it crashes each request of a correct
/// app, one at a time, and fails on any finding that appears, listed here or not.
const RAISED_ON_A_REFUSAL: &[(&str, &[&str])] = &[
    (SIGN_OUT_ON_GET.rule_id, &["private-after-get-logout"]),
    // The other session refused after a request without the right password (backlog 0492).
    (
        SESSIONS_END_WITHOUT_PASSWORD.rule_id,
        &["private-end-2-after-wrong", "private-end-2-after"],
    ),
    (
        COMPOSITION_RULES.rule_id,
        &["signup-lower", "login-lower", "private-lower"],
    ),
    (
        LONG_PASSWORD.rule_id,
        &["signup-long", "login-long", "private-long"],
    ),
    (RESET_REVEALS_ACCOUNT.rule_id, &["reset-request-"]),
    (SIGNIN_REVEALS_ACCOUNT.rule_id, &["reveal-"]),
    (SIGNUP_REVEALS_ACCOUNT.rule_id, &["signup-reveal-"]),
    (NO_BRUTE_FORCE_LIMIT.rule_id, &["guess-"]),
];

/// Whether `id` is one of `requests`: equal to one, or the page fetched for its form's token
/// first (`signup-short-page`), or starting with one that ends in `-`.
/// Names on each signed-in credit the recorded answers its entry in `RESTS_ON_A_REFUSAL` lists, so a
/// credit names only answers that were actually sent (ADR-082, backlog 0229, part 1).
pub(crate) fn name_credits(verified: &mut [crate::Verified], exchanges: &[recording::Recorded]) {
    for credit in verified.iter_mut() {
        let Some((_, requests)) = RESTS_ON_A_REFUSAL
            .iter()
            .find(|(rule, _)| *rule == credit.check_id)
        else {
            continue;
        };
        credit.evidence = exchanges
            .iter()
            .map(|exchange| exchange.id.clone())
            .filter(|id| one_of(id, requests))
            .collect();
    }
}

fn one_of(id: &str, requests: &[&str]) -> bool {
    let id = id.strip_suffix("-page").unwrap_or(id);
    requests
        .iter()
        .any(|r| id == *r || (r.ends_with('-') && id.starts_with(r)))
}

/// Moves each pass that rests on a refusal, and each finding raised from one, to not assessed when
/// one of its requests crashed.
fn withhold_what_rests_on_a_crash(crashed: &[(String, String)], out: &mut Outcome) {
    if crashed.is_empty() {
        return;
    }
    let mut kept = Vec::new();
    for credit in std::mem::take(&mut out.verified) {
        let rests_on = RESTS_ON_A_REFUSAL
            .iter()
            .find(|(rule, _)| *rule == credit.check_id)
            .map_or(&[][..], |(_, requests)| *requests);
        let answers: Vec<String> = crashed
            .iter()
            .filter(|(id, _)| one_of(id, rests_on))
            .map(|(id, status)| format!("{id} ({status})"))
            .collect();
        if answers.is_empty() {
            kept.push(credit);
            continue;
        }
        out.not_assessed.push((
            credit.requirement_ids.join(", "),
            format!(
                "`{}` would have been credited ({}) because the app did not let something \
                 through, but the app crashed or did not answer rather than refusing: {}. A crash \
                 is not an answer to whether it would have let it through, so this is not \
                 credited. Fix the error, and run it again.",
                credit.check_id,
                credit.scope,
                answers.join(", ")
            ),
        ));
    }
    out.verified = kept;

    let mut kept = Vec::new();
    for finding in std::mem::take(&mut out.findings) {
        let rests_on = RAISED_ON_A_REFUSAL
            .iter()
            .find(|(rule, _)| *rule == finding.rule_id)
            .map_or(&[][..], |(_, requests)| *requests);
        let answers: Vec<String> = crashed
            .iter()
            .filter(|(id, _)| one_of(id, rests_on))
            .map(|(id, status)| format!("{id} ({status})"))
            .collect();
        if answers.is_empty() {
            kept.push(finding);
            continue;
        }
        out.not_assessed.push((
            finding.requirement_ids.join(", "),
            format!(
                "`{}` (\"{}\") would have been reported because the app seemed to refuse \
                 something, but the app crashed or did not answer rather than refusing: {}. A \
                 crash is not an answer, so this is neither reported nor credited. Fix the error, \
                 and run it again.",
                finding.rule_id,
                finding.title,
                answers.join(", ")
            ),
        ));
    }
    out.findings = kept;
}

/// The checks themselves, in order; `run_with` wraps them.
fn run_checks(
    http: &mut dyn Http,
    users: &UsersSection,
    accounts: &Accounts,
    seeded: bool,
    (policy, slow): (&sv_manifest::PolicySection, bool),
    oauth_client: Option<&sv_manifest::OauthClientSection>,
) -> Outcome {
    let mut counted = Counted {
        inner: http,
        asked: 0,
    };
    let http = &mut counted;
    let mut out = Outcome::default();
    let problems = users.problems();
    if !problems.is_empty() {
        out.not_assessed.push((
            "V8.2.1, V8.2.2, V7.2.4, V7.4.1, V3.5.1, V14.3.2, V7.4.4".to_owned(),
            format!(
                "[stack.run.users] in stackvet.toml cannot be used: {}.",
                problems.join("; ")
            ),
        ));
        return out;
    }
    // The Secure attribute cannot be judged here at all, and saying so beats a finding every app
    // would get: inside the fence the app is reached over plain HTTP.
    out.not_assessed.push((
        "V3.3.1".to_owned(),
        "Whether cookies carry Secure: the app was reached over plain HTTP inside the fence, where \
         many apps rightly leave it off. Check the production settings."
            .to_owned(),
    ));

    // A sign-up citing a value the seed did not print is sent as written, and says so (backlog 0536).
    if let Some(signup) = &users.signup {
        let missing: Vec<String> = seed_names(signup)
            .into_iter()
            .filter(|name| !accounts.seed_values.contains_key(name))
            .collect();
        if !missing.is_empty() {
            out.steps.push(format!(
                "the sign-up cites {}, which the seed did not print as a `NAME=value` line, so each sign-up is sent \
                 with the placeholder as written",
                missing
                    .iter()
                    .map(|n| format!("`{{seed:{n}}}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
    // What A's sign-up answered, for the reason when A then cannot open a private page (backlog 0599).
    let mut signed_up_a: Option<String> = None;
    if !seeded && let Some(signup) = &users.signup {
        for (who, account) in [("a", &accounts.a), ("b", &accounts.b)] {
            let response = sign_up(http, users, signup, who, account);
            out.steps.push(format!(
                "signed up {} ({})",
                who.to_uppercase(),
                status(&response)
            ));
            if who == "a" {
                signed_up_a = Some(signed_up(signup, response.as_ref(), account));
            }
        }
    }

    // 1. Private pages, as nobody. This needs no account, so it runs whatever happens next.
    //    A page that answers exactly as the front page does is the app's page shell: a single-page
    //    app sends everybody the same page and draws it in the browser from what it fetches after.
    //    That answer is neither the private page served nor refused, so the page is set aside here
    //    and every check after this one is given the private pages that are left.
    let anonymous = Session::default();
    let front = http.send(&get("front-anonymous", "/", &anonymous));
    let mut served_anonymously = Vec::new();
    let mut shells = Vec::new();
    for path in &users.private {
        let response = http.send(&get("private-anonymous", path, &anonymous));
        if is_page_shell(&response, &front) {
            shells.push(path.clone());
        } else if let Some(page) = response.filter(|r| (200..300).contains(&r.status)) {
            served_anonymously.push((path.clone(), page));
        }
    }
    let judged;
    let users = if shells.is_empty() {
        users
    } else {
        out.steps.push(
            format!(
                "{} answered exactly as the front page does, so not judged: {}",
                shells.join(", "),
                if shells.len() == 1 {
                    "it is"
                } else {
                    "they are"
                }
            ) + " the app's page shell",
        );
        judged = UsersSection {
            private: users
                .private
                .iter()
                .filter(|p| !shells.contains(p))
                .cloned()
                .collect(),
            ..users.clone()
        };
        &judged
    };
    if !shells.is_empty() && users.private.is_empty() {
        out.not_assessed.push((
            PRIVATE_PAGE.requirement_ids.join(", "),
            format!(
                "Every private page ({}) answered somebody not signed in exactly as the front page \
                 does: the app's page shell, which a single-page app sends everybody and fills in \
                 the browser. Whether the page is private is decided by what the browser fetches \
                 next. List those addresses (`/api/me`, for example) under `private` in \
                 stackvet.toml, and they are asked instead.",
                shells.join(", ")
            ),
        ));
    }

    // 1a. The sign-in page asked for twice, as two strangers: one anti-forgery token for everybody is no
    //     token at all. This needs no account either.
    quiet!(shared_token_check(http, users, &mut out));

    // 2. Signing in, and showing it worked. Without this every check below would be comparing
    //    refusals to refusals.
    let Some(a) = sign_in(http, users, "a", &accounts.a, &mut out.steps) else {
        let pages: Vec<String> = served_anonymously.into_iter().map(|(p, _)| p).collect();
        out.findings.extend(served_without_signing_in(&pages));
        out.not_assessed.push((
            "V8.2.1, V8.2.2, V7.2.4, V7.4.1, V3.5.1, V3.3.2, V3.3.4, V14.3.2, V7.4.4".to_owned(),
            "Signing in as the first test user got no answer from the app.".to_owned(),
        ));
        return out;
    };
    // A page served to somebody not signed in is read beside the same page served to the signed-in user (backlog
    // 0560, ADR-119): a sign-in form served with 200 at the private address is a 2xx too. Without the user's own
    // page to read it beside, it is a finding as before.
    // Every page served to somebody not signed in, judged or not: none of them is credited below.
    let served_anonymously: Vec<String> = {
        let mut opened = Vec::new();
        let mut unlike = Vec::new();
        let paths = served_anonymously.iter().map(|(p, _)| p.clone()).collect();
        for (i, (path, page)) in served_anonymously.into_iter().enumerate() {
            let own = http.send(&get(&format!("private-a-beside-{i}"), &path, &a.session));
            match own.filter(|r| (200..300).contains(&r.status)) {
                Some(own) if !control::shows_page(&own, &page) => unlike.push(path),
                _ => opened.push(path),
            }
        }
        out.findings.extend(served_without_signing_in(&opened));
        if !unlike.is_empty() {
            out.not_assessed.push((
            PRIVATE_PAGE.requirement_ids.join(", "),
            format!(
                "Asked as somebody not signed in, {} answered 200 with a page that shows little of \
                 what the signed-in user was served there, such as a sign-in form served with 200 \
                 rather than a redirect to one. It is neither a finding nor a refusal.",
                unlike.join(", ")
            ),
        ));
        }
        paths
    };
    let confirm_path = users.private.first().cloned();
    let signed_in_works = match &confirm_path {
        Some(path) => {
            let response = http.send(&get("private-a", path, &a.session));
            let works = ok(&response) && !served_anonymously.contains(path);
            out.steps.push(format!(
                "signed in as A and opened {path} ({})",
                status(&response)
            ));
            works
        }
        None => false,
    };
    if confirm_path.is_some() && !signed_in_works && served_anonymously.is_empty() {
        if a.limited {
            // Not the sign-in request, the accounts, or the page: the app's limit refused it, and
            // `run_with` says so, with what to change, once the run is over.
            out.steps.push(
                "signing in as A was refused by the app's limit on sign-in attempts".to_owned(),
            );
            return out;
        }
        out.not_assessed.push((
            NEEDS_A_SESSION.to_owned(),
            format!(
                "Signing in as the first test user did not open {} — the sign-in request, the \
                 accounts, or the page is not what stackvet.toml says — so nothing here can say \
                 what a signed-in user can do. To tell which: {}{}.",
                confirm_path.as_deref().unwrap_or(""),
                a.answer,
                signed_up_a
                    .as_deref()
                    .map(|said| format!("; before that, {said}"))
                    .unwrap_or_default()
            ),
        ));
        return out;
    }
    // Each private page after the first opened as A too (backlog 0561, ADR-119): a listed address that refuses
    // everybody, a wrong path or a page that needs a role A lacks, is refused to a stranger as well, and that
    // shows nothing about whether it is private.
    let refused_to_a: Vec<String> = if signed_in_works && served_anonymously.is_empty() {
        users
            .private
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(i, path)| {
                let response = http.send(&get(&format!("private-a-also-{i}"), path, &a.session));
                (!ok(&response)).then(|| format!("{path} ({})", status(&response)))
            })
            .collect()
    } else {
        Vec::new()
    };
    if !refused_to_a.is_empty() {
        out.not_assessed.push((
            PRIVATE_PAGE.requirement_ids.join(", "),
            format!(
                "{} refused the signed-in user as well as somebody not signed in, so whether {} \
                 private is not shown. Check the {} in stackvet.toml: a page every signed-in user \
                 can open is what this asks about.",
                refused_to_a.join(", "),
                if refused_to_a.len() == 1 {
                    "it is"
                } else {
                    "they are"
                },
                if refused_to_a.len() == 1 {
                    "address"
                } else {
                    "addresses"
                }
            ),
        ));
    }
    let judged_pages = users.private.len() - refused_to_a.len();
    if signed_in_works && served_anonymously.is_empty() {
        out.verified.push(crate::Verified::new(
            PRIVATE_PAGE.rule_id,
            PRIVATE_PAGE.requirement_ids,
            format!(
                "{} private page{}, refused to somebody not signed in and opened by a signed-in user{}",
                judged_pages,
                if judged_pages == 1 { "" } else { "s" },
                if shells.is_empty() {
                    String::new()
                } else {
                    format!(
                        "; {} answered as the front page does and {} not judged",
                        shells.join(", "),
                        if shells.len() == 1 { "was" } else { "were" }
                    )
                }
            ),
        )
        // Each private page listed is asked; one judged is one sample (ADR-053, Later), and a page left out is
        // a part not shown.
        .in_part_if(judged_pages == 1 || !refused_to_a.is_empty()));
    }

    // 2a. The same private pages, as nobody but naming A in a header a proxy would add. Here,
    //     because only now is it known that signing in opens them and a stranger does not.
    if signed_in_works {
        let refused: Vec<String> = users
            .private
            .iter()
            .filter(|p| !served_anonymously.contains(p))
            .cloned()
            .collect();
        quiet!(identity_header_check(
            http,
            &accounts.a.user,
            &refused,
            &a.session,
            &mut out
        ));
    }

    // 2b. The session timeouts, which mean waiting. Here, while A's password is still the one it
    //     was made with — later checks change it when there is no sign-up — and with sessions of
    //     its own, so nothing below is using them.
    let waited = asked!(
        out,
        http,
        "session_timeout_checks",
        &["V7.3.1", "V7.3.2"],
        session_timeout_checks(
            http,
            users,
            &accounts.a,
            confirm_path.as_deref().filter(|_| signed_in_works),
            policy,
            slow,
            &mut out
        )
    );
    // 2c. A's session sat unused through that wait, and an app with an idle timeout has ended it:
    //     every check below would be asking with a session the app no longer knows, and reading
    //     its refusals as answers. So A signs in afresh, and is shown working again as in step 2.
    let a = if waited {
        match sign_in_again(
            http,
            users,
            &accounts.a,
            confirm_path.as_deref().filter(|_| signed_in_works),
            &mut out,
        ) {
            Some(fresh) => fresh,
            None => return out,
        }
    } else {
        a
    };

    // 3. A record A owns, read as B and as nobody; then the same request from another site. First
    //    among the signed-in checks because A reading back what A made is the second way to show
    //    the session works — the only way, when the private page turned out to be open to all.
    let owned_read = asked!(
        out,
        http,
        "owned_checks",
        &["V3.5.1", "V3.5.2", "V8.2.2", "V8.2.3", "V15.3.1"],
        owned_checks(http, users, accounts, &a, &mut out)
    );

    // 3½. Another site's Origin on the private pages, as A (ADR-055): the place a page that lets any
    //     site read it does harm, which the anonymous question to the health path never reaches.
    quiet!(cross_site::private_pages(http, users, &a, &mut out));

    // 4. The session cookie, and whether signing in made a new one, once it is shown which cookies
    //    carry the session.
    let carried = sign_in_cookies_carry_session(
        http,
        &a,
        confirm_path.as_deref().filter(|_| signed_in_works),
    );
    asked!(
        out,
        http,
        "session_checks",
        &["V3.3.2", "V3.3.4", "V7.2.4"],
        session_checks(
            &a,
            signed_in_works || owned_read.is_some(),
            carried,
            ["login-page-a", "login-a"],
            &mut out
        )
    );

    // 4b. The markers the log check reads afterwards. Done here because the successful one has to
    //     be a sign-in that really worked, and the refused one a page that is really private.
    quiet!(plant_log_markers(
        http,
        users,
        accounts,
        signed_in_works,
        &mut out
    ));

    // 5. The private pages themselves, read with A's session: what they let a browser keep, and
    //    whether they show a way out. Before anything that signs another account in, so the
    //    session that opened them is the one step 2 showed working.
    asked!(
        out,
        http,
        "private_page_checks",
        &[
            "V3.4.3", "V3.4.4", "V3.4.5", "V3.4.6", "V7.4.4", "V14.2.2", "V14.3.2", "V3.3.2",
            "V3.3.4"
        ],
        private_page_checks(http, users, &a, carried, &mut out)
    );

    // 5b. The same pages drawn in a real browser, when stackvet.toml asks for one, with A's
    //     cookies: whether the way out can be seen, and whether text typed into a form comes back
    //     as text. Here for the same reason as step 5, and it signs nobody else in.
    asked!(
        out,
        http,
        "checks",
        &["V7.4.4"],
        crate::browser::checks(
            http,
            users,
            &a.session,
            &accounts.a,
            signed_in_works,
            &crate::browser::token(&accounts.spare),
            &mut out
        )
    );

    // 6. Admin pages, as an ordinary user, confirmed against the admin.
    // The admin's own page at each address, which the role checks below read their answers beside.
    let admins_pages = asked!(
        out,
        http,
        "admin_checks",
        &["V8.2.1", "V8.3.1"],
        admin_checks(http, users, accounts, &a, &mut out)
    );

    // 6a. Three more, each reading something the run already has or sending one more request:
    //     a session value this check invented, the rules the sign-up form states, and whether the
    //     record read back above carried fields that should not leave the server.
    asked!(
        out,
        http,
        "invented_session_check",
        &["V7.2.1"],
        invented_session_check(
            http,
            &a,
            confirm_path.clone().filter(|_| signed_in_works).as_deref(),
            &users.private,
            &served_anonymously,
            carried,
            &mut out
        )
    );
    quiet!(client_side_validation_check(
        http, users, accounts, &mut out
    ));
    // 6a'. The app's own sign-in token, when it is a JWT, changed two ways and sent alone. With
    //      A's token, which nothing here changes.
    asked!(
        out,
        http,
        "app_token_checks",
        &["V9.1.1", "V9.1.2", "V9.1.3"],
        app_token_checks(
            http,
            &a,
            confirm_path.clone().filter(|_| signed_in_works).as_deref(),
            &mut out
        )
    );

    // 6b. Uploads, with A's session, before anything below signs another account in. Placed here
    //     rather than at the end because it needs a working session and nothing it does disturbs
    //     one: it posts files and fetches them back.
    asked!(
        out,
        http,
        "upload_checks",
        &[
            "V1.3.4", "V3.2.1", "V5.2.1", "V5.2.2", "V5.3.1", "V5.3.2", "V5.4.1", "V5.4.2",
            "V5.4.3"
        ],
        each_upload::on_each_upload(
            &users.upload,
            |_| true,
            &mut out,
            |route, out| upload_checks(http, users, route, &a, out)
        )
    );

    // 6b'. A member's own page, saved through each `creates` entry marked `page = true` and fetched where it is read
    //      back (backlog 0535, ADR-126): with A's session, which it only adds a record with.
    if users.creates.iter().any(|c| c.page) {
        asked!(
            out,
            http,
            "saved_page_checks",
            &["V3.2.1"],
            saved_page_checks(http, users, &a, &mut out)
        );
    }

    // 6c. A flow of several steps, gone through in order with A's session and then skipped as B,
    //     signed in afresh. Neither disturbs A's session.
    asked!(
        out,
        http,
        "flow_checks",
        &["V2.3.1"],
        flow_checks(http, users, accounts, &a, &mut out)
    );

    // 6d. Database conditions added to values the app reads from an address, with A's session:
    //     requests that only read, which change nothing.
    quiet!(sql_injection_check(
        http,
        users,
        &a,
        owned_read.as_deref(),
        &mut out
    ));

    // 7. What sign-up and sign-in let through: passwords and default accounts. These sign in as
    //    other accounts, so A's session is untouched for the sign-out below.
    let confirm = confirm_path.clone().filter(|_| signed_in_works);
    asked!(
        out,
        http,
        "password_checks",
        &[
            "V6.2.1", "V6.2.4", "V6.2.5", "V6.2.8", "V6.2.9", "V6.2.11", "V6.2.12"
        ],
        password_checks(http, users, accounts, confirm.as_deref(), policy, &mut out)
    );
    quiet!(signup_reveals_account_check(
        http, users, accounts, &mut out
    ));
    quiet!(signup_replaces_account_check(
        http,
        users,
        accounts,
        confirm.as_deref(),
        &mut out
    ));
    quiet!(default_account_check(
        http,
        users,
        confirm.as_deref(),
        &mut out
    ));
    asked!(
        out,
        http,
        "password_field_checks",
        &["V6.2.6", "V6.2.7", "V6.4.2"],
        password_field_checks(http, users, Some(&a.session), &mut out)
    );

    // 8. Logging out, which ends A's session.
    asked!(
        out,
        http,
        "logout_check",
        &["V7.4.1", "V14.3.1"],
        logout_check(
            http,
            users,
            &a,
            confirm_path
                .clone()
                .filter(|_| signed_in_works)
                .or(owned_read),
            &mut out
        )
    );

    // 9. Last, because each signs A in again, and an app that allows one session per user would
    //    end the one the checks above were using.
    quiet!(password_in_url_check(
        http,
        users,
        &accounts.a,
        confirm.as_deref(),
        &mut out
    ));
    asked!(
        out,
        http,
        "session_id_check",
        &["V7.2.2"],
        session_id_check(
            http,
            users,
            accounts,
            &a,
            confirm.as_deref(),
            signed_in_works,
            &mut out
        )
    );
    quiet!(sign_out_on_get_check(
        http,
        users,
        &accounts.a,
        confirm.as_deref(),
        &mut out
    ));
    // And signing out in a real browser, with a sign-in of its own: clicking the app's sign-out
    // ends that session, so it goes here, after the checks that needed A's.
    if users.browser.is_some() {
        let fresh = confirm
            .as_ref()
            .and_then(|_| sign_in(http, users, "a-browser", &accounts.a, &mut out.steps))
            .map(|s| s.session);
        asked!(
            out,
            http,
            "sign_out_check",
            &["V14.3.1"],
            crate::browser::sign_out_check(http, users, fresh.as_ref(), &mut out)
        );
    }
    // And what the app keeps in the browser after a sign-in through its own form: another sign-in,
    // so it goes here too.
    asked!(
        out,
        http,
        "storage_check",
        &["V10.1.1", "V14.3.3"],
        crate::browser_storage::storage_check(
            http,
            users,
            &accounts.a,
            confirm.is_some(),
            &mut out
        )
    );

    // 9b to 9h: the checks that sign in on their own or change what the app holds, late in the run for the reasons
    // each gives, in `late_checks`.
    out = late_checks(
        http,
        Late {
            users,
            accounts,
            policy,
            slow,
            confirm: &confirm,
            confirm_path,
            signed_in_works,
            admins_pages,
            oauth_client,
        },
        out,
    );

    // 10. Last of all, because it changes a password: with an account made for it when there is a
    //    sign-up, and with A's own when there is not.
    asked!(
        out,
        http,
        "change_password_checks",
        &["V6.2.2", "V6.2.3", "V6.3.7", "V7.4.3"],
        change_password_checks(http, users, accounts, confirm.as_deref(), &mut out)
    );
    asked!(
        out,
        http,
        "change_email_check",
        &["V7.5.1"],
        change_email_check(http, users, accounts, confirm.as_deref(), &mut out)
    );
    asked!(
        out,
        http,
        "account_change_checks",
        &["V7.5.1"],
        account_changes::account_change_checks(http, users, accounts, confirm.as_deref(), &mut out)
    );
    asked!(
        out,
        http,
        "delete_account_check",
        &["V7.4.2"],
        delete_account_check(http, users, accounts, confirm.as_deref(), &mut out)
    );
    asked!(
        out,
        http,
        "reset_checks",
        &["V6.3.8", "V6.4.3"],
        reset_checks(http, users, accounts, confirm.as_deref(), &mut out)
    );
    asked!(
        out,
        http,
        "activation_checks",
        &["V6.4.1"],
        activation_checks(http, users, accounts, confirm.as_deref(), &mut out)
    );
    asked!(
        out,
        http,
        "email_code_checks",
        &["V6.5.1", "V6.5.4", "V6.6.2", "V6.6.3"],
        email_code_checks(http, users, accounts, confirm.as_deref(), &mut out)
    );
    asked!(
        out,
        http,
        "email_code_lifetime",
        &["V6.5.5"],
        email_code_lifetime(http, users, accounts, confirm.as_deref(), slow, &mut out)
    );
    asked!(
        out,
        http,
        "totp_checks",
        &["V6.5.1", "V6.5.5"],
        totp_checks(http, users, accounts, confirm.as_deref(), &mut out)
    );

    // 10. After everything else, without exception. This one deliberately provokes the app into
    //     refusing requests, and a limiter that counts by address rather than by account would then
    //     be refusing every check above too. Running it last means the worst it can cost is itself.
    asked!(
        out,
        http,
        "brute_force_check",
        &["V6.3.1", "V15.3.4"],
        brute_force_check(http, users, accounts, policy, &mut out)
    );
    // And codes after passwords: both set out to be refused, and this one is the newer.
    asked!(
        out,
        http,
        "email_code_guessing",
        &["V6.6.3"],
        email_code_guessing(http, users, accounts, confirm.as_deref(), policy, &mut out)
    );
    // The only sign-ins after the guesses: three wrong passwords would use up part of a limit that
    // counts by address before the guessing check, and here a limit still refusing leaves them
    // uncompared rather than misread.
    quiet!(signin_reveals_account_check(
        http, users, accounts, &mut out
    ));

    out
}

/// What checks 9b to 9h of `run_checks` read of what the run has so far.
struct Late<'a> {
    users: &'a UsersSection,
    accounts: &'a Accounts,
    policy: &'a sv_manifest::PolicySection,
    slow: bool,
    confirm: &'a Option<String>,
    confirm_path: Option<String>,
    signed_in_works: bool,
    admins_pages: Vec<Option<ProbeResponse>>,
    oauth_client: Option<&'a sv_manifest::OauthClientSection>,
}

/// Checks 9b to 9h of `run_checks`, in its order, each through `asked!` or `quiet!` as there (`check_guard_tests` reads
/// this body with `run_checks`'s): the WebSocket, the redirects, the token's expiry, the admin actions, the role
/// fields, the once action, the bursts, the archives, and ending sessions. Moved out of `run_checks` on 21 October
/// 2026 (backlog 0324), word for word; `out` comes in and goes back, so each call still names `&mut out`.
fn late_checks(http: &mut Counted<'_>, late: Late<'_>, mut out: Outcome) -> Outcome {
    let Late {
        users,
        accounts,
        policy,
        slow,
        confirm,
        confirm_path,
        signed_in_works,
        admins_pages,
        oauth_client,
    } = late;

    // 9b. A private WebSocket, with a sign-in of its own that it signs out at the end: after
    //     everything that needed A's first session.
    asked!(
        out,
        http,
        "websocket_session_checks",
        &["V4.4.2", "V4.4.3", "V4.4.4"],
        websocket_session_checks(http, users, &accounts.a, &mut out)
    );
    // 9b'. Where the sign-in and sign-out send the browser when given an address outside the app:
    //     sessions of their own, and nothing changed.
    quiet!(open_redirect_check(http, users, &accounts.a, &mut out));
    quiet!(page_redirect_check(http, users, &accounts.a, &mut out));
    // 9b'''. The app's own authorization server, asked through the test client stackvet.toml declares (ADR-151): a
    //       sign-in of its own, nothing the other checks rely on changed, and before the password changes below.
    quiet!(oauth_client::authorization_server_checks(
        http,
        users,
        accounts,
        oauth_client,
        &mut out
    ));
    // 9b''. The app's own sign-in token, waited out until it expires: a sign-in of its own, and
    //      before the password changes below, which can change A's.
    quiet!(app_token_expiry_check(
        http,
        users,
        &accounts.a,
        confirm.as_deref(),
        slow,
        &mut out
    ));

    // 9c. Admin actions, sent by A and then by the admin. Late, because an action changes what the
    //     app holds and the checks above have had what they needed; before the password changes
    //     below, one of which can change A's own password, and before the checks that set out to be
    //     refused and can leave the app refusing everybody, the admin included.
    asked!(
        out,
        http,
        "admin_action_checks",
        &["V8.2.1", "V8.3.1"],
        admin_action_checks(http, users, accounts, confirm.as_deref(), &mut out)
    );
    // 9d. A role written into the sign-up form, with two accounts made for it. Before the password
    //     changes for the same reason as the admin actions.
    quiet!(role_field_check(
        http,
        users,
        accounts,
        (confirm.as_deref(), &admins_pages),
        &mut out
    ));
    //     The same fields sent with an email change, by an account made for it (finding 13(a)).
    quiet!(email_role_check(
        http,
        users,
        accounts,
        (confirm.as_deref(), &admins_pages),
        &mut out
    ));
    // 9e. The action that should go through once, sent many times at the same instant by A.
    asked!(
        out,
        http,
        "once_check",
        &["V2.3.4"],
        once_check(http, users, accounts, &mut out)
    );
    // 9f. A burst of creations by B, held to the stated limit. Before the password changes, which can
    //     change B's password too (a reset); it sets out to be refused, so it waits the minute out
    //     afterwards before anything else is asked.
    asked!(
        out,
        http,
        "burst_check",
        &["V2.4.1"],
        burst_check(http, users, &accounts.b, policy, &mut out)
    );
    // 9f'. A business limit the documentation states, held on the server (V2.3.2): after the burst, which waits its
    //     minute out, and as A, whose records the burst did not make.
    asked!(
        out,
        http,
        "business_limit_check",
        &["V2.3.2"],
        business_limit_check(http, users, &accounts.a, policy, &mut out)
    );
    // 9g. Compressed files past the stated limits, with a sign-in of A's own: after every other
    //     upload, since an app that unpacks one may fall over; before the password changes below,
    //     which can change A's.
    asked!(
        out,
        http,
        "archive_checks",
        &["V5.2.3"],
        // A route that says it unpacks none is not one V5.2.3 is asked of.
        each_upload::on_each_upload(
            &users.upload,
            |route| route
                .unpacks_archives
                .as_ref()
                .is_none_or(|f| !f.is_empty()),
            &mut out,
            |route, out| {
                archives::archive_checks(http, users, route, accounts, confirm.as_deref(), out)
            }
        )
    );

    // 9h. Ending sessions (backlog 0492, ADR-125): each ends the first user's sessions, so after every check that uses
    //     A's session; and before the password change below, which may change A's own password.
    if signed_in_works && let Some(confirm) = &confirm_path {
        quiet!(end_other_sessions_check(
            http, users, accounts, confirm, &mut out
        ));
        quiet!(admin_end_sessions_check(
            http, users, accounts, confirm, &mut out
        ));
    }
    out
}

/// What a sign-up answered, in a sentence for a reason given later (backlog 0599): its status, the fields sent (names
/// only), and the words its page showed, unless the page repeats the password.
fn signed_up(
    signup: &RequestTemplate,
    response: Option<&ProbeResponse>,
    account: &Account,
) -> String {
    match response {
        None => "the sign-up of the first test user got no answer".to_owned(),
        Some(r) => format!(
            "the sign-up of the first test user answered {}; {}",
            r.status,
            refused::sent_and_said(signup, r, refused::repeats_password(r, account))
        ),
    }
}

/// Signs an account up through the app's own form.
/// Makes an account through `signup`, and — when the app activates accounts with an emailed code
/// (`activation`) — activates it, so every account a check signs up can sign in as that check
/// expects. Quietly: a check that needs to watch activation happen calls `sign_up_only`.
pub(crate) fn sign_up(
    http: &mut dyn Http,
    users: &UsersSection,
    signup: &RequestTemplate,
    who: &str,
    account: &Account,
) -> Option<ProbeResponse> {
    let Some(activation) = &users.activation else {
        return sign_up_only(http, signup, who, account);
    };
    let before = http.mail(&account.user, 0).map_or(0, |m| m.len());
    let answer = sign_up_only(http, signup, who, account);
    let code = code_patterns(
        activation.code_pattern.as_deref(),
        "activate|activation|verify|confirm|welcome",
    )
    .ok()
    .and_then(|patterns| {
        let mail = http.mail(&account.user, before + 1)?;
        mail.get(before..)?
            .last()
            .and_then(|m| reset_code(m, &patterns))
    });
    if let Some(code) = code {
        let values = Values {
            user: &account.user,
            code: &code,
            ..Default::default()
        };
        let mut session = Session::default();
        send_template(
            http,
            &format!("activate-{who}"),
            &activation.use_code,
            &values,
            &mut session,
            &[],
        );
    }
    answer
}

#[cfg(test)]
mod account_changes_left_out_tests;
#[cfg(test)]
mod account_changes_tests;
#[cfg(test)]
mod admin_actions_two_tests;
#[cfg(test)]
mod asked_tests;
#[cfg(test)]
mod attribute_tests;
#[cfg(test)]
mod blank_page_tests;
#[cfg(test)]
mod check_guard_tests;
#[cfg(test)]
mod crash_on_signup_tests;
#[cfg(test)]
mod email_role_tests;
#[cfg(test)]
mod failing_requirements_tests;
#[cfg(test)]
mod fake_app;
#[cfg(test)]
mod form_token_tests;
#[cfg(test)]
mod in_part_tests;
#[cfg(test)]
mod in_turn_tests;
#[cfg(test)]
mod limit_tests;
#[cfg(test)]
mod log_bypass_tests;
#[cfg(test)]
mod log_codes_tests;
#[cfg(test)]
mod log_sign_ins_tests;
#[cfg(test)]
mod no_signup_right_password_tests;
#[cfg(test)]
mod non_ascii_tests;
#[cfg(test)]
mod oauth_client_fake;
#[cfg(test)]
mod oauth_client_pkce_tests;
#[cfg(test)]
mod oauth_client_tests;
#[cfg(test)]
mod owner_field_tests;
#[cfg(test)]
mod page_cookie_tests;
#[cfg(test)]
mod private_pages_opened_tests;
#[cfg(test)]
mod resignup_tests;
#[cfg(test)]
mod saved_page_tests;
#[cfg(test)]
mod seed_values_tests;
#[cfg(test)]
mod sessions_end_tests;
#[cfg(test)]
mod short_names_tests;
#[cfg(test)]
mod sign_in_limit_advice_tests;
#[cfg(test)]
mod storage_check_tests;
#[cfg(test)]
mod stored_markup_tests;

#[cfg(test)]
mod business_limit_tests;
#[cfg(test)]
mod input_limits_tests;
#[cfg(test)]
mod timings_tests;
#[cfg(test)]
mod unset_keys_tests;

#[cfg(test)]
mod tests;

/// A rate limiter's answer is not the app's answer to the question (V8.2.1 and wherever a refusal
/// is read). Found on 28 September 2026: the private-page check read anything but 2xx as "refused",
/// so a limiter's 429 was credited as the app refusing a stranger, and the sign-out check read one
/// as a GET having ended a session.
#[cfg(test)]
mod rate_limit_tests;

#[cfg(test)]
mod crash_tests;

/// A single-page app's page shell is not its private page (gap analysis 2.2). Such an app sends
/// everybody the same page for every address and draws it in the browser from what it fetches
/// next, so `/account` answering 200 to a stranger was reported as a private page served to anyone,
/// and every check that needs the signed-in control then read "not assessed".
#[cfg(test)]
mod page_shell_tests;

#[cfg(test)]
mod setup_sweep_tests;
#[cfg(test)]
mod signin_form_on_200_tests;
