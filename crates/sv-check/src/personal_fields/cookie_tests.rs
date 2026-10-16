//! Backlog 0610: a session cookie that is JSON in base64 is read for its field names, as a JWT's payload is, and
//! nothing else is.

use super::*;
use crate::encoding::{Alphabet, base64};

fn coded(json: &str, alphabet: Alphabet, pad: bool) -> String {
    base64(json.as_bytes(), alphabet, pad)
}

fn cookie(name: &str, value: &str) -> (String, String) {
    (name.to_owned(), value.to_owned())
}

#[test]
fn a_flask_session_cookie_is_read_for_its_field_names() {
    let value = format!(
        "{}.ZX1a2b.c2lnbmF0dXJl",
        coded(
            r#"{"user_id":7,"email":"x@example.test","_fresh":true}"#,
            Alphabet::Web,
            false
        )
    );
    let found = cookie_fields(&[cookie("session", &value)]).expect("read");
    assert_eq!(found.fields, ["_fresh", "email", "user_id"]);
    assert!(found.cookie);
    assert_eq!(found.carried, "the cookie `session`");
    let said = found.said();
    assert!(
        said.contains("session cookie") && said.contains("JSON in base64"),
        "{said}"
    );
    assert!(
        said.contains("`email` (looks like personal data)"),
        "{said}"
    );
    assert!(!said.contains("x@example.test"), "never a value: {said}");
}

#[test]
fn an_express_cookie_session_padded_and_url_coded_is_read() {
    let value = coded(
        r#"{"passport":{"user":"ab"},"role":"admin"}"#,
        Alphabet::Standard,
        true,
    )
    .replace('=', "%3D");
    let found = cookie_fields(&[cookie("express:sess", &value)]).expect("read");
    assert_eq!(found.fields, ["passport", "role"]);
}

#[test]
fn an_opaque_or_compressed_or_empty_cookie_is_not_read() {
    for value in [
        "s%3A3f9a1c0e7b2d4e6f8a0b1c2d3e4f5a6b.Zx9y",
        ".eJyrVkosLcmIL8nPTs1TslIqLU4tUgIAUdgHAw.ZX1a.sig",
        &coded("{}", Alphabet::Web, false),
        &coded("[1,2,3]", Alphabet::Web, false),
        "0123456789abcdef0123456789abcdef",
    ] {
        assert!(cookie_fields(&[cookie("sid", value)]).is_none(), "{value}");
    }
}
