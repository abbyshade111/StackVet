//! Backlog 0526: the checks inside the suites, and the waiting for the app's limit, reach the report's timings after
//! the requests, each named as inside the suites, so the total counts none of them.

use super::*;

#[test]
fn the_checks_follow_the_requests_named_as_inside_the_suites() {
    let start = std::time::Instant::now();
    let checks = [
        ("the check `brute_force_check`".to_owned(), 125_300),
        (
            "waiting for the app's limit on requests".to_owned(),
            120_000,
        ),
    ];
    let got = timings(
        &[("Reading the app's files", start)],
        start,
        &[],
        &[("the questions asked as the test users", 150_390)],
        &[("login-a".to_owned(), 30)],
        &checks,
    );
    let named: Vec<(&str, u64)> = got.iter().map(|t| (t.what.as_str(), t.took_ms)).collect();
    assert_eq!(
        named[1..],
        [
            (
                "the running app, the questions asked as the test users",
                150_390
            ),
            ("the request login-a", 30),
            ("inside the suites, the check `brute_force_check`", 125_300),
            (
                "inside the suites, waiting for the app's limit on requests",
                120_000
            ),
        ]
    );
}
