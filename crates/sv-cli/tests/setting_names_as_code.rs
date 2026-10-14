//! A `stackvet.toml` table named in the reports' fixed wording is written as code, `` `[stack.run.users]` ``, not as
//! plain text (backlog 0406). The Markdown reports escape a bracket outside code so an app's text cannot make a link,
//! and some viewers then show `\[stack.run.users\]` wrongly: Antigravity's showed "under stack.run.users" three times
//! over, in the one sentence telling the person how to get the sign-in checks to run. Code is shown as written by
//! every viewer, and nothing in it needs escaping. The app's own text is still escaped (`markdown::inert`).

use std::path::{Path, PathBuf};
use std::process::Command;

/// The tables of `stackvet.toml`, the first part of each name.
const TABLES: &[&str] = &[
    "stack",
    "app",
    "data",
    "policy",
    "capabilities",
    "design",
    "checked-by-hand",
    "finding-review",
    "tests",
    "tools",
    "ai",
    "exclude",
    "manifest",
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Each table name in `line` written as plain text in a sentence: after a space or `(`, not already in code, and not a
/// TOML example (a header ending a line, or one followed by `key =`). `next` is the line after, for a name that ends
/// a line continued onto it.
fn bare_names(line: &str, next: &str) -> Vec<String> {
    let mut found = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while let Some(at) = line[i..].find('[').map(|a| a + i) {
        i = at + 1;
        if at == 0 || !matches!(bytes[at - 1], b' ' | b'(') {
            continue;
        }
        let open = if line[at..].starts_with("[[") { 2 } else { 1 };
        let name_end = line[at + open..]
            .find(|c: char| !(c.is_ascii_lowercase() || c == '.' || c == '-'))
            .map_or(line.len(), |e| at + open + e);
        let name = &line[at + open..name_end];
        let table = name.split('.').next().unwrap_or_default();
        if !TABLES.contains(&table) || !line[name_end..].starts_with(&"]".repeat(open)) {
            continue;
        }
        let after = &line[name_end + open..];
        let toml_key = |s: &str| {
            let s = s.trim_start();
            s.split_once(" = ").is_some_and(|(key, _)| {
                !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase() || c == '-')
            })
        };
        let in_code = after.starts_with('`') || after.starts_with('(');
        let header = after.starts_with("\\n");
        let example = toml_key(after) || (after.trim() == "\\" && toml_key(next));
        if !(in_code || header || example) {
            found.push(line[at..name_end + open].to_owned());
        }
    }
    found
}

#[test]
fn the_matcher_finds_a_bare_name_and_passes_over_code_and_toml() {
    // The setup: what the test below holds every file to.
    assert_eq!(
        bare_names("set it under [stack.run.users].", ""),
        ["[stack.run.users]"]
    );
    assert_eq!(
        bare_names("one (in [[finding-review]]) more", ""),
        ["[[finding-review]]"]
    );
    assert_eq!(
        bare_names("Answer them in the [design] \\", "section of"),
        ["[design]"]
    );
    for passed in [
        "set it under `[stack.run.users]`.",
        "  [policy]\\n    fix-within-days = 7",
        "  [stack.run.users] not-offered = [\"x\"]",
        "an [unrelated] word",
        "a[stack] with no space before it",
    ] {
        assert!(bare_names(passed, "").is_empty(), "{passed}");
    }
    assert!(bare_names("  [data] \\", "  categories = []").is_empty());
}

/// The files whose words can reach the reports: each crate's own code, and the steps for checks made by hand. Test
/// files are left out, and so is `sv-manifest/src/spec.rs`, the commented stackvet.toml `sv init` writes, which is
/// TOML and not Markdown.
fn sources() -> Vec<PathBuf> {
    let mut out = vec![root().join("data/human-checks.json")];
    let mut stack = vec![root().join("crates")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            if path.is_dir() {
                if name != "tests" && name != "target" {
                    stack.push(path);
                }
            } else if name.ends_with(".rs")
                && !name.ends_with("_tests.rs")
                && name != "tests.rs"
                && path.components().any(|c| c.as_os_str() == "src")
                && !path.ends_with("sv-manifest/src/spec.rs")
            {
                out.push(path);
            }
        }
    }
    out
}

#[test]
fn no_table_name_is_plain_text_in_the_fixed_wording() {
    let files = sources();
    assert!(files.len() > 100, "the setup: the sources were found");
    let mut bare = Vec::new();
    for path in &files {
        let text = std::fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        let mut in_tests = false;
        for (n, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            // An inline test module runs to the end of the file; a one-line `mod name;` only declares one.
            if trimmed.starts_with("#[cfg(test)]") {
                in_tests |= lines[n + 1..]
                    .iter()
                    .map(|l| l.trim())
                    .find(|l| !l.starts_with("#["))
                    .is_some_and(|l| l.starts_with("mod ") && l.ends_with('{'));
            }
            if in_tests || trimmed.starts_with("//") {
                continue;
            }
            let next = lines.get(n + 1).copied().unwrap_or_default();
            for name in bare_names(line, next) {
                bare.push(format!(
                    "{}:{}: {name}",
                    path.strip_prefix(root()).unwrap_or(path).display(),
                    n + 1
                ));
            }
        }
    }
    assert!(
        bare.is_empty(),
        "write each as code, `[name]`: {}",
        bare.join("\n")
    );
}

#[test]
fn no_example_report_shows_an_escaped_table_name() {
    // What the person reads: each example app's Markdown reports, made as `sv report` makes them, hold no `\[...\]`
    // of a table's name. The text the app supplies can still be escaped; these apps name no table.
    let scratch = std::env::temp_dir().join(format!("sv-setting-names-{}", std::process::id()));
    std::fs::remove_dir_all(&scratch).ok();
    let mut read = 0;
    for app in [
        "flask-booking",
        "notes-with-users",
        "oidc-notes",
        "partly-passing",
        "tested-notes",
    ] {
        let dir = scratch.join(app);
        std::fs::create_dir_all(&dir).unwrap();
        for entry in std::fs::read_dir(root().join("examples").join(app)).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                std::fs::copy(entry.path(), dir.join(entry.file_name())).unwrap();
            }
        }
        let out = Command::new(env!("CARGO_BIN_EXE_sv"))
            .args(["report", ".", "--no-tools"])
            .current_dir(&dir)
            .output()
            .unwrap();
        for name in ["compliance.md", "security.md", "low-findings.md"] {
            let Ok(text) = std::fs::read_to_string(dir.join("stackvet-report").join(name)) else {
                continue;
            };
            read += 1;
            for table in TABLES {
                assert!(
                    !text.contains(&format!("\\[{table}\\]"))
                        && !text.contains(&format!("\\[{table}.")),
                    "{app}/{name} shows an escaped [{table}...] (sv said: {})",
                    String::from_utf8_lossy(&out.stderr)
                );
            }
        }
    }
    std::fs::remove_dir_all(&scratch).ok();
    assert!(
        read >= 10,
        "the setup: the reports were written ({read} read)"
    );
}
