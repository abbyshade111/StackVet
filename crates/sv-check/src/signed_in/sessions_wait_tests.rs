//! The line a long `--slow` wait says as it goes (backlog 0247).

use super::*;

#[test]
fn the_wait_says_how_far_it_is_in_whole_minutes_of_the_whole() {
    assert_eq!(
        waiting_line(0, 16 * 60),
        "waiting out the session timeouts: 0 of 16 minutes"
    );
    assert_eq!(
        waiting_line(4 * 60 + 30, 16 * 60),
        "waiting out the session timeouts: 4 of 16 minutes"
    );
    // A wait of 61 minutes and a second is said as 62, never less than it is.
    assert_eq!(
        waiting_line(0, 61 * 60 + 1),
        "waiting out the session timeouts: 0 of 62 minutes"
    );
}
