//! The live site's answer finds a header whatever case its name is asked for in, as the running app's does (backlog
//! 0461); until then it matched exactly, which was safe only while every caller asked in lowercase.

use super::*;

#[test]
fn a_header_asked_for_in_any_case_is_found() {
    let answer = parse_head("HTTP/2 200 \r\nstrict-transport-security: max-age=1\r\n");
    assert_eq!(
        answer.header("Strict-Transport-Security"),
        Some("max-age=1")
    );
    assert_eq!(
        answer.header("STRICT-TRANSPORT-SECURITY"),
        Some("max-age=1")
    );
}
