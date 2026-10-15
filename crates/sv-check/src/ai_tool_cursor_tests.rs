//! Cursor's `.cursor/mcp.json` is read as `.mcp.json` is (backlog 0534): Cursor's MCP page
//! (https://cursor.com/docs/mcp, kept in `docs/research/CURSOR-MCP-2026-10-15.md`) gives it the same
//! `mcpServers` block, each server with `command` and `args` or with `url`. Cursor's other files are
//! still named and not read: that page does not describe them.

use super::tests::rules;
use super::*;
use std::fs;

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "sv-ai-tool-cursor-{name}-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &std::path::Path, relative: &str, text: &str) {
    let path = dir.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// The three kinds of server on Cursor's page: one started unpinned, one started pinned, and one
/// connected to, its header written with Cursor's `${env:NAME}` in place of a key.
const SERVERS: &str = r#"{
  "mcpServers": {
    "gh": {"command": "npx", "args": ["-y", "@modelcontextprotocol/server-github"]},
    "fetch": {"type": "stdio", "command": "uvx", "args": ["mcp-server-fetch==2025.1.17"],
              "envFile": "${workspaceFolder}/.env"},
    "remote": {"url": "https://mcp.example.test/mcp",
               "headers": {"Authorization": "Bearer ${env:MY_SERVICE_TOKEN}"}}
  }
}"#;

#[test]
fn each_server_in_cursor_s_file_is_named_as_in_claude_code_s() {
    let dir = scratch("servers");
    write(&dir, ".cursor/mcp.json", SERVERS);
    let files = read(&Listing::of(&dir), rules());
    assert_eq!(files.read, [".cursor/mcp.json"], "{files:?}");
    assert!(files.not_read.is_empty(), "{:?}", files.not_read);
    assert_eq!(files.notes.len(), 3, "{:?}", files.notes);
    assert!(
        files
            .notes
            .iter()
            .all(|n| n.tool == "Cursor" && n.file == ".cursor/mcp.json")
    );
    let note = |name: &str| {
        files
            .notes
            .iter()
            .find(|n| n.what.contains(&format!("`{name}`")))
            .unwrap_or_else(|| panic!("no note for {name}: {:?}", files.notes))
    };
    let gh = note("gh");
    assert!(
        gh.what.contains("which names no exact version of"),
        "{}",
        gh.what
    );
    assert_eq!(gh.risk, "AST07");
    let fetch = note("fetch");
    assert_eq!(
        fetch.what,
        "starts the MCP server `fetch` with `uvx mcp-server-fetch==2025.1.17`"
    );
    assert_eq!(fetch.risk, "AST02");
    let remote = note("remote");
    assert_eq!(
        remote.what,
        "connects to the MCP server `remote` at `https://mcp.example.test/mcp`"
    );
    assert_eq!(remote.risk, "AST02");
}

#[test]
fn cursor_s_file_says_nothing_of_starting_servers_without_asking() {
    // Cursor's page: "Cursor asks for approval before using MCP tools by default", and its file
    // has no setting like Claude Code's `enableAllProjectMcpServers`, so no note says otherwise.
    let dir = scratch("asking");
    write(&dir, ".cursor/mcp.json", SERVERS);
    let files = read(&Listing::of(&dir), rules());
    assert_eq!(files.notes.len(), 3, "{:?}", files.notes);
    assert!(
        files
            .notes
            .iter()
            .all(|n| !n.what.contains("without asking")),
        "{:?}",
        files.notes
    );
}

#[test]
fn a_cursor_file_under_a_folder_is_read_and_the_others_beside_it_are_not() {
    let dir = scratch("beside");
    write(
        &dir,
        "web/.cursor/mcp.json",
        r#"{"mcpServers": {"gh": {"command": "npx", "args": ["-y", "pkg@1.2.3"]}}}"#,
    );
    write(&dir, "web/.cursor/rules/style.mdc", "Use tabs.");
    let files = read(&Listing::of(&dir), rules());
    assert_eq!(files.read, ["web/.cursor/mcp.json"], "{files:?}");
    assert_eq!(files.notes.len(), 1, "{:?}", files.notes);
    assert_eq!(files.not_read.len(), 1, "{:?}", files.not_read);
    assert!(
        files.not_read[0].starts_with("`web/.cursor/rules/style.mdc` (Cursor's documentation"),
        "{:?}",
        files.not_read
    );
}

#[test]
fn a_cursor_file_that_is_not_json_is_named_as_not_read() {
    let dir = scratch("broken");
    write(&dir, ".cursor/mcp.json", r#"{"mcpServers": {"gh": "#);
    let files = read(&Listing::of(&dir), rules());
    assert!(files.read.is_empty() && files.notes.is_empty(), "{files:?}");
    assert_eq!(
        files.not_read,
        ["`.cursor/mcp.json` (it is not JSON `sv` could read)"]
    );
}
