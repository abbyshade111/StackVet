//! Which Rust files are compiled only for tests, read the same way by every check (backlog 0654, ADR-139). A test
//! module kept in a file of its own carries no attribute of its own: the line that declares it in another file,
//! `#[cfg(test)] mod tests;` or `#[cfg(all(test, unix))] #[path = "x_tests.rs"] mod tests;`, is what says Rust builds it
//! only for tests (ADR-136). This reads those lines from the text, without a parser: a declaration is attributes, then
//! `mod name;`, at the top level of the file. Comments, strings, and everything inside braces are passed over, so a
//! `mod name;` inside an inline `mod x { ... }` block, whose file Rust looks for elsewhere, is not misread.

use std::collections::HashMap;
use std::path::{Component, Path};

/// How many files up a chain of declarations is followed: a module declared in a module declared in a test module.
const DEEPEST: usize = 8;

/// Whether a `cfg` attribute's condition holds only in a test build: `test`, or `all(...)` with such a condition among
/// its own arguments. `any(test, unix)` holds in any Unix build and `not(test)` in every other, so neither is.
/// `attribute` is the whole attribute, `#[cfg(...)]` or `#![cfg(...)]`, spaces anywhere.
pub fn cfg_test_only(attribute: &str) -> bool {
    let text: String = attribute.chars().filter(|c| !c.is_whitespace()).collect();
    let inner = text
        .strip_prefix("#![cfg(")
        .or_else(|| text.strip_prefix("#[cfg("))
        .and_then(|rest| rest.strip_suffix(")]"));
    inner.is_some_and(test_only)
}

fn test_only(condition: &str) -> bool {
    if condition == "test" {
        return true;
    }
    match condition
        .strip_prefix("all(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        Some(arguments) => split_top(arguments).into_iter().any(test_only),
        None => false,
    }
}

/// `a,b(c,d),e` split at its own commas: `a`, `b(c,d)`, `e`.
fn split_top(text: &str) -> Vec<&str> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0usize, 0);
    for (i, c) in text.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                out.push(&text[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&text[start..]);
    out
}

/// The file's top level as text: comments blanked, everything inside braces blanked, strings kept as written (a
/// `#[path = "..."]` is a string), and line breaks kept, so a declaration can be read off it with a pattern.
fn top_level(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let (mut i, mut depth) = (0usize, 0usize);
    let blank = |c: char| if c == '\n' { '\n' } else { ' ' };
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        // A comment, to its end.
        if c == '/' && next == Some('/') {
            while i < chars.len() && chars[i] != '\n' {
                out.push(' ');
                i += 1;
            }
            continue;
        }
        if c == '/' && next == Some('*') {
            let mut nesting = 0usize;
            while i < chars.len() {
                if chars[i] == '/' && chars.get(i + 1) == Some(&'*') {
                    nesting += 1;
                    out.push_str("  ");
                    i += 2;
                } else if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                    nesting -= 1;
                    out.push_str("  ");
                    i += 2;
                    if nesting == 0 {
                        break;
                    }
                } else {
                    out.push(blank(chars[i]));
                    i += 1;
                }
            }
            continue;
        }
        // A string, raw or not, to its end: kept at the top level, blanked inside braces, its braces never counted.
        let raw_hashes = (c == 'r' && matches!(next, Some('"') | Some('#')))
            .then(|| chars[i + 1..].iter().take_while(|&&h| h == '#').count())
            .filter(|&n| chars.get(i + 1 + n) == Some(&'"'));
        if c == '"' || raw_hashes.is_some() {
            let hashes = raw_hashes.unwrap_or(0);
            let open = if raw_hashes.is_some() { 2 + hashes } else { 1 };
            let start = i;
            i += open;
            while i < chars.len() {
                if raw_hashes.is_none() && chars[i] == '\\' {
                    i += 2;
                    continue;
                }
                if chars[i] == '"'
                    && chars[i + 1..]
                        .iter()
                        .take(hashes)
                        .filter(|&&h| h == '#')
                        .count()
                        == hashes
                {
                    i += 1 + hashes;
                    break;
                }
                i += 1;
            }
            let end = i.min(chars.len());
            for &ch in &chars[start..end] {
                out.push(if depth == 0 { ch } else { blank(ch) });
            }
            continue;
        }
        // A character literal (`'{'`), not a lifetime (`'a`): its brace is not counted.
        if c == '\'' {
            let len = if next == Some('\\') {
                chars[i + 2..]
                    .iter()
                    .position(|&ch| ch == '\'')
                    .map(|p| p + 3)
            } else if chars.get(i + 2) == Some(&'\'') {
                Some(3)
            } else {
                None
            };
            if let Some(len) = len {
                for _ in 0..len.min(chars.len() - i) {
                    out.push(' ');
                }
                i += len;
                continue;
            }
        }
        match c {
            '{' => {
                depth += 1;
                out.push(' ');
            }
            '}' => {
                depth = depth.saturating_sub(1);
                out.push(' ');
            }
            _ => out.push(if depth == 0 { c } else { blank(c) }),
        }
        i += 1;
    }
    out
}

