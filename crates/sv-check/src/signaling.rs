//! V17.3.2: malformed messages sent to the app's WebRTC signaling WebSocket, and a normal one after them, on a fresh
//! connection, must still be answered (backlog 0625, part 4; ADR-149).
//!
//! The signaling server passes a call's setup between the two people in it. V17.3.2 asks that it keeps working when a
//! client sends it nonsense. Which message is normal, and what its answer holds, is the app's own, so stackvet.toml
//! names them (`signaling` under `[stack.run]`), and nothing is asked without them.
//!
//! Three connections go to the copy of the app `sv` started, one after another: the normal message alone, whose answer
//! must hold the word, or nothing is concluded; malformed messages, after which the server may close the connection, a
//! fair answer to a client sending nonsense; and the normal message again. No answer to the last, after the first was
//! answered, is a finding. Answered again is credited in part: a few kinds of malformed message, one client, one
//! message after.

use crate::finding::{Confidence, Finding, Location, Severity};
use crate::probes::{ProbeRequest, ProbeResponse};
use crate::verified::Verified;
use sv_manifest::Signaling;

pub const STOPPED: &str = "probe.signaling-stops-after-malformed";

const BEFORE: &str = "signaling-before";
const MALFORMED: &str = "signaling-malformed";
const AFTER: &str = "signaling-after";

/// The handshake key every request sends; the answer's accept value is not checked, since what is judged is whether
/// the normal message is answered.
const KEY: &str = "c3Ytc2lnbmFsaW5nLXByb2Jl";

/// The masking key each frame is sent with: RFC 6455 asks a client to mask, and says nothing of how the key is chosen.
const MASK: [u8; 4] = [0x37, 0xfa, 0x21, 0x3d];

/// One masked text frame, as a client writes it (RFC 6455, section 5.2).
pub fn text_frame(payload: &[u8]) -> Vec<u8> {
    let mut out = vec![0x81];
    match payload.len() {
        n if n < 126 => out.push(0x80 | n as u8),
        n if n <= u16::MAX as usize => {
            out.push(0x80 | 126);
            out.extend_from_slice(&(n as u16).to_be_bytes());
        }
        n => {
            out.push(0x80 | 127);
            out.extend_from_slice(&(n as u64).to_be_bytes());
        }
    }
    out.extend_from_slice(&MASK);
    out.extend(payload.iter().enumerate().map(|(i, b)| b ^ MASK[i % 4]));
    out
}

/// The malformed messages: text that is not JSON, JSON of the wrong shape, a number too large for any field, an empty
/// message, and one far longer than any other.
fn malformed() -> Vec<Vec<u8>> {
    vec![
        b"{\"type\": \"offer\", \"sdp\": ".to_vec(),
        b"[1, 2, 3]".to_vec(),
        b"{\"type\": 1e999999, \"candidate\": {\"port\": 99999999999999999999}}".to_vec(),
        Vec::new(),
        vec![b'x'; 70_000],
    ]
}

fn handshake(id: &str, path: &str, frames: Vec<u8>) -> ProbeRequest {
    ProbeRequest {
        id: id.to_owned(),
        method: "GET".to_owned(),
        path: path.to_owned(),
        headers: vec![
            ("Upgrade".into(), "websocket".into()),
            ("Connection".into(), "Upgrade".into()),
            ("Sec-WebSocket-Key".into(), KEY.into()),
            ("Sec-WebSocket-Version".into(), "13".into()),
        ],
        // The frames, written after the handshake as a client writes them (ADR-149).
        body: Some(frames),
    }
}

pub fn requests(signaling: Option<&Signaling>) -> Vec<ProbeRequest> {
    let Some(signaling) = signaling else {
        return Vec::new();
    };
    let normal = text_frame(signaling.message.as_bytes());
    vec![
        handshake(BEFORE, &signaling.path, normal.clone()),
        handshake(
            MALFORMED,
            &signaling.path,
            malformed().iter().flat_map(|m| text_frame(m)).collect(),
        ),
        handshake(AFTER, &signaling.path, normal),
    ]
}

/// Whether the connection was taken and the answer holds the word.
fn answered(responses: &[ProbeResponse], id: &str, word: &str) -> bool {
    responses
        .iter()
        .find(|r| r.id == id)
        .is_some_and(|r| r.status == 101 && r.body.contains(word))
}

/// What the signaling probe showed: findings, credits, and what was not assessed, with why.
pub fn evaluate(
    responses: &[ProbeResponse],
    signaling: Option<&Signaling>,
    verified_so_far: &[Verified],
) -> (Vec<Finding>, Vec<Verified>, Vec<(String, String)>) {
    let (mut findings, mut verified, mut not_assessed) = (Vec::new(), Vec::new(), Vec::new());
    // Left out, nothing is said: most apps have no WebRTC, and V17.3.2 stays not verified for one that does.
    let Some(signaling) = signaling else {
        return (findings, verified, not_assessed);
    };
    let (path, word) = (&signaling.path, &signaling.answer);
    if !answered(responses, BEFORE, word) {
        not_assessed.push((
            "V17.3.2".to_owned(),
            format!(
                "The normal message sent to {path} was not answered with \"{word}\" before anything else was sent, so \
                 what happens after malformed ones could not be seen. Check `signaling` under `[stack.run]`: the path, \
                 a message the app answers, and a word its answer holds."
            ),
        ));
    } else if answered(responses, AFTER, word) {
        verified.push(
            Verified::new(
                STOPPED,
                &["V17.3.2"],
                format!(
                    "after malformed messages on one connection to {path} (not JSON, the wrong shape, a number too \
                     large, an empty message, and one of 70,000 characters), the normal message on a fresh connection \
                     was answered as before: a few kinds of message, from one client"
                ),
            )
            .with_evidence(vec![BEFORE.into(), MALFORMED.into(), AFTER.into()])
            .in_part(),
        );
    } else {
        findings.push(crate::finding::found(Finding {
            evidence: vec![BEFORE.into(), MALFORMED.into(), AFTER.into()],
            also_reported_by: Vec::new(),
            fingerprint: String::new(),
            earlier_fingerprints: Vec::new(),
            marked_test_code: false,
            bundled_library: None,
            outranked: None,
            also_on_this_line: Vec::new(),
            help_uri: None,
            rule_id: STOPPED.into(),
            title: "The signaling server stops answering after malformed messages".into(),
            severity: Severity::High,
            confidence: Confidence::Medium,
            location: Location::running_app(),
            secret: None,
            requirement_ids: vec!["V17.3.2".into()],
            cwe: vec!["CWE-20".into()],
            description: format!(
                "The normal message sent to {path} was answered with \"{word}\". After one connection sent malformed \
                 messages (not JSON, the wrong shape, a number too large, an empty message, and one of 70,000 \
                 characters), the same normal message on a fresh connection was not."
            ),
            impact: "One person, or one broken client, can stop calls being set up for everyone, by sending the \
                     signaling server a message it does not expect."
                .into(),
            fix: "Read each signaling message inside a guard: parse it, check its shape and its sizes, and drop or refuse \
                  one that does not fit, closing that client's connection if need be, so one bad message never stops \
                  the server or its other connections."
                .into(),
        }));
    }
    let mut together = verified_so_far.to_vec();
    together.extend(verified.iter().cloned());
    crate::verified::unless_credited(STOPPED, &together);
    (findings, verified, not_assessed)
}

#[cfg(test)]
mod tests;
