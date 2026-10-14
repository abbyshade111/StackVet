//! Since ADR-111 the outside tools run with every `sv report` at a terminal unless `--no-tools` is given, and
//! `--tools` changes nothing. Three sentences of the report still said they ran "only with `sv report --tools`",
//! in exactly the runs where they had not run (backlog 0429). Nothing the report writes names `--tools` now.

use std::path::{Path, PathBuf};

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn the_report_never_tells_anyone_to_give_tools() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    // The setup: the files that write the "Outside tools" section are among those read.
    for file in ["lib.rs", "markdown.rs", "html.rs"] {
        assert!(
            files.iter().any(|f| f.ends_with(file)),
            "the setup: {file} is not in {}",
            src.display()
        );
    }
    let mut said = Vec::new();
    for file in &files {
        let code = std::fs::read_to_string(file).unwrap();
        for (n, line) in code.lines().enumerate() {
            if line.replace("--no-tools", "").contains("--tools") {
                said.push(format!("{}:{}: {}", file.display(), n + 1, line.trim()));
            }
        }
    }
    assert!(
        said.is_empty(),
        "the outside tools run unless --no-tools (ADR-111); `--tools` does nothing:\n{}",
        said.join("\n")
    );
}
