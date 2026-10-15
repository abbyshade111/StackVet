//! Backlog 0543: `stackvet_check` tells the AI coding tool, before anything else, which lines in stackvet.toml only it
//! vouches for, and what each counts for; and says none of it when there are none.

use super::tests::{call, scratch_app, text};
use super::*;

/// What `stackvet_check` says of an app whose stackvet.toml has `tables` added.
fn checked_with(name: &str, tables: &str) -> String {
    let root = scratch_app(name, "tested-notes");
    let toml = root.join("app/stackvet.toml");
    let mut manifest = std::fs::read_to_string(&toml).unwrap();
    assert!(
        !manifest.contains("[design]") && !manifest.contains("[checked-by-hand]"),
        "the example grew answers of its own; this test would be adding to them"
    );
    manifest.push_str(tables);
    std::fs::write(&toml, manifest).unwrap();
    let server = Server::new(&root).unwrap();
    let checked = call(&server, "stackvet_check", json!({ "path": "app" }));
    assert_eq!(checked["isError"], false, "{}", text(&checked));
    std::fs::remove_dir_all(&root).ok();
    text(&checked).to_owned()
}

#[test]
fn the_check_names_each_line_the_tool_vouched_for_itself() {
    let said = checked_with(
        "self-confirmed",
        r#"
[design]
"V4.1.3" = { answer = "yes", by = "ai-tool", confirmed = { by = "ai-tool", on = "2026-10-14", how = "Read the code." } }

[checked-by-hand]
"V12.1.2" = { result = "done", on = "2026-10-14", by = "ai-tool", how = "Tried it; it held." }
"#,
    );
    let not_examined = said
        .find("NOT EXAMINED")
        .expect("the check has a not-examined section");
    let line = said
        .find("2 lines in stackvet.toml that only your AI coding tool vouches for")
        .unwrap_or_else(|| panic!("the line is not said: {said}"));
    assert!(line > not_examined, "it is among what was not examined");
    for words in [
        "A confirmation by the AI coding tool counts for nothing",
        "Only `sv review`, run by the person in their own terminal",
        "counts only as the tool's word, never as a check",
        "[design] V4.1.3, confirmed by the AI coding tool",
        "[checked-by-hand] V12.1.2, checked by hand by the AI coding tool",
    ] {
        assert!(
            said.contains(words),
            "the check does not say {words:?}: {said}"
        );
    }
}

#[test]
fn nothing_is_said_of_it_without_such_lines() {
    // The control: the tool's own answer, without a confirmation, is its word as always, and not this line.
    let said = checked_with(
        "not-self-confirmed",
        "\n[design]\n\"V4.1.3\" = { answer = \"yes\", by = \"ai-tool\" }\n",
    );
    assert!(
        said.contains("1 the AI coding tool answered yes to"),
        "the setup reached the check: {said}"
    );
    assert!(
        !said.contains("only your AI coding tool vouches for"),
        "{said}"
    );
}
