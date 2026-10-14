//! The tests of both copies of each, kept with their expectations (backlog 0461).

use super::*;

#[test]
fn first_line_survives_empty_and_blank_output() {
    // From sv-run's docker.rs.
    assert_eq!(first_line(""), "no detail");
    assert_eq!(first_line("\n\n  \n"), "no detail");
    assert_eq!(first_line("\n  real message  \nsecond"), "real message");
}

#[test]
fn first_line_is_the_first_line_trimmed() {
    // What adapters.rs's copy did with a message that was not blank, which is all it was given.
    assert_eq!(first_line("  it broke  \nat line 3"), "it broke");
    assert_eq!(first_line("one line"), "one line");
    assert_eq!(first_line("windows\r\nline"), "windows");
}

#[test]
fn a_cut_says_how_many_characters_it_left_out() {
    assert_eq!(cut_with_count("abcdefghij", 4), "abcd… (6 more characters)");
    assert_eq!(cut_with_count("abcd", 4), "abcd");
    assert_eq!(cut_with_count("abc", 4), "abc");
    assert_eq!(cut_with_count("", 0), "");
    assert_eq!(cut_with_count("abc", 0), "… (3 more characters)");
    // Characters, not bytes: a cut never splits one.
    assert_eq!(cut_with_count("ééééé", 2), "éé… (3 more characters)");
}
