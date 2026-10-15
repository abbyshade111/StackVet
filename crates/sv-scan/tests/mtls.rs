//! The derived condition `mtls` (backlog 0495): whether the app asks clients for a certificate, which decides whether
//! V12.1.3 applies at all. Asked for in the server's own settings, or possibly by a front server whose settings file
//! the repository ships; neither, and the requirement does not apply.

use std::path::PathBuf;
use sv_frameworks::Condition;
use sv_scan::{Evidence, Signatures, scan};

fn mtls(fixture: &str) -> sv_scan::Answer {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sigs =
        Signatures::load(&root.join("../../data/tech-signatures.json")).expect("signatures load");
    let report = scan(&root.join("tests/fixtures").join(fixture), &sigs).expect("scan");
    report
        .answers
        .into_iter()
        .find(|a| a.condition == Condition::Mtls)
        .expect("mtls answered")
}

#[test]
fn a_server_that_asks_for_a_client_certificate_is_found_in_its_own_code() {
    let answer = mtls("node-mtls");
    assert_eq!(answer.value, Some(true), "{:?}", answer.evidence);
    assert!(
        matches!(&answer.evidence, Evidence::Source { file, .. } if file == "server.js"),
        "{:?}",
        answer.evidence
    );
}

#[test]
fn a_front_servers_settings_file_keeps_the_requirement_in() {
    // nginx may ask for the certificate, and its settings are not read for what they say, so the file itself counts.
    let answer = mtls("flask-behind-nginx");
    assert_eq!(answer.value, Some(true), "{:?}", answer.evidence);
    assert!(
        matches!(&answer.evidence, Evidence::File { path } if path.ends_with("nginx.conf")),
        "{:?}",
        answer.evidence
    );
}

#[test]
fn an_app_that_asks_for_none_is_answered_no() {
    let answer = mtls("node-websockets");
    assert_eq!(answer.value, Some(false), "{:?}", answer.evidence);
}
