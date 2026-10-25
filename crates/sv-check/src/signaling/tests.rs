//! Backlog 0625, part 4, ADR-149: the signaling probe's frames, its three connections, and what their answers conclude.

use super::*;

fn signaling() -> Signaling {
    Signaling {
        path: "/ws".into(),
        message: "{\"type\":\"ping\"}".into(),
        answer: "pong".into(),
    }
}

fn answer(id: &str, status: u16, body: &str) -> ProbeResponse {
    ProbeResponse {
        id: id.into(),
        status,
        headers: Vec::new(),
        body: body.into(),
    }
}

#[test]
fn a_frame_is_written_as_rfc_6455_s_own_example() {
    // RFC 6455, section 5.7: "A single-frame masked text message" holding "Hello", with this masking key.
    assert_eq!(
        text_frame(b"Hello"),
        [
            0x81, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58
        ]
    );
    // A length past 125 takes two more bytes, and past 65,535 eight (section 5.2).
    let medium = text_frame(&[b'a'; 300]);
    assert_eq!(&medium[..4], &[0x81, 0x80 | 126, 0x01, 0x2c]);
    assert_eq!(medium.len(), 2 + 2 + 4 + 300);
    let long = text_frame(&[b'a'; 70_000]);
    assert_eq!(long[1], 0x80 | 127);
    assert_eq!(u64::from_be_bytes(long[2..10].try_into().unwrap()), 70_000);
    // Unmasked again with the key, the payload is what was given.
    let unmasked: Vec<u8> = long[14..]
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ MASK[i % 4])
        .collect();
    assert!(unmasked.iter().all(|b| *b == b'a'));
}

#[test]
fn three_connections_are_asked_in_order_and_none_without_the_entry() {
    assert!(requests(None).is_empty());
    let asked = requests(Some(&signaling()));
    assert_eq!(
        asked.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        [BEFORE, MALFORMED, AFTER]
    );
    assert!(asked.iter().all(|r| {
        r.path == "/ws"
            && r.headers
                .iter()
                .any(|(k, v)| k == "Upgrade" && v == "websocket")
    }));
    // The first and last send the normal message; the middle one the five malformed ones.
    assert_eq!(asked[0].body, asked[2].body);
    assert_eq!(
        asked[0].body.as_deref(),
        Some(text_frame(b"{\"type\":\"ping\"}").as_slice())
    );
    let middle = asked[1].body.as_deref().unwrap();
    assert!(middle.len() > 70_000, "the long message is among them");
}

#[test]
fn answered_after_malformed_messages_is_credited_in_part_and_not_answered_is_a_finding() {
    let s = signaling();
    let up = |after: &str| {
        vec![
            answer(BEFORE, 101, "\u{fffd}\u{fffd}{\"type\":\"pong\"}"),
            answer(MALFORMED, 101, ""),
            answer(AFTER, 101, after),
        ]
    };
    let (findings, verified, not_assessed) = evaluate(&up("{\"type\":\"pong\"}"), Some(&s), &[]);
    assert!(findings.is_empty() && not_assessed.is_empty());
    assert_eq!(verified.len(), 1);
    assert!(verified[0].in_part && verified[0].requirement_ids == ["V17.3.2"]);
    let (findings, verified, _) = evaluate(&up(""), Some(&s), &[]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, STOPPED);
    assert!(verified.is_empty());
    // The server gone altogether after the malformed ones is the same finding.
    let gone = vec![answer(BEFORE, 101, "pong"), answer(MALFORMED, 101, "")];
    assert_eq!(evaluate(&gone, Some(&s), &[]).0.len(), 1);
}

#[test]
fn without_the_control_nothing_is_concluded_and_without_the_entry_nothing_is_said() {
    let s = signaling();
    for before in [answer(BEFORE, 101, "hello"), answer(BEFORE, 404, "pong")] {
        let responses = vec![before, answer(AFTER, 101, "")];
        let (findings, verified, not_assessed) = evaluate(&responses, Some(&s), &[]);
        assert!(findings.is_empty() && verified.is_empty());
        assert_eq!(not_assessed.len(), 1);
        assert!(
            not_assessed[0].1.contains("was not answered with \"pong\""),
            "{}",
            not_assessed[0].1
        );
    }
    let (findings, verified, not_assessed) = evaluate(&[], None, &[]);
    assert!(findings.is_empty() && verified.is_empty() && not_assessed.is_empty());
}
