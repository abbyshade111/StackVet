//! Backlog 0526: the checks inside a suite, and its waiting for the app's limit on requests, are named after the
//! slowest parts, never among them and never in the total, so a slow suite says where its time went.

use super::*;

fn timing(what: &str, took_ms: u64) -> Timing {
    Timing {
        what: what.to_owned(),
        took_ms,
    }
}

#[test]
fn the_trial_suite_says_where_its_time_went() {
    let timings = [
        timing("Running the app, when asked with --run", 160_000),
        timing(
            &format!("{SUITE_TIMING}the questions asked as the test users"),
            150_390,
        ),
        timing(
            &format!("{CHECK_TIMING}the check `brute_force_check`"),
            125_300,
        ),
        timing(
            &format!("{CHECK_TIMING}waiting for the app's limit on requests"),
            120_000,
        ),
        timing(&format!("{CHECK_TIMING}the check `password_checks`"), 9_000),
        timing(&format!("{CHECK_TIMING}the check `owned_checks`"), 4_000),
        timing(&format!("{CHECK_TIMING}the check `logout_check`"), 300),
    ];
    let line = slowest_of(&timings).unwrap();
    assert!(
        line.starts_with("This run took 160.0 s."),
        "counted once: {line}"
    );
    assert!(
        line.ends_with(
            " Inside them, the longest: the check `brute_force_check` (125.3 s), waiting for the app's limit on \
             requests (120.0 s), the check `password_checks` (9.0 s)."
        ),
        "{line}"
    );
    let parts = line.split(" Inside them").next().unwrap();
    assert!(
        !parts.contains(CHECK_TIMING) && !parts.contains("brute_force"),
        "not among the slowest parts: {line}"
    );
}

#[test]
fn checks_under_a_second_are_not_named() {
    let timings = [
        timing("Running the app, when asked with --run", 9_000),
        timing(&format!("{CHECK_TIMING}the check `logout_check`"), 999),
    ];
    let line = slowest_of(&timings).unwrap();
    assert!(!line.contains("Inside them"), "{line}");
}
