//! Backlog 0375 (ADR-153): the server's wire turns paths both ways when it is told the Windows folder: a Windows file
//! address it is sent is read inside, and the path in its answer goes back in Windows form. Told nothing, it is as
//! it was.

use super::host_paths::HostPaths;
use super::protocol::serve_with;
use super::*;

fn answer(host: Option<HostPaths>, root: &Path, request: &str) -> String {
    let server = Server::new(root).unwrap();
    let mut out = Vec::new();
    serve_with(
        &server,
        std::io::Cursor::new(format!("{request}\n")),
        &mut out,
        host,
    )
    .unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn a_windows_path_is_read_inside_and_the_answer_names_the_folder_in_windows_form() {
    let root = std::env::temp_dir().join(format!("sv-mcp-host-paths-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let root = sv_frameworks::paths::canonical(&root).unwrap();
    let inside = root.display().to_string();
    // A folder that is not there, under the Windows root: refused, with the root named.
    let request = r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"stackvet_status","arguments":{"path":"C:\\Users\\you\\app\\nowhere-sv"}}}"#;
    let told = answer(
        Some(HostPaths::given(&inside, r"C:\Users\you\app")),
        &root,
        request,
    );
    let plain = answer(None, &root, request);
    std::fs::remove_dir_all(&root).ok();
    // The setup: told nothing, the refusal names the folder inside, so there is something to turn.
    assert!(plain.contains(&inside), "{plain}");
    // Told the folder: the path was read under the root, and the answer names the folder only in the person's form.
    assert!(
        !told.contains(&inside),
        "the inside path went back as it was: {told}"
    );
    assert!(told.contains(r"C:\\Users\\you\\app"), "{told}");
    assert!(told.contains("nowhere-sv"), "{told}");
}

#[test]
fn a_folder_named_by_its_windows_path_is_found_under_the_root() {
    let root = std::env::temp_dir().join(format!("sv-mcp-host-paths-found-{}", std::process::id()));
    std::fs::create_dir_all(root.join("sub")).unwrap();
    let root = sv_frameworks::paths::canonical(&root).unwrap();
    let inside = root.display().to_string();
    let request = r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"stackvet_status","arguments":{"path":"C:\\Users\\you\\app\\sub"}}}"#;
    let told = answer(
        Some(HostPaths::given(&inside, r"C:\Users\you\app")),
        &root,
        request,
    );
    let plain = answer(None, &root, request);
    std::fs::remove_dir_all(&root).ok();
    // The setup: told nothing, a Windows path is no folder inside the root.
    assert!(plain.contains("is not a folder inside"), "{plain}");
    // Told the folder, it is the folder `sub` under the root, and is read.
    assert!(!told.contains("is not a folder inside"), "{told}");
}
