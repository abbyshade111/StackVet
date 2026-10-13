//! What `curl` prints is kept to `CURL_OUTPUT_LIMIT` on each stream (backlog 0327). `sv run` caps every command's
//! output; `sv probe` read curl's whole output, so a site answering with header lines that never end filled memory for
//! the 15 seconds curl is given. Past the limit curl is now stopped, and the request is said not to have been read.

use super::*;
use std::time::{Duration, Instant};

#[cfg(unix)]
fn shell(script: &str) -> std::process::Command {
    let mut command = std::process::Command::new("sh");
    command.args(["-c", script]);
    command
}

#[cfg(unix)]
#[test]
fn a_program_that_never_stops_printing_is_stopped_at_the_limit() {
    let started = Instant::now();
    // `yes` prints for ever, as a site sending header lines without end would through curl.
    let printed = run_capped(
        &mut shell("exec yes 'X-Endless: aaaaaaaaaaaaaaaa'"),
        64 * 1024,
    )
    .unwrap();
    assert!(printed.cut);
    assert_eq!(printed.stdout.len(), 64 * 1024);
    assert!(
        !printed.status.success(),
        "it was stopped, not left to finish"
    );
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "it was read to its end rather than stopped: {:?}",
        started.elapsed()
    );
}

#[cfg(unix)]
#[test]
fn standard_error_is_held_to_the_same_limit() {
    let printed = run_capped(&mut shell("exec yes 'curl: endless' 1>&2"), 64 * 1024).unwrap();
    assert!(printed.cut);
    assert_eq!(printed.stderr.len(), 64 * 1024);
}

#[cfg(unix)]
#[test]
fn an_answer_under_the_limit_is_kept_whole_with_its_status() {
    // The control: what curl prints for a real site, headers and an error together, is read as it was.
    let printed = run_capped(
        &mut shell(
            "printf 'HTTP/1.1 200 OK\\r\\nServer: x\\r\\n\\r\\n'; echo 'curl: (6) no' 1>&2; exit 6",
        ),
        CURL_OUTPUT_LIMIT,
    )
    .unwrap();
    assert!(!printed.cut);
    assert_eq!(printed.stdout, b"HTTP/1.1 200 OK\r\nServer: x\r\n\r\n");
    assert_eq!(printed.stderr, b"curl: (6) no\n");
    assert_eq!(printed.status.code(), Some(6));
}

#[test]
fn the_reason_given_for_a_cut_answer_says_it_was_not_read() {
    let why = too_long();
    assert!(why.contains("1 MB"), "{why}");
    assert!(why.contains("not read"), "{why}");
}

#[cfg(unix)]
#[test]
fn a_cut_answer_is_not_read_as_the_sites_headers() {
    // What curl printed before it was stopped: a status line and headers that read as a whole answer.
    let printed = run_capped(
        &mut shell(
            "printf 'HTTP/1.1 200 OK\\r\\nX-Frame-Options: DENY\\r\\n'; exec yes 'X-Endless: a'",
        ),
        64 * 1024,
    )
    .unwrap();
    // The setup: the start is a real answer, which would be read as one if the cut were ignored.
    assert!(printed.cut);
    assert!(printed.stdout.starts_with(b"HTTP/1.1 200 OK\r\n"));
    let mut whole = printed.stdout.clone();
    whole.extend_from_slice(b"\r\n");
    assert_eq!(
        read_curl_output(&String::from_utf8_lossy(&whole), "x").status,
        200
    );

    let answer = answer_from(&printed, "x");
    assert_eq!(answer.status, 0);
    assert!(answer.headers.is_empty(), "{:?}", answer.headers);
    assert_eq!(answer.failure, Some(too_long()));
}
