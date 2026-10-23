//! When the build record's calls were made, said for a person to read (backlog 0635).

use super::span_said;

#[test]
fn one_day_reads_as_a_date_and_two_times() {
    assert_eq!(
        span_said(Some("2026-10-22T22:03:54Z"), Some("2026-10-22T22:42:27Z")),
        ", on 2026-10-22, from 22:03 to 22:42 UTC"
    );
    assert_eq!(
        span_said(Some("2026-10-22T22:03:54Z"), None),
        ", on 2026-10-22 at 22:03 UTC"
    );
}

#[test]
fn two_days_or_unreadable_times_keep_the_record_s_own() {
    assert_eq!(
        span_said(Some("2026-10-22T23:50:00Z"), Some("2026-10-23T00:10:00Z")),
        ", from 2026-10-22T23:50:00Z to 2026-10-23T00:10:00Z"
    );
    assert_eq!(
        span_said(Some("yesterday"), Some("today")),
        ", from yesterday to today"
    );
    assert_eq!(span_said(None, None), "");
}
