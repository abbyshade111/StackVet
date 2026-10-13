//! What the app runs on, for the bill of materials beside its packages (backlog 0402): the app itself, the container
//! image `stackvet.toml` starts it in, the language, and the modules of the language's own library its code imports.
//!
//! An app on Python's own library alone had a bill of materials listing nothing, which read as "nothing ships" (the
//! owner's trial, 13 October 2026). Python's flaws are fixed in Python's versions, and a flaw in one of its modules
//! matters only if the code imports it, so both are listed, each saying where `sv` got it. None of this is a package,
//! and none of it changes whether the list of packages is complete.

use std::collections::{BTreeMap, BTreeSet};

/// A source file larger than this is not read for its imports: it is not code a person wrote.
const MOST_BYTES: u64 = 2_000_000;

/// What the app runs on, as far as `sv` can tell.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RunsOn {
    /// The app's name, from `stackvet.toml`.
    pub app: Option<String>,
    /// The container image `stackvet.toml` starts it in (`[stack.run] image`).
    pub image: Option<String>,
    /// That image's digest, read from the local Docker when `--run` started it.
    pub image_digest: Option<String>,
    /// The language, when the app has code in one whose own library `sv` can name.
    pub language: Option<Language>,
}

/// A language the app's code is written in, and the parts of its own library the code imports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language {
    /// `python`.
    pub name: &'static str,
    /// The version, when the image says it.
    pub version: Option<String>,
    /// Where the version came from, or why there is none, in words.
    pub version_from: String,
    /// Each module of the language's own library the code imports, with the files that import it.
    pub modules: BTreeMap<String, BTreeSet<String>>,
}

/// What the app in `listing` runs on: its name and image from `stackvet.toml`, the image's digest when the app was run,
/// and the language read from its files.
pub fn runs_on(
    listing: &sv_scan::files::Listing,
    app: &str,
    image: Option<&str>,
    image_digest: Option<&str>,
) -> RunsOn {
    let image = image.map(str::trim).filter(|i| !i.is_empty());
    RunsOn {
        app: Some(app.trim())
            .filter(|a| !a.is_empty())
            .map(str::to_owned),
        image: image.map(str::to_owned),
        image_digest: image_digest.map(str::to_owned),
        language: python(listing, image),
    }
}

/// Python, when the app has Python files: its version from an official `python` image's tag, and the modules of its
/// own library the code imports.
fn python(listing: &sv_scan::files::Listing, image: Option<&str>) -> Option<Language> {
    let stdlib = python_stdlib();
    let mut any = false;
    let mut modules: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in listing.app_files() {
        if !entry.relative.ends_with(".py") {
            continue;
        }
        any = true;
        let path = listing.root.join(&entry.relative);
        let Ok(meta) = std::fs::symlink_metadata(&path) else {
            continue;
        };
        if !meta.is_file() || meta.len() > MOST_BYTES {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for module in python_imports(&text) {
            let top = module.split('.').next().unwrap_or_default();
            if stdlib.contains(top) {
                modules
                    .entry(module)
                    .or_default()
                    .insert(entry.relative.clone());
            }
        }
    }
    if !any {
        return None;
    }
    let (version, version_from) = match image.and_then(python_image_version) {
        Some(version) => (
            Some(version),
            format!(
                "the tag of the image stackvet.toml starts the app in, {}: the version it names, not one read from the \
                 running app",
                image.unwrap_or_default()
            ),
        ),
        None => (
            None,
            match image {
                Some(image) => format!(
                    "not known: the image stackvet.toml starts the app in, {image}, is not an official python image \
                     whose tag names a version"
                ),
                None => "not known: stackvet.toml names no image to start the app in".to_owned(),
            },
        ),
    };
    Some(Language {
        name: "python",
        version,
        version_from,
        modules,
    })
}

/// The modules a Python file imports, each as written (`http.server`), from `import` and `from ... import` lines.
/// A relative import (`from . import x`) is the app's own, and `__future__` is no module, so neither is returned.
pub fn python_imports(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or_default().trim();
        if let Some(rest) = line.strip_prefix("import ") {
            for part in rest.split(',') {
                let name = part.split_whitespace().next().unwrap_or_default();
                push(&mut out, name);
            }
        } else if let Some(rest) = line.strip_prefix("from ") {
            let mut words = rest.split_whitespace();
            if let (Some(name), Some("import")) = (words.next(), words.next()) {
                push(&mut out, name);
            }
        }
    }
    out
}

fn push(out: &mut BTreeSet<String>, name: &str) {
    let name = name.trim_matches(|c: char| c == '(' || c == ')' || c == ';');
    let plain = !name.is_empty()
        && !name.starts_with('.')
        && name != "__future__"
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    if plain {
        out.insert(name.to_owned());
    }
}

/// The version an official `python` image's tag names: `3.12` from `python:3.12-slim`, `3.12.7` from
/// `docker.io/library/python:3.12.7-bookworm`. `None` for another image, a tag that names no version, or `latest`.
pub fn python_image_version(image: &str) -> Option<String> {
    let image = image.split('@').next().unwrap_or_default();
    let (name, tag) = image.rsplit_once(':')?;
    let official = matches!(
        name,
        "python"
            | "library/python"
            | "docker.io/python"
            | "docker.io/library/python"
            | "index.docker.io/library/python"
    );
    if !official {
        return None;
    }
    let version: String = tag
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let version = version.trim_end_matches('.');
    (version.starts_with(|c: char| c.is_ascii_digit()) && version.contains('.'))
        .then(|| version.to_owned())
}

/// The top-level modules of Python's own library (`data/python-stdlib.json`), the ones a later Python dropped included.
fn python_stdlib() -> BTreeSet<String> {
    let path = sv_frameworks::data::file("python-stdlib.json");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return BTreeSet::new();
    };
    let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else {
        return BTreeSet::new();
    };
    ["modules", "removed_in_later_pythons"]
        .iter()
        .flat_map(|key| doc[*key].as_array().cloned().unwrap_or_default())
        .filter_map(|m| m.as_str().map(str::to_owned))
        .collect()
}
