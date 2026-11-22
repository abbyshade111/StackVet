//! `av proxy` (ADR-159, decision 10; backlog 0716, part 1), run as an agent's settings run it: in front of a tool
//! server, over standard input and output. The server here is a few lines of shell that answer by a call's id.

#![cfg(unix)]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use sv_frameworks::paths::Canonical;

fn folder(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("av-proxy-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonical().unwrap()
}

/// A tool server: call 1 is answered, 2 is refused, 3 fails in the tool, 4 is never answered, and any other line is
/// sent back as it came.
const SERVER: &str = r#"while IFS= read -r line; do
  case "$line" in
    *'"id":1,'*) printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"content":[{"type":"text","text":"THE ANSWER TEXT"}]}}' ;;
    *'"id":2,'*) printf '%s\n' '{"jsonrpc":"2.0","id":2,"error":{"code":-32001,"message":"not allowed"}}' ;;
    *'"id":3,'*) printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{"content":[],"isError":true}}' ;;
    *'"id":4,'*) ;;
    *) printf '%s\n' "$line" ;;
  esac
done"#;

fn call(id: u32, tool: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"{tool}","arguments":{arguments}}}}}"#
    )
}

/// Runs `av proxy` in front of `server`, hands it `input`, and gives its exit code, what it relayed back, and what
/// it said on standard error.
fn proxy(app: &Path, state: &Path, server: &[&str], input: &[u8]) -> (i32, Vec<u8>, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_av"))
        .args(["proxy", "--app", app.to_str().unwrap(), "--"])
        .args(server)
        .env("XDG_STATE_HOME", state)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("av runs");
    child.stdin.take().unwrap().write_all(input).unwrap();
    let out = child.wait_with_output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        out.stdout,
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// The record's lines for `app`, with `state` as the folder `av` keeps its records under.
fn record(app: &Path, state: &Path) -> Vec<serde_json::Value> {
    let key = av_record::plain::sha256(app.display().to_string().as_bytes());
    let path = state
        .join("stackvet/proxy-records")
        .join(format!("{}.jsonl", &key[..16]));
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|why| panic!("{}: {why}", path.display()));
    text.lines()
        .map(|l| serde_json::from_str(l).expect("a line of JSON"))
        .collect()
}

