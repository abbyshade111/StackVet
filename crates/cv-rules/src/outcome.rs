//! The five outcomes of a setting (the design entry, "A setting that is missing, not only one that is wrong"), and how
//! each becomes evidence in `sv`'s own terms: a [`Finding`], a [`Verified`] credit, or a [`NotAssessed`] line.
//!
//! | Outcome | Evidence |
//! |---|---|
//! | Set, safe | A credit, always in part: one file says it, which is not the running system |
//! | Set, unsafe | A finding |
//! | Missing, default safe | A credit in part, with the default's source: an upgrade could change it |
//! | Missing, default unsafe | A finding, naming the default and the page it was read from |
//! | Unknown | Not assessed, with what would settle it: never a pass, never a failure |

use crate::defaults::KnownDefault;
use crate::unit::Layer;
use serde::Serialize;
use sv_core::{Confidence, Finding, Location, Severity, Verified};

/// Where a value was read: a file and line, and its layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Place {
    pub file: String,
    pub line: usize,
    pub layer: Layer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", rename_all = "kebab-case")]
pub enum Outcome {
    SetSafe { value: String, at: Place },
    SetUnsafe { value: String, at: Place },
    MissingDefaultSafe { defaults: Vec<KnownDefault> },
    MissingDefaultUnsafe { defaults: Vec<KnownDefault> },
    Unknown { why: String },
}

impl Outcome {
    pub fn name(&self) -> &'static str {
        match self {
            Outcome::SetSafe { .. } => "set, safe",
            Outcome::SetUnsafe { .. } => "set, unsafe",
            Outcome::MissingDefaultSafe { .. } => "missing, default safe",
            Outcome::MissingDefaultUnsafe { .. } => "missing, default unsafe",
            Outcome::Unknown { .. } => "not assessed",
        }
    }
}

/// What a rule says about its findings and credits, once per rule.
#[derive(Debug, Clone, Copy)]
pub struct Rule {
    pub id: &'static str,
    pub setting: &'static str,
    /// Cited only after reading each one's text in `data/frameworks/asvs-5.0.0.json`. Empty means exactly that.
    pub requirement_ids: &'static [&'static str],
    pub cwe: &'static [&'static str],
    pub severity: Severity,
    pub title: &'static str,
    pub impact: &'static str,
    pub fix: &'static str,
    /// What a credit covers, in a sentence.
    pub scope: &'static str,
}

/// One setting of one unit, settled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Assessed {
    pub rule_id: &'static str,
    pub setting: &'static str,
    /// The unit it is about: "compose service `web` in compose.yaml".
    pub unit: String,
    /// Where the unit is, for a finding about a setting that is missing.
    pub unit_at: Place,
    #[serde(flatten)]
    pub outcome: Outcome,
    /// The other tool's rule that reports this one instead (one rule, one home: ADR-157, decision 1). Its finding is
    /// then left to it, and the listing says so.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_by: Option<&'static str>,
}

/// Something that could not be settled, and what would settle it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NotAssessed {
    pub rule_id: String,
    pub requirement_ids: Vec<String>,
    pub unit: String,
    pub why: String,
}

/// The evidence a list of settled settings makes.
#[derive(Debug, Default, Serialize)]
pub struct Evidence {
    pub findings: Vec<Finding>,
    pub verified: Vec<Verified>,
    pub not_assessed: Vec<NotAssessed>,
}

impl Evidence {
    pub fn add(&mut self, rule: &Rule, assessed: &Assessed) {
        match &assessed.outcome {
            Outcome::SetSafe { value, at } => self.verified.push(
                Verified::new(
                    rule.id,
                    rule.requirement_ids,
                    format!(
                        "{} {}: `{}` is `{value}` in {} line {}. {}",
                        rule.scope,
                        assessed.unit,
                        rule.setting,
                        at.file,
                        at.line,
                        "One file says so; what runs may be set elsewhere."
                    ),
                )
                .in_part(),
            ),
            Outcome::MissingDefaultSafe { defaults } => self.verified.push(
                Verified::new(
                    rule.id,
                    rule.requirement_ids,
                    format!(
                        "{} {}: `{}` is not set, and the default is safe ({}). An upgrade could change the default.",
                        rule.scope,
                        assessed.unit,
                        rule.setting,
                        cited(defaults)
                    ),
                )
                .in_part(),
            ),
            Outcome::SetUnsafe { .. } if assessed.reported_by.is_some() => {}
            Outcome::SetUnsafe { value, at } => self.findings.push(finding(
                rule,
                Location {
                    file: at.file.clone(),
                    line: at.line,
                },
                Confidence::High,
                format!(
                    "{} sets `{}` to `{value}` ({} line {}).",
                    assessed.unit, rule.setting, at.file, at.line
                ),
            )),
            Outcome::MissingDefaultUnsafe { defaults } => self.findings.push(finding(
                rule,
                Location {
                    file: assessed.unit_at.file.clone(),
                    line: assessed.unit_at.line,
                },
                // The deployment may set it somewhere this folder does not hold.
                Confidence::Medium,
                format!(
                    "{} does not set `{}`, and the default is unsafe: {}",
                    assessed.unit,
                    rule.setting,
                    defaults
                        .iter()
                        .map(|d| format!("{} ({})", d.why, d.sources[0].url))
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
            )),
            Outcome::Unknown { why } => self.not_assessed.push(NotAssessed {
                rule_id: rule.id.to_owned(),
                requirement_ids: rule.requirement_ids.iter().map(|s| (*s).to_owned()).collect(),
                unit: assessed.unit.clone(),
                why: why.clone(),
            }),
        }
    }
}

fn cited(defaults: &[KnownDefault]) -> String {
    defaults
        .iter()
        .map(|d| format!("{}: {}, {}", d.subject, d.default, d.sources[0].url))
        .collect::<Vec<_>>()
        .join("; ")
}

fn finding(
    rule: &Rule,
    location: Location,
    confidence: Confidence,
    description: String,
) -> Finding {
    sv_core::finding::found(Finding {
        rule_id: rule.id.to_owned(),
        title: rule.title.to_owned(),
        severity: rule.severity,
        confidence,
        location,
        secret: None,
        requirement_ids: rule
            .requirement_ids
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        cwe: rule.cwe.iter().map(|s| (*s).to_owned()).collect(),
        description,
        impact: rule.impact.to_owned(),
        fix: rule.fix.to_owned(),
        also_reported_by: Vec::new(),
        evidence: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        outranked: None,
    })
}
