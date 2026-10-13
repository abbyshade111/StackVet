//! No secret is written into the text of a script in this repository's own workflows (backlog 0318): each
//! is handed to its step through `env:` and read from the environment. A secret in a `run:` block is part
//! of the script GitHub writes to disk and runs, and a shell error that echoes a line shows it before the
//! log's masking catches up. The Docker Hub sign-in steps were written this way; the publish step's
//! sign-in to ghcr.io was not until 13 October 2026.

use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every line of every `run:` script in `text`, with its line number: a `run:` line's own text after
/// the key, and each line after it indented deeper than the key.
fn script_lines(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut inside: Option<usize> = None;
    for (n, line) in text.lines().enumerate() {
        let indent = line.len() - line.trim_start().len();
        if let Some(key_indent) = inside {
            if line.trim().is_empty() || indent > key_indent {
                out.push((n + 1, line.to_owned()));
                continue;
            }
            inside = None;
        }
        let trimmed = line.trim_start().trim_start_matches("- ");
        if let Some(rest) = trimmed.strip_prefix("run:") {
            let key_indent = line.len() - trimmed.len();
            out.push((n + 1, rest.to_owned()));
            inside = Some(key_indent);
        }
    }
    out
}

fn secrets_in_scripts(text: &str) -> Vec<String> {
    script_lines(text)
        .into_iter()
        .filter(|(_, l)| {
            let compact: String = l.chars().filter(|c| !c.is_whitespace()).collect();
            compact.contains("${{secrets.") || compact.contains("${{github.token}}")
        })
        .map(|(n, l)| format!("line {n}: {}", l.trim()))
        .collect()
}

#[test]
fn the_reader_finds_a_secret_written_into_a_script() {
    // The setup: the reader sees a secret in a block script and in a one-line one, and none in `env:`.
    let workflow = "jobs:\n  a:\n    steps:\n      - name: one\n        env:\n          T: ${{ secrets.X }}\n        \
                    run: |\n          echo hi\n          echo \"${{ secrets.Y }}\" | tool\n      - run: echo ${{ github.token }}\n";
    let found = secrets_in_scripts(workflow);
    assert_eq!(found.len(), 2, "{found:#?}");
    assert!(found[0].starts_with("line 9:"), "{found:#?}");
}

#[test]
fn no_workflow_writes_a_secret_into_a_script() {
    let mut files: Vec<PathBuf> = std::fs::read_dir(repo().join(".github/workflows"))
        .expect("the workflows folder")
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "yml" || e == "yaml"))
        .collect();
    files.push(repo().join("action.yml"));
    // The setup: the workflows were found, the one with the publish step among them.
    assert!(files.iter().any(|f| f.ends_with("rust.yml")), "{files:?}");
    let mut found = Vec::new();
    for file in &files {
        let text = std::fs::read_to_string(file).unwrap().replace("\r\n", "\n");
        for hit in secrets_in_scripts(&text) {
            found.push(format!("{}: {hit}", file.file_name().unwrap().to_string_lossy()));
        }
    }
    assert!(found.is_empty(), "hand these to the step through env: instead\n{found:#?}");
}
