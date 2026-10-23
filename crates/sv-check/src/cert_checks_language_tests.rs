//! Backlog 0645, ADR-137: a switch-off written for one language is read in files of that language and in files of no
//! programming language of their own, and not in a file of another, where it is text.

use super::*;
use crate::test_support::scratch;

fn found_in(dir: &std::path::Path) -> Vec<String> {
    let mut report = ConfigReport::default();
    check(&Listing::of(dir), &mut report);
    let mut files: Vec<String> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == CHECKS_OFF)
        .map(|f| f.location.file.clone())
        .collect();
    files.sort();
    files
}

#[test]
fn a_switch_off_counts_where_its_language_runs_and_not_in_another_languages_text() {
    let dir = scratch("cert-languages");
    let python = "ssl._create_default_https_context = ssl._create_unverified_context\n";
    let node = "https.globalAgent.options.rejectUnauthorized = false\n";
    for (file, text) in [
        // Where each runs.
        ("app.py", format!("import ssl\n{python}")),
        ("server.js", node.to_owned()),
        ("server.ts", node.to_owned()),
        (
            "start.sh",
            format!("#!/bin/sh\npython -c \"import ssl; {}\"\n", python.trim()),
        ),
        (
            "Dockerfile",
            format!(
                "FROM python:3.12\nRUN python -c \"import ssl; {}\"\n",
                python.trim()
            ),
        ),
        // Text in a file of another language: a scanner's own table of what it looks for.
        (
            "src/patterns.rs",
            format!("const PY: &str = \"{}\";\n", python.trim()),
        ),
        (
            "src/Patterns.java",
            format!("String node = \"{}\";\n", node.trim()),
        ),
        ("tools/check.py", format!("NODE = \"{}\"\n", node.trim())),
    ] {
        let path = dir.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    assert_eq!(
        found_in(&dir),
        ["Dockerfile", "app.py", "server.js", "server.ts", "start.sh"]
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn deno_s_flag_is_a_command_line_flag_and_counts_in_any_file() {
    let dir = scratch("cert-deno");
    std::fs::write(
        dir.join("main.ts"),
        "const args = [\"run\", \"--unsafely-ignore-certificate-errors\", \"app.ts\"];\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("run.py"),
        "subprocess.run([\"deno\", \"run\", \"--unsafely-ignore-certificate-errors\", \"app.ts\"])\n",
    )
    .unwrap();
    assert_eq!(found_in(&dir), ["main.ts", "run.py"]);
    std::fs::remove_dir_all(&dir).ok();
}
