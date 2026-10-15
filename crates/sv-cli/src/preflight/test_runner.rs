//! Whether the program the test command runs is among the packages `install = true` installs (backlog 0532). In the
//! three-tool trial of 14 October 2026 an app's `test = "pytest -q"` named pytest only in `requirements-dev.txt`, the
//! install step reads `requirements.txt` alone, and the run printed `sh: 1: pytest: not found`: honest, but learned
//! only after a full run, and the AI coding tool read "tests failed" as the app's fault. Here it is said before.
//!
//! Read from the text alone, like the rest of the preflight: the runner is the command's first program, or the module
//! after `python -m`, or the package after `npx`, or the first program of the `package.json` script `npm test` runs.
//! A runner this does not know (`go test`, `cargo test`, a script of the app's own) is not judged.

use super::{Answer, Item, Manifest, app, sv};
use std::path::Path;

/// Where a test runner comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum From {
    /// PyPI, through requirements.txt.
    Python,
    /// The npm registry, through package.json.
    Node,
}

/// The package a known test runner is installed as, and where it comes from.
fn package_of(program: &str) -> Option<(&'static str, From)> {
    Some(match program {
        "pytest" | "py.test" => ("pytest", From::Python),
        "nose2" => ("nose2", From::Python),
        "tox" => ("tox", From::Python),
        "behave" => ("behave", From::Python),
        "jest" => ("jest", From::Node),
        "vitest" => ("vitest", From::Node),
        "mocha" => ("mocha", From::Node),
        "ava" => ("ava", From::Node),
        "jasmine" => ("jasmine", From::Node),
        "cypress" => ("cypress", From::Node),
        "playwright" => ("@playwright/test", From::Node),
        _ => return None,
    })
}

/// The test runner `test` starts, as `(program, package, where from)`, or `None` when it is not one this knows.
fn runner(test: &str, app_dir: &Path, depth: u8) -> Option<(String, &'static str, From)> {
    // The first command only: `pytest -q && echo done` runs pytest.
    let first = test.split(['&', ';', '|']).next().unwrap_or(test);
    let words: Vec<&str> = first
        .split_whitespace()
        .skip_while(|w| (w.contains('=') && !w.starts_with('-')) || *w == "sudo")
        .collect();
    let tool = words.first()?.rsplit('/').next()?;
    let program = match tool {
        "python" | "python3" if words.get(1) == Some(&"-m") => (*words.get(2)?).to_owned(),
        "npx" => (*words.iter().skip(1).find(|w| !w.starts_with('-'))?).to_owned(),
        "npm" | "yarn" | "pnpm" if depth == 0 => {
            let script = match (words.get(1).copied(), words.get(2).copied()) {
                (Some("test" | "t"), _) => "test",
                (Some("run" | "run-script"), Some(name)) => name,
                (Some(name), _) if tool != "npm" => name,
                _ => return None,
            };
            let package: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(app_dir.join("package.json")).ok()?)
                    .ok()?;
            let line = package["scripts"][script].as_str()?;
            return runner(line, app_dir, depth + 1);
        }
        other => other.to_owned(),
    };
    let (package, from) = package_of(&program)?;
    Some((program, package, from))
}

/// A requirement line's package name, compared as pip compares them: case and `-`, `_`, `.` alike.
fn requirement_name(line: &str) -> Option<String> {
    let line = line.split('#').next()?.trim();
    if line.is_empty() || line.starts_with('-') {
        return None;
    }
    let end = line
        .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')))
        .unwrap_or(line.len());
    Some(normal(&line[..end]))
}

fn normal(name: &str) -> String {
    name.to_ascii_lowercase().replace(['_', '.'], "-")
}

/// The line of `text` that names `package`, if one does.
fn line_naming(text: &str, package: &str) -> Option<String> {
    text.lines()
        .find(|line| requirement_name(line).as_deref() == Some(&normal(package)))
        .map(|line| line.trim().to_owned())
}

/// The preflight's answer about the test runner, or `None` when there is nothing to say: no test command, no
/// `install = true` (the image is then where the runner comes from, which the text cannot show), or a runner this
/// does not know.
pub(super) fn item(manifest: &Manifest, app_dir: &Path) -> Option<Item> {
    let run = &manifest.stack.run;
    if run.install != Some(true) {
        return None;
    }
    let test = run.test.as_deref()?.trim();
    let (program, package, from) = runner(test, app_dir, 0)?;
    let (file, installed) = match from {
        From::Python => {
            let text =
                std::fs::read_to_string(app_dir.join("requirements.txt")).unwrap_or_default();
            ("requirements.txt", line_naming(&text, package).is_some())
        }
        From::Node => {
            let text = std::fs::read_to_string(app_dir.join("package.json")).unwrap_or_default();
            let named = serde_json::from_str::<serde_json::Value>(&text).is_ok_and(|json| {
                ["dependencies", "devDependencies"]
                    .iter()
                    .any(|list| json[list].get(package).is_some())
            });
            ("package.json", named)
        }
    };
    if installed {
        return Some(Item::new(
            "test-runner",
            Answer::Looks,
            vec![sv(format!(
                "The test command runs `{program}`, and {file}, which `install = true` installs, names `{package}`."
            ))],
        ));
    }
    let mut says = vec![sv(format!(
        "The test command runs `{program}`, and {file}, the file `install = true` installs from, does not name `{package}`."
    ))];
    // The likeliest place for it: a second requirements file the install step does not read.
    if from == From::Python
        && let Some((other, line)) = other_requirements(app_dir, package)
    {
        says.push(sv(format!(
            " It is in {other}, which the install does not read: "
        )));
        says.push(app(format!("`{line}`")));
        says.push(sv("."));
    }
    says.push(sv(format!(
        " Add it to {file}{}, or the tests stop at once with \"{program}: not found\" and count as failed. If the image `[stack.run]` names already has it, ignore this.",
        match from {
            From::Python => " with an exact version",
            From::Node => " (in devDependencies)",
        }
    )));
    Some(Item::new("test-runner", Answer::Look, says))
}

/// A requirements file other than requirements.txt, in the app's top folder, that names `package`, and its line.
fn other_requirements(app_dir: &Path, package: &str) -> Option<(String, String)> {
    let mut names: Vec<String> = std::fs::read_dir(app_dir)
        .ok()?
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter(|n| n.starts_with("requirements") && n.ends_with(".txt") && n != "requirements.txt")
        .collect();
    names.sort();
    names.into_iter().find_map(|name| {
        let text = std::fs::read_to_string(app_dir.join(&name)).ok()?;
        Some((name, line_naming(&text, package)?))
    })
}

#[cfg(test)]
#[path = "test_runner_tests.rs"]
mod tests;
