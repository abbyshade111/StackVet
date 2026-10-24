//! A few lines for each web framework, given to the AI coding tool after the coding rules when the
//! app's package lists name the framework (ADR-143, backlog 0663).
//!
//! The coding rules (`coding_rules`) say how the tool behaves; none says how to write the code in a
//! given framework, so the tool building a Flask app is never told that Jinja escapes for it until
//! `|safe` is used on a value from the request, which is what the check then finds. These lines say
//! it first. `data/stack-guidance.json` holds them, in this project's own words.
//!
//! # What holds each line true
//!
//! - **A framework is found only by a package the app declares** (`sv_scan::deps`). Finding none is
//!   not evidence the app uses none: a package list `sv` could not read is named beside the answer.
//! - **Every line names the code rules that find its fault**, and `load` refuses an id
//!   `data/ast-rules.json` does not have. A line cites what those rules cite, so no citation here
//!   is made on its own.
//! - **Every line carries a fault**: a few lines of the framework's code with the mistake in it,
//!   which the named rule must report (`tests/stack_guidance.rs`). They are never shown.
//!
//! # What it is worth
//!
//! Nothing, as evidence, like the coding rules: no requirement changes status because a line was
//! given.

use crate::ast::AstRules;
use anyhow::{Result, ensure};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use sv_scan::deps::{Declared, Unread};

