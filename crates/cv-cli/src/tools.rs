//! The outside tools `cv` runs over the deployment files it read: hadolint, Checkov, and Trivy (backlog 0693; ADR-157,
//! part 3). They are described in `data/cv-adapters.json` and run by `sv`'s own runner (`sv_check::adapters`, ADR-018),
//! so a tool is found, timed, judged by its exit code, read as SARIF, and redacted exactly as `sv`'s are. A tool that is
//! not installed, or did not finish, is said so, and is never a clean pass. A tool's finding that one of `cv`'s own
//! rules also made is one finding, `cv`'s, naming the tool's rule (`also_reported_by`; the owner's choice, 21 November
//! 2026).

use cv_rules::unit::Layer;
use serde::Serialize;
use std::path::Path;
use sv_check::adapters::{Adapter, Adapters, NotRunCause, Outcome, PrivateFolder, run_one_handed};
use sv_check::secrets::SecretRules;
use sv_core::Finding;

/// The tools' rules that report what `cv`'s own rule does, by `cv`'s rule: each read against what it detects
/// (hadolint 2.15.1, Checkov 3.4.6, Trivy 0.74.0, 21 November 2026). One of these on the same file as `cv`'s finding
/// is folded into it.
pub const AGREES: &[(&str, &[&str])] = &[(
    cv_rules::rules::CONTAINER_ROOT.id,
    &[
        "hadolint.DL3002",
        "checkov.CKV_DOCKER_8",
        "checkov.CKV_K8S_23",
        "trivy.DS-0002",
        "trivy.KSV-0012",
    ],
)];

/// One tool's part in a run, as `report.json` and the terminal say it.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ToolLine {
    pub id: String,
    pub name: String,
    /// "ran", "ran in part", or "not run".
    pub status: &'static str,
    /// The first line the tool gave for its version, redacted as `sv` redacts what a tool says.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Why it did not run, or what it was kept from looking at.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub said: Vec<String>,
    /// The files it was handed by name, relative to the folder; empty when it was given the folder.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub handed: Vec<String>,
    /// How many findings it made, and how many of those `cv`'s own rules had made too.
    pub findings: usize,
    pub agreed: usize,
}

/// What the tools found, and how each run went.
#[derive(Debug, Default, Serialize)]
pub struct Tools {
    pub tools: Vec<ToolLine>,
    /// The findings no rule of `cv`'s made.
    pub findings: Vec<Finding>,
}

/// The tools as `data/cv-adapters.json` describes them.
pub fn load() -> anyhow::Result<Adapters> {
    Adapters::load(&sv_frameworks::data::file("cv-adapters.json"))
}

/// The files a tool is handed: the deployment files `cv` read, of the kinds the tool reads (its `language`, in
/// `cv`'s names for the layers). A file `cv` could not read is never handed, so a tool cannot quietly pass over it.
pub fn handed(adapter: &Adapter, read: &[(Layer, String)]) -> Vec<String> {
    let kinds: Vec<&str> = adapter.language.split(',').map(str::trim).collect();
    let mut files: Vec<String> = read
        .iter()
        .filter(|(layer, _)| kinds.contains(&layer_kind(*layer)))
        .map(|(_, file)| file.clone())
        .collect();
    files.sort();
    files.dedup();
    files
}

fn layer_kind(layer: Layer) -> &'static str {
    match layer {
        Layer::Dockerfile => "dockerfile",
        Layer::Compose => "compose",
        Layer::Kubernetes => "kubernetes",
        Layer::TerraformPlan => "terraform_plan",
        Layer::DjangoSettings => "django_settings",
    }
}

