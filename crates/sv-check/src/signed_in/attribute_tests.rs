//! `attribute` reads a value quoted three ways, and reads the same once its pattern is kept: the
//! patterns are compiled once per name and reused (backlog 0463), so a second call for a name, and
//! a call for another name between, must give what the first did.

use super::attribute;

#[test]
fn each_quoting_form_is_read_and_read_again_the_same() {
    let tags = [
        (r#"<input name="token" value="a b">"#, "a b"),
        ("<input name='token' value='c'>", "c"),
        ("<input name=token value=d>", "d"),
        // The name in another case is still the attribute.
        (r#"<input NAME="token" VALUE="e">"#, "e"),
    ];
    for round in 0..2 {
        for (tag, value) in tags {
            assert_eq!(
                attribute(tag, "value").as_deref(),
                Some(value),
                "{round}: {tag}"
            );
            // Another name between, so the kept patterns are not one.
            assert_eq!(
                attribute(tag, "name").as_deref(),
                Some("token"),
                "{round}: {tag}"
            );
        }
    }
    // A name that is only the end of another attribute's is not it.
    assert_eq!(attribute(r#"<input data-value="x">"#, "value"), None);
    assert_eq!(attribute("<input disabled>", "disabled"), None);
}
