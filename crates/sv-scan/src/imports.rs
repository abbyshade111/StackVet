//! Which packages the app's code imports, and from where (backlog 0538). In the three-tool trial of
//! 14 October 2026 `httpx`, declared in requirements.txt because the tests used it as a test client,
//! was read as the app calling outside services; the AI coding tool moved it and `pytest` to a file
//! of their own, and the container could no longer run the tests. A package that only the app's
//! tests import is not evidence of what the app does.
//!
//! Read as lines, for Python and JavaScript or TypeScript only. A package no file imports, or one
//! imported under a name other than its own (`Pillow` as `PIL`), is not seen here, and counts as
//! before: only a package seen imported by tests and by nothing else is set aside.

use std::collections::{BTreeMap, BTreeSet};

/// The packages imported in the app's files, by ecosystem as `deps::Declared` names it.
#[derive(Debug, Default)]
pub struct Imports {
    /// Imported by a file that is not a test.
    by_app: BTreeSet<(&'static str, String)>,
    /// Imported by a test file: the first such file.
    by_tests: BTreeMap<(&'static str, String), String>,
}

impl Imports {
    /// Notes what one file imports. `language` is the listing's language for the file.
    pub fn read(&mut self, language: &str, relative: &str, contents: &str) {
        self.read_as(
            language,
            relative,
            contents,
            crate::files::is_test_path(relative),
        );
    }

    /// The same, with whether the file is test code already read (`Listing::is_test_code`, which knows Rust's test
    /// modules too).
    pub fn read_as(&mut self, language: &str, relative: &str, contents: &str, test: bool) {
        let (ecosystem, names) = match language {
            "python" => ("Python", python(contents)),
            "javascript" | "typescript" => ("npm", javascript(contents)),
            _ => return,
        };
        for name in names {
            let key = (ecosystem, name);
            if test {
                self.by_tests
                    .entry(key)
                    .or_insert_with(|| relative.to_owned());
            } else {
                self.by_app.insert(key);
            }
        }
    }

    /// The test file that imports a declared package, when tests import it and nothing else does.
    pub fn only_in_tests(&self, ecosystem: &str, package: &str) -> Option<&str> {
        let ecosystem = match ecosystem {
            "Python" => "Python",
            "npm" => "npm",
            _ => return None,
        };
        let key = (ecosystem, normal(ecosystem, package));
        if self.by_app.contains(&key) {
            return None;
        }
        self.by_tests.get(&key).map(String::as_str)
    }
}

/// A package's name as an import writes it: Python's lowercased with `-` and `.` as `_`; npm's
/// lowercased.
fn normal(ecosystem: &str, name: &str) -> String {
    let name = name.trim().to_lowercase();
    if ecosystem == "Python" {
        name.replace(['-', '.'], "_")
    } else {
        name
    }
}

/// The top-level modules of `import a, b.c` and `from a.b import c`, leaving out relative imports.
fn python(contents: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in contents.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix("from ") {
            let module = rest.split_whitespace().next().unwrap_or("");
            if !module.starts_with('.') {
                out.extend(top(module, '.'));
            }
        } else if let Some(rest) = line.strip_prefix("import ") {
            let rest = rest.split('#').next().unwrap_or("");
            for part in rest.split(',') {
                let module = part.split_whitespace().next().unwrap_or("");
                out.extend(top(module, '.'));
            }
        }
    }
    out.into_iter().map(|m| normal("Python", &m)).collect()
}

/// The packages of `require('x')`, `import(...)`, `from 'x'`, and `import 'x'`, leaving out paths.
fn javascript(contents: &str) -> Vec<String> {
    let mut out = Vec::new();
    for marker in ["require(", "import(", "from ", "import "] {
        let mut at = 0;
        while let Some(found) = contents[at..].find(marker) {
            let start = at + found + marker.len();
            at = start;
            let rest = contents[start..].trim_start();
            let Some(quote) = rest
                .chars()
                .next()
                .filter(|c| matches!(c, '\'' | '"' | '`'))
            else {
                continue;
            };
            let Some(end) = rest[1..].find(quote) else {
                continue;
            };
            let spec = &rest[1..1 + end];
            if spec.is_empty() || spec.starts_with('.') || spec.starts_with('/') {
                continue;
            }
            let spec = spec.strip_prefix("node:").unwrap_or(spec);
            let name = if spec.starts_with('@') {
                spec.splitn(3, '/').take(2).collect::<Vec<_>>().join("/")
            } else {
                spec.split('/').next().unwrap_or(spec).to_owned()
            };
            out.push(normal("npm", &name));
        }
    }
    out
}

fn top(module: &str, separator: char) -> Option<String> {
    let first = module.split(separator).next()?.trim_end_matches(';');
    (!first.is_empty() && first.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| first.to_owned())
}

#[cfg(test)]
mod imports_tests;
