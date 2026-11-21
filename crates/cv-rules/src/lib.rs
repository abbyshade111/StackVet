//! ConfigVet's rules over the files that say how an app is deployed (ADR-157; backlog 0687, part 2 of the design
//! entry's build order). Files only: nothing in this crate opens a network connection.
//!
//! A run reads the folder once (`sv_scan::files::Listing`, the walk `sv` uses), turns each deployment layer into
//! [`unit::Unit`]s (a Dockerfile, a compose file, Kubernetes manifests, a Terraform plan as JSON), and asks each rule
//! of [`rules`] about every unit. Each answer is one of five outcomes ([`outcome::Outcome`]), and each outcome becomes
//! evidence in `sv`'s own types (`sv_core::Finding`, `sv_core::Verified`) or a line saying what was not assessed.
//!
//! One rule has one home (ADR-157, decision 1): a check `sv` already has stays in `sv`. Where a rule here meets a
//! value `sv` reports (a Dockerfile's final stage setting `NODE_ENV=development`, `sv`'s
//! `config.development-server-started`), it lists the value and leaves the finding to `sv`.

pub mod compose;
pub mod defaults;
pub mod dockerfile;
pub mod kubernetes;
pub mod outcome;
pub mod rules;
pub mod terraform;
pub mod unit;

use serde::Serialize;
use std::path::Path;
use sv_scan::files::{Entry, Listing};
use unit::Unit;

/// A Dockerfile, read.
#[derive(Debug, Clone)]
pub struct Dockerfile {
    pub file: String,
    pub stages: Vec<dockerfile::Stage>,
}

/// A file the rules wanted and could not read, and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Unread {
    pub file: String,
    pub why: String,
}

/// What the folder holds that the rules read.
#[derive(Debug, Default)]
pub struct Folder {
    pub dockerfiles: Vec<Dockerfile>,
    /// Compose services, Kubernetes containers, and ECS containers.
    pub units: Vec<Unit>,
    /// `package.json` files, with their text.
    pub package_jsons: Vec<(String, String)>,
    /// Python dependency files (`requirements*.txt`, `pyproject.toml`, `Pipfile`), with their text.
    pub python_deps: Vec<(String, String)>,
    /// Django settings modules (`settings.py`, or a `.py` file in a `settings` folder), with their text.
    pub django_settings: Vec<(String, String)>,
    /// The deployment files read, by layer.
    pub read: Vec<(unit::Layer, String)>,
    pub unread: Vec<Unread>,
}

/// Whether a file's name says it is a Dockerfile: `Dockerfile`, `Dockerfile.prod`, `api.Dockerfile`.
pub fn is_dockerfile(file_name: &str) -> bool {
    let lower = file_name.to_ascii_lowercase();
    lower == "dockerfile" || lower.starts_with("dockerfile.") || lower.ends_with(".dockerfile")
}

fn is_python_deps(file_name: &str) -> bool {
    let lower = file_name.to_ascii_lowercase();
    (lower.starts_with("requirements") && lower.ends_with(".txt"))
        || lower == "pyproject.toml"
        || lower == "pipfile"
}

fn is_django_settings(relative: &str) -> bool {
    relative.ends_with(".py")
        && (relative == "settings.py"
            || relative.ends_with("/settings.py")
            || relative.starts_with("settings/")
            || relative.contains("/settings/"))
}

impl Folder {
    /// Reads every file of the folder the rules use. Test code is left out, as `sv` leaves it out.
    pub fn read(root: &Path) -> Folder {
        let listing = Listing::of(root);
        let mut folder = Folder::default();
        for entry in listing.app_files() {
            if listing.is_test_code(&entry.relative) {
                continue;
            }
            folder.take(entry);
        }
        folder
    }

    fn take(&mut self, entry: &Entry) {
        let name = entry.file_name();
        let relative = entry.relative.as_str();
        let lower = name.to_ascii_lowercase();
        let yaml = lower.ends_with(".yml") || lower.ends_with(".yaml");
        let wanted = is_dockerfile(name)
            || yaml
            || lower.ends_with(".json")
            || is_python_deps(name)
            || is_django_settings(relative);
        if !wanted || relative.starts_with(".github/") {
            return;
        }
        let text = match entry.read_text() {
            Ok(text) => text,
            Err(why) => {
                // Only a file whose name alone says the rules need it is worth naming as unread.
                if is_dockerfile(name) || compose::is_compose(name) || lower == "package.json" {
                    self.unread.push(Unread {
                        file: relative.to_owned(),
                        why: why.explain().to_owned(),
                    });
                }
                return;
            }
        };
        if is_dockerfile(name) {
            match dockerfile::parse(&text) {
                Ok(stages) => {
                    self.read
                        .push((unit::Layer::Dockerfile, relative.to_owned()));
                    self.dockerfiles.push(Dockerfile {
                        file: relative.to_owned(),
                        stages,
                    });
                }
                Err(why) => self.unread(relative, why),
            }
        } else if yaml && compose::is_compose(name) {
            self.layer(
                unit::Layer::Compose,
                relative,
                compose::read(relative, &text),
            );
        } else if yaml && kubernetes::looks_like(&text) {
            self.layer(
                unit::Layer::Kubernetes,
                relative,
                kubernetes::read(relative, &text),
            );
        } else if lower == "package.json" {
            self.package_jsons.push((relative.to_owned(), text));
        } else if lower.ends_with(".json") && terraform::looks_like(&text) {
            self.layer(
                unit::Layer::TerraformPlan,
                relative,
                terraform::read(relative, &text),
            );
        } else if is_python_deps(name) {
            self.python_deps.push((relative.to_owned(), text));
        } else if is_django_settings(relative) {
            self.django_settings.push((relative.to_owned(), text));
        }
    }

    fn layer(&mut self, layer: unit::Layer, relative: &str, units: Result<Vec<Unit>, String>) {
        match units {
            Ok(units) => {
                self.read.push((layer, relative.to_owned()));
                self.units.extend(units);
            }
            Err(why) => self.unread(relative, why),
        }
    }

    fn unread(&mut self, relative: &str, why: String) {
        self.unread.push(Unread {
            file: relative.to_owned(),
            why,
        });
    }

    pub fn dockerfile(&self, relative: &str) -> Option<&Dockerfile> {
        self.dockerfiles.iter().find(|d| d.file == relative)
    }
}

/// Everything one `cv check` found.
#[derive(Debug, Serialize)]
pub struct Checked {
    pub assessed: Vec<outcome::Assessed>,
    #[serde(flatten)]
    pub evidence: outcome::Evidence,
    pub read: Vec<(unit::Layer, String)>,
    pub unread: Vec<Unread>,
}

/// Reads the folder and runs every rule, with the given table of defaults.
pub fn check(root: &Path, table: &defaults::Table) -> Checked {
    let folder = Folder::read(root);
    let mut assessed = Vec::new();
    let mut evidence = outcome::Evidence::default();
    for (rule, settled) in rules::run(&folder, table) {
        for one in settled {
            evidence.add(rule, &one);
            assessed.push(one);
        }
    }
    Checked {
        assessed,
        evidence,
        read: folder.read,
        unread: folder.unread,
    }
}

#[cfg(test)]
#[path = "parsers_tests.rs"]
mod parsers_tests;

#[cfg(test)]
#[path = "rules_tests.rs"]
mod rules_tests;
