//! `tools/long_files.py` (backlog 0662; ADR-152): the weekly list of long files and functions counts a file's lines
//! and a function's by its braces, with braces in strings, characters, and comments not counted, and lists test code
//! apart. The two numbers are read from the script, so the test and the job never disagree on them.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn script(args: &[&str]) -> serde_json::Value {
    let out = Command::new("python3")
        .arg("-I")
        .arg(root().join("tools/long_files.py"))
        .args(args)
        .output()
        .expect("python3 runs the script");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("the script prints JSON")
}

/// A function named `name` that runs to exactly `lines` lines, its body full of braces that are not code.
fn function(name: &str, lines: usize) -> String {
    let tricky = [
        "    let a = \"{ not a brace }\";",
        "    let b = '{';",
        "    let c = '}';",
        "    let d = r#\"}}} \"quoted\" {\"#;",
        "    // a comment with a brace {",
        "    /* a block } comment, /* nested { */ still a comment } */",
        "    let e: &'static str = \"\\\"}\";",
        "    let f = b'{';",
    ];
    let mut out = vec![format!("fn {name}<'a>(x: &'a str) -> &'a str {{")];
    out.extend(tricky.iter().map(|l| (*l).to_owned()));
    while out.len() < lines - 2 {
        out.push("    let _ = 1;".to_owned());
    }
    out.push("    x".to_owned());
    out.push("}".to_owned());
    assert_eq!(out.len(), lines);
    out.join("\n") + "\n"
}

fn write(dir: &Path, relative: &str, text: &str) {
    let path = dir.join(relative);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// `text` padded with plain lines to exactly `lines` lines.
fn padded(text: &str, lines: usize) -> String {
    let mut out: Vec<String> = text.lines().map(str::to_owned).collect();
    while out.len() < lines {
        out.push("// padding".to_owned());
    }
    out.join("\n") + "\n"
}

#[test]
fn the_list_counts_lines_and_braces_as_a_person_would_and_lists_test_code_apart() {
    let numbers = script(&["--thresholds"]);
    let file_lines = numbers["file_lines"].as_u64().unwrap() as usize;
    let function_lines = numbers["function_lines"].as_u64().unwrap() as usize;
    // The setup: the numbers backlog 0324 used.
    assert_eq!((file_lines, function_lines), (3000, 150));

    let dir = std::env::temp_dir().join(format!("sv-long-files-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let long = function("long_one", function_lines + 1);
    let at_the_line = function("just_at_the_line", function_lines);
    let in_tests = format!(
        "#[cfg(test)]\nmod tests {{\n    use super::*;\n{}}}\n",
        function("long_test_helper", function_lines + 1)
    );
    write(
        &dir,
        "crates/x/src/big.rs",
        &padded(&format!("{long}{at_the_line}{in_tests}"), file_lines + 1),
    );
    write(&dir, "crates/x/src/exactly.rs", &padded("", file_lines));
    write(
        &dir,
        "crates/x/src/thing_tests.rs",
        &padded("", file_lines + 5),
    );
    let found = script(&["--json", "--root", dir.to_str().unwrap()]);
    std::fs::remove_dir_all(&dir).ok();

    let files: Vec<(String, u64, bool)> = found["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["file"].as_str().unwrap().to_owned(),
                f["lines"].as_u64().unwrap(),
                f["test"].as_bool().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        files,
        [
            (
                "crates/x/src/thing_tests.rs".to_owned(),
                (file_lines + 5) as u64,
                true
            ),
            (
                "crates/x/src/big.rs".to_owned(),
                (file_lines + 1) as u64,
                false
            ),
        ],
        "a file at the line is not listed, and a test file is marked"
    );
    let functions: Vec<(String, u64, u64, bool)> = found["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["function"].as_str().unwrap().to_owned(),
                f["line"].as_u64().unwrap(),
                f["lines"].as_u64().unwrap(),
                f["test"].as_bool().unwrap(),
            )
        })
        .collect();
    let test_helper_line = (2 * function_lines + 1 + 4) as u64;
    assert_eq!(
        functions,
        [
            ("long_one".to_owned(), 1, (function_lines + 1) as u64, false),
            (
                "long_test_helper".to_owned(),
                test_helper_line,
                (function_lines + 1) as u64,
                true
            ),
        ],
        "the braces in strings, characters, and comments are not counted, a function at the line is not listed, \
         and a function in a test module is marked"
    );
}
