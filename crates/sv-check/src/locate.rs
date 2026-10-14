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
    /// `require_role` and `requireRole` are one name.
    pub names: String,
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
}

struct Compiled {
    locator: Locator,
    names: Regex,
    queries: BTreeMap<String, Query>,
}

pub struct Locators {
    compiled: Vec<Compiled>,
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
                queries,
            });
        }
        Ok(Locators { compiled })
    }

    pub fn all(&self) -> impl Iterator<Item = &Locator> {
        self.compiled.iter().map(|c| &c.locator)
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
            // Most files hold no name any locator knows, and are not parsed for one.
            let words: BTreeSet<String> = word_names(&source).collect();
            let wanted: Vec<&Compiled> = wanted
                .into_iter()
                .filter(|c| words.iter().any(|w| c.names.is_match(w)))
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

#[cfg(test)]
mod locate_tests;
