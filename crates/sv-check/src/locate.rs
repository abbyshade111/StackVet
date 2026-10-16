//! Where in the app's code a question to the owner is about: lines found by the names in them, and
//! never judged (backlog 0487).
//!
//! A question asks the owner to settle something only a person can, such as whether authorization is
//! enforced on the server (V8.3.1). In the Antigravity trial of 13 October 2026 the AI coding tool
//! answered the questions with tips about the code, and several did not hold once the owner asked for
//! the lines (backlog 0397). A locator gives the owner the lines to read: the places whose names say
//! they check a role or a permission.
//!
//! # It finds, and does not judge
//!
//! A locator is not a rule. It lives in its own file (`data/locators.json`) and its own type, so
//! nothing that credits or fails a requirement can read it: it makes no finding, it credits nothing
//! when it finds nothing, and it holds no rule back. A line it found is where to look, not evidence
//! (`human::POINTERS_SAY`): `requireAdmin` may check nothing at all, and a check may be named
//! something no locator knows.

use anyhow::Result;
use regex::Regex;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

/// The most places one locator keeps in one app: a question shows a few, and says how many more.
pub const MOST_PLACES: usize = 200;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Locator {
    pub id: String,
    /// The requirements whose question these lines are about. A locator speaks to the question, not
    /// to the requirement: it is never a check of it.
    pub questions: Vec<String>,
    /// What the lines are, in plain words, shown above them.
    pub finds: String,
    /// The names that count, matched against a name with its case lowered and its `_` taken out, so
    /// `require_role` and `requireRole` are one name. A locator that finds a shape rather than a name
    /// (a query with `owner = ?`, a 403 sent) matches the captured text made plain the same way.
    pub names: String,
    /// Places to leave out: a place is not kept when the condition of the nearest `if` it sits under,
    /// made plain the same way, matches. A 403 sent because a form's anti-forgery token or a password
    /// was wrong is not a check of what a person may do (backlog 0518).
    #[serde(default)]
    pub unless: Option<String>,
    /// Per language, a tree-sitter query whose `@defined` captures the name of a function being
    /// written and `@used` the name of one being called or passed.
    pub queries: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LocatorFile {
    #[serde(rename = "_comment", default)]
    _comment: String,
    locators: Vec<Locator>,
    /// Locators that look in the bill of materials rather than the code (backlog 0490).
    #[serde(default)]
    packages: Vec<PackageLocator>,
}

/// A locator that finds packages by their names in the app's bill of materials, for a question whose answer sits in
/// how one of them is set up rather than in a line of the app's code (backlog 0490). It finds and does not judge, as a
/// locator of lines does: a package in the list is not evidence that it is used, or of how.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PackageLocator {
    pub id: String,
    /// The requirements whose question these packages are about.
    pub questions: Vec<String>,
    /// What the packages are, in plain words, shown before them.
    pub finds: String,
    /// What the report already says about them, shown after them.
    pub says: String,
    /// Per ecosystem, by the bill of materials' name for it (`Python`, `Ruby`), the package names that count, matched
    /// with case lowered and `_` read as `-`.
    pub names: BTreeMap<String, Vec<String>>,
}

impl PackageLocator {
    /// Each package of `components` this locator names, as a person reads it: the name and version, and, for a version
    /// read from what was asked for rather than a lockfile, that it may not be the one installed.
    pub fn find(&self, components: &[crate::sbom::Component]) -> Vec<String> {
        let mut found: Vec<String> = components
            .iter()
            .filter(|c| {
                self.names.get(&c.ecosystem).is_some_and(|names| {
                    names
                        .iter()
                        .any(|n| package_name(n) == package_name(&c.name))
                })
            })
            .map(|c| match c.source {
                crate::sbom::VersionSource::Locked => format!("{} {}", c.name, c.version),
                crate::sbom::VersionSource::Declared => format!(
                    "{} {} (the version asked for: no lockfile says which is installed)",
                    c.name, c.version
                ),
            })
            .collect();
        found.sort();
        found.dedup();
        found
    }
}

/// A package's name as two spellings of it compare: `Gunicorn` and `gunicorn`, `uvicorn_x` and `uvicorn-x`.
fn package_name(name: &str) -> String {
    name.trim().to_lowercase().replace('_', "-")
}

