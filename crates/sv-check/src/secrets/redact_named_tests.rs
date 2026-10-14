//! A credential handed over by name where the assignment shapes do not reach (backlog 0283): a
//! parameter of a web address, whose `https:` the name-and-value pattern takes for the name, and a
//! command's flag, which has no `=` or `:` at all.

use super::*;

fn rules() -> &'static SecretRules {
    SecretRules::shared().expect("the rules sv ships")
}

/// A random token no vendor's shape describes, built from pieces so this file holds none.
fn token() -> String {
    ["Lm3qT7", "vR2kW9", "pZ5xN1", "c8B"].concat()
}

#[test]
fn a_credential_in_a_web_address_parameter_is_cut() {
    let rules = rules();
    let token = token();
    for text in [
        format!("https://mcp.example.com/sse?api_key={token}"),
        format!("https://gateway.example.com/?page=2&token={token}#top"),
        format!("see <https://hooks.example.com/in?access_token={token}> for the hook"),
    ] {
        let (out, n) = redact_text(rules, &text);
        assert!(!out.contains(&token), "{out}");
        assert!(out.contains("[redacted: Lm3q…"), "{out}");
        assert_eq!(n, 1, "{out}");
    }
    // A parameter whose name says nothing of a credential keeps its value.
    let (out, n) = redact_text(rules, "https://example.com/list?page=2&sort=name");
    assert_eq!(
        (out.as_str(), n),
        ("https://example.com/list?page=2&sort=name", 0)
    );
}

#[test]
fn a_credential_after_a_flag_named_for_it_is_cut() {
    let rules = rules();
    let token = token();
    for text in [
        format!("npx notes-mcp@1.2.3 --token {token}"),
        format!("server --api-key={token} --port 8080"),
        format!("run -password {token}"),
    ] {
        let (out, n) = redact_text(rules, &text);
        assert!(!out.contains(&token), "{out}");
        assert!(out.contains("[redacted: Lm3q…"), "{out}");
        assert_eq!(n, 1, "{out}");
    }
    // A flag whose name says nothing of a credential keeps its value, and so does the next flag
    // after a credential's flag given no value.
    for text in [
        "server --port 8080 --host 0.0.0.0",
        "server --no-token --port 8080",
    ] {
        let (out, n) = redact_text(rules, text);
        assert_eq!((out.as_str(), n), (text, 0));
    }
}
