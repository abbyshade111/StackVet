//! One reader of a response's status line and headers, for the running app's answers and the live site's (backlog
//! 0461). Until then the live site's reader skipped a proxy's own status line and the running app's did not, and
//! only one of the two ways to ask for a header lowercased the name asked for.

use super::*;

#[test]
fn a_proxys_own_status_line_is_not_the_sites() {
    let (status, headers) = read_head(
        "HTTP/1.1 200 Connection established\r\n\r\nHTTP/2 403 \r\nX-Frame-Options: DENY\r\n",
    );
    assert_eq!(status, Some(403));
    assert_eq!(header_in(&headers, "x-frame-options"), Some("DENY"));
}

#[test]
fn a_proxys_line_after_the_sites_headers_does_not_wipe_them() {
    let (status, headers) =
        read_head("HTTP/2 200 \r\nX-A: b\r\nHTTP/1.1 200 Connection Established\r\n");
    assert_eq!(status, Some(200));
    assert_eq!(header_in(&headers, "x-a"), Some("b"));
}

#[test]
fn the_last_response_is_the_one_that_answered() {
    let (status, headers) =
        read_head("HTTP/1.1 301 Moved\r\nLocation: /next\r\n\r\nHTTP/1.1 200 OK\r\nX-A: b\r\n");
    assert_eq!(status, Some(200));
    assert_eq!(header_in(&headers, "location"), None);
}

#[test]
fn no_status_line_is_no_status() {
    assert_eq!(read_head(""), (None, Vec::new()));
    assert_eq!(read_head("connection refused").0, None);
    assert_eq!(read_head("HTTP/1.1 notanumber OK\r\n").0, None);
}

#[test]
fn a_header_is_found_whatever_case_it_is_asked_for_or_arrives_in() {
    let (_, headers) =
        read_head("HTTP/1.1 200 OK\r\nContent-TYPE: text/html\r\nLocation: https://a.test/b\r\n");
    for asked in ["content-type", "Content-Type", "CONTENT-TYPE"] {
        assert_eq!(header_in(&headers, asked), Some("text/html"), "{asked}");
    }
    // Split at the first colon only.
    assert_eq!(header_in(&headers, "location"), Some("https://a.test/b"));
}
