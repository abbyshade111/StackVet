//! A form post the app refused is quoted in the page's own words (backlog 0585), as a refused sign-in is (0527): "The
//! form was out of date" tells a person the token was the trouble, not the app.

use super::*;

#[test]
fn a_refused_form_post_quotes_what_the_page_said() {
    let o = run(App {
        text: "refused",
        ..Default::default()
    });
    let why = unassessed(&o, "V3.2.2").unwrap_or_default();
    assert!(
        why.contains("was refused (403), and the page said: \u{201c}Bad Request The form was out of date. Reload"),
        "{why}"
    );
    // The character that would turn the rest of the line around on a terminal is not passed on.
    assert!(!why.contains('\u{202e}'), "{why:?}");
}

#[test]
fn a_refused_form_post_with_no_words_says_only_the_status() {
    let o = run(App {
        text: "refused-silent",
        ..Default::default()
    });
    let why = unassessed(&o, "V3.2.2").unwrap_or_default();
    assert!(
        why.contains("was refused (403), so there was nothing to look for"),
        "{why}"
    );
    assert!(!why.contains("the page said"), "{why}");
}