#[test]
fn every_line_is_relayed_as_it_came_and_each_call_is_written_down_with_its_answer() {
    let app = folder("relay");
    let state = folder("relay-state");
    // Built from pieces, so no key is written here whole.
    let key = format!(
        "{}{}",
        "sk-ant-api03-", "A1b2C3d4E5f6G7h8I9j0K1l2M3n4O5p6Q7r8S9t0U1v2W3x4Y5z6"
    );
    let mut input = Vec::new();
    for line in [
        r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{}}"#.to_owned(),
        call(1, "read_file", r#"{"path":"src/app.py"}"#),
        call(2, "delete_file", r#"{"path":"README.md"}"#),
        call(
            3,
            "http_post",
            &format!(r#"{{"url":"https://example.com","password":"{key}"}}"#),
        ),
        call(4, "slow_tool", "{}"),
        "this is not a message at all".to_owned(),
    ] {
        input.extend_from_slice(line.as_bytes());
        input.push(b'\n');
    }
    // And a line that is not text.
    input.extend_from_slice(b"\xff\xfe raw bytes\n");

    let (code, out, err) = proxy(&app, &state, &["sh", "-c", SERVER], &input);
    assert_eq!(code, 0, "{err}");
    let relayed = String::from_utf8_lossy(&out);
    // What the server sent comes back whole and in order, the answer's text included: nothing is changed on the way.
    let expected: Vec<&str> = relayed.lines().collect();
    assert_eq!(expected.len(), 6, "{relayed}");
    assert_eq!(
        expected[0],
        r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{}}"#
    );
    assert!(expected[1].contains("THE ANSWER TEXT"));
    assert_eq!(expected[4], "this is not a message at all");
    assert!(
        out.ends_with(b"\xff\xfe raw bytes\n"),
        "bytes that are not text are relayed too: {out:?}"
    );

    let lines = record(&app, &state);
    let said: Vec<(&str, &str)> = lines
        .iter()
        .map(|l| (l["tool"].as_str().unwrap(), l["answer"].as_str().unwrap()))
        .collect();
    assert_eq!(
        said,
        [
            ("read_file", "ok"),
            ("delete_file", "refused"),
            ("http_post", "tool-error"),
            ("slow_tool", "unanswered"),
        ]
    );
    assert_eq!(lines[1]["code"], -32001);
    assert_eq!(lines[0]["summary"], r#"{"path":"src/app.py"}"#);
    // Never an answer's text, and never a key.
    let whole = lines.iter().map(|l| l.to_string()).collect::<String>();
    assert!(
        !whole.contains("THE ANSWER TEXT") && !whole.contains("not allowed"),
        "{whole}"
    );
    assert!(!whole.contains(&key[10..]), "the key is in the record");
    // Each line is chained to the one before it.
    assert!(
        lines
            .iter()
            .all(|l| l["chain"].as_str().is_some_and(|c| c.len() == 64)),
        "{whole}"
    );
    // The record is the person's alone.
    use std::os::unix::fs::PermissionsExt;
    let path = std::fs::read_dir(state.join("stackvet/proxy-records"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    // And nothing was written into the app's folder.
    assert_eq!(std::fs::read_dir(&app).unwrap().count(), 0);
}

#[test]
fn a_record_that_cannot_be_written_is_said_and_the_relaying_goes_on() {
    let app = folder("unwritten");
    let state = folder("unwritten-state");
    // Where the records' folder would go there is a file, so nothing can be written there.
    std::fs::create_dir_all(state.join("stackvet")).unwrap();
    std::fs::write(state.join("stackvet/proxy-records"), "in the way").unwrap();
    let input = format!("{}\n", call(1, "read_file", "{}"));
    let (code, out, err) = proxy(&app, &state, &["sh", "-c", SERVER], input.as_bytes());
    assert_eq!(code, 0, "{err}");
    assert!(
        String::from_utf8_lossy(&out).contains("THE ANSWER TEXT"),
        "the answer still came back"
    );
    assert!(
        err.contains("could not be written down") && err.contains("1 call was not written down"),
        "{err}"
    );
}

#[test]
fn the_server_s_exit_code_is_av_s_and_a_server_that_cannot_be_started_exits_3() {
    let app = folder("exit");
    let state = folder("exit-state");
    let (code, _, _) = proxy(&app, &state, &["sh", "-c", "exit 7"], b"");
    assert_eq!(code, 7);
    let (code, _, err) = proxy(&app, &state, &["/nowhere/no-such-server"], b"");
    assert_eq!(code, 3);
    assert!(err.contains("could not be started"), "{err}");
}

#[test]
fn av_proxy_s_own_options_end_at_the_two_dashes_and_the_rest_is_the_server_s() {
    let app = folder("options");
    let state = folder("options-state");
    // `--app` after the dashes belongs to the server: here, to `sh`, which runs `exit 5` and ignores it.
    let (code, _, err) = proxy(
        &app,
        &state,
        &["sh", "-c", "exit 5", "--app", "/elsewhere"],
        b"",
    );
    assert_eq!(code, 5, "{err}");
    let run = |args: &[&str]| {
        let out = Command::new(env!("CARGO_BIN_EXE_av"))
            .args(args)
            .output()
            .unwrap();
        (
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).into_owned(),
        )
    };
    for (args, why) in [
        (vec!["proxy", "sh"], "needs `--`"),
        (vec!["proxy", "--"], "needs the tool server's command"),
        (
            vec!["proxy", "--record-everything", "--", "sh"],
            "does not know --record-everything",
        ),
        (
            vec!["proxy", "--app", "/nowhere/at/all", "--", "sh"],
            "is not a folder av can open",
        ),
    ] {
        let (code, err) = run(&args);
        assert_eq!(code, Some(3), "{args:?}: {err}");
        assert!(err.contains(why), "{args:?}: {err}");
    }
}
