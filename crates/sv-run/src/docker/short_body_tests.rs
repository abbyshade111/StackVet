//! Backlog 0598 (ADR-021): an answer that came with less body than its `Content-Length` said is no answer. In the clean
//! Cursor + Flash run of 15 October 2026 the sign-in page answered 200 with `Content-Length: 5415` and no body at all
//! (uvicorn's "Response content longer than Content-Length"), was blank in a browser, and counted as answered.

use super::*;

const TRIAL: &str =
    "HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=utf-8\r\ncontent-length: 5415\r\n\r\n";

#[test]
fn a_body_shorter_than_its_length_is_short_and_a_whole_one_is_not() {
    assert_eq!(short_body(TRIAL, false), Some((0, 5415)));
    let cut = "HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nhello";
    assert_eq!(short_body(cut, false), Some((5, 10)));
    let whole = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
    assert_eq!(short_body(whole, false), None);
    // Anything after the body (what `nc` itself says, a trailing line) is never taken for less.
    assert_eq!(short_body(&format!("{whole}\n"), false), None);
    // No length said: nothing to hold it to.
    assert_eq!(short_body("HTTP/1.0 200 OK\r\n\r\n", false), None);
}

#[test]
fn an_answer_with_no_body_by_http_s_rules_is_never_short() {
    // `HEAD`, and 204, 304 and 1xx, carry a length and no body.
    assert_eq!(short_body(TRIAL, true), None);
    for status in ["204 No Content", "304 Not Modified", "100 Continue"] {
        let raw = format!("HTTP/1.1 {status}\r\nContent-Length: 5415\r\n\r\n");
        assert_eq!(short_body(&raw, false), None, "{status}");
    }
}

#[test]
fn the_backend_reads_a_short_answer_as_none_and_writes_it_down() {
    let backend = DockerBackend::new();
    assert!(backend.whole_answer("login-page", TRIAL, false).is_none());
    let whole = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
    assert_eq!(
        backend.whole_answer("home", whole, false).map(|a| a.status),
        Some(200)
    );
    assert_eq!(
        backend.short_answers(),
        vec!["login-page (200, 0 of 5415 bytes)".to_owned()]
    );
}

#[test]
fn the_app_s_own_line_on_it_is_found() {
    let log = "INFO: 172.18.0.3 - \"GET /login HTTP/1.0\" 200 OK\n\
               ERROR: Exception in ASGI application\n\
               RuntimeError: Response content longer than Content-Length\n";
    assert_eq!(
        content_length_line(log).as_deref(),
        Some("RuntimeError: Response content longer than Content-Length")
    );
    assert_eq!(content_length_line("INFO: all well\n"), None);
}
