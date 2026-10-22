//! Asking without signing in, and waiting out an app's rate limiter: a limiter's answer is waited for once and
//! never taken for the app's. Moved out of `signed_in/mod.rs` unchanged (backlog 0324).

use super::*;

/// The questions asked as somebody not signed in, each through the same wait as the signed-in
/// ones, and what came back.
///
/// An answer that is still the rate limiter's after the wait is left out, as a request that got no
/// answer is: the checks that read these (`probes::evaluate`, `probes::verified`,
/// `probes::evaluate_api`, `running::evaluate`) judge whatever they are given as the app's, so a
/// limiter's page on `/` would be a security-headers finding the app does not deserve, and a
/// limiter's 429 on `/.git/HEAD` would be credited as the folder not exposed. The requests left
/// out are returned, as "id (status)", for the run to say which they were.
pub fn ask_anonymously(
    http: &mut dyn Http,
    requests: &[ProbeRequest],
) -> (Vec<ProbeResponse>, Vec<String>) {
    ask_anonymously_within(http, requests, &std::cell::Cell::new(0))
}

/// `ask_anonymously`, with the waiting for a limiter counted against `spent`, the seconds already
/// waited in this run: one budget for the whole run (`MOST_WAITING`), not one per suite.
pub fn ask_anonymously_within(
    http: &mut dyn Http,
    requests: &[ProbeRequest],
    spent: &std::cell::Cell<u64>,
) -> (Vec<ProbeResponse>, Vec<String>) {
    let mut patient = Patient::within(http, spent);
    // In one go when the way to the app allows, up to the first answer that is a rate limiter's.
    // From there on, one at a time as before: that answer is waited out and the request asked
    // again, and so is each after it, so the waiting is what it was. Each of those was also asked
    // in the one go, which a limiter counts; it was already saying no.
    let in_turn = patient.inner.send_in_turn(requests).unwrap_or_default();
    let mut answers = Vec::new();
    let mut from = 0;
    for answer in in_turn.into_iter().take(requests.len()) {
        if answer.as_ref().is_some_and(|a| rate_limited(a).is_some()) {
            break;
        }
        answers.extend(answer);
        from += 1;
    }
    answers.extend(
        requests[from..]
            .iter()
            .filter_map(|request| patient.send(request))
            .filter(|answer| rate_limited(answer).is_none()),
    );
    (answers, patient.still_limited)
}

/// The app, with a rate limiter's answer waited out once.
///
/// A 429 from a limiter says nothing about the question asked: a private page refused with 429 was
/// never shown to the app's own sign-in check, and a check that reads "not 2xx" as "refused" would
/// credit a refusal nobody made (V8.2.1, and wherever else a refusal is read), or report one (a
/// sign-out that seemed to end a session). So a limited answer is waited out, as long as the app
/// asks and at most a minute, and the request sent once more. Not for a request whose id says it is
/// a guess, or part of a burst (`burst-`): those checks send requests on purpose to see the limiter
/// answer, and a wait would both change what they measure and send one more than they count.
pub struct Patient<'a> {
    pub(super) inner: &'a mut dyn Http,
    /// Requests the limiter was still answering after the wait, as "id (status)".
    pub(super) still_limited: Vec<String>,
    /// The path sign-ins are sent to (`[stack.run.users] login`), without its query.
    pub(super) login_path: Option<String>,
    /// Of `still_limited`, the sign-ins: requests other than GET to `login_path`, by id.
    pub(super) limited_sign_ins: Vec<String>,
    /// Seconds waited so far in this run, against `MOST_WAITING`: shared with the other suites of
    /// the run when made with `within`, so the run as a whole waits at most that long.
    pub(super) waited: WaitedSoFar<'a>,
    /// Requests answered with a server error (5xx) or not answered at all: (request id, status or
    /// "no answer").
    pub(super) crashed: Vec<(String, String)>,
    /// Each of `sv`'s ordinary sign-ups the app answered with a server error, with its status and its fields by name
    /// and length (backlog 0528).
    pub(super) crashed_signups: Vec<(String, u16, String)>,
    /// Each wait for the limiter, as (request id, seconds), whether or not the limiter let the request through after
    /// it: until backlog 0526 only the ones it still refused were said, so minutes of waiting went unexplained.
    pub(super) waits: Vec<(String, u64)>,
}

/// All the waiting one run does for a rate limiter. Each wait is at most a minute, but a limiter
/// answering every request would otherwise hold a run up for a minute a request; past this, a
/// limited answer is recorded as the limiter's without waiting.
pub const MOST_WAITING: u64 = 300;

/// The seconds a run has spent waiting for a limiter: this suite's own count, or the run's, shared.
pub(super) enum WaitedSoFar<'a> {
    Own(u64),
    Shared(&'a std::cell::Cell<u64>),
}

