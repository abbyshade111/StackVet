//! What the email sign-in package reader reports, and what it leaves alone (backlog 0625, part 12).

use super::*;
use crate::sbom::{Component, VersionSource};

fn sbom(packages: &[(&str, &str)]) -> Sbom {
    Sbom {
        components: packages
            .iter()
            .map(|(ecosystem, name)| Component {
                name: (*name).to_owned(),
                version: "1.0.0".to_owned(),
                ecosystem: (*ecosystem).to_owned(),
                source: VersionSource::Locked,
            })
            .collect(),
        ..Default::default()
    }
}

#[test]
fn a_package_for_signing_in_by_email_is_reported_in_each_ecosystem() {
    for (ecosystem, name) in [
        ("Python", "django-sesame"),
        ("Python", "Django_Sesame"),
        ("npm", "passport-magic-link"),
        ("npm", "@magic-sdk/admin"),
        ("Ruby", "devise-passwordless"),
        ("PHP", "grosv/laravel-passwordless-login"),
    ] {
        let bill = sbom(&[(ecosystem, name)]);
        let found = email_sign_in_packages(&bill);
        assert_eq!(found.len(), 1, "{ecosystem} {name}: {found:?}");
    }
}

#[test]
fn a_package_of_the_same_name_in_another_ecosystem_and_other_sign_in_packages_are_left_alone() {
    for (ecosystem, name) in [
        // The npm and Ruby `passwordless` are read; a Python one of that name is another package.
        ("Python", "passwordless"),
        ("npm", "next-auth"),
        ("npm", "passport-local"),
        ("Python", "django-allauth"),
    ] {
        assert!(
            email_sign_in_packages(&sbom(&[(ecosystem, name)])).is_empty(),
            "{ecosystem} {name}"
        );
    }
}

#[test]
fn the_configuration_checks_report_the_package_at_the_line_that_names_it() {
    let dir = std::env::temp_dir().join(format!("sv-email-sign-in-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("requirements.txt"),
        "Django==5.1\ndjango-sesame==3.2.2\n",
    )
    .unwrap();
    let report = crate::config::check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    let found: Vec<(&str, usize, &[String])> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == EMAIL_SIGN_IN_PACKAGE)
        .map(|f| {
            (
                f.location.file.as_str(),
                f.location.line,
                f.requirement_ids.as_slice(),
            )
        })
        .collect();
    assert_eq!(found, [("requirements.txt", 2, &["V6.3.6".to_owned()][..])]);
}

fn folder_with(manifest: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "sv-email-code-{}-{}",
        std::process::id(),
        manifest.len()
    ));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("stackvet.toml"), manifest).unwrap();
    dir
}

const USERS: &str = "[stack.run.users]\nseed = \"python seed.py\"\nlogin = { path = \"/login\", form = { email = \"{user}\", password = \"{password}\" } }\nprivate = [\"/account\"]\n";

#[test]
fn an_email_code_entry_is_reported_at_its_line() {
    let manifest = format!(
        "{USERS}email-code = {{ request = {{ path = \"/login/code\", form = {{ email = \"{{user}}\" }} }}, use = {{ path = \"/login/verify\", form = {{ code = \"{{code}}\" }} }} }}\n"
    );
    let dir = folder_with(&manifest);
    // The setup: the file parses, with the entry.
    let parsed = sv_manifest::Manifest::parse(&manifest, &dir.join("stackvet.toml"))
        .expect("the manifest parses");
    assert!(
        parsed
            .stack
            .run
            .users
            .as_ref()
            .is_some_and(|u| u.email_code.is_some())
    );
    let report = crate::config::check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    let found: Vec<(&str, usize)> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == EMAIL_CODE_DECLARED)
        .map(|f| (f.location.file.as_str(), f.location.line))
        .collect();
    assert_eq!(found, [("stackvet.toml", 5)]);
}

#[test]
fn without_an_email_code_entry_or_with_it_commented_out_nothing_is_reported() {
    for manifest in [
        USERS.to_owned(),
        format!("{USERS}# email-code = {{ request = {{ path = \"/x\" }} }}\n"),
    ] {
        let dir = folder_with(&manifest);
        assert_eq!(email_code_declared(&dir), None, "{manifest}");
        std::fs::remove_dir_all(&dir).ok();
    }
}