/// One `mod name;` in a file: the files it may be, and whether it is compiled only for tests.
struct Declared {
    files: Vec<String>,
    test_only: bool,
}

/// The modules `source` declares as files at its top level, each with the paths Rust would read it from. `declarer` is
/// the file's path, relative to the app.
fn declared_modules(source: &str, declarer: &str) -> Vec<Declared> {
    static DECLARATION: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"((?:#\[[^\]]*\]\s*)*)(?:\bpub(?:\s*\([^)]*\))?\s+)?\bmod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
            .expect("a fixed pattern")
    });
    static ATTRIBUTE: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"#\[[^\]]*\]").expect("a fixed pattern"));
    static PATH: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r#"^#\[\s*path\s*=\s*"([^"]*)"\s*\]$"#).expect("a fixed pattern")
    });
    let declaring = Path::new(declarer);
    let folder = declaring.parent().unwrap_or(Path::new(""));
    let stem = declaring
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    // `mod.rs`, `lib.rs`, and `main.rs` hold their modules beside them; any other file, in a folder of its own name.
    let home = if matches!(stem, "mod" | "lib" | "main") {
        folder.to_path_buf()
    } else {
        folder.join(stem)
    };
    let text = top_level(source);
    DECLARATION
        .captures_iter(&text)
        .map(|c| {
            let attributes: Vec<&str> = ATTRIBUTE
                .find_iter(c.get(1).map_or("", |m| m.as_str()))
                .map(|m| m.as_str())
                .collect();
            let name = &c[2];
            let path = attributes
                .iter()
                .find_map(|a| PATH.captures(a).map(|p| p[1].to_owned()));
            // A `#[path]` outside any inline module is read from the declaring file's own folder.
            let files = match path {
                Some(p) => vec![normal(&folder.join(p))],
                None => vec![
                    normal(&home.join(format!("{name}.rs"))),
                    normal(&home.join(name).join("mod.rs")),
                ],
            };
            Declared {
                files,
                test_only: attributes.iter().any(|a| cfg_test_only(a)),
            }
        })
        .collect()
}

/// Whether the file's first attribute says the whole file is for tests: `#![cfg(test)]` and its kin.
fn whole_file_test_only(source: &str) -> bool {
    static INNER: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"#!\[[^\]]*\]").expect("a fixed pattern"));
    INNER
        .find_iter(&top_level(source))
        .any(|m| cfg_test_only(m.as_str()))
}

/// A relative path with `.` and `..` worked out and `/` between its parts, as the listing names its files.
fn normal(path: &Path) -> String {
    let mut parts: Vec<String> = Vec::new();
    for c in path.components() {
        match c {
            Component::ParentDir => {
                parts.pop();
            }
            Component::Normal(p) => parts.push(p.to_string_lossy().into_owned()),
            _ => {}
        }
    }
    parts.join("/")
}

