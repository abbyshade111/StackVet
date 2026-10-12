//! `sv mcp` started from a folder whose name a shell would split: the command the tool is told to
//! have the person run quotes `sv`'s path, so it works when pasted into a terminal. CI's own build
//! folder holds no space, so without this the quoting is tested only where someone's does (backlog
//! 0269). Not on Windows: there `sv` names itself by the short form of its path, which holds no
//! space, so the folder below would test nothing.
#![cfg(not(windows))]

use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn a_path_with_a_space_and_a_quote_is_quoted_in_the_command_the_tool_is_given() {
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("sv-quoted-path-{}", std::process::id()))
        .join("it's a folder");
    let _ = std::fs::remove_dir_all(folder.parent().unwrap());
    std::fs::create_dir_all(&folder).unwrap();
    let built = std::path::Path::new(env!("CARGO_BIN_EXE_sv"));
    let copy = folder.join(built.file_name().unwrap());
    std::fs::copy(built, &copy).expect("sv copies into the folder");

    let examples = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut child = Command::new(&copy)
        .args(["mcp", "--root"])
        .arg(&examples)
        .env("SV_BUILD_LOOP_RECORD", "off")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the copied sv starts");
    let hello = json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"0"}}});
    {
        let stdin = child.stdin.as_mut().unwrap();
        writeln!(stdin, "{hello}").unwrap();
    }
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let reply: Value = serde_json::from_str(
        stdout
            .lines()
            .next()
            .unwrap_or_else(|| panic!("no reply: {}", String::from_utf8_lossy(&output.stderr))),
    )
    .unwrap();
    let instructions = reply["result"]["instructions"].as_str().unwrap();

    let program = sv_frameworks::paths::canonical(&copy).unwrap();
    let shown = program.to_string_lossy();
    // The setup is what it claims: the path really holds what a shell would split or end on.
    assert!(shown.contains(' ') && shown.contains('\''), "{shown}");
    let quoted = format!("'{}'", shown.replace('\'', "'\\''"));
    assert!(
        instructions.contains(&format!("`{quoted} report ")),
        "{instructions}"
    );
    // And never bare, which a shell would cut at the space.
    assert!(
        !instructions.contains(&format!("`{shown} report ")),
        "{instructions}"
    );
    let _ = std::fs::remove_dir_all(folder.parent().unwrap());
}
