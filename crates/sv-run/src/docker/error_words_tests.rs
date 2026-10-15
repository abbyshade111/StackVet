//! Backlog 0527: an error answer longer than the cut keeps the words a person would read on it, so a refused
//! sign-in's reason reaches the run's steps and seen.json; an answer that is not an error keeps what it always did.

use super::*;

/// The trial's page: inline styles longer than the cut, then the one sentence that says why.
fn page() -> String {
    format!(
        "<html><head><style>{}</style></head><body><p class=\"error\">The CSRF token is missing.</p></body></html>",
        ".a { color: red } ".repeat(400)
    )
}

#[test]
fn an_error_page_keeps_its_words_past_the_cut() {
    let page = page();
    assert!(
        page.chars().count() > KEPT_CHARS,
        "the setup: the page runs past the cut"
    );
    assert!(
        !kept_body(&page).contains("CSRF token"),
        "the setup: the cut alone loses it"
    );
    let kept = kept_answer(400, &page);
    assert!(
        kept.ends_with("\n[…]\nThe CSRF token is missing."),
        "{}",
        &kept[kept.len() - 80..]
    );
}

#[test]
fn a_page_that_is_not_an_error_or_is_short_keeps_what_it_did() {
    let page = page();
    assert_eq!(kept_answer(200, &page), kept_body(&page));
    assert_eq!(kept_answer(302, &page), kept_body(&page));
    let short = "<p>Bad request</p>";
    assert_eq!(kept_answer(400, short), short);
}

#[test]
fn the_words_kept_are_bounded() {
    let page = format!("<p>{}</p>", "word ".repeat(2000));
    let kept = kept_answer(500, &page);
    let after = kept.rsplit("\n[…]\n").next().unwrap();
    assert_eq!(after.chars().count(), ERROR_WORDS, "{after}");
}
