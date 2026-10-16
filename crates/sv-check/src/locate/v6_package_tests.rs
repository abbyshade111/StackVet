//! Backlog 0609: the rate-limit libraries beside V6.1.1 and the second-factor libraries beside V6.3.3, named from the
//! bill of materials, and nothing they do not know.

use super::*;
use crate::sbom::{Component, VersionSource};

fn locator(id: &str) -> PackageLocator {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load")
    .packages()
    .iter()
    .find(|l| l.id == id)
    .cloned()
    .unwrap_or_else(|| panic!("{id} is not shipped"))
}

fn component(ecosystem: &str, name: &str) -> Component {
    Component {
        name: name.to_owned(),
        version: "1.0.0".to_owned(),
        ecosystem: ecosystem.to_owned(),
        source: VersionSource::Locked,
    }
}

#[test]
fn rate_limit_libraries_are_named_beside_v6_1_1() {
    let l = locator("locate.rate-limit-packages");
    assert_eq!(l.questions, ["V6.1.1"]);
    let found = l.find(&[
        component("npm", "express-rate-limit"),
        component("Python", "Flask_Limiter"),
        component("npm", "express"),
    ]);
    assert_eq!(found, ["Flask_Limiter 1.0.0", "express-rate-limit 1.0.0"]);
}

#[test]
fn second_factor_libraries_are_named_beside_v6_3_3() {
    let l = locator("locate.second-factor-packages");
    assert_eq!(l.questions, ["V6.3.3"]);
    let found = l.find(&[
        component("Python", "pyotp"),
        component("npm", "@simplewebauthn/server"),
        component("Python", "requests"),
    ]);
    assert_eq!(found, ["@simplewebauthn/server 1.0.0", "pyotp 1.0.0"]);
}
