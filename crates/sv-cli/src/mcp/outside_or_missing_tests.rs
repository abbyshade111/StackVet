//! `resources/read` gives one answer for a folder outside the root and one that does not exist (backlog 0308), as
//! `app_dir` does for a tool's path. Two answers would let whoever asks learn which folders exist anywhere on the
//! computer: existence only, never contents, but the server's own rule is one answer.
use super::tests::{read, scratch_app};
use super::*;

/// The error message of reading `path`, with the URI it quotes taken out.
fn refusal(server: &Server, path: &Path) -> String {
    let uri = file_uri(path).unwrap();
    let reply = read(server, &uri);
    assert_eq!(reply["error"]["code"], RESOURCE_NOT_FOUND, "{uri}: {reply}");
    reply["error"]["message"]
        .as_str()
        .unwrap()
        .replace(&uri, "<uri>")
}

#[test]
fn a_folder_outside_the_root_and_one_that_is_not_there_get_the_same_answer() {
    let root = scratch_app("outside-or-missing", "flask-booking");
    let outside = std::env::temp_dir().join(format!(
        "sv-mcp-outside-or-missing-elsewhere-{}",
        std::process::id()
    ));
    std::fs::remove_dir_all(&outside).ok();
    std::fs::create_dir_all(&outside).unwrap();
    let missing = outside.with_file_name(format!(
        "{}-not-there",
        outside.file_name().unwrap().to_string_lossy()
    ));
    // The setup: one folder really is there, outside the root, and the other really is not.
    assert!(outside.is_dir() && !outside.starts_with(&root));
    assert!(!missing.exists());
    let server = Server::new(&root).unwrap();

    let there = refusal(&server, &outside.join("report.json"));
    let not_there = refusal(&server, &missing.join("report.json"));
    assert_eq!(there, not_there);
    // A folder inside the root that is not there gets the same answer too.
    let inside = refusal(&server, &root.join("app/no-such-folder/report.json"));
    assert_eq!(inside, there);
    std::fs::remove_dir_all(&outside).ok();
    std::fs::remove_dir_all(&root).ok();
}
