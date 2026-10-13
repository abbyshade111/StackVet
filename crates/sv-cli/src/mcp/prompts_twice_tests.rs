//! Backlog 0362: `stackvet_guidance` and `stackvet_before` give each prompt's text in their words and again in their
//! structured result, on purpose. A client reads one or the other, and a coding prompt cannot be fetched by its id,
//! so a client that reads only the structured result must find every prompt's text there.

use super::tests::{call, scratch_app, text};
use super::*;

/// Every prompt in `part` of a structured result: each has its text, and the words name it too.
fn prompts_whole(result: &Value, part: &str) -> usize {
    let prompts = result["structuredContent"][part]
        .as_array()
        .cloned()
        .unwrap_or_default();
    for p in &prompts {
        let body = p["text"].as_str().unwrap_or("");
        assert!(
            !body.trim().is_empty(),
            "{part}: {} has no text: {p}",
            p["id"]
        );
        let title = p["title"].as_str().unwrap_or("");
        assert!(
            text(result).contains(title),
            "{part}: the words do not name {title:?}"
        );
    }
    prompts.len()
}

#[test]
fn a_client_reading_the_structured_result_gets_every_prompts_text() {
    let root = scratch_app("prompts-twice", "flask-booking");
    let server = Server::new(&root).unwrap();
    let guidance = call(&server, "stackvet_guidance", json!({ "path": "app" }));
    let brief = call(
        &server,
        "stackvet_before",
        json!({ "path": "app", "feature": "sign-in" }),
    );
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(guidance["isError"], false, "{guidance}");
    assert_eq!(brief["isError"], false, "{brief}");
    let given = prompts_whole(&guidance, "prompts")
        + prompts_whole(&brief, "prompts")
        + prompts_whole(&brief, "codingPrompts");
    // Prompts were given, so the checks above are about something.
    assert!(given > 0, "no prompt in either answer");
}
