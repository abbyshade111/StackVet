//! Backlog 0352: the line of a finding comes from where its text's line breaks are, found once, rather than by
//! counting them from the start of the file again for every finding. The line found must not change.

use super::*;

#[test]
fn the_line_of_each_place_is_one_more_than_the_line_breaks_before_it() {
    for text in ["", "one line", "a\nb\n\nc\n", "\n\n", "é\nü\r\nz"] {
        let lines = Lines::new(text);
        for at in 0..=text.len() + 2 {
            let before = &text.as_bytes()[..at.min(text.len())];
            let counted = before.iter().filter(|b| **b == b'\n').count() + 1;
            assert_eq!(lines.of(at), counted, "{text:?} at {at}");
        }
    }
}
