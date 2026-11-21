//! Every place that checks the published image's signature, or tells a person how to, requires it to have been signed
//! from `main` (backlog 0711). The publish job's `if:` and its Environment are in a workflow file a branch can change,
//! so only the signature's own record of the branch keeps an image built elsewhere from passing.
//!
//! The design entries, the backlog, and the decision records are history and are not read: they say what was true
//! when they were written.

use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn every_image_signature_check_requires_main() {
    let listed = Command::new("git")
        .current_dir(root())
        .args([
            "grep",
            "-l",
            "-e",
            "gh attestation verify oci://",
            "-e",
            "gh attestation verify \"oci://",
            "--",
            ".",
            ":!docs/design",
            ":!docs/backlog",
            ":!docs/adr",
            ":!crates/sv-cli/tests/image_signed_from_main.rs",
        ])
        .output()
        .expect("git runs");
    let files: Vec<String> = String::from_utf8_lossy(&listed.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    // The setup: the Action, the README, and the publish job's comment are found, so a search that read nothing
    // cannot pass.
    for expected in ["action.yml", "README.md", ".github/workflows/rust.yml"] {
        assert!(
            files.iter().any(|f| f == expected),
            "{expected} not found among {files:?}"
        );
    }
    for file in &files {
        let text = std::fs::read_to_string(root().join(file)).unwrap();
        // Each check, to the end of its command (a `\` continues it onto the next line, a comment's on the next too).
        for (at, _) in text.match_indices("gh attestation verify") {
            let rest = &text[at..];
            let end = rest
                .match_indices('\n')
                .map(|(i, _)| i)
                .find(|&i| {
                    let line_end = rest[..i].trim_end();
                    !line_end.ends_with('\\') && !rest[i + 1..].trim_start().starts_with("# --")
                })
                .unwrap_or(rest.len());
            let command = &rest[..end];
            if !command.contains("oci://") {
                continue;
            }
            assert!(
                command.contains("--source-ref refs/heads/main"),
                "{file}: an image signature check that does not require main: {command}"
            );
        }
    }
}
