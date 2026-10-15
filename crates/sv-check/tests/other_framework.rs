//! A finding from a rule written for a web framework the app does not use decides nothing (backlog 0521;
//! ADR-111, Later). In the three-tool trial of 14 October 2026, Semgrep's Django rule for a form with no
//! `{% csrf_token %}` fired on a FastAPI app's and a Flask app's Jinja forms, which carry their token in a
//! hidden input, and marked V3.5.1 "needs attention" on both.

use std::path::PathBuf;
use sv_check::adapters::{self, Adapters};
use sv_check::finding::Outranked;
use sv_scan::deps::{Declared, Unread};

const DJANGO_FORM: &str = "python.django.security.django-no-csrf-token.django-no-csrf-token";
const JINJA_SAFE: &str =
    "python.flask.security.xss.audit.template-unescaped-with-safe.template-unescaped-with-safe";
const FLASK_DEBUG: &str = "python.flask.security.audit.debug-enabled.debug-enabled";

fn adapters() -> Adapters {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json");
    Adapters::load(&data).expect("the adapter file loads")
}

/// The findings one Semgrep report of `rules` makes, each on its own line of `templates/login.html`.
fn found(adapters: &Adapters, rules: &[&str]) -> Vec<sv_check::Finding> {
    let semgrep = adapters
        .all()
        .iter()
        .find(|a| a.id == "semgrep")
        .expect("semgrep is listed");
    let results: Vec<serde_json::Value> = rules
        .iter()
        .enumerate()
        .map(|(i, id)| {
            serde_json::json!({
                "ruleId": id,
                "level": "warning",
                "message": { "text": "a finding" },
                "locations": [{ "physicalLocation": {
                    "artifactLocation": { "uri": "templates/login.html" },
                    "region": { "startLine": i + 1 }
                }}]
            })
        })
        .collect();
    let sarif = serde_json::json!({
        "version": "2.1.0",
        "runs": [{ "tool": { "driver": { "name": "Semgrep OSS", "rules": [] } }, "results": results }]
    })
    .to_string();
    let findings = adapters::parse_sarif(semgrep, &sarif).expect("the report reads");
    assert_eq!(findings.len(), rules.len(), "every result made a finding");
    findings
}

fn declared(packages: &[(&str, &str)]) -> Vec<Declared> {
    packages
        .iter()
        .map(|(ecosystem, name)| Declared {
            ecosystem: (*ecosystem).to_owned(),
            manifest: "requirements.txt".to_owned(),
            name: (*name).to_owned(),
        })
        .collect()
}

fn outranked(findings: &[sv_check::Finding], rule: &str) -> Option<Outranked> {
    findings
        .iter()
        .find(|f| f.rule_id == format!("semgrep.{rule}"))
        .expect("the rule's finding is there")
        .outranked
        .clone()
}

#[test]
fn a_django_rule_on_a_flask_app_is_set_apart_and_counts_against_nothing() {
    let adapters = adapters();
    let mut findings = found(&adapters, &[DJANGO_FORM, FLASK_DEBUG]);
    assert!(
        findings[0].requirement_ids.iter().any(|r| r == "V3.5.1"),
        "the Django rule is mapped to V3.5.1: {:?}",
        findings[0].requirement_ids
    );
    adapters::set_apart_other_frameworks(
        &adapters,
        &mut findings,
        &declared(&[("Python", "Flask"), ("Python", "flask_wtf")]),
        &[],
    );
    assert_eq!(
        outranked(&findings, DJANGO_FORM),
        Some(Outranked::OtherFramework {
            framework: "Django".to_owned()
        })
    );
    assert!(!findings[0].withholds_credit());
    // Flask's own rule, on a Flask app, still counts.
    assert_eq!(outranked(&findings, FLASK_DEBUG), None);
    assert!(findings[1].withholds_credit());
}

#[test]
fn a_django_app_keeps_its_django_finding() {
    let adapters = adapters();
    let mut findings = found(&adapters, &[DJANGO_FORM]);
    // Named in another case, as pip reads names without regard to it.
    adapters::set_apart_other_frameworks(
        &adapters,
        &mut findings,
        &declared(&[("Python", "DJANGO")]),
        &[],
    );
    assert_eq!(outranked(&findings, DJANGO_FORM), None);
}

#[test]
fn without_a_python_package_list_read_whether_the_app_uses_django_is_not_known() {
    let adapters = adapters();
    // No package list in Python at all: the Django finding counts as before.
    let mut findings = found(&adapters, &[DJANGO_FORM]);
    adapters::set_apart_other_frameworks(
        &adapters,
        &mut findings,
        &declared(&[("npm", "express")]),
        &[],
    );
    assert_eq!(outranked(&findings, DJANGO_FORM), None);
    // A Python list read, and another that could not be: the packages are not all known.
    let mut findings = found(&adapters, &[DJANGO_FORM]);
    adapters::set_apart_other_frameworks(
        &adapters,
        &mut findings,
        &declared(&[("Python", "flask")]),
        &[Unread {
            manifest: "pyproject.toml".to_owned(),
            why: "it could not be read".to_owned(),
        }],
    );
    assert_eq!(outranked(&findings, DJANGO_FORM), None);
}

#[test]
fn a_jinja_template_rule_in_the_flask_pack_still_counts_on_a_fastapi_app() {
    // Flask's template rules read Jinja, which FastAPI apps use too: they are the template engine's.
    let adapters = adapters();
    let mut findings = found(&adapters, &[JINJA_SAFE]);
    adapters::set_apart_other_frameworks(
        &adapters,
        &mut findings,
        &declared(&[("Python", "fastapi"), ("Python", "jinja2")]),
        &[],
    );
    assert_eq!(outranked(&findings, JINJA_SAFE), None);
}
