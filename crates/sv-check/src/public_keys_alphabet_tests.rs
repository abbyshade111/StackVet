//! A Supabase key is a JSON Web Token, whose parts are base64url (RFC 7515, section 2). This check once read a part
//! written in the standard alphabet too, while the signed-in checks refused one, so the same text was a token to one
//! check and not a token to another (backlog 0455). Both now read through `crate::encoding::read_token_part`.

use super::*;
use crate::encoding::{Alphabet, base64};

/// A Supabase key whose payload is written in `alphabet`, with a claim chosen so that the two alphabets differ in it.
fn key(alphabet: Alphabet) -> String {
    let head = base64(br#"{"alg":"HS256","typ":"JWT"}"#, Alphabet::Web, false);
    let body = br#"{"iss":"supabase","ref":"abc","role":"service_role","note":"???>>>"}"#;
    let body = base64(body, alphabet, false);
    format!(
        "{head}.{body}.{}",
        ["Qv7r", "Lm2x", "Pd9k", "Ze4w", "Ht6n"].concat()
    )
}

fn kinds(value: &str) -> Vec<String> {
    findings_in(
        ".env.local",
        &format!("VITE_SUPABASE_URL=https://abc.supabase.co\nVITE_SUPABASE_KEY={value}\n"),
    )
    .into_iter()
    .map(|f| f.description)
    .collect()
}

#[test]
fn a_service_role_key_is_read_in_the_web_alphabet_and_not_the_standard_one() {
    let (web, standard) = (key(Alphabet::Web), key(Alphabet::Standard));
    // The setup holds only if the two alphabets wrote different payloads.
    assert!(
        standard.contains(['+', '/']) && web != standard,
        "the payload must use the characters the alphabets differ in: {standard}"
    );
    let found = kinds(&web);
    assert!(
        found
            .iter()
            .any(|d| d.contains("is given a Supabase service-role key")),
        "{found:?}"
    );
    let found = kinds(&standard);
    assert!(
        !found.iter().any(|d| d.contains("service-role")),
        "a payload with `+` or `/` is not a token's part: {found:?}"
    );
}
