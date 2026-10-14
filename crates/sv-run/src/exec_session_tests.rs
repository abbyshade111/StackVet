//! The session's framing, held without Docker: the loop runs in this computer's own `sh`, and only its `-`
//! exchange is used, which hands each request straight back (backlog 0341). That a real sidecar answers through it is
//! held by `docker`'s `requests_through_one_session_reach_a_real_server_with_one_exec`, where there is Docker.

use super::*;

fn local(mark: &str) -> Option<Session> {
    let mut command = Command::new("sh");
    command.args(["-c", &loop_script(mark)]);
    Session::start(command, mark)
}

fn mark() -> String {
    format!("@@sv-session-{}-", crate::random_hex(8))
}

#[test]
fn a_session_starts_only_once_its_framing_hands_every_byte_back() {
    if cfg!(windows) {
        println!("no sh here; this needs one");
        return;
    }
    let mark = mark();
    assert!(local(&mark).is_some(), "the check exchange failed");
    // A shell that ends at once fails the check, and no session is made.
    let mut command = Command::new("sh");
    command.args(["-c", "read -r a; exit 3"]);
    let started = Instant::now();
    assert!(Session::start(command, &mark).is_none());
    assert!(started.elapsed() < Duration::from_secs(10));
    // So does one that answers with the right marker and the wrong bytes.
    let mut command = Command::new("sh");
    command.args([
        "-c",
        &format!("while read -r a; do read -r b; printf '\\n%s0\\n' '{mark}'; done"),
    ]);
    let started = Instant::now();
    assert!(Session::start(command, &mark).is_none());
    assert!(started.elapsed() < Duration::from_secs(10));
}

#[test]
fn many_requests_go_through_one_session_each_answered_whole_and_in_turn() {
    if cfg!(windows) {
        println!("no sh here; this needs one");
        return;
    }
    let mark = mark();
    let mut session = local(&mark).expect("a session");
    let large: Vec<u8> = (0..300_000u32).map(|i| (i % 251) as u8).collect();
    let requests: Vec<Vec<u8>> = vec![
        b"GET / HTTP/1.0\r\nHost: app\r\n\r\n".to_vec(),
        // What looks like the marker, but is not this session's.
        format!("@@sv-session-0000-0\n{}", "x".repeat(10)).into_bytes(),
        b"no newline at the end".to_vec(),
        b"\n\n\n".to_vec(),
        large,
        b"after the large one".to_vec(),
    ];
    for request in &requests {
        let back = session
            .exchange("-", 0, request, ANSWER_LIMIT)
            .expect("an answer");
        assert_eq!(back.0, 0);
        assert_eq!(&back.1, request, "{} bytes", request.len());
    }
}

#[test]
fn a_session_that_ends_says_so_and_does_not_hang() {
    if cfg!(windows) {
        println!("no sh here; this needs one");
        return;
    }
    let mark = mark();
    let mut session = local(&mark).expect("a session");
    // Stopped from outside, as removing the sidecar stops an exec into it.
    session.stop();
    let started = Instant::now();
    let failed = session.exchange("-", 0, b"anything", ANSWER_LIMIT);
    assert!(
        matches!(failed, Err(Failed::NotSent) | Err(Failed::Lost { .. })),
        "{failed:?}"
    );
    assert!(started.elapsed() < Duration::from_secs(10));
}

#[test]
fn the_loop_sends_each_request_from_inside_the_sidecar_as_the_one_exec_did() {
    let script = loop_script("@@m-");
    // The same exchange as `exchange_script`: `nc -e` from the sidecar, stopped at 15 seconds.
    assert!(
        script.contains("timeout 15 nc -w 5 \"$host\" \"$port\" -e sh -c \"cat $f; cat 1>&2\""),
        "{script}"
    );
    // Read a line at a time, so the shell never takes more than one request's worth.
    assert!(script.contains("IFS= read -r body"), "{script}");
    // The request file goes, every time, and the code is `rm`'s, as an exec of its own ended with.
    assert!(script.contains("rm -f \"$f\"; code=$?;"), "{script}");
}

#[test]
fn an_answer_is_read_whole_however_its_pieces_fall() {
    let mark = "@@sv-session-ab12-";
    let answer = b"HTTP/1.1 200 OK\r\n\r\nan answer that ends\nin a newline\n".to_vec();
    let mut came = answer.clone();
    came.extend_from_slice(format!("\n{mark}0\n").as_bytes());
    // Every way of cutting it in two, and one byte at a time.
    for cut in 0..=came.len() {
        let mut reading = Reading::new(mark, came[..cut].to_vec());
        let mut read = reading.answer();
        if read.is_none() {
            reading.got.extend_from_slice(&came[cut..]);
            read = reading.answer();
        }
        assert_eq!(read, Some((0, answer.clone(), Vec::new())), "cut at {cut}");
    }
    let mut reading = Reading::new(mark, Vec::new());
    let mut read = None;
    for byte in &came {
        assert!(read.is_none(), "an answer before its end came");
        reading.got.push(*byte);
        read = reading.answer();
    }
    assert_eq!(read, Some((0, answer, Vec::new())));
}

#[test]
fn an_end_line_with_another_mark_is_part_of_the_answer() {
    let mut reading = Reading::new(
        "@@sv-session-ab12-",
        b"x\n@@sv-session-ff00-0\ny\n@@sv-session-ab12-7\nnext".to_vec(),
    );
    assert_eq!(
        reading.answer(),
        Some((7, b"x\n@@sv-session-ff00-0\ny".to_vec(), b"next".to_vec()))
    );
}
