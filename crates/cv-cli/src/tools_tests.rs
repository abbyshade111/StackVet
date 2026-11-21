//! The tools' part of `cv check` that needs no tool installed: which files each is handed, the descriptions as data,
//! and how a tool's finding is folded into `cv`'s own.

use super::*;
use sv_core::finding::{Confidence, Location, Severity};

fn finding(rule: &str, file: &str) -> Finding {
    Finding {
        rule_id: rule.to_owned(),
        title: rule.to_owned(),
        severity: Severity::Medium,
        confidence: Confidence::High,
        location: Location {
            file: file.to_owned(),
            line: 1,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        evidence: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        outranked: None,
    }
}

#[test]
fn the_three_tools_load_and_each_is_kept_offline_and_from_the_app_s_settings() {
    let adapters = load().expect("data/cv-adapters.json loads");
    let ids: Vec<&str> = adapters.all().iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, ["hadolint", "checkov", "trivy"]);
    for adapter in adapters.all() {
        let args = adapter.run.args.join(" ");
        assert!(
            !adapter.network,
            "{} is said to reach the network",
            adapter.id
        );
        // Each is given a settings file of cv's own, measured to keep the app's from turning rules off.
        assert!(args.contains("{config}"), "{}: {args}", adapter.id);
        let must: &[&str] = match adapter.id.as_str() {
            "hadolint" => &["--disable-ignore-pragma"],
            "checkov" => &["--skip-download"],
            "trivy" => &[
                "--skip-check-update",
                "--skip-version-check",
                "--disable-telemetry",
                "--ignorefile {config}",
            ],
            other => panic!("an adapter this test does not know: {other}"),
        };
        for flag in must {
            assert!(args.contains(flag), "{} without {flag}: {args}", adapter.id);
        }
    }
    let checkov = adapters.all().iter().find(|a| a.id == "checkov").unwrap();
    assert_eq!(
        checkov.report_in_database.as_deref(),
        Some("results_sarif.sarif")
    );
    assert!(checkov.switched_off_by.iter().any(|f| f == ".checkov.yaml"));
}

#[test]
fn a_tool_is_handed_only_the_files_cv_read_of_the_kinds_it_reads() {
    let adapters = load().unwrap();
    let read = vec![
        (Layer::Dockerfile, "Dockerfile".to_owned()),
        (Layer::Compose, "compose.yaml".to_owned()),
        (Layer::Kubernetes, "k8s/web.yaml".to_owned()),
        (Layer::DjangoSettings, "app/settings.py".to_owned()),
    ];
    let by = |id: &str| handed(adapters.all().iter().find(|a| a.id == id).unwrap(), &read);
    assert_eq!(by("hadolint"), ["Dockerfile"]);
    // Neither Checkov nor Trivy reads compose files (measured: neither reported the privileged compose service).
    assert_eq!(by("checkov"), ["Dockerfile", "k8s/web.yaml"]);
    assert!(
        handed(
            adapters.all().iter().find(|a| a.id == "hadolint").unwrap(),
            &[]
        )
        .is_empty()
    );
}

#[test]
fn a_tool_s_finding_of_cv_s_own_rule_in_the_same_file_is_folded_into_cv_s() {
    let root = cv_rules::rules::CONTAINER_ROOT.id;
    let mut own = vec![finding(root, "Dockerfile")];
    assert!(fold_into(
        &mut own,
        &finding("hadolint.DL3002", "Dockerfile")
    ));
    assert!(fold_into(&mut own, &finding("trivy.DS-0002", "Dockerfile")));
    assert_eq!(
        own[0].also_reported_by,
        ["hadolint.DL3002", "trivy.DS-0002"]
    );
    // The same rule again is named once.
    assert!(fold_into(
        &mut own,
        &finding("hadolint.DL3002", "Dockerfile")
    ));
    assert_eq!(own[0].also_reported_by.len(), 2);
    // Another file, or a rule that is not cv's, stays a finding of its own.
    assert!(!fold_into(
        &mut own,
        &finding("hadolint.DL3002", "api/Dockerfile")
    ));
    assert!(!fold_into(
        &mut own,
        &finding("hadolint.DL3007", "Dockerfile")
    ));
}

#[test]
fn every_rule_a_tool_is_said_to_agree_with_is_one_its_map_describes() {
    // A rule named in AGREES and missing from the tool's map would be folded without anyone having read it.
    let adapters = load().unwrap();
    for (_, theirs) in AGREES {
        for rule in *theirs {
            let (tool, id) = rule.split_once('.').unwrap();
            let adapter = adapters
                .all()
                .iter()
                .find(|a| a.id == tool)
                .unwrap_or_else(|| panic!("{tool}"));
            assert!(
                adapter.rules.contains_key(id),
                "{rule} is not in {tool}'s map"
            );
        }
    }
}

#[test]
fn a_tool_s_finding_is_put_in_cv_s_terms_before_it_is_folded() {
    let adapters = load().unwrap();
    let hadolint = adapters.all().iter().find(|a| a.id == "hadolint").unwrap();
    // As hadolint 2.15.1 gave it on 21 November 2026: the file as it was handed, and the runner's fallback title.
    let mut mapped = finding("hadolint.DL3002", "./Dockerfile");
    mapped.title = "hadolint reported DL3002".to_owned();
    tidy(hadolint, &mut mapped);
    assert_eq!(mapped.location.file, "Dockerfile");
    assert_eq!(mapped.title, "The last USER of the Dockerfile is root");
    let mut own = vec![finding(cv_rules::rules::CONTAINER_ROOT.id, "Dockerfile")];
    assert!(fold_into(&mut own, &mapped), "tidied, it meets cv's own");
    // A rule the map does not describe takes the tool's own message.
    let mut unmapped = finding("hadolint.DL3009", "./Dockerfile");
    unmapped.title = "hadolint reported DL3009".to_owned();
    unmapped.description = "Delete the apt-get lists after installing something\nmore".to_owned();
    tidy(hadolint, &mut unmapped);
    assert_eq!(
        unmapped.title,
        "Delete the apt-get lists after installing something"
    );
}