/// Remembers what each file declares, so a folder of test modules is read once.
#[derive(Debug, Default)]
pub struct TestModules {
    declared: HashMap<String, Vec<(Vec<String>, bool)>>,
    whole_file: HashMap<String, bool>,
    answers: HashMap<String, bool>,
}

impl TestModules {
    /// Whether `file`, relative to `root`, is Rust compiled only for tests: a file that says so of itself
    /// (`#![cfg(test)]`), or a module some file beside it or above it declares for tests only, or declares inside such
    /// a module.
    pub fn is_test_only(&mut self, root: &Path, file: &str) -> bool {
        let file = normal(Path::new(file));
        self.whole(root, &file) || self.answer(root, &file, 0)
    }

    fn answer(&mut self, root: &Path, file: &str, depth: usize) -> bool {
        if let Some(known) = self.answers.get(file) {
            return *known;
        }
        // Until it is known, a file is not test code: a loop of `#[path]` declarations ends there.
        self.answers.insert(file.to_owned(), false);
        let found = depth < DEEPEST
            && declarers(root, file).into_iter().any(|declarer| {
                let names_it: Vec<bool> = self
                    .declarations(root, &declarer)
                    .iter()
                    .filter(|(files, _)| files.iter().any(|f| f == file))
                    .map(|(_, test_only)| *test_only)
                    .collect();
                !names_it.is_empty()
                    && (names_it.contains(&true)
                        || self.whole(root, &declarer)
                        || self.answer(root, &declarer, depth + 1))
            });
        self.answers.insert(file.to_owned(), found);
        found
    }

    fn declarations(&mut self, root: &Path, declarer: &str) -> &Vec<(Vec<String>, bool)> {
        self.declared.entry(declarer.to_owned()).or_insert_with(|| {
            std::fs::read_to_string(root.join(declarer))
                .map(|source| {
                    declared_modules(&source, declarer)
                        .into_iter()
                        .map(|d| (d.files, d.test_only))
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    fn whole(&mut self, root: &Path, file: &str) -> bool {
        *self.whole_file.entry(file.to_owned()).or_insert_with(|| {
            std::fs::read_to_string(root.join(file))
                .map(|source| whole_file_test_only(&source))
                .unwrap_or(false)
        })
    }
}

/// The Rust files that could declare `file` as a module: those in its own folder (a `#[path]` beside it, or `name.rs`
/// declared by a `mod.rs`, `lib.rs`, or `main.rs` there), in the folder above (`folder.rs` declaring `folder/name.rs`,
/// or a `#[path]` into it), and for a `mod.rs`, the folder above that (`folder.rs` declaring `folder/name/mod.rs`).
fn declarers(root: &Path, file: &str) -> Vec<String> {
    let path = Path::new(file);
    let own = path.parent().map(Path::to_path_buf).unwrap_or_default();
    let mut folders = vec![own.clone()];
    if let Some(above) = own.parent() {
        folders.push(above.to_path_buf());
        if path.file_name().is_some_and(|n| n == "mod.rs")
            && let Some(two_up) = above.parent()
        {
            folders.push(two_up.to_path_buf());
        }
    }
    let mut out = Vec::new();
    for folder in folders {
        let Ok(entries) = std::fs::read_dir(root.join(&folder)) else {
            continue;
        };
        let mut names: Vec<String> = entries
            .filter_map(|e| e.ok()?.file_name().into_string().ok())
            .filter(|n| n.ends_with(".rs"))
            .collect();
        names.sort();
        for name in names {
            let candidate = normal(&folder.join(&name));
            if candidate != file {
                out.push(candidate);
            }
        }
    }
    out
}

#[cfg(test)]
#[path = "rust_tests_tests.rs"]
mod tests;
