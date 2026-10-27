//! The OSCAL export (ADR-156, backlog 0678): held to NIST's own schema for OSCAL 1.2.3 assessment results, offline,
//! and to what it may conclude. Only *checked* is `satisfied` and only *needs attention* is `not-satisfied`, each for
//! an ASVS objective; every other status is an observation with no finding; *not verified* has neither; and no NIST
//! SP 800-53 control is ever a reviewed control or the target of a finding.

use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn data(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data")
        .join(file)
}

fn read(path: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// What NIST's schema says is wrong with `document`, as lines; empty when it is valid.
fn schema_errors(document: &Value) -> Vec<String> {
    let schema = read(&data("oscal-assessment-results-1.2.3.schema.json"));
    let id = schema["$id"]
        .as_str()
        .expect("the schema names itself")
        .to_owned();
    let mut schemas = boon::Schemas::new();
    let mut compiler = boon::Compiler::new();
    compiler.enable_format_assertions();
    compiler
        .add_resource(&id, schema)
        .expect("the schema loads");
    let index = compiler
        .compile(&id, &mut schemas)
        .expect("the schema compiles");
    match schemas.validate(document, index) {
        Ok(()) => Vec::new(),
        Err(e) => format!("{e:#}").lines().map(str::to_owned).collect(),
    }
}

/// What `document` concludes that `report` does not allow, as lines; empty when it says no more than the report.
fn says_more_than(document: &Value, report: &Value) -> Vec<String> {
    let mut wrong = Vec::new();
    let result = &document["assessment-results"]["results"][0];
    let status: std::collections::BTreeMap<String, String> = report["requirements"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["id"].as_str().unwrap().to_ascii_lowercase(),
                r["status"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    for finding in result["findings"].as_array().into_iter().flatten() {
        let target = finding["target"]["target-id"].as_str().unwrap_or("");
        let control = target.strip_suffix("_obj").unwrap_or(target);
        let state = finding["target"]["status"]["state"].as_str().unwrap_or("");
        match (status.get(control).map(String::as_str), state) {
            (Some("checked"), "satisfied") | (Some("needs-attention"), "not-satisfied") => {}
            (s, state) => wrong.push(format!("{target} is {state}, and the report says {s:?}")),
        }
    }
    let concluded: Vec<&str> = result["findings"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|f| f["target"]["target-id"].as_str())
        .collect();
    for (control, s) in &status {
        let should = s == "checked" || s == "needs-attention";
        if control.starts_with('v')
            && should != concluded.contains(&format!("{control}_obj").as_str())
        {
            wrong.push(format!(
                "{control} is {s}, and a finding is {}",
                if should { "missing" } else { "there" }
            ));
        }
    }
    let observed: Vec<&str> = result["observations"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|o| o["title"].as_str())
        .collect();
    for (control, s) in &status {
        if s == "not-verified"
            && observed
                .iter()
                .any(|t| t.to_ascii_lowercase().starts_with(&format!("{control}:")))
        {
            wrong.push(format!(
                "{control} is not verified, and something is observed of it"
            ));
        }
    }
    // No 800-53 control is reviewed or concluded: the reviewed controls are ASVS's, by their ids.
    for selection in result["reviewed-controls"]["control-selections"]
        .as_array()
        .into_iter()
        .flatten()
    {
        for c in selection["include-controls"]
            .as_array()
            .into_iter()
            .flatten()
        {
            let id = c["control-id"].as_str().unwrap_or("");
            if !id.starts_with('v') {
                wrong.push(format!(
                    "{id} is a reviewed control, and only ASVS requirements are"
                ));
            }
        }
    }
    wrong
}

/// A report holding one ASVS requirement at each status, as `report.json` writes them.
fn every_status() -> Value {
    let row = |id: &str, status: &str, field: &str, whose: Option<&str>| {
        let mut credit = json!({ "check_id": format!("check.{id}"), "scope": "app" });
        if let Some(w) = whose {
            credit["whose"] = json!(w);
        }
        let mut r = json!({
            "id": id, "chapter": "x", "level": 1, "status": status, "findings": [], "information": [],
            "checked_by": [], "supported_by": [], "tested_by": [], "documented_by": [], "attested_by": [], "by_hand": [],
        });
        if !field.is_empty() {
            r[field] = json!([credit]);
        }
        r
    };
    let mut attention = row("V1.2.4", "needs-attention", "", None);
    attention["findings"] = json!(["finding-1"]);
    json!({
        "app_name": "Example",
        "target_level": 1,
        "sv": { "version": "0.1.0", "commit": "abc", "uncommitted_changes": false },
        "run_record": { "started": "2026-10-27T12:00:00Z", "started_unix_ms": 0, "securevibe_toml_sha256": "x", "run_id": "0123456789ab" },
        "requirements": [
            attention,
            row("V6.2.1", "checked", "checked_by", None),
            row("V7.3.1", "checked-in-part", "checked_by", None),
            row("V8.2.1", "app-tested", "tested_by", None),
            row("V13.1.1", "documented", "documented_by", Some("the owner")),
            row("V6.3.1", "by-hand", "by_hand", Some("the owner")),
            row("V8.3.1", "attested", "attested_by", Some("the owner")),
            row("V8.2.2", "stated", "attested_by", Some("the AI coding tool")),
            row("V3.4.1", "not-verified", "", None),
            row("C1.1.1", "checked", "checked_by", None),
        ],
        "requirement_text": {
            "V1.2.4": "a", "V6.2.1": "b", "V7.3.1": "c", "V8.2.1": "d", "V13.1.1": "e", "V6.3.1": "f", "V8.3.1": "g",
            "V8.2.2": "h", "V3.4.1": "i", "C1.1.1": "j",
        },
    })
}

fn export_of(report: &Value) -> Value {
    sv_cli::oscal::assessment_results(
        report,
        &read(&data("asvs-800-53-crosswalk.json")),
        &sv_cli::oscal::Provenance {
            report_sha256: "0".repeat(64),
            seal: Some("v1:key:mac".to_owned()),
            app_commit: Some(("0123456789abcdef".to_owned(), false)),
            image: None,
        },
    )
    .expect("it exports")
}

#[test]
fn every_status_exports_to_a_document_nists_schema_accepts() {
    let document = export_of(&every_status());
    let errors = schema_errors(&document);
    assert!(errors.is_empty(), "{}", errors.join("\n"));
    // The schema check is only worth having if it can fail: a document missing what OSCAL requires is refused.
    let mut broken = document.clone();
    broken["assessment-results"]["metadata"]
        .as_object_mut()
        .unwrap()
        .remove("oscal-version");
    assert!(
        !schema_errors(&broken).is_empty(),
        "the schema accepted a document with no oscal-version"
    );
}

#[test]
fn only_checked_and_needs_attention_are_concluded() {
    let report = every_status();
    let document = export_of(&report);
    let wrong = says_more_than(&document, &report);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    let result = &document["assessment-results"]["results"][0];
    assert_eq!(result["findings"].as_array().unwrap().len(), 2);
    // Every ASVS requirement but the one not verified is observed, each by its method; AISVS is left to report.json.
    let methods: Vec<(String, String)> = result["observations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| {
            (
                o["title"].as_str().unwrap().to_owned(),
                o["methods"][0].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    assert_eq!(methods.len(), 8, "{methods:?}");
    for (title, method) in [
        ("V1.2.4: needs-attention", "TEST"),
        ("V6.2.1: checked", "TEST"),
        ("V7.3.1: checked-in-part", "TEST"),
        ("V8.2.1: app-tested", "TEST"),
        ("V13.1.1: documented", "EXAMINE"),
        ("V6.3.1: by-hand", "INTERVIEW"),
        ("V8.3.1: attested", "INTERVIEW"),
        ("V8.2.2: stated", "INTERVIEW"),
    ] {
        assert!(
            methods.contains(&(title.to_owned(), method.to_owned())),
            "{title} as {method}: {methods:?}"
        );
    }
    // The AI coding tool's answer is the tool's, never the owner's.
    let stated = result["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["title"] == "V8.2.2: stated")
        .unwrap();
    let actor = &stated["origins"][0]["actors"][0];
    assert_eq!(actor["type"], "tool");
    let ai_tool = result["local-definitions"]["components"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["uuid"] == actor["actor-uuid"])
        .expect("the actor is a component");
    assert!(
        ai_tool["title"]
            .as_str()
            .unwrap()
            .contains("AI coding tool")
    );
    // The related 800-53 controls are links of the objective, and only that.
    let objective = document["assessment-results"]["local-definitions"]["objectives-and-methods"]
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["control-id"] == "v6.2.1")
        .unwrap();
    assert!(
        objective["links"]
            .as_array()
            .unwrap()
            .iter()
            .all(|l| l["rel"] == "related"
                && l["text"]
                    .as_str()
                    .unwrap()
                    .contains("not assessed as a control")),
        "{objective}"
    );
}

#[test]
fn a_document_that_concludes_more_than_the_report_is_caught() {
    // The break this guard is for: *checked in part* exported as satisfied. Done here to the document, as a wrong
    // mapping in `oscal::told` would do it, and to *not verified*, which must never be either.
    let report = every_status();
    let document = export_of(&report);
    for (target, state) in [("v7.3.1_obj", "satisfied"), ("v3.4.1_obj", "not-satisfied")] {
        let mut broken = document.clone();
        let findings = broken["assessment-results"]["results"][0]["findings"]
            .as_array_mut()
            .unwrap();
        let mut extra = findings[0].clone();
        extra["target"]["target-id"] = json!(target);
        extra["target"]["status"]["state"] = json!(state);
        findings.push(extra);
        let wrong = says_more_than(&broken, &report);
        assert!(
            wrong.iter().any(|w| w.starts_with(target)),
            "{target} as {state} was not caught: {wrong:?}"
        );
        // And it is still valid OSCAL: the schema alone would never have caught it.
        assert!(schema_errors(&broken).is_empty());
    }
}

#[test]
fn a_status_this_sv_does_not_know_is_not_guessed_at() {
    assert_eq!(sv_cli::oscal::told("passed"), None);
    assert_eq!(sv_cli::oscal::told("not-verified"), None);
    for s in [
        "checked-in-part",
        "app-tested",
        "documented",
        "by-hand",
        "attested",
        "stated",
    ] {
        assert_eq!(sv_cli::oscal::told(s).unwrap().concluded, None, "{s}");
    }
}

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

#[test]
fn sv_report_and_sv_export_write_the_same_valid_document_and_refuse_a_changed_report() {
    let sv = env!("CARGO_BIN_EXE_sv");
    let scratch = std::env::temp_dir().join(format!("sv-oscal-{}", std::process::id()));
    std::fs::remove_dir_all(&scratch).ok();
    let app = scratch.join("app");
    copy(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/flask-booking"),
        &app,
    );
    let home = scratch.join("home");
    std::fs::create_dir_all(&home).unwrap();
    let run = |args: &[&str]| -> Output {
        Command::new(sv)
            .args(args)
            .env("HOME", &home)
            .env("XDG_CONFIG_HOME", home.join(".config"))
            .env_remove("STACKVET_IMAGE")
            .output()
            .expect("sv runs")
    };
    let app_arg = app.to_str().unwrap();
    let from_report = scratch.join("from-report.json");

    let report = run(&[
        "report",
        app_arg,
        "--no-tools",
        "--oscal",
        from_report.to_str().unwrap(),
    ]);
    let said = String::from_utf8_lossy(&report.stdout);
    assert!(
        report.status.code().is_some_and(|c| c <= 2),
        "{said}\n{}",
        String::from_utf8_lossy(&report.stderr)
    );
    assert!(
        said.contains("Wrote the OSCAL assessment results to"),
        "{said}"
    );
    let document = read(&from_report);
    let errors = schema_errors(&document);
    assert!(errors.is_empty(), "{}", errors.join("\n"));
    let report_json = read(&app.join("stackvet-report").join("report.json"));
    let wrong = says_more_than(&document, &report_json);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    // The folder holds only sv's files still, and its seal still holds.
    assert!(
        !app.join("stackvet-report")
            .join("from-report.json")
            .exists()
    );

    // The same report exported later gives the same document, byte for byte.
    let later = run(&["export", "--oscal", app_arg]);
    assert!(
        later.status.success(),
        "{}",
        String::from_utf8_lossy(&later.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&later.stdout),
        std::fs::read_to_string(&from_report).unwrap()
    );

    // Not into the report folder.
    let inside = app.join("stackvet-report").join("oscal.json");
    let refused = run(&[
        "export",
        "--oscal",
        app_arg,
        "--out",
        inside.to_str().unwrap(),
    ]);
    assert!(!refused.status.success());
    assert!(!inside.exists());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("inside the report folder"));

    // A report changed after it was sealed is not exported.
    let file = app.join("stackvet-report").join("report.json");
    let text = std::fs::read_to_string(&file).unwrap();
    let edited = text.replacen("\"not-verified\"", "\"checked\"", 1);
    assert_ne!(
        edited, text,
        "the report holds a requirement not verified to change"
    );
    std::fs::write(&file, edited).unwrap();
    let changed = run(&["export", "--oscal", app_arg]);
    std::fs::remove_dir_all(&scratch).ok();
    assert!(!changed.status.success());
    assert!(changed.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&changed.stderr).contains("sv cannot show it wrote the report")
    );
}