/// Runs every tool over the folder, folding into `own` each tool finding one of `cv`'s rules made too.
pub fn run(
    adapters: &Adapters,
    folder: &Path,
    read: &[(Layer, String)],
    own: &mut [Finding],
    rules: &SecretRules,
) -> Tools {
    let mut out = Tools::default();
    let listing = sv_scan::files::Listing::of(folder);
    let private = match PrivateFolder::new_in(&std::env::temp_dir()) {
        Ok(private) => private,
        Err(e) => {
            for adapter in adapters.all() {
                out.tools.push(not_run(
                    adapter,
                    format!("cv could not make a private folder for its report ({e})"),
                ));
            }
            return out;
        }
    };
    for adapter in adapters.all() {
        let files = handed(adapter, read);
        let gives_folder = !adapter.run.args.iter().any(|a| a == "{files}");
        if gives_folder && files.is_empty() {
            // Given the folder, a tool would read files `cv` did not, and finding nothing there would vouch for none.
            out.tools.push(not_run(
                adapter,
                format!("this folder has none of the files {} reads", adapter.name),
            ));
            continue;
        }
        let report = private.path().join(format!("{}.sarif", adapter.id));
        let (outcome, record) = run_one_handed(adapter, &listing, &files, &report, rules);
        let version = record.version.clone().filter(|v| !v.is_empty());
        match outcome {
            Outcome::NotRun { why, cause } => {
                let mut line = not_run(adapter, why);
                if cause != NotRunCause::NotInstalled {
                    line.version = version;
                }
                out.tools.push(line);
            }
            Outcome::Ran {
                findings,
                looked_away,
                handed,
                ..
            } => {
                let made = findings.len();
                let mut agreed = 0;
                for mut finding in findings {
                    tidy(adapter, &mut finding);
                    if fold_into(own, &finding) {
                        agreed += 1;
                    } else {
                        out.findings.push(finding);
                    }
                }
                out.tools.push(ToolLine {
                    id: adapter.id.clone(),
                    name: adapter.name.clone(),
                    status: if looked_away.is_empty() {
                        "ran"
                    } else {
                        "ran in part"
                    },
                    version,
                    said: looked_away,
                    handed,
                    findings: made,
                    agreed,
                });
            }
        }
    }
    out
}

/// A tool's finding in `cv`'s terms. A file handed by name comes back as `./Dockerfile` from hadolint, and `cv` names it
/// `Dockerfile`, so the two would never meet. hadolint's report describes none of its rules, so `sv`'s runner titles its
/// findings "hadolint reported DL3007"; the map's words, or else the tool's own message, say what was found instead.
pub fn tidy(adapter: &Adapter, finding: &mut Finding) {
    if let Some(file) = finding.location.file.strip_prefix("./") {
        finding.location.file = file.to_owned();
    }
    let id = finding
        .rule_id
        .strip_prefix(&format!("{}.", adapter.id))
        .unwrap_or(&finding.rule_id)
        .to_owned();
    if finding.title == format!("{} reported {id}", adapter.name) {
        let said = finding.description.lines().next().unwrap_or("").trim();
        if let Some(rule) = adapter.rules.get(&id) {
            finding.title = capitalized(&rule.what);
        } else if !said.is_empty() {
            finding.title = said.to_owned();
        }
    }
}

fn capitalized(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .map(|c| c.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

fn not_run(adapter: &Adapter, why: String) -> ToolLine {
    ToolLine {
        id: adapter.id.clone(),
        name: adapter.name.clone(),
        status: "not run",
        version: None,
        said: vec![why],
        handed: Vec::new(),
        findings: 0,
        agreed: 0,
    }
}

/// Folds a tool's finding into `cv`'s own finding of the same weakness in the same file, naming the tool's rule.
/// Whether it did.
pub fn fold_into(own: &mut [Finding], tool: &Finding) -> bool {
    let Some((cv_rule, _)) = AGREES
        .iter()
        .find(|(_, theirs)| theirs.contains(&tool.rule_id.as_str()))
    else {
        return false;
    };
    let Some(mine) = own
        .iter_mut()
        .find(|f| f.rule_id == *cv_rule && f.location.file == tool.location.file)
    else {
        return false;
    };
    if !mine.also_reported_by.contains(&tool.rule_id) {
        mine.also_reported_by.push(tool.rule_id.clone());
    }
    true
}

#[cfg(test)]
#[path = "tools_tests.rs"]
mod tools_tests;
