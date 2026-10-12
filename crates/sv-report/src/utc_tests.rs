//! Report times are labeled UTC in the words a person reads (backlog 0255).

use super::*;

#[test]
fn a_time_in_utc_is_written_with_utc_named() {
    assert_eq!(
        utc_for_a_person("2026-10-04T18:55:02Z"),
        "2026-10-04 18:55:02 UTC"
    );
}

#[test]
fn anything_else_is_left_as_it_was() {
    for stamp in ["", "2026-10-04", "2026-10-04T18:55:02+02:00", "yesterday"] {
        assert_eq!(utc_for_a_person(stamp), stamp);
    }
}
