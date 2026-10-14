//! A requirement's reason for not applying never states something untrue about the app (backlog 0403). In the owner's
//! trial, an app with no AI feature was told its "conversation history is plain rows in the app's own database" and that
//! moderation "can be turned on later with the AI_MODERATION setting": reasons written for SecureVibe v1's own app, and
//! labelled as though the owner had said them. Two guards:
//! - an app with no AI feature is told that, for every requirement a condition about the AI turns off;
//! - no reason names a setting, a file, or a feature that none of its scope's conditions establishes.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::{ApplicabilityConfig, ConditionContext, bucket};
use sv_frameworks::{Condition, Source};

fn data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

#[test]
fn an_app_with_no_ai_feature_is_told_that_and_nothing_about_an_ai() {
    let frameworks = Frameworks::load(&data().join("frameworks")).unwrap();
    let config = ApplicabilityConfig::load_v2(
        &data().join("knowledge"),
        &data().join("applicability-v2.json"),
    )
    .unwrap();
    // Every condition answered no, so each requirement a rule turns off is decided, and the AI's is "no" too.
    let mut ctx = ConditionContext::default();
    for c in Condition::ALL {
        if !matches!(c, Condition::Always | Condition::Never) {
            ctx.set(*c, false);
        }
    }
    let buckets = bucket(&frameworks, &config, &ctx, 3);
    let about_the_ai: Vec<_> = buckets
        .not_applicable
        .iter()
        .filter(|na| na.condition == Condition::Ai)
        .collect();
    // The setup: the AI requirements were turned off, among them those the trial's report gave other reasons for.
    for id in ["C8.1.1", "C7.3.1", "C9.2.1"] {
        assert!(
            about_the_ai.iter().any(|na| na.id == id),
            "{id} is not ruled out by the AI's answer: {:?}",
            buckets.not_applicable.iter().find(|na| na.id == id)
        );
    }
    for na in &about_the_ai {
        assert!(
            na.reason.starts_with("This app has no AI features"),
            "{}: {}",
            na.id,
            na.reason
        );
        assert_eq!(na.source, Source::Claim);
    }
    // No requirement is ruled out by a condition about what the AI does, when there is no AI.
    let inner: Vec<&str> = buckets
        .not_applicable
        .iter()
        .filter(|na| na.condition.within() == Some(Condition::Ai))
        .map(|na| na.id.as_str())
        .collect();
    assert!(inner.is_empty(), "{inner:?}");
}

/// Words that name a feature, and the conditions whose "no" establishes that the app lacks it. A reason may say the app
/// lacks a feature only when one of its scope's conditions does.
const FEATURES: &[(&str, &[&str])] = &[
    ("payment", &["payments"]),
    ("background job", &["scheduler", "multiple-services"]),
    ("job queue", &["scheduler", "multiple-services"]),
    ("send email", &["email"]),
    ("not on the internet", &["internet"]),
    ("file upload", &["uploads"]),
    ("vector database", &["rag", "ai-history"]),
    ("document store", &["rag", "web-search"]),
    ("conversation history", &["ai-history"]),
    ("search the web", &["web-search", "rag"]),
    ("no ai features", &["ai"]),
    ("no sign-in", &["auth"]),
    ("without https", &["tls", "internet"]),
    (
        "other services over the network",
        &["external-apis", "multiple-services"],
    ),
];

/// Words that only ever describe one app: a file, a setting of its own, or how one app happened to be built.
const ONE_APPS_DETAIL: &[&str] = &[
    "database file",
    "local database",
    "plain rows",
    "loopback",
    "opaque values",
    "built-in safety",
    "turned on later",
    "still screened",
    "ready-made model",
    "pinned by name",
    "monitored by the vendor",
];

#[test]
fn no_reason_names_what_its_conditions_do_not_establish() {
    let read = |name: &str| -> serde_json::Value {
        serde_json::from_str(&std::fs::read_to_string(data().join(name)).unwrap()).unwrap()
    };
    let base = read("knowledge/applicability.json");
    let overlay = read("applicability-v2.json");
    let replaced: BTreeSet<String> = overlay["rules"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["scope"].as_str().unwrap().to_owned())
        .collect();
    // The rules in force: the overlay's, and the base file's at every scope it does not replace.
    let rules: Vec<&serde_json::Value> = base["rules"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| !replaced.contains(r["scope"].as_str().unwrap()))
        .chain(overlay["rules"].as_array().unwrap())
        .collect();
    let mut conditions: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for r in &rules {
        conditions
            .entry(r["scope"].as_str().unwrap())
            .or_default()
            .insert(r["condition"].as_str().unwrap());
    }
    assert!(rules.len() > 300, "the setup: the rules were read");
    let setting = regex_lite_setting();
    let mut untrue = Vec::new();
    for r in &rules {
        let Some(reason) = r["notApplicableReason"].as_str() else {
            continue;
        };
        let scope = r["scope"].as_str().unwrap();
        // 0468 rewords this one's reason, and holds it on its own branch.
        if scope == "V6.6.2" {
            continue;
        }
        let lower = reason.to_lowercase();
        let held = &conditions[scope];
        if let Some(word) = setting(reason) {
            untrue.push(format!("{scope}: names a setting, {word}"));
        }
        for detail in ONE_APPS_DETAIL {
            if lower.contains(detail) {
                untrue.push(format!("{scope}: describes one app, \"{detail}\""));
            }
        }
        for (feature, by) in FEATURES {
            if lower.contains(feature) && !by.iter().any(|c| held.contains(c)) {
                untrue.push(format!(
                    "{scope}: says \"{feature}\", which none of {held:?} establishes"
                ));
            }
        }
    }
    // The default reasons, for a rule that states none, name only their own condition's answer.
    for c in Condition::ALL {
        let reason = c.default_not_applicable_reason();
        if let Some(word) = setting(reason) {
            untrue.push(format!("default for {}: names a setting, {word}", c.name()));
        }
        for detail in ONE_APPS_DETAIL {
            if reason.to_lowercase().contains(detail) {
                untrue.push(format!(
                    "default for {}: describes one app, \"{detail}\"",
                    c.name()
                ));
            }
        }
    }
    assert!(untrue.is_empty(), "{}", untrue.join("\n"));
}

/// The first word of `text` written as a setting's name is: capitals, digits, and at least one `_`
/// (`AI_MODERATION`). Without a regular-expression crate, by hand.
fn regex_lite_setting() -> impl Fn(&str) -> Option<String> {
    |text: &str| {
        text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .find(|w| {
                w.contains('_')
                    && w.chars().any(|c| c.is_ascii_uppercase())
                    && w.chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            })
            .map(str::to_owned)
    }
}
