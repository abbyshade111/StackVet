//! What a sign-in's answer said, for when it did not sign in (backlog 0527). In the three-tool trial of 14 October
//! 2026 the run said only "signed in as A (400)", and sixteen signed-in checks stopped with no word of why; the page
//! said its anti-forgery token was refused, and nothing showed it. Now the step and the reason name the fields that
//! were sent (their names, never their values) and quote the words the page showed.

use super::RequestTemplate;
use crate::probes::ProbeResponse;

/// How many characters of the page's words are quoted.
const SAID_CHARS: usize = 300;

/// The names of the fields the sign-in sent, in backticks: the form's, or the JSON body's.
pub(crate) fn fields_sent(login: &RequestTemplate) -> String {
    let names: Vec<String> = if login.form.is_empty() {
        login.json.keys().map(|k| format!("`{k}`")).collect()
    } else {
        login.form.keys().map(|k| format!("`{k}`")).collect()
    };
    if names.is_empty() {
        "no fields".to_owned()
    } else {
        format!(
            "the field{} {}",
            if names.len() == 1 { "" } else { "s" },
            names.join(", ")
        )
    }
}

/// Whether the page repeats the test account's password: then its words are not quoted at all. Only this yes or no
/// is taken from the password, so nothing written can carry it.
pub(crate) fn repeats_password(response: &ProbeResponse, password: &str) -> bool {
    password.chars().count() >= 4
        && (response.body.contains(password)
            || crate::text::visible_words(&response.body).contains(password))
}

/// The words the page showed, cut to [`SAID_CHARS`], with no character that would move or hide text on a terminal.
/// `None` when it showed none.
pub(crate) fn page_said(response: &ProbeResponse) -> Option<String> {
    let words: String = crate::text::visible_words(&response.body)
        .chars()
        .filter(|c| !c.is_control() && !matches!(c, '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'))
        .collect();
    let words = words.trim();
    (!words.is_empty()).then(|| crate::text::cut_with_count(words, SAID_CHARS))
}

/// What the sign-in sent and what its page said: "it sent the fields `a`, `b`, and the page said: “…”".
/// `repeats_password` is [`repeats_password`]'s answer: such a page is not quoted.
pub(crate) fn sent_and_said(
    login: &RequestTemplate,
    response: &ProbeResponse,
    repeats_password: bool,
) -> String {
    let said = if repeats_password {
        ", and the page repeated the test account's password, so its words are not quoted"
            .to_owned()
    } else {
        match page_said(response) {
            Some(words) => format!(", and the page said: \u{201c}{words}\u{201d}"),
            None => ", and the page showed no words".to_owned(),
        }
    };
    format!("it sent {}{said}", fields_sent(login))
}

/// The sign-in's answer in a sentence, for a reason given later: its status, then [`sent_and_said`].
pub(crate) fn answer(
    login: &RequestTemplate,
    response: &ProbeResponse,
    repeats_password: bool,
) -> String {
    format!(
        "the sign-in answered {}; {}",
        response.status,
        sent_and_said(login, response, repeats_password)
    )
}

#[cfg(test)]
#[path = "refused_tests.rs"]
mod tests;
