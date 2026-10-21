//! Backlog 0490: a package locator names the web server package and its version beside the request-smuggling question
//! (V4.2.1), from the bill of materials, says when the version is only the one asked for, and names nothing it does
//! not know.

use super::*;
use crate::sbom::{Component, VersionSource};

fn shipped() -> Locators {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load")
}

fn web_server() -> PackageLocator {
    shipped()
        .packages()
        .iter()
        .find(|l| l.id == "locate.web-server")
        .cloned()
        .expect("the shipped file has the web server's package locator")
}

fn component(ecosystem: &str, name: &str, version: &str, source: VersionSource) -> Component {
    Component {
        name: name.to_owned(),
        version: version.to_owned(),
        ecosystem: ecosystem.to_owned(),
        source,
    }
}

#[test]
fn the_web_server_speaks_to_the_smuggling_question() {
    assert_eq!(web_server().questions, ["V4.2.1", "V4.2.3", "V4.2.4"]);
}

#[test]
fn a_locked_server_is_named_with_its_version() {
    let found = web_server().find(&[
        component("Python", "flask", "3.0.3", VersionSource::Locked),
        component("Python", "Gunicorn", "22.0.0", VersionSource::Locked),
        component("Ruby", "puma", "6.4.2", VersionSource::Locked),
    ]);
    assert_eq!(found, ["Gunicorn 22.0.0", "puma 6.4.2"]);
}

#[test]
fn a_version_only_asked_for_says_so() {
    let found = web_server().find(&[component(
        "Python",
        "uvicorn",
        "0.30.1",
        VersionSource::Declared,
    )]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].starts_with("uvicorn 0.30.1 "), "{}", found[0]);
    assert!(
        found[0].contains("no lockfile says which is installed"),
        "{}",
        found[0]
    );
}

#[test]
fn a_name_in_another_ecosystem_or_no_server_at_all_finds_nothing() {
    // `puma` is a Ruby server; an npm package of that name is something else.
    assert!(
        web_server()
            .find(&[
                component("npm", "puma", "1.0.0", VersionSource::Locked),
                component("Python", "requests", "2.32.3", VersionSource::Locked),
            ])
            .is_empty()
    );
}

#[test]
fn every_question_a_package_locator_names_is_asked_by_a_catalog() {
    let data = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let design = crate::design::Questions::load(&data.join("design-questions.json")).unwrap();
    let notes = crate::notes::Catalog::load(&data.join("security-notes.json")).unwrap();
    let human = crate::human::HumanChecks::load(&data.join("human-checks.json")).unwrap();
    let locators = shipped();
    assert!(
        !locators.packages().is_empty(),
        "the setup: a package locator"
    );
    for locator in locators.packages() {
        for id in &locator.questions {
            assert!(
                design.get(id).is_some()
                    || notes.sections.iter().any(|s| &s.id == id)
                    || human.checks.iter().any(|c| &c.id == id),
                "{} points for {id}, which no catalog asks the owner about",
                locator.id
            );
        }
    }
}

#[test]
fn the_smuggling_question_says_why_nothing_here_can_check_it() {
    let data = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let design = crate::design::Questions::load(&data.join("design-questions.json")).unwrap();
    let why = design
        .get("V4.2.1")
        .and_then(|q| q.why_not_checked.as_deref())
        .expect("V4.2.1 says why nothing here can check it");
    assert!(why.contains("not in the app's folder"), "{why}");
    let wanted: std::collections::BTreeSet<String> = ["V4.2.1".to_owned()].into();
    let notes = crate::notes::Catalog::load(&data.join("security-notes.json")).unwrap();
    let human = crate::human::HumanChecks::load(&data.join("human-checks.json")).unwrap();
    let list = crate::human::checklist(&notes, &design, &human, &wanted);
    assert_eq!(list.len(), 1);
    assert_eq!(
        list[0].why_not_checked.as_deref(),
        Some(why),
        "the reason reaches the row"
    );
}