struct Compiled {
    locator: Locator,
    names: Regex,
    unless: Option<Regex>,
    queries: BTreeMap<String, Query>,
}

pub struct Locators {
    compiled: Vec<Compiled>,
    packages: Vec<PackageLocator>,
}

/// One place a locator found.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Located {
    pub file: String,
    pub line: usize,
    /// A function written here, rather than one called: where a rule is kept comes before the places
    /// that use it.
    pub defined: bool,
}

/// What every locator found in one app, by locator id.
#[derive(Debug, Default)]
pub struct Found {
    pub places: BTreeMap<String, Vec<Located>>,
    /// Places past `MOST_PLACES`, counted and not kept.
    pub more: BTreeMap<String, usize>,
}

impl Locators {
    pub fn load(path: &Path) -> Result<Locators> {
        let file: LocatorFile = sv_frameworks::data::read_json(path)?;
        let mut compiled = Vec::new();
        let mut ids = BTreeSet::new();
        for locator in file.locators {
            if !ids.insert(locator.id.clone()) {
                anyhow::bail!("{}: two locators are called {}", path.display(), locator.id);
            }
            if locator.questions.is_empty() || locator.finds.trim().is_empty() {
                anyhow::bail!(
                    "{}: the locator {} names no question or does not say what it finds",
                    path.display(),
                    locator.id
                );
            }
            let names = Regex::new(&locator.names).map_err(|e| {
                anyhow::anyhow!("{}: the names of {}: {e}", path.display(), locator.id)
            })?;
            let unless = locator
                .unless
                .as_deref()
                .map(Regex::new)
                .transpose()
                .map_err(|e| {
                    anyhow::anyhow!("{}: the unless of {}: {e}", path.display(), locator.id)
                })?;
            let mut queries = BTreeMap::new();
            for (language, source) in &locator.queries {
                for (grammar_name, key) in grammars_for(language) {
                    let Some(grammar) = crate::ast::grammar(grammar_name) else {
                        anyhow::bail!(
                            "{}: {} has a query for {language}, which `sv` does not read",
                            path.display(),
                            locator.id
                        );
                    };
                    let query = Query::new(&grammar, source).map_err(|e| {
                        anyhow::anyhow!(
                            "{}: the {language} query of {}: {e}",
                            path.display(),
                            locator.id
                        )
                    })?;
                    let captures = query.capture_names();
                    if !captures.iter().any(|c| *c == "defined" || *c == "used") {
                        anyhow::bail!(
                            "{}: the {language} query of {} captures neither @defined nor @used",
                            path.display(),
                            locator.id
                        );
                    }
                    queries.insert(key.to_owned(), query);
                }
            }
            compiled.push(Compiled {
                locator,
                names,
                unless,
                queries,
            });
        }
        for locator in &file.packages {
            if !ids.insert(locator.id.clone()) {
                anyhow::bail!("{}: two locators are called {}", path.display(), locator.id);
            }
            if locator.questions.is_empty()
                || locator.finds.trim().is_empty()
                || locator.says.trim().is_empty()
                || locator.names.values().all(Vec::is_empty)
            {
                anyhow::bail!(
                    "{}: the package locator {} names no question or no package, or does not say what it finds \
                     or what the report says of it",
                    path.display(),
                    locator.id
                );
            }
        }
        Ok(Locators {
            compiled,
            packages: file.packages,
        })
    }

    pub fn all(&self) -> impl Iterator<Item = &Locator> {
        self.compiled.iter().map(|c| &c.locator)
    }

    /// The locators that look in the bill of materials (backlog 0490).
    pub fn packages(&self) -> &[PackageLocator] {
        &self.packages
    }