#[derive(Debug, Clone, Deserialize)]
pub struct StackGuidance {
    /// One sentence saying whose lines these are, rendered under them wherever they go.
    pub credit: String,
    pub frameworks: Vec<Framework>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Framework {
    pub id: String,
    pub name: String,
    /// Ecosystem, by `sv_scan::ecosystems`' name for it ("Python", "npm"), → the packages that show
    /// the framework is used.
    pub packages: BTreeMap<String, Vec<String>>,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Line {
    pub line: String,
    /// The code rules that find the fault this line guards against.
    pub checks: Vec<String>,
    /// The fault, written in the framework, for the test alone.
    pub faults: Vec<Fault>,
    /// The requirements the checks cite, filled in by `load`.
    #[serde(skip)]
    pub cites: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fault {
    pub check: String,
    pub language: String,
    pub file: String,
    pub code: String,
}

/// A framework the app's package lists name, and the lists that name it, by file name alone.
///
/// Only the file name: it is one of the fixed names `sv_scan::ecosystems` looks for, while the
/// folders above it are named by whoever wrote the app, and these lines go into `AGENTS.md`, which the
/// AI tool reads as orders (ADR-066, point 5: what comes from the app is never given as an instruction).
#[derive(Debug, Clone)]
pub struct Found<'a> {
    pub framework: &'a Framework,
    pub manifests: Vec<String>,
}

impl StackGuidance {
    /// Reads the file, refusing a line that names a code rule `rules` does not hold, has no fault, or
    /// carries a fault for a check it does not name.
    pub fn load(path: &Path, rules: &AstRules) -> Result<StackGuidance> {
        let mut guidance: StackGuidance = sv_frameworks::data::read_json(path)?;
        let cited: BTreeMap<&str, &[String]> = rules
            .rules()
            .map(|r| (r.id.as_str(), r.requirement_ids.as_slice()))
            .collect();
        let mut ids = BTreeSet::new();
        for framework in &mut guidance.frameworks {
            ensure!(
                ids.insert(framework.id.clone()),
                "{}: the framework {} is listed twice",
                path.display(),
                framework.id
            );
            ensure!(
                framework.packages.values().any(|names| !names.is_empty()),
                "{}: {} names no package, so it could never be found",
                path.display(),
                framework.id
            );
            for line in &mut framework.lines {
                ensure!(
                    !line.checks.is_empty(),
                    "{}: a line for {} names no check: {}",
                    path.display(),
                    framework.id,
                    line.line
                );
                let mut cites = Vec::new();
                for check in &line.checks {
                    let Some(requirements) = cited.get(check.as_str()) else {
                        anyhow::bail!(
                            "{}: a line for {} names {check}, which is not a code rule",
                            path.display(),
                            framework.id
                        );
                    };
                    for id in requirements.iter() {
                        if !cites.contains(id) {
                            cites.push(id.clone());
                        }
                    }
                }
                ensure!(
                    !cites.is_empty(),
                    "{}: a line for {} cites nothing through its checks: {}",
                    path.display(),
                    framework.id,
                    line.line
                );
                line.cites = cites;
                ensure!(
                    !line.faults.is_empty(),
                    "{}: a line for {} has no fault for its check to find: {}",
                    path.display(),
                    framework.id,
                    line.line
                );
                for fault in &line.faults {
                    ensure!(
                        line.checks.contains(&fault.check),
                        "{}: a fault for {} is for {}, which its line does not name",
                        path.display(),
                        framework.id,
                        fault.check
                    );
                }
                // Each check the line names must be shown finding something, or the line could name
                // one that never looks at the framework's code.
                for check in &line.checks {
                    ensure!(
                        line.faults.iter().any(|f| &f.check == check),
                        "{}: a line for {} names {check} with no fault for it to find",
                        path.display(),
                        framework.id
                    );
                }
            }
        }
        Ok(guidance)
    }

    /// The frameworks `declared` names, in the file's order.
    pub fn found(&self, declared: &[Declared]) -> Vec<Found<'_>> {
        self.frameworks
            .iter()
            .filter_map(|framework| {
                let manifests: BTreeSet<String> = declared
                    .iter()
                    .filter(|d| {
                        framework.packages.get(&d.ecosystem).is_some_and(|names| {
                            names.iter().any(|n| same_package(&d.ecosystem, n, &d.name))
                        })
                    })
                    .map(|d| sv_scan::ecosystems::file_name(&d.manifest).to_owned())
                    .collect();
                (!manifests.is_empty()).then(|| Found {
                    framework,
                    manifests: manifests.into_iter().collect(),
                })
            })
            .collect()
    }

    /// The lines for `found`, as Markdown, under a heading of their own. With none found, it says
    /// which frameworks were looked for, and any package list that could not be read, so a silence is
    /// never taken for "no framework".
    pub fn markdown(&self, found: &[Found<'_>], unread: &[Unread]) -> String {
        let mut out = String::from("## Lines for the frameworks this app uses\n\n");
        if found.is_empty() {
            let names: Vec<&str> = self.frameworks.iter().map(|f| f.name.as_str()).collect();
            out.push_str(&format!(
                "None of the frameworks `sv` has lines for ({}) is named in the app's package lists.\n\n",
                names.join(", ")
            ));
        } else {
            out.push_str(
                "`sv` found these frameworks in the app's package lists. Each line names the `sv` check \
                 that looks for the mistake it guards against, and the requirements that check cites. \
                 Like the rules above, they are instructions, not a check.\n\n",
            );
            for Found {
                framework,
                manifests,
            } in found
            {
                out.push_str(&format!(
                    "### {} (named in {})\n\n",
                    framework.name,
                    manifests.join(", ")
                ));
                for line in &framework.lines {
                    let checks: Vec<String> =
                        line.checks.iter().map(|c| format!("`{c}`")).collect();
                    out.push_str(&format!(
                        "- {} (checked by {}; {})\n",
                        line.line,
                        checks.join(", "),
                        line.cites.join(", ")
                    ));
                }
                out.push('\n');
            }
        }
        // By file name, as above; the reason is `sv`'s own words, never the file's.
        for list in unread {
            out.push_str(&format!(
                "`sv` could not read a {} ({}), so a framework named only there is not seen.\n\n",
                sv_scan::ecosystems::file_name(&list.manifest),
                list.why
            ));
        }
        out.push_str(&format!("{}\n", self.credit));
        out
    }
}

/// Whether a declared name is the listed package. Python's package index treats case, `-`, `_`, and
/// `.` as one (PEP 503), so `Flask` in a requirements file is `flask`; npm, crates, gems, and Go
/// modules are compared as written.
fn same_package(ecosystem: &str, listed: &str, declared: &str) -> bool {
    if ecosystem == "Python" {
        let normal = |s: &str| s.to_ascii_lowercase().replace(['_', '.'], "-");
        normal(listed) == normal(declared)
    } else {
        listed == declared
    }
}
