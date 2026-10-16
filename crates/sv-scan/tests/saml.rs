//! The derived condition `saml` (backlog 0500): whether the app takes sign-ins through SAML, which decides whether
//! V6.8.3 (each SAML assertion used once) applies at all. A SAML library among the pinned packages, or named in the
//! code, and it does; neither, in an app read in full, and it does not; packages nobody pinned, and it is not known.

use std::path::{Path, PathBuf};
use sv_frameworks::Condition;
use sv_scan::{Evidence, Signatures, scan};

/// A Flask app with `packages` both asked for and locked, or only asked for when `locked` is false, and `code` as its
/// app.py.
fn app(tag: &str, packages: &str, locked: bool, code: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-saml-{tag}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("requirements.txt"), packages).unwrap();
    if locked {
        std::fs::write(dir.join("requirements.lock"), packages).unwrap();
    }
    std::fs::write(dir.join("app.py"), code).unwrap();
    dir
}

fn saml(dir: &Path) -> sv_scan::Answer {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sigs =
        Signatures::load(&root.join("../../data/tech-signatures.json")).expect("signatures load");
    let report = scan(dir, &sigs).expect("scan");
    assert!(report.unpinned.is_empty() || !dir.join("requirements.lock").exists());
    report
        .answers
        .into_iter()
        .find(|a| a.condition == Condition::Saml)
        .expect("saml answered")
}

const FLASK: &str = "from flask import Flask\napp = Flask(__name__)\n";

#[test]
fn a_saml_library_among_the_packages_is_found() {
    let dir = app(
        "package",
        "flask==3.0.0\npython3-saml==1.16.0\n",
        true,
        FLASK,
    );
    let answer = saml(&dir);
    assert_eq!(answer.value, Some(true), "{:?}", answer.evidence);
    assert!(
        matches!(&answer.evidence, Evidence::Dependency { name, .. } if name == "python3-saml"),
        "{:?}",
        answer.evidence
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_saml_library_named_in_the_code_is_found() {
    let dir = app(
        "code",
        "flask==3.0.0\n",
        true,
        "from onelogin.saml2.auth import OneLogin_Saml2_Auth\nfrom flask import Flask\n",
    );
    assert_eq!(saml(&dir).value, Some(true));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn an_app_read_in_full_with_no_saml_is_answered_no() {
    let dir = app("none", "flask==3.0.0\n", true, FLASK);
    let answer = saml(&dir);
    assert_eq!(answer.value, Some(false), "{:?}", answer.evidence);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn packages_nobody_pinned_leave_it_unknown() {
    let dir = app("unpinned", "flask\n", false, FLASK);
    assert_eq!(
        saml(&dir).value,
        None,
        "an unpinned list cannot rule SAML out"
    );
    std::fs::remove_dir_all(&dir).ok();
}
