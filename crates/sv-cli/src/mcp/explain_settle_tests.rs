//! Backlog 0426: `stackvet_explain` says what kind of run could settle a requirement, so the AI coding tool can answer
//! "what would it take?" without a terminal, and says it is what could, not a credit.

use super::tests::{call, scratch_app, text};
use super::*;

#[test]
fn explain_says_what_could_settle_a_requirement() {
    let root = scratch_app("explain-settle", "tested-notes");
    let server = Server::new(&root).unwrap();
    // One only the running app reaches, and one no check of sv's reaches.
    let running = call(&server, "stackvet_explain", json!({ "id": "C2.1.2" }));
    assert_eq!(running["isError"], false, "{}", text(&running));
    let said = running["structuredContent"]["couldSettle"]
        .as_str()
        .unwrap();
    assert!(
        said.contains("a check of the running app (`sv report --run`)"),
        "{said}"
    );
    assert!(text(&running).contains("What could settle it:"));
    assert!(text(&running).contains("not a credit"));

    let none = call(&server, "stackvet_explain", json!({ "id": "V8.3.1" }));
    let said = none["structuredContent"]["couldSettle"].as_str().unwrap();
    assert!(said.starts_with("no check of sv's speaks to it"), "{said}");
    std::fs::remove_dir_all(&root).ok();
}