    /// Every place each locator finds in the app's own files, in file and line order, with the
    /// functions written before the places they are called.
    pub fn find(&self, listing: &sv_scan::files::Listing) -> Found {
        let mut found = Found::default();
        for entry in listing.app_files() {
            let Some(language) = entry.language else {
                continue;
            };
            let lower = entry.relative.to_lowercase();
            let key = if language == "typescript" && lower.ends_with(".tsx") {
                "tsx"
            } else {
                language
            };
            let wanted: Vec<&Compiled> = self
                .compiled
                .iter()
                .filter(|c| c.queries.contains_key(key))
                .collect();
            if wanted.is_empty() {
                continue;
            }
            let Ok(source) = entry.read_text() else {
                continue;
            };
            // Most files hold no name any locator knows, and are not parsed for one. A locator whose names are a
            // shape rather than one word (`owner = ?`) is looked for in the whole text, made plain the same way.
            let words: BTreeSet<String> = word_names(&source).collect();
            let flat = plain(&source);
            let wanted: Vec<&Compiled> = wanted
                .into_iter()
                .filter(|c| words.iter().any(|w| c.names.is_match(w)) || c.names.is_match(&flat))
                .collect();
            if wanted.is_empty() {
                continue;
            }
            for c in wanted {
                for place in places(c, key, &entry.relative, &source) {
                    let kept = found.places.entry(c.locator.id.clone()).or_default();
                    if kept.len() < MOST_PLACES {
                        kept.push(place);
                    } else {
                        *found.more.entry(c.locator.id.clone()).or_default() += 1;
                    }
                }
            }
        }
        for places in found.places.values_mut() {
            places.sort_by(|a, b| {
                b.defined
                    .cmp(&a.defined)
                    .then_with(|| a.file.cmp(&b.file))
                    .then_with(|| a.line.cmp(&b.line))
            });
        }
        found
    }
}

/// The grammars a language's query is compiled for, and the key each is kept under: TypeScript's
/// query serves `.tsx` files too, through the grammar that reads JSX.
fn grammars_for(language: &str) -> Vec<(&str, &str)> {
    if language == "typescript" {
        vec![("typescript", "typescript"), ("tsx", "tsx")]
    } else {
        vec![(language, language)]
    }
}

/// A name with its case lowered and its `_` taken out.
pub fn plain(name: &str) -> String {
    name.chars()
        .filter(|c| *c != '_')
        .flat_map(char::to_lowercase)
        .collect()
}

fn word_names(source: &str) -> impl Iterator<Item = String> + '_ {
    source
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|w| !w.is_empty())
        .map(plain)
}

/// The places in one file, one per line.
fn places(c: &Compiled, key: &str, relative: &str, source: &str) -> Vec<Located> {
    let Some(query) = c.queries.get(key) else {
        return Vec::new();
    };
    let Some(grammar) = crate::ast::grammar(key) else {
        return Vec::new();
    };
    let mut parser = Parser::new();
    if parser.set_language(&grammar).is_err() {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
    let names = query.capture_names();
    let mut by_line: BTreeMap<usize, bool> = BTreeMap::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = matches.next() {
        for capture in m.captures() {
            let defined = match names[capture.index as usize] {
                "defined" => true,
                "used" => false,
                _ => continue,
            };
            let Ok(text) = capture.node.utf8_text(source.as_bytes()) else {
                continue;
            };
            if !c.names.is_match(&plain(text)) {
                continue;
            }
            if let Some(unless) = &c.unless
                && guarding_condition(capture.node, source)
                    .is_some_and(|condition| unless.is_match(&plain(condition)))
            {
                continue;
            }
            let line = capture.node.start_position().row + 1;
            let was = by_line.entry(line).or_insert(defined);
            *was = *was || defined;
        }
    }
    by_line
        .into_iter()
        .map(|(line, defined)| Located {
            file: relative.to_owned(),
            line,
            defined,
        })
        .collect()
}

/// The condition of the nearest `if` (or `elif`) a node sits under, not counting one whose condition
/// the node is part of.
fn guarding_condition<'a>(node: tree_sitter::Node, source: &'a str) -> Option<&'a str> {
    let mut child = node;
    while let Some(parent) = child.parent() {
        if matches!(parent.kind(), "if_statement" | "elif_clause")
            && let Some(condition) = parent.child_by_field_name("condition")
            && condition.id() != child.id()
            && !(condition.start_byte() <= node.start_byte()
                && node.end_byte() <= condition.end_byte())
        {
            return condition.utf8_text(source.as_bytes()).ok();
        }
        child = parent;
    }
    None
}

#[cfg(test)]
mod locate_tests;

#[cfg(test)]
mod examples_tests;

#[cfg(test)]
mod package_locator_tests;

#[cfg(test)]
mod encoding_tests;

#[cfg(test)]
mod availability_tests;
