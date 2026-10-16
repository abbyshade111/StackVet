//! Tests of `docker.rs`: how often the readiness poll asks. Moved out of `docker.rs` unchanged (backlog 0324).

use super::*;

#[test]
fn the_readiness_poll_starts_at_a_quarter_second_and_doubles_to_two() {
    let ms: Vec<u128> = (0..6).map(|n| ready_pause(n).as_millis()).collect();
    assert_eq!(ms, [250, 500, 1_000, 2_000, 2_000, 2_000]);
    // Over the minute a start may take, that is still a try every two seconds at most.
    let first_four: u128 = ms[..4].iter().sum();
    assert_eq!(first_four, 3_750);
}
