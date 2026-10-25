//! Backlog 0625, part 4, ADR-149: a WebSocket handshake that carries messages is written as a client writes one, the
//! frames straight after the headers; a handshake with no messages, and every other request, are written as before.

use super::*;
use sv_check::probes::ProbeRequest;

fn upgrade(body: Option<Vec<u8>>) -> ProbeRequest {
    ProbeRequest {
        id: "ws".into(),
        method: "GET".into(),
        path: "/ws".into(),
        headers: vec![
            ("Upgrade".into(), "websocket".into()),
            ("Connection".into(), "Upgrade".into()),
            (
                "Sec-WebSocket-Key".into(),
                "c3Ytc2lnbmFsaW5nLXByb2Jl".into(),
            ),
            ("Sec-WebSocket-Version".into(), "13".into()),
        ],
        body,
    }
}

#[test]
fn the_frames_follow_the_handshake_with_no_length_and_no_close() {
    let frames = sv_check::signaling::text_frame(b"Hello");
    let raw = request_bytes(&upgrade(Some(frames.clone())), "app").expect("written");
    let head_end = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("a blank line")
        + 4;
    let head = String::from_utf8(raw[..head_end].to_vec()).unwrap();
    assert!(
        head.starts_with("GET /ws HTTP/1.1\r\nHost: app\r\n"),
        "{head}"
    );
    assert!(
        !head.contains("Content-Length") && !head.contains("Connection: close"),
        "{head}"
    );
    assert_eq!(head.matches("Connection:").count(), 1, "{head}");
    assert_eq!(&raw[head_end..], frames.as_slice());
}

#[test]
fn a_handshake_with_no_messages_and_an_ordinary_request_are_as_before() {
    let plain = String::from_utf8(request_bytes(&upgrade(None), "app").unwrap()).unwrap();
    assert!(
        plain.starts_with("GET /ws HTTP/1.0\r\nHost: app\r\nConnection: close\r\n"),
        "{plain}"
    );
    let post = ProbeRequest {
        id: "p".into(),
        method: "POST".into(),
        path: "/notes".into(),
        headers: Vec::new(),
        body: Some(b"a=1".to_vec()),
    };
    let post = String::from_utf8(request_bytes(&post, "app").unwrap()).unwrap();
    assert!(post.contains("Content-Length: 3\r\n\r\na=1"), "{post}");
}

/// A small signaling server, written with aiohttp, whose WebSocket parser is its own and not `sv`'s: it answers
/// `{"type":"ping"}` with `{"type":"pong"}`, and drops what it cannot read. With `FRAGILE=1` it stops altogether on a
/// message it cannot read, as a signaling server without a guard does.
const SERVER: &str = r#"
import asyncio, json, os, sys
from aiohttp import web, WSMsgType
fragile = os.environ.get("FRAGILE") == "1"
async def ws(request):
    s = web.WebSocketResponse(max_msg_size=0)
    await s.prepare(request)
    async for m in s:
        if m.type != WSMsgType.TEXT:
            continue
        try:
            body = json.loads(m.data)
            if body.get("type") == "ping":
                await s.send_str('{"type":"pong"}')
        except Exception:
            if fragile:
                os._exit(1)
    return s
app = web.Application()
app.router.add_get("/ws", ws)
web.run_app(app, host="127.0.0.1", port=int(sys.argv[1]), print=None)
"#;

/// Sends `raw` to the server and reads what comes back within a second.
fn exchange(port: u16, raw: &[u8]) -> Option<String> {
    use std::io::{Read, Write};
    let mut stream = std::net::TcpStream::connect(("127.0.0.1", port)).ok()?;
    stream
        .set_read_timeout(Some(std::time::Duration::from_millis(1000)))
        .ok()?;
    stream.write_all(raw).ok()?;
    let mut got = Vec::new();
    let mut buf = [0u8; 8192];
    while let Ok(n) = stream.read(&mut buf) {
        if n == 0 {
            break;
        }
        got.extend_from_slice(&buf[..n]);
    }
    Some(String::from_utf8_lossy(&got).into_owned())
}

/// Starts the server, sends each of the probe's three connections in turn, and gives back what each answer said.
fn against_real_server(fragile: bool) -> Option<Vec<String>> {
    let ready = std::process::Command::new("python3")
        .args(["-c", "import aiohttp"])
        .output()
        .ok()?;
    if !ready.status.success() {
        return None;
    }
    let dir = sv_check::test_support::scratch("signaling-server");
    let script = dir.join("server.py");
    std::fs::write(&script, SERVER).unwrap();
    let port = std::net::TcpListener::bind(("127.0.0.1", 0))
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    // Not `-I`: the script is this test's own, and aiohttp may be among the user's own packages, which `-I` leaves out,
    // as the check above does not.
    let mut child = std::process::Command::new("python3")
        .arg(&script)
        .arg(port.to_string())
        .env("FRAGILE", if fragile { "1" } else { "0" })
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while std::net::TcpStream::connect(("127.0.0.1", port)).is_err()
        && std::time::Instant::now() < deadline
    {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let signaling = sv_manifest::Signaling {
        path: "/ws".into(),
        message: "{\"type\":\"ping\"}".into(),
        answer: "pong".into(),
    };
    let answers = sv_check::signaling::requests(Some(&signaling))
        .iter()
        .map(|r| exchange(port, &request_bytes(r, "127.0.0.1").unwrap()).unwrap_or_default())
        .collect();
    let _ = child.kill();
    let _ = child.wait();
    std::fs::remove_dir_all(&dir).ok();
    Some(answers)
}

#[test]
fn a_real_websocket_server_reads_the_frames_and_one_that_stops_is_told_from_one_that_does_not() {
    let Some(sturdy) = against_real_server(false) else {
        println!(
            "no Python with aiohttp here, so no real WebSocket server was run; the frames are held to RFC 6455's example"
        );
        return;
    };
    // The setup: the server took the handshake and answered the normal message.
    assert!(sturdy[0].starts_with("HTTP/1.1 101"), "{}", sturdy[0]);
    assert!(
        sturdy[0].contains("pong"),
        "the normal message was read: {}",
        sturdy[0]
    );
    assert!(
        sturdy[2].contains("pong"),
        "and again after the malformed ones: {}",
        sturdy[2]
    );
    let fragile = against_real_server(true).expect("the same server, made fragile");
    assert!(fragile[0].contains("pong"), "{}", fragile[0]);
    assert!(
        !fragile[2].contains("pong"),
        "a server that stopped answers nothing: {}",
        fragile[2]
    );
}
