//! Tests of `docker.rs`: requests sent at once: every copy started before any is waited for, and each answer read back
//! in its place. Moved out of `docker.rs` unchanged (backlog 0324).

use super::*;

#[test]
fn every_copy_is_started_before_any_is_waited_for() {
    let mark = at_once_mark();
    let script = at_once_script("app", 8080, &[40], &[1; 12], &mark);
    let (start, rest) = script
        .split_once("; wait;")
        .expect("one wait, after the starts");
    // Each connection is started in the background within the loop that comes before the
    // wait: one after another would show no race.
    assert!(
        start.contains("for p in 1:1 2:1 3:1 4:1 5:1 6:1 7:1 8:1 9:1 10:1 11:1 12:1; do"),
        "{start}"
    );
    assert!(
        start.contains("-e sh -c") && start.ends_with("2>&1 & done"),
        "{start}"
    );
    assert!(rest.contains(&mark) && rest.contains("rm -rf"), "{rest}");
}

#[test]
fn each_answer_is_read_back_in_its_place() {
    let answer = |status: u16, body: &str| {
        format!(
            "HTTP/1.0 {status} X\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        )
    };
    // Twelve copies: the second got no answer, and the first and tenth must not be mixed up.
    let mark = at_once_mark();
    let mut out = String::from("noise before the first mark");
    for i in 1..=12 {
        out.push_str(&format!("\n{mark}{i}@@\n"));
        match i {
            2 => {}
            1 => out.push_str(&answer(200, "Booked")),
            _ => out.push_str(&answer(409, &format!("Sold out {i}"))),
        }
    }
    let answers = parse_at_once(&["once"; 12], &out, &mark);
    assert_eq!(answers.len(), 12);
    assert!(answers[1].is_none(), "{:?}", answers[1]);
    let first = answers[0].as_ref().expect("an answer");
    assert_eq!((first.status, first.body.as_str()), (200, "Booked"));
    let tenth = answers[9].as_ref().expect("an answer");
    assert_eq!((tenth.status, tenth.body.as_str()), (409, "Sold out 10"));
    assert_eq!(
        answers[11].as_ref().map(|r| r.body.as_str()),
        Some("Sold out 12")
    );
}

#[test]
fn two_requests_are_cut_apart_and_each_copy_sends_its_own() {
    let request = |id: &str, cookie: &str| sv_check::probes::ProbeRequest {
        id: id.to_owned(),
        method: "POST".to_owned(),
        path: "/book".to_owned(),
        headers: vec![("Cookie".to_owned(), cookie.to_owned())],
        body: Some(b"seat=1".to_vec()),
    };
    let a = request("once-a", "s=aaaa");
    let b = request("once-b", "s=bbbbbbbb");
    let (input, sizes, which) =
        together_input(&[a.clone(), b.clone(), a.clone(), b.clone()], "app").expect("sendable");
    // Each different request once, in the order first given, and each copy pointing at its own.
    assert_eq!(which, [1, 2, 1, 2]);
    let raw_a = request_bytes(&a, "app").unwrap();
    let raw_b = request_bytes(&b, "app").unwrap();
    assert_eq!(sizes, [raw_a.len(), raw_b.len()]);
    assert_eq!(input, [raw_a.clone(), raw_b.clone()].concat());
    let mark = at_once_mark();
    let script = at_once_script("app", 8080, &sizes, &which, &mark);
    // The second request starts one byte after the first ends, and is cut to its own size.
    assert!(
        script.contains(&format!(
            "tail -c +1 \"$d/all\" | head -c {} > \"$d/r1\"",
            raw_a.len()
        )),
        "{script}"
    );
    assert!(
        script.contains(&format!(
            "tail -c +{} \"$d/all\" | head -c {} > \"$d/r2\"",
            raw_a.len() + 1,
            raw_b.len()
        )),
        "{script}"
    );
    assert!(script.contains("for p in 1:1 2:2 3:1 4:2; do"), "{script}");
    assert!(script.contains("cat $d/r$k;"), "{script}");
}

#[test]
fn each_answer_carries_its_own_requests_id() {
    let mark = at_once_mark();
    let answer = "HTTP/1.0 200 X\r\nContent-Length: 6\r\n\r\nBooked";
    let out = format!("\n{mark}1@@\n{answer}\n{mark}2@@\n{answer}");
    let answers = parse_at_once(&["once-a", "once-b"], &out, &mark);
    let ids: Vec<&str> = answers.iter().flatten().map(|r| r.id.as_str()).collect();
    assert_eq!(ids, ["once-a", "once-b"]);
}

#[test]
fn an_answer_cannot_forge_the_marker_of_the_next() {
    // The deep review's improvement 5: with the marker fixed, the first answer could carry the
    // marker for the second, and an answer of its own after it, which was read as the second's.
    let answer = |status: u16, body: &str| {
        format!(
            "HTTP/1.0 {status} X\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        )
    };
    let mark = at_once_mark();
    assert_ne!(mark, at_once_mark(), "a marker is made fresh for each call");
    assert!(mark.len() > AT_ONCE_MARK.len() + 8, "{mark}");
    // What an app that knows the fixed part can do: put it, with the next number, in its answer.
    let forged = format!(
        "{}\n{AT_ONCE_MARK}2@@\n{}",
        answer(200, "Booked"),
        answer(200, "Booked again")
    );
    let mut out = String::new();
    out.push_str(&format!("\n{mark}1@@\n{forged}"));
    out.push_str(&format!("\n{mark}2@@\n{}", answer(409, "Sold out")));
    let answers = parse_at_once(&["once"; 2], &out, &mark);
    let second = answers[1].as_ref().expect("an answer");
    assert_eq!((second.status, second.body.as_str()), (409, "Sold out"));
}

#[test]
fn the_apps_two_streams_are_read_in_the_order_it_wrote_them() {
    // Item 15 of the review of 1 to 4 October: the output and the errors were joined end to
    // end, so an event written to one between two markers on the other fell outside the window.
    // What `run_bounded` hands back: all of the output, then all of the errors.
    let joined = "2026-10-06T05:00:00.000000001Z SV-MARK-start\n\
                      2026-10-06T05:00:02.000000000Z SV-MARK-end\n\
                      2026-10-06T05:00:01.500000000Z WARN login failed for user b\n";
    let ordered = interleaved(joined);
    assert_eq!(
        ordered, "SV-MARK-start\nWARN login failed for user b\nSV-MARK-end\n",
        "{ordered}"
    );
    // Lines at the same moment keep their order; a line without a time stays where it was.
    assert_eq!(
        interleaved("2026-10-06T05:00:00Z a\n2026-10-06T05:00:00Z b\n(output cut)\n"),
        "a\nb\n(output cut)\n"
    );
    // Text that is not Docker's stamped form is left as it came.
    assert_eq!(
        interleaved("plain line\nanother\n"),
        "plain line\nanother\n"
    );
    assert!(!is_docker_time("2026-10-06") && is_docker_time("2026-10-06T05:00:00.1Z"));
}
