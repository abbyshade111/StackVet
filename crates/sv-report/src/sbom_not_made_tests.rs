//! `sbom.cdx.json` for a report made without a bill of materials (backlog 0449): the file is written all the same, so
//! it says it is not complete and why, and the page says it holds no bill. An empty list of packages alone reads as an
//! app that ships nothing.

use crate::next_step_tests::empty_report;

#[test]
fn the_empty_file_says_no_bill_was_made_and_is_not_complete() {
    let report = empty_report(Vec::new());
    assert!(
        report.sbom.is_none(),
        "the setup: a report made without one"
    );
    let document: serde_json::Value =
        serde_json::from_str(&crate::sbom_document(&report)).expect("JSON");
    assert_eq!(document["bomFormat"], "CycloneDX");
    assert_eq!(document["specVersion"], "1.5");
    assert_eq!(document["components"], serde_json::json!([]));
    let properties = document["metadata"]["properties"]
        .as_array()
        .expect("the caveats, where sv sbom puts them");
    let value = |name: &str| {
        properties
            .iter()
            .find(|p| p["name"] == name)
            .and_then(|p| p["value"].as_str())
            .unwrap_or_else(|| panic!("{name}: {document}"))
            .to_owned()
    };
    assert_eq!(value("securevibe:complete"), "false");
    assert_eq!(value("securevibe:not-made"), crate::SBOM_NOT_MADE);
}

#[test]
fn the_pages_say_the_file_holds_no_bill() {
    let report = empty_report(Vec::new());
    let line = crate::sbom_line(&report);
    assert!(line.starts_with(crate::SBOM_FILE), "{line}");
    assert!(line.contains(crate::SBOM_NOT_MADE), "{line}");
    let compliance = crate::markdown::compliance(&report);
    assert!(compliance.contains(&line), "{compliance}");
    assert!(
        crate::html::page(&report).contains("holds no software bill of materials"),
        "the page"
    );
}

#[test]
fn a_report_with_a_bill_writes_it_as_it_is() {
    let mut report = empty_report(Vec::new());
    report.sbom = Some(crate::SbomFile {
        json: "{\"made\": true}".into(),
        components: 3,
        complete: true,
    });
    assert_eq!(crate::sbom_document(&report), "{\"made\": true}");
    assert!(crate::sbom_line(&report).contains("3 packages"));
}
