//! What the pass-through's record keeps of a call and its answer (backlog 0716, part 1).

use super::*;

fn rules() -> &'static sv_check::secrets::SecretRules {
    sv_check::secrets::SecretRules::shared().expect("the rules sv ships")
}

fn call(id: &str, tool: &str, arguments: Value) -> Vec<u8> {
    json!({ "jsonrpc": "2.0", "id": serde_json::from_str::<Value>(id).unwrap(), "method": "tools/call",
            "params": { "name": tool, "arguments": arguments } })
    .to_string()
    .into_bytes()
}

#[test]
fn a_call_to_a_tool_is_read_with_a_summary_and_a_fingerprint_of_its_arguments() {
    let (id, c) = call_in(
        &call("7", "read_file", json!({ "path": "src/app.py" })),
        rules(),
    )
    .unwrap();
    assert_eq!(id, "7");
    assert_eq!(c.tool, "read_file");
    assert_eq!(c.summary, r#"{"path":"src/app.py"}"#);
    assert_eq!(c.args_bytes, c.summary.len());
    assert_eq!(c.args_sha256, crate::plain::sha256(c.summary.as_bytes()));
    // An id that is a string is a different key from the same digits as a number.
    let (id, _) = call_in(&call("\"7\"", "read_file", json!({})), rules()).unwrap();
    assert_eq!(id, "\"7\"");
}

#[test]
fn what_is_not_a_call_to_a_tool_is_not_recorded() {
    for line in [
        json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list" }).to_string(),
        json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {} }).to_string(),
        // A notice has no id: nothing will answer it.
        json!({ "jsonrpc": "2.0", "method": "tools/call", "params": { "name": "x" } }).to_string(),
        json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {} }).to_string(),
        "not json at all".to_owned(),
        "[1, 2, 3]".to_owned(),
        String::new(),
    ] {
        assert_eq!(call_in(line.as_bytes(), rules()), None, "{line}");
    }
}

#[test]
fn a_key_in_the_arguments_is_cut_and_the_summary_is_one_short_line() {
    // Built from pieces, so no key is written here whole.
    let key = format!(
        "{}{}",
        "sk-ant-api03-", "A1b2C3d4E5f6G7h8I9j0K1l2M3n4O5p6Q7r8S9t0U1v2W3x4Y5z6"
    );
    let (_, c) = call_in(
        &call(
            "1",
            "http_post",
            json!({ "url": "https://example.com", "password": key, "note": "a\nb\u{1b}[31m" }),
        ),
        rules(),
    )
    .unwrap();
    assert!(
        !c.summary.contains(&key[10..]),
        "the key is in the summary: {}",
        c.summary
    );
    assert!(!c.summary.chars().any(char::is_control), "{:?}", c.summary);
    // The fingerprint is of the arguments as they were, so two calls with the same arguments match.
    assert_eq!(c.args_sha256.len(), 64);
    // Long arguments are cut, and the size says how long they were.
    let long = "x".repeat(5000);
    let (_, c) = call_in(
        &call("2", "write_file", json!({ "content": long })),
        rules(),
    )
    .unwrap();
    assert!(
        c.summary.chars().count() < SUMMARY_CHARS + 40,
        "{}",
        c.summary.len()
    );
    assert!(c.args_bytes > 5000);
    assert!(!c.summary.contains(&"x".repeat(SUMMARY_CHARS + 1)));
}

#[test]
fn an_answer_is_ok_a_tool_s_own_failure_or_a_refusal_with_the_server_s_number() {
    for (line, answer) in [
        (
            json!({ "jsonrpc": "2.0", "id": 7, "result": { "content": [] } }),
            Answer::Ok,
        ),
        (
            json!({ "jsonrpc": "2.0", "id": 7, "result": { "content": [], "isError": false } }),
            Answer::Ok,
        ),
        (
            json!({ "jsonrpc": "2.0", "id": 7, "result": { "content": [], "isError": true } }),
            Answer::ToolError,
        ),
        (
            json!({ "jsonrpc": "2.0", "id": 7, "error": { "code": -32001, "message": "not allowed" } }),
            Answer::Refused { code: -32001 },
        ),
    ] {
        assert_eq!(
            answer_in(line.to_string().as_bytes()),
            Some(("7".to_owned(), answer)),
            "{line}"
        );
    }
    for line in [
        // The server's own request and notice, and what is not a message.
        json!({ "jsonrpc": "2.0", "id": 3, "method": "sampling/createMessage" }).to_string(),
        json!({ "jsonrpc": "2.0", "method": "notifications/progress" }).to_string(),
        json!({ "jsonrpc": "2.0", "id": null, "error": { "code": -32700 } }).to_string(),
        "garbage".to_owned(),
    ] {
        assert_eq!(answer_in(line.as_bytes()), None, "{line}");
    }
}

#[test]
fn a_record_s_line_holds_the_answer_and_never_its_text() {
    let (_, c) = call_in(&call("1", "read_file", json!({ "path": "a" })), rules()).unwrap();
    let ok = line_of("T", &c, Some(Answer::Ok));
    assert_eq!(ok["answer"], "ok");
    assert_eq!(ok["tool"], "read_file");
    assert!(ok.get("code").is_none());
    let refused = line_of("T", &c, Some(Answer::Refused { code: -32001 }));
    assert_eq!(
        (refused["answer"].as_str(), refused["code"].as_i64()),
        (Some("refused"), Some(-32001))
    );
    assert_eq!(line_of("T", &c, None)["answer"], "unanswered");
    let keys: Vec<&str> = ok.as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        [
            "answer",
            "args_bytes",
            "args_sha256",
            "at",
            "summary",
            "tool"
        ],
        "nothing else is kept"
    );
}

#[test]
fn a_line_past_the_most_read_is_not_read_as_a_call() {
    let big = call(
        "1",
        "write_file",
        json!({ "content": "x".repeat(MAX_LINE) }),
    );
    assert!(big.len() > MAX_LINE);
    assert_eq!(call_in(&big, rules()), None);
}
