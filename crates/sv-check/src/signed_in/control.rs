//! An answer read beside the control request's answer, never against a status threshold alone (backlog 0560,
//! ADR-119).
//!
//! A probe that sends a broken value has, in hand, the answer to an ordinary request of the same kind. A form
//! refused on a page re-rendered with 200 and "Enter a valid email address" is under 400 like an acceptance, so
//! "under 400" read it as accepted (backlog 0523). What settles it is whether the answer looks like the control's:
//! the same kind of status, the same place the browser is sent, and a page of the same shape. When it does not,
//! the probe cannot tell from this answer alone, and says what it compared.

use crate::probes::ProbeResponse;

/// How an answer stands beside the control's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Beside {
    /// Answered the way the control did.
    Like,
    /// Refused: a 4xx where the control was not refused.
    Refused,
    /// Neither: what was compared, in words, for the step and the report.
    Unclear(String),
}

/// `answer` beside `control`, which was not refused: same status class, the same redirect target (its path alone,
/// as what follows `?` may be a token of the app's own), and, for two successes, a page of the same shape.
pub(super) fn beside(control: &ProbeResponse, answer: &Option<ProbeResponse>) -> Beside {
    let Some(answer) = answer else {
        return Beside::Unclear("the app did not answer".to_owned());
    };
    let (was, is) = (control.status / 100, answer.status / 100);
    if is == 4 && was != 4 {
        return Beside::Refused;
    }
    if is == 5 {
        return Beside::Unclear(format!(
            "the app answered {}, an error of its own",
            answer.status
        ));
    }
    if was != is {
        return Beside::Unclear(format!(
            "{} where the control {}",
            answer.status,
            what_it_did(control)
        ));
    }
    if is == 3 {
        let (to, control_to) = (target(answer), target(control));
        if to.map(path_shape) != control_to.map(path_shape) {
            return Beside::Unclear(format!(
                "sent the browser to {} where the control was sent to {}",
                to.unwrap_or("nowhere named"),
                control_to.unwrap_or("nowhere named")
            ));
        }
    }
    if is == 2
        && let Some(differs) = shape_differs(control, answer)
    {
        return Beside::Unclear(differs);
    }
    Beside::Like
}

fn what_it_did(control: &ProbeResponse) -> String {
    match (control.status / 100, target(control)) {
        (3, Some(to)) => format!("redirected to {to}"),
        (3, None) => "redirected".to_owned(),
        _ => format!("answered {}", control.status),
    }
}

/// Where a redirect sends the browser: the path alone, without the origin, the query, or the fragment.
fn target(r: &ProbeResponse) -> Option<&str> {
    let to = r.header("location")?;
    let to = super::admin::strip_origin(to);
    to.split(['?', '#']).next()
}

/// A path with each part that holds a digit read as an id, so the redirect to one new record (`/notes/7`) is
/// the same place as to the next (`/notes/8`).
pub(super) fn path_shape(path: &str) -> String {
    path.split('/')
        .map(|part| {
            if part.chars().any(|c| c.is_ascii_digit()) {
                "{id}"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// How two successful answers differ in shape, when they do in a way this can read: JSON objects with other
/// top-level names, or a form shown where the control showed none.
fn shape_differs(control: &ProbeResponse, answer: &ProbeResponse) -> Option<String> {
    let keys = |r: &ProbeResponse| {
        serde_json::from_str::<serde_json::Value>(&r.body)
            .ok()
            .and_then(|v| v.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()))
    };
    if let (Some(mut was), Some(mut is)) = (keys(control), keys(answer)) {
        was.sort();
        is.sort();
        return (was != is).then(|| {
            format!(
                "an answer with the names {} where the control's had {}",
                names(&is),
                names(&was)
            )
        });
    }
    let form = |r: &ProbeResponse| {
        let body = r.body.to_ascii_lowercase();
        body.contains("<form") || body.contains("<input")
    };
    (form(answer) && !form(control)).then(|| {
        format!(
            "{} with a form on it, where the control's page had none",
            answer.status
        )
    })
}

/// Whether `answer` shows the page `reference` is: at least half of the reference's pieces of text (the words
/// between its tags, cut at line ends, three characters or more) are in the answer. The same page served to
/// somebody else differs in a name or two; a page refused on a 200 ("Access denied") shares little of it. The same
/// body is the same page; otherwise a reference with no such piece shows nothing either way, and is `false`.
pub(super) fn shows_page(reference: &ProbeResponse, answer: &ProbeResponse) -> bool {
    // The same answer, even an empty one, is the same page.
    if reference.body == answer.body {
        return true;
    }
    // The words between tags, not the tags: two pages built from the same `<form>` and `<button>` are not
    // the same page.
    let mut text = String::new();
    let mut in_tag = false;
    for c in reference.body.chars() {
        match c {
            '<' => {
                in_tag = true;
                text.push('\n');
            }
            '>' => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    let pieces: Vec<&str> = text
        .split('\n')
        .map(str::trim)
        .filter(|p| p.chars().count() >= 3)
        .collect();
    if pieces.is_empty() {
        return false;
    }
    let found = pieces.iter().filter(|p| answer.body.contains(**p)).count();
    found * 2 >= pieces.len()
}

fn names(keys: &[String]) -> String {
    if keys.is_empty() {
        return "none".to_owned();
    }
    keys.iter()
        .map(|k| format!("`{k}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod control_tests;
