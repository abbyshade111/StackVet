//! The tests the five copies each carried, moved here with their expectations unchanged, and the two decoders'
//! disagreement held shut.

use super::*;

use Alphabet::{Standard, Web};
use Padding::{Optional, Required};

#[test]
fn base64_is_the_standard_one_and_reads_back_one_way_only() {
    // From ssh_format.rs, where the strict pair was.
    for (bytes, text) in [
        (&b""[..], ""),
        (b"f", "Zg=="),
        (b"fo", "Zm8="),
        (b"foo", "Zm9v"),
        (b"foob", "Zm9vYg=="),
        (b"fooba", "Zm9vYmE="),
        (b"foobar", "Zm9vYmFy"),
        (&[0xfb, 0xff], "+/8="),
    ] {
        assert_eq!(base64(bytes, Standard, true), text);
        assert_eq!(
            unbase64(text, Standard, Required).as_deref(),
            Some(bytes),
            "{text}"
        );
    }
    assert_eq!(base64(b"f", Standard, false), "Zg");
    for bad in ["Zg=", "Zh==", "Zg==Zg==", "Z===", "Zm9v!A==", "Zm9"] {
        assert_eq!(unbase64(bad, Standard, Required), None, "{bad}");
    }
}

#[test]
fn base64_matches_the_known_encodings() {
    // From sv-run's docker.rs: checked against values anybody can verify, rather than against this function's own
    // output.
    let padded = |bytes: &[u8]| base64(bytes, Standard, true);
    assert_eq!(padded(b""), "");
    assert_eq!(padded(b"f"), "Zg==");
    assert_eq!(padded(b"fo"), "Zm8=");
    assert_eq!(padded(b"foo"), "Zm9v");
    assert_eq!(padded(b"foob"), "Zm9vYg==");
    assert_eq!(padded(b"fooba"), "Zm9vYmE=");
    assert_eq!(padded(b"foobar"), "Zm9vYmFy");
    assert_eq!(
        padded(b"GET / HTTP/1.0\r\n\r\n"),
        "R0VUIC8gSFRUUC8xLjANCg0K"
    );
}

#[test]
fn base64_without_padding_in_either_alphabet() {
    // From browser.rs.
    assert_eq!(base64(b"foobar", Standard, false), "Zm9vYmFy");
    assert_eq!(base64(b"fooba", Standard, false), "Zm9vYmE");
    assert_eq!(base64(&[0xfb, 0xff], Standard, false), "+/8");
    assert_eq!(base64(&[0xfb, 0xff], Web, false), "-_8");
}

#[test]
fn a_web_tokens_part_reads_back_padded_or_not() {
    // From signed_in/tokens.rs.
    let part = base64(b"any bytes at all?", Web, false);
    assert_eq!(
        unbase64(&part, Web, Optional).unwrap(),
        b"any bytes at all?"
    );
    assert_eq!(unbase64("YQ==", Web, Optional).unwrap(), b"a");
    assert_eq!(unbase64("YQ", Web, Optional).unwrap(), b"a");
    assert_eq!(unbase64("", Web, Optional).unwrap(), b"");
    // One character over a whole group stands for no bytes at all, and a padded text is padded whole.
    for bad in ["Y", "YWJjZ", "YQ=", "YR", "YQ==YQ"] {
        assert_eq!(unbase64(bad, Web, Optional), None, "{bad}");
    }
}

#[test]
fn a_web_tokens_part_is_base64url_only() {
    // RFC 7515, section 2: a token's parts are base64url. The check that reads a Supabase key's role once took `+` and
    // `/` too, while the one that reads a signed-in token refused them, so one token was two things to two checks.
    let bytes = [0xfb, 0xff, 0xbf];
    assert_eq!(base64(&bytes, Web, false), "-_-_");
    assert_eq!(unbase64("-_-_", Web, Optional).as_deref(), Some(&bytes[..]));
    for standard in ["+/+/", "+_-_", "-_-/"] {
        assert_eq!(unbase64(standard, Web, Optional), None, "{standard}");
    }
    // And the other way: the standard alphabet's readers refuse the web one.
    assert_eq!(unbase64("-_-_", Standard, Required), None);
}
