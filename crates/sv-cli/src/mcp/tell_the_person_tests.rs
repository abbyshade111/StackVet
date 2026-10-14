//! Backlog 0401: every check ends its summary by asking the AI coding tool to tell the person what it changed, and to
//! ask before deleting anything or silencing a finding; and says `sv` cannot see what it told them.

use super::tests::{call, scratch_app, text};
use super::*;

#[test]
fn every_check_asks_the_tool_to_tell_the_person_what_it_changed() {
    let root = scratch_app("tell-the-person", "tested-notes");
    let server = Server::new(&root).unwrap();
    let checked = call(&server, "stackvet_check", json!({ "path": "app" }));
    assert_eq!(checked["isError"], false, "{}", text(&checked));
    let said = text(&checked);
    for words in [
        "tell the person in plain words what you changed",
        "anything you deleted",
        "any finding you changed code to make go away rather than fixed",
        "Ask them before you delete anything or silence a finding",
        "sv cannot see what you tell them",
    ] {
        assert!(said.contains(words), "the check does not say {words:?}");
    }
    std::fs::remove_dir_all(&root).ok();
}
