//! `line_of` counts from 1, and is the line a finding names.

use super::*;

#[test]
fn the_first_line_is_line_one_and_a_newline_ends_its_line() {
    let text = "one\ntwo\n\nfour é\n";
    assert_eq!(line_of(text, 0), 1);
    assert_eq!(
        line_of(text, 3),
        1,
        "the newline itself is on the line it ends"
    );
    assert_eq!(line_of(text, 4), 2);
    assert_eq!(line_of(text, 8), 3, "an empty line is a line");
    assert_eq!(line_of(text, text.find('é').unwrap()), 4);
    assert_eq!(
        line_of(text, text.len()),
        5,
        "the end of a text that ends in a newline is the next line"
    );
    assert_eq!(line_of("", 0), 1);
}
