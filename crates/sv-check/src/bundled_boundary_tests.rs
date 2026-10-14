//! `library_in` reads only the start of a file, cut at a byte count; a cut that falls inside a character must move to
//! the character's start rather than end the run (backlog 0457).

use super::*;

#[test]
fn a_cut_inside_a_character_moves_to_its_start() {
    // Each count is placed one byte into a two-byte character, for each of the two cuts.
    for cut in [KNOWN_WITHIN, READ] {
        let text = format!("{}é{}", "a".repeat(cut - 1), "b".repeat(READ));
        assert!(
            !text.is_char_boundary(cut),
            "the setup puts the cut inside `é`"
        );
        assert_eq!(library_in(&text), None, "{cut}");
    }
    // And a library is still known when its banner sits before such a cut.
    let banner = "/*! jQuery v3.6.1 | (c) OpenJS Foundation and other contributors | jquery.org/license */\n";
    let text = format!(
        "{banner}{}é{}",
        "a".repeat(KNOWN_WITHIN - 1 - banner.len()),
        "b".repeat(READ)
    );
    assert!(!text.is_char_boundary(KNOWN_WITHIN));
    assert_eq!(library_in(&text).as_deref(), Some("jQuery 3.6.1"));
}
