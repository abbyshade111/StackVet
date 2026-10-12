//! A page with characters that change length when lowered (backlog 0278): `tags` finds positions in a
//! lowered copy and cuts the page at them, so the copy must keep every byte where it was.

use super::fake_app::*;
use super::*;

#[test]
fn a_tag_after_characters_that_change_length_when_lowered_is_cut_whole() {
    // `İ` grows from two bytes to three when fully lowered, the Kelvin sign shrinks from three to
    // one, and `Ω` (the ohm sign) from three to two: each moves every position after it.
    for before in [
        "\u{130}stanbul",
        "\u{212a}elvin",
        "\u{2126}hm",
        "\u{130}\u{212a}\u{2126}",
    ] {
        let page = format!("<h1>{before}</h1><form><INPUT name=csrf_token value=abc123></form>");
        let found = tags(&page, "input");
        assert_eq!(
            found,
            ["<INPUT name=csrf_token value=abc123"],
            "after {before:?}"
        );
        assert_eq!(attribute(&found[0], "value").as_deref(), Some("abc123"));
    }
}

#[test]
fn a_sign_in_page_with_a_turkish_heading_signs_in_as_the_plain_one_does() {
    let clean = run_against(Flaws::default(), &users());
    let turkish = run_against(
        Flaws {
            non_ascii_heading: true,
            ..Default::default()
        },
        &users(),
    );
    // The setup is real: the plain app signs in and the signed-in checks credit something, so an
    // equal list below is not two empty ones.
    assert!(!clean.verified.is_empty(), "{:#?}", clean.findings);
    let ids = |o: &Outcome| {
        let mut ids: Vec<String> = o
            .findings
            .iter()
            .map(|f| f.rule_id.clone())
            .chain(o.verified.iter().map(|v| v.check_id.clone()))
            .collect();
        ids.sort();
        ids
    };
    assert_eq!(ids(&turkish), ids(&clean), "{:#?}", turkish.findings);
}
