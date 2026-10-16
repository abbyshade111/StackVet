//! Backlog 0609: a package locator says "none" only where none can be said, each condition withholding it on its own.

use super::*;
use crate::sbom::{Component, Sbom, VersionSource};

fn second_factor() -> PackageLocator {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load")
    .packages()
    .iter()
    .find(|l| l.id == "locate.second-factor-packages")
    .cloned()
    .expect("the second-factor locator is shipped")
}

fn sbom(components: &[(&str, &str)]) -> Sbom {
    Sbom {
        components: components
            .iter()
            .map(|(ecosystem, name)| Component {
                name: (*name).to_owned(),
                version: "1.0.0".to_owned(),
                ecosystem: (*ecosystem).to_owned(),
                source: VersionSource::Locked,
            })
            .collect(),
        ..Sbom::default()
    }
}

fn found(names: &[&str]) -> Vec<String> {
    names.iter().map(|n| (*n).to_owned()).collect()
}

#[test]
fn none_is_said_for_a_read_list_with_no_such_package() {
    let l = second_factor();
    assert!(
        l.none.is_some(),
        "the setup's locator has a sentence for none"
    );
    assert!(l.says_none(&sbom(&[("Python", "flask")]), &found(&["Python"])));
}

#[test]
fn none_is_not_said_when_any_one_condition_fails() {
    let l = second_factor();
    let python = found(&["Python"]);
    // A package it names.
    assert!(!l.says_none(&sbom(&[("Python", "flask"), ("Python", "pyotp")]), &python));
    // No packages at all.
    assert!(!l.says_none(&sbom(&[]), &python));
    // A list that could not be read.
    let mut unread = sbom(&[("Python", "flask")]);
    unread
        .unread
        .push(("npm".to_owned(), "not JSON".to_owned()));
    assert!(!l.says_none(&unread, &python));
    // A package list found of an ecosystem it does not name, with nothing of it in the bill of materials.
    assert!(!l.says_none(&sbom(&[("Python", "flask")]), &found(&["Python", "Go"])));
    // A package of an ecosystem it does not name.
    assert!(!l.says_none(
        &sbom(&[("Python", "flask"), ("Go", "github.com/pquerna/otp")]),
        &python
    ));
    // A locator with no sentence for none.
    let mut quiet = l.clone();
    quiet.none = None;
    assert!(!quiet.says_none(&sbom(&[("Python", "flask")]), &python));
}
