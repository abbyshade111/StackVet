//! A credential in a form field, `<input name="csrf_token" value="…">`, whose value is an attribute the
//! name-and-value shapes do not read (backlog 0378): a page `sv` kept in `seen.json` showed a form's anti-forgery
//! token whole, and "What StackVet saw" quotes those pages.

use super::*;

fn rules() -> SecretRules {
    SecretRules::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/secret-rules.json"),
    )
    .expect("rules load")
}

/// A random token no vendor's shape describes, built from pieces so this file holds none.
fn token() -> String {
    ["EPurR_", "XL9CZk", "h0uLCb", "NZo620"].concat()
}

#[test]
fn a_form_token_is_cut_whichever_order_its_attributes_come_in() {
    let t = token();
    for page in [
        format!(
            r#"<form><input type="hidden" name="csrf_token" value="{t}"><button>Book</button></form>"#
        ),
        format!(r#"<input value="{t}" type="hidden" name="csrf_token">"#),
        format!(r#"<input type=hidden name=_csrf value={t}>"#),
        format!(r#"<INPUT NAME='authenticity_token' VALUE='{t}'>"#),
    ] {
        // The setup: the token is in the page before it is cut.
        assert!(page.contains(&t), "{page}");
        let (cut, n) = redact_text(&rules(), &page);
        assert!(!cut.contains(&t), "the token survived: {cut}");
        assert!(n >= 1, "the cut is counted: {cut}");
    }
}

#[test]
fn a_field_that_holds_no_credential_keeps_its_value() {
    let page = r#"<input type="text" name="title" value="Club evening"><input type="hidden" name="next" value="/account">"#;
    let (cut, n) = redact_text(&rules(), page);
    assert!(
        cut.contains("Club evening") && cut.contains("/account"),
        "{cut}"
    );
    assert_eq!(n, 0, "{cut}");
}
