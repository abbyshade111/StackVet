//! The owner's `backlog-board` names the board's path, and the dashboard links only what it names (backlog
//! 0227, ADR-086). Reading the file is tested through its text; `current` reads the real config folder.

use super::*;

#[test]
fn the_first_line_names_the_board_and_surrounding_space_is_left_out() {
    assert_eq!(
        named_in("  /Users/owner/code/stackvet-backlog.html \nsecond line ignored\n"),
        Some(PathBuf::from("/Users/owner/code/stackvet-backlog.html"))
    );
}

#[test]
fn an_empty_first_line_names_nothing() {
    assert_eq!(named_in(""), None);
    assert_eq!(named_in("\n/some/path.html\n"), None);
    assert_eq!(named_in("   \n"), None);
}