impl WaitedSoFar<'_> {
    pub(super) fn get(&self) -> u64 {
        match self {
            WaitedSoFar::Own(n) => *n,
            WaitedSoFar::Shared(cell) => cell.get(),
        }
    }

    pub(super) fn add(&mut self, seconds: u64) {
        match self {
            WaitedSoFar::Own(n) => *n += seconds,
            WaitedSoFar::Shared(cell) => cell.set(cell.get() + seconds),
        }
    }
}

impl<'a> Patient<'a> {
    pub fn new(inner: &'a mut dyn Http) -> Self {
        Patient {
            inner,
            still_limited: Vec::new(),
            login_path: None,
            limited_sign_ins: Vec::new(),
            waited: WaitedSoFar::Own(0),
            crashed: Vec::new(),
            crashed_signups: Vec::new(),
            waits: Vec::new(),
        }
    }

    /// As `new`, with the waiting counted against `spent`, shared by every suite of the run.
    pub fn within(inner: &'a mut dyn Http, spent: &'a std::cell::Cell<u64>) -> Self {
        let mut patient = Patient::new(inner);
        patient.waited = WaitedSoFar::Shared(spent);
        patient
    }

    /// What the limiter and the crashes left, said in `out` as every suite says it: the credits a
    /// limiter that kept answering puts in doubt become not assessed, the findings are kept with a
    /// note, and what rests on a request that crashed is withheld (ADR-021). Called once a suite is
    /// done asking.
    pub fn settle(&mut self, out: &mut Outcome) {
        withhold_what_rests_on_a_crash(&self.crashed, out);
        crashed_on_ordinary(&self.crashed_signups, out);
        settle_waits(&self.waits, out);
        settle_limited(self, out);
    }
}

impl Patient<'_> {
    /// The app's answer, with a rate limiter's waited out once.
    pub(super) fn answer(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let first = self.inner.send(request);
        if request.id.contains("guess") || request.id.starts_with("burst-") {
            return first;
        }
        let Some(wait) = first.as_ref().and_then(rate_limited) else {
            return first;
        };
        if self.waited.get() + wait > MOST_WAITING {
            if let Some(r) = &first {
                self.still_limited.push(format!(
                    "{} ({}, not waited for: {MOST_WAITING} seconds already spent waiting)",
                    request.id, r.status
                ));
                self.note_sign_in(request);
            }
            return first;
        }
        self.waited.add(wait);
        self.waits.push((request.id.clone(), wait));
        self.inner.wait(wait);
        let second = self.inner.send(request);
        if let Some(r) = second.as_ref().filter(|r| rate_limited(r).is_some()) {
            self.still_limited
                .push(format!("{} ({})", request.id, r.status));
            self.note_sign_in(request);
        }
        second
    }

    /// Notes `request`, which the limiter is still answering, as a refused sign-in when it is one:
    /// sent to the sign-in path, and not the GET for the sign-in page.
    pub(super) fn note_sign_in(&mut self, request: &ProbeRequest) {
        let path = request.path.split('?').next().unwrap_or_default();
        if request.method != "GET" && self.login_path.as_deref() == Some(path) {
            self.limited_sign_ins.push(request.id.clone());
        }
    }
}

impl Http for Patient<'_> {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let answer = self.answer(request);
        // A crash, or no answer at all, is not the app refusing. Recorded for every request,
        // the guesses included, and read against `RESTS_ON_A_REFUSAL` once the run is over.
        if let Some(status) = crash_of(answer.as_ref()) {
            self.crashed.push((request.id.clone(), status));
        }
        if let Answer::Crashed(status) = answer_of(answer.as_ref())
            && is_ordinary_signup(&request.id)
        {
            self.crashed_signups
                .push((request.id.clone(), status, fields_by_length(request)));
        }
        answer
    }

    fn send_together(&mut self, requests: &[ProbeRequest]) -> Option<Vec<Option<ProbeResponse>>> {
        // Not waited out: a limiter's answer to a copy sent together is part of what was asked.
        let answers = self.inner.send_together(requests)?;
        for (request, answer) in requests.iter().zip(&answers) {
            if let Some(status) = crash_of(answer.as_ref()) {
                self.crashed.push((request.id.clone(), status));
            }
        }
        Some(answers)
    }

    fn mail(&mut self, to: &str, at_least: usize) -> Option<Vec<String>> {
        self.inner.mail(to, at_least)
    }

    fn now(&mut self) -> u64 {
        self.inner.now()
    }

    fn wait(&mut self, seconds: u64) {
        self.inner.wait(seconds);
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
        self.inner.browser(job)
    }

    fn model(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.inner.model(request)
    }

    fn model_address(&mut self) -> Option<String> {
        self.inner.model_address()
    }
}
