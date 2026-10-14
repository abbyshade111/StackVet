//! What the server says when it cannot do a tool's job names what the AI coding tool can do, not only what a person
//! types at a terminal (backlog 0475): a tool told to "run `sv init`" or to give "--out" cannot.
use super::tests::{call, text};
use super::*;

/// An app folder with code and no stackvet.toml, under a root of its own.
fn no_manifest(tag: &str) -> PathBuf {
    let root = sv_check::test_support::scratch(&format!("terminal-words-{tag}"));
    std::fs::create_dir_all(root.join("app")).unwrap();
    std::fs::write(root.join("app/app.py"), "print('hi')\n").unwrap();
    root
}

#[test]
fn every_tool_that_needs_stackvet_toml_names_stackvet_spec() {
    let root = no_manifest("spec");
    let server = Server::new(&root).unwrap();
    for tool in [
        "stackvet_check",
        "stackvet_write_report",
        "stackvet_bundle",
        "stackvet_plan",
        "stackvet_preflight",
        "stackvet_status",
    ] {
        let result = call(&server, tool, json!({ "path": "app" }));
        let said = text(&result);
        assert!(
            said.contains("stackvet.toml"),
            "{tool} says nothing of the file: {said}"
        );
        assert!(
            said.contains("stackvet_spec"),
            "{tool} does not name stackvet_spec: {said}"
        );
    }
}

#[test]
fn a_report_folder_of_someone_elses_names_out_for_the_tool() {
    let root = no_manifest("out");
    std::fs::copy(
        super::tests::examples().join("flask-booking/stackvet.toml"),
        root.join("app/stackvet.toml"),
    )
    .unwrap();
    std::fs::create_dir_all(root.join("app/docs")).unwrap();
    std::fs::write(root.join("app/docs/notes.md"), "The team's own notes.\n").unwrap();
    let server = Server::new(&root).unwrap();
    let result = call(
        &server,
        "stackvet_write_report",
        json!({ "path": "app", "out": "docs" }),
    );
    assert_eq!(result["isError"], true, "the setup: the folder is refused");
    let said = text(&result);
    assert!(
        said.contains("already holds files sv did not write"),
        "{said}"
    );
    assert!(
        said.contains("`out` in stackvet_write_report"),
        "the tool is told only the terminal's --out: {said}"
    );
    // The app's own file is untouched.
    assert_eq!(
        std::fs::read_to_string(root.join("app/docs/notes.md")).unwrap(),
        "The team's own notes.\n"
    );
}
