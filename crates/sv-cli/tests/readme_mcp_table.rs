//! Backlog 0480: the README lists the MCP server's tools as a table, one row each, with what each writes and
//! whether it counts toward a requirement. Held here to the tools the server really lists, asked of the server
//! itself: a tool added, renamed, or dropped without its row fails, and so does a count in words that has moved.

use std::path::{Path, PathBuf};
use std::process::Command;

const SV: &str = env!("CARGO_BIN_EXE_sv");

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The names of the tools the MCP server lists.
fn tool_names() -> Vec<String> {
    use std::io::Write;
    let root = std::env::temp_dir().join(format!("sv-readme-mcp-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let mut child = Command::new(SV)
        .args(["mcp", "--root", root.to_str().unwrap()])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    for line in [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    ] {
        writeln!(stdin, "{line}").unwrap();
    }
    drop(stdin);
    let out = child.wait_with_output().unwrap();
    std::fs::remove_dir_all(&root).ok();
    let listed = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .find(|v| v["id"] == 2)
        .expect("the server answers tools/list");
    listed["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap().to_owned())
        .collect()
}

fn in_words(n: usize) -> &'static str {
    [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ]
    .get(n)
    .copied()
    .unwrap_or("more than twenty")
}

#[test]
fn the_readme_table_has_one_row_for_each_tool_the_server_lists() {
    let mut names = tool_names();
    assert!(
        names.len() > 5,
        "the setup: the server lists its tools: {names:?}"
    );
    let readme = std::fs::read_to_string(repo().join("README.md")).unwrap();
    let start = readme
        .find("\n## From inside your AI coding tool\n")
        .expect("the README has its MCP section");
    let section = &readme[start..start + 1 + readme[start + 1..].find("\n## ").unwrap()];
    let mut rows: Vec<String> = section
        .lines()
        .filter_map(|l| l.strip_prefix("| `stackvet_"))
        .map(|rest| format!("stackvet_{}", &rest[..rest.find('`').unwrap()]))
        .collect();
    names.sort();
    rows.sort();
    assert_eq!(
        rows, names,
        "the README's table of tools is not the server's list"
    );
    assert!(
        section.contains(&format!("It offers {} tools.", in_words(names.len()))),
        "the README does not count the server's {} tools in words",
        names.len()
    );
}
