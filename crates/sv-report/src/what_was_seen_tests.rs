//! "What StackVet saw" (backlog 0378): an exchange read as its request and its answer, with only the words a person
//! would see on the page, and never the markup a hidden form field's value lives in.

use super::*;

fn exchange(
    id: &str,
    method: &str,
    path: &str,
    status: u16,
    headers: &[(&str, &str)],
    body: &str,
) -> seen::Exchange {
    seen::Exchange {
        id: id.into(),
        method: method.into(),
        path: path.into(),
        status,
        headers: headers
            .iter()
            .map(|(n, v)| ((*n).to_owned(), (*v).to_owned()))
            .collect(),
        body: body.into(),
        prompt: None,
    }
}

#[test]
fn a_reply_is_quoted_by_its_visible_words_only() {
    let page = "<!doctype html><html><head><title>Sign in</title><style>p{color:red}</style></head><body>\
                <p><a href=\"/\">Home</a></p><form><input type=\"hidden\" name=\"csrf_token\" value=\"secret-ish-value\">\
                <p>Wrong email &amp; password.</p></form><script>var k='hidden';</script></body></html>";
    let line = exchange_line(&exchange("guess-0", "POST", "/login", 401, &[], page));
    assert_eq!(
        line,
        "POST /login answered 401: \"Home Wrong email & password.\""
    );
    // The setup: the value and the script were in the page, so leaving them out is the code's doing.
    assert!(page.contains("secret-ish-value") && page.contains("var k"));
    assert!(
        !line.contains("secret-ish-value") && !line.contains("var k") && !line.contains("Sign in")
    );
}

#[test]
fn a_redirect_says_where_it_sent_the_browser_and_a_long_reply_is_cut() {
    let line = exchange_line(&exchange(
        "burst-1",
        "POST",
        "/notes",
        303,
        &[("location", "/notes/3")],
        "",
    ));
    assert_eq!(
        line,
        "POST /notes answered 303, sending the browser to /notes/3"
    );
    let long = format!("<p>{}</p>", "word ".repeat(100));
    let line = exchange_line(&exchange("x", "GET", "/", 200, &[], &long));
    assert!(line.ends_with("…\""), "{line}");
    assert!(line.chars().count() < SAW_CHARS + 40, "{line}");
}

#[test]
fn a_reading_of_the_container_says_what_it_was_doing() {
    let r = seen::Reading {
        id: "liveness-2".into(),
        after: "after the sign-in questions".into(),
        status: "running".into(),
        restarts: 1,
        exit_code: 0,
        out_of_memory: false,
        answered: false,
    };
    assert_eq!(
        reading_line(&r),
        "The app's container, after the sign-in questions: it was running, restarted 1 time, and did not answer its health path"
    );
}
