//! The time said when a slow stage ends, and `--quiet` (backlog 0247).

use super::*;

#[test]
fn a_stage_of_five_seconds_or_more_says_its_time_and_a_quicker_one_does_not() {
    assert_eq!(took_line(2, 4.9), None);
    assert_eq!(
        took_line(2, 5.0).as_deref(),
        Some("sv report: 3 of 10 took 5 s")
    );
    assert_eq!(
        took_line(8, 48.4).as_deref(),
        Some("sv report: 9 of 10 took 48 s")
    );
}
