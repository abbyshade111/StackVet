//! What came back to a question, before any check reads it: the app's own answer, or a crash, a limiter, or nothing,
//! and which requests a crash leaves standing. Moved out of `signed_in/mod.rs` unchanged (backlog 0324).

use super::*;

/// What came back to a question, before any check reads it (ADR-021): the app's own answer, or one
/// of the three things that are not one. Every check that reads whether the app answered reads it
/// through here (`answer_of`), so the rule is written once. Until 8 October 2026 it was written in
/// seven places with three definitions, one of them missing the limiter's 503 (the architecture
/// assessment of that day, item 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer<'a> {
    /// The app answered the question: a status below 500 that is not a limiter's.
    Answered(&'a ProbeResponse),
    /// Nothing came back, or nothing that was HTTP.
    Silent,
    /// A server error that is not the limiter's: the app failed, which says nothing about whether
    /// it would have let the request through.
    Crashed(u16),
    /// A rate limiter's: 429, or 503 with `Retry-After`. The request was stopped before it reached
    /// the check it was asking about. `retry_after` is what the app asks to wait, in seconds.
    Limited { status: u16, retry_after: u64 },
}

impl<'a> Answer<'a> {
    /// The app's own answer, when there is one.
    pub fn answered(self) -> Option<&'a ProbeResponse> {
        match self {
            Answer::Answered(r) => Some(r),
            _ => None,
        }
    }

    /// Whether a rate limiter answered in the app's place.
    pub fn is_limited(self) -> bool {
        matches!(self, Answer::Limited { .. })
    }

    /// Whether the app crashed or did not answer: not a refusal, and not the limiter's either.
    pub fn is_crash_or_silence(self) -> bool {
        matches!(self, Answer::Silent | Answer::Crashed(_))
    }
}

/// How a crash or a silence is written down for `RESTS_ON_A_REFUSAL`: the status, or "no answer".
/// The sign-ups `sv` makes with ordinary values, the kind a person makes: its two test users, the control account, and
/// the long password (backlog 0528). Not the ones that send something odd on purpose.
pub(super) const ORDINARY_SIGNUPS: &[&str] =
    &["signup-a", "signup-b", "signup-control", "signup-long"];

pub(super) fn is_ordinary_signup(id: &str) -> bool {
    ORDINARY_SIGNUPS.contains(&id)
}

/// A request's fields by name and length, never a value: "`email` (31 characters), `password` (73 characters)".
pub(super) fn fields_by_length(request: &ProbeRequest) -> String {
    let body = request
        .body
        .as_deref()
        .map(String::from_utf8_lossy)
        .unwrap_or_default();
    let fields: Vec<(String, usize)> = match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(serde_json::Value::Object(map)) => map
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().map_or(0, |s| s.chars().count())))
            .collect(),
        _ => body
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .map(|(k, v)| (decode(k), decode(v).chars().count()))
            .collect(),
    };
    fields
        .into_iter()
        .filter(|(k, _)| !k.is_empty())
        .map(|(k, n)| format!("`{k}` ({n} character{})", if n == 1 { "" } else { "s" }))
        .collect::<Vec<_>>()
        .join(", ")
}

/// A form value with `+` and `%XX` read back.
pub(super) fn decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
                match hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                    Some(b) => {
                        out.push(b);
                        i += 2;
                    }
                    None => out.push(b'%'),
                }
            }
            b => out.push(b),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// One finding for the ordinary sign-ups the app crashed on (backlog 0528, ADR-021, Later): the request named by its
/// fields and their lengths, never a value. What the crash would have answered stays withheld.
pub(super) fn crashed_on_ordinary(crashed: &[(String, u16, String)], out: &mut Outcome) {
    if crashed.is_empty() {
        return;
    }
    let said: Vec<String> = crashed
        .iter()
        .map(|(id, status, fields)| format!("`{id}` answered {status}, sent with {fields}"))
        .collect();
    out.findings.push(finding_on(
        crashed.iter().map(|(id, _, _)| id.clone()).collect(),
        &CRASHED_ON_ORDINARY,
        "The app crashed on a request a person could make",
        Severity::Low,
        format!(
            "A sign-up with ordinary values got a server error rather than an answer: {}. Nothing the crash would \
             have answered is credited or reported; the crash itself is the finding. The app's log for that request \
             says which exception it did not handle.",
            said.join("; ")
        ),
    ));
}

pub(super) fn crash_of(response: Option<&ProbeResponse>) -> Option<String> {
    match answer_of(response) {
        Answer::Silent => Some("no answer".to_owned()),
        Answer::Crashed(status) => Some(status.to_string()),
        Answer::Answered(_) | Answer::Limited { .. } => None,
    }
}

/// What `response` is, by the one rule (ADR-021).
pub fn answer_of(response: Option<&ProbeResponse>) -> Answer<'_> {
    let Some(r) = response else {
        return Answer::Silent;
    };
    if let Some(retry_after) = rate_limited(r) {
        return Answer::Limited {
            status: r.status,
            retry_after,
        };
    }
    if r.status == 0 {
        return Answer::Silent;
    }
    if r.status >= 500 {
        return Answer::Crashed(r.status);
    }
    Answer::Answered(r)
}

/// Seconds to wait before asking again, when `response` is a rate limiter's rather than an answer to
/// the question: 429, or 503 with `Retry-After`. The app's own `Retry-After` in seconds, at most a
/// minute; five seconds when it gives none, or gives a date.
pub(crate) fn rate_limited(response: &ProbeResponse) -> Option<u64> {
    let retry_after = response.header("retry-after");
    match response.status {
        429 => {}
        503 if retry_after.is_some() => {}
        _ => return None,
    }
    Some(
        retry_after
            .and_then(|v| v.trim().parse::<u64>().ok())
            .unwrap_or(5)
            .min(60),
    )
}
