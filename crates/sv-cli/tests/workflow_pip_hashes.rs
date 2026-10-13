//! Every Python package a workflow installs is pinned to its files' hashes (backlog 0288). `pip install
//! py7zr==1.1.3` pinned one package and left the eight it pulls in to whatever was newest on PyPI, and pip runs a
//! package's setup code as it installs it, before the tests. Each `pip install` in `.github/workflows` now reads a
//! requirements file with `--require-hashes`, and every package in that file carries at least one `--hash`.

use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The line numbers (from 1) of `pip install` commands that do not install from a requirements file with
/// `--require-hashes`, and the requirements files the others name.
fn unhashed_installs(text: &str) -> (Vec<usize>, Vec<String>) {
    let mut bad = Vec::new();
    let mut files = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let code = line.split('#').next().unwrap_or("");
        if !code.contains("pip install") {
            continue;
        }
        let words: Vec<&str> = code.split_whitespace().collect();
        let file = words
            .iter()
            .position(|w| *w == "-r" || *w == "--requirement")
            .and_then(|at| words.get(at + 1));
        match file {
            Some(file) if words.contains(&"--require-hashes") => files.push(file.to_string()),
            _ => bad.push(i + 1),
        }
    }
    (bad, files)
}

/// The packages in a requirements file that carry no `--hash`.
fn packages_without_a_hash(text: &str) -> Vec<String> {
    let mut missing = Vec::new();
    let mut current: Option<(String, bool)> = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if !line.starts_with(' ') {
            if let Some((name, false)) = current.take() {
                missing.push(name);
            }
            let name = trimmed
                .split([' ', ';', '\\'])
                .next()
                .unwrap_or("")
                .to_string();
            current = Some((name, false));
        }
        if trimmed.contains("--hash=sha256:")
            && let Some((_, hashed)) = current.as_mut()
        {
            *hashed = true;
        }
    }
    if let Some((name, false)) = current {
        missing.push(name);
    }
    missing
}

#[test]
fn the_readers_see_an_install_without_hashes() {
    let (bad, files) = unhashed_installs(
        "run: |\n  python3 -m pip install py7zr==1.1.3\n  python3 -m pip install --require-hashes -r tools/x.txt\n  pip install -r tools/y.txt\n  # pip install in a comment\n",
    );
    assert_eq!(bad, vec![2, 4]);
    assert_eq!(files, vec!["tools/x.txt".to_string()]);

    let missing = packages_without_a_hash(
        "# a comment\nalpha==1.0 \\\n    --hash=sha256:00\n    # via beta\nbeta==2.0 ; python_version < '4'\ngamma==3.0 \\\n    --hash=sha256:11\n",
    );
    assert_eq!(missing, vec!["beta==2.0".to_string()]);
}

#[test]
fn every_workflow_installs_python_packages_by_hash() {
    let dir = repo().join(".github/workflows");
    let mut installs = 0;
    let mut files = Vec::new();
    let mut failures = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("the workflows folder is readable") {
        let path = entry.expect("a workflow entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("yml") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("a workflow is readable");
        let (bad, named) = unhashed_installs(&text);
        installs += bad.len() + named.len();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        failures.extend(bad.iter().map(|line| format!("{name}:{line}")));
        files.extend(named);
    }
    // The archive tests' reader is installed on Linux and on Windows; if neither install is found, this test has
    // stopped looking at what it guards.
    assert!(
        installs >= 2,
        "found {installs} pip installs in the workflows"
    );
    assert!(
        failures.is_empty(),
        "pip installs without --require-hashes -r <file>: {failures:?}"
    );
    files.sort();
    files.dedup();
    for file in files {
        let text = std::fs::read_to_string(repo().join(&file))
            .unwrap_or_else(|e| panic!("{file}, named by a workflow, is not readable: {e}"));
        let missing = packages_without_a_hash(&text);
        assert!(missing.is_empty(), "{file}: no --hash for {missing:?}");
    }
}
