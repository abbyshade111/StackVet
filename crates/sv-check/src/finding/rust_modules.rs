//! Which Rust files are modules Rust compiles only for its tests (backlog 0644, ADR-136). A test module that is a file
//! of its own, declared `#[cfg(test)] mod tests;` or `#[cfg(test)] #[path = "x_tests.rs"] mod tests;`, carries no
//! attribute of its own: the line that declares it, in another file, is what says Rust builds it only for tests. So the
//! declaring files are read, by the paths Rust itself would take, and a module declared inside such a file is one too.

use std::collections::HashMap;
use std::path::{Component, Path};

/// How many files up a chain of declarations is followed: a module declared in a module declared in a test module.
const DEEPEST: usize = 8;

/// Remembers what each file declares, so a folder of test modules is read once.
#[derive(Default)]
pub(super) struct TestModules {
    declared: HashMap<String, Vec<Declared>>,
    whole_file_test: HashMap<String, bool>,
    answers: HashMap<String, bool>,
}

/// One `mod name;` in a file: the files it may be, and whether it is under `#[cfg(test)]`.
struct Declared {
    files: Vec<String>,
    under_test: bool,
}

impl TestModules {
    /// Whether `file`, relative to `app_dir`, is a module some file beside it or above it declares for tests only.
    pub(super) fn is_test_module(&mut self, app_dir: &Path, file: &str) -> bool {
        self.answer(app_dir, &normal(Path::new(file)), 0)
    }

    fn answer(&mut self, app_dir: &Path, file: &str, depth: usize) -> bool {
        if let Some(known) = self.answers.get(file) {
            return *known;
        }
        // Until it is known, a file is not a test module: a loop of `#[path]` declarations ends there.
        self.answers.insert(file.to_owned(), false);
        let found = depth < DEEPEST
            && declarers(app_dir, file).into_iter().any(|declarer| {
                let names_it = self
                    .declarations(app_dir, &declarer)
                    .iter()
                    .filter(|d| d.files.iter().any(|f| f == file))
                    .map(|d| d.under_test)
                    .collect::<Vec<_>>();
                !names_it.is_empty()
                    && (names_it.contains(&true)
                        || self.whole_file(app_dir, &declarer)
                        || self.answer(app_dir, &declarer, depth + 1))
            });
        self.answers.insert(file.to_owned(), found);
        found
    }

    fn declarations(&mut self, app_dir: &Path, declarer: &str) -> &Vec<Declared> {
        self.declared.entry(declarer.to_owned()).or_insert_with(|| {
            std::fs::read_to_string(app_dir.join(declarer))
                .map(|source| declared_modules(&source, declarer))
                .unwrap_or_default()
        })
    }

    fn whole_file(&mut self, app_dir: &Path, file: &str) -> bool {
        *self
            .whole_file_test
            .entry(file.to_owned())
            .or_insert_with(|| {
                std::fs::read_to_string(app_dir.join(file))
                    .map(|source| super::rust_test_lines(&source) == [(1, usize::MAX)])
                    .unwrap_or(false)
            })
    }
}

/// The Rust files that could declare `file` as a module: those in its own folder (a `#[path]` beside it, or `name.rs`
/// declared by a `mod.rs`, `lib.rs`, or `main.rs` there), in the folder above (`folder.rs` declaring `folder/name.rs`,
/// or a `#[path]` into it), and for a `mod.rs`, the folder above that (`folder.rs` declaring `folder/name/mod.rs`).
fn declarers(app_dir: &Path, file: &str) -> Vec<String> {
    let path = Path::new(file);
    let mut out = Vec::new();
    let own = path.parent().map(Path::to_path_buf).unwrap_or_default();
    let mut folders = vec![own.clone()];
    if let Some(above) = own.parent() {
        folders.push(above.to_path_buf());
        // `folder/name/mod.rs` is declared from the folder that holds `folder.rs`, two up.
        if path.file_name().is_some_and(|n| n == "mod.rs")
            && let Some(two_up) = above.parent()
        {
            folders.push(two_up.to_path_buf());
        }
    }
    for folder in folders {
        let Ok(entries) = std::fs::read_dir(app_dir.join(&folder)) else {
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

/// The modules `source` declares at its top level as files (`mod name;`, not `mod name { ... }`), each with the paths
/// Rust would read it from and whether it is under `#[cfg(test)]`. `declarer` is the file's path, relative to the app.
fn declared_modules(source: &str, declarer: &str) -> Vec<Declared> {
    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .is_err()
    {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
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
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for item in root.named_children(&mut cursor) {
        if item.kind() != "mod_item" || item.child_by_field_name("body").is_some() {
            continue;
        }
        let Some(name) = item
            .child_by_field_name("name")
            .map(|n| &source[n.byte_range()])
        else {
            continue;
        };
        let mut under_test = false;
        let mut path_attribute = None;
        let mut before = item.prev_named_sibling();
        while let Some(node) = before {
            match node.kind() {
                "attribute_item" => {
                    let text: String = source[node.byte_range()]
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    under_test |= text == "#[cfg(test)]";
                    if let Some(p) = text
                        .strip_prefix("#[path=\"")
                        .and_then(|rest| rest.strip_suffix("\"]"))
                    {
                        path_attribute = Some(p.to_owned());
                    }
                }
                "line_comment" | "block_comment" => {}
                _ => break,
            }
            before = node.prev_named_sibling();
        }
        // A `#[path]` outside any inline module is read from the declaring file's own folder.
        let files = match path_attribute {
            Some(p) => vec![normal(&folder.join(p))],
            None => vec![
                normal(&home.join(format!("{name}.rs"))),
                normal(&home.join(name).join("mod.rs")),
            ],
        };
        out.push(Declared { files, under_test });
    }
    out
}

/// A relative path with `.` and `..` worked out and `/` between its parts, as findings name their files.
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

#[cfg(test)]
#[path = "rust_modules_tests.rs"]
mod tests;
