//! A Python server given an empty host listens on every address, which is one of the signs `internet` looks for
//! (backlog 0399). Before, only `0.0.0.0` was looked for, and an app whose default was changed to `""` showed nothing.
//! Finding it only adds requirements (ADR-015: either side saying yes is a yes); not finding it still settles nothing.

use std::path::PathBuf;
use sv_frameworks::Condition;
use sv_scan::{Evidence, Signatures, scan};

fn data(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data")
        .join(file)
}

/// What the scan of a one-file Python app says about `internet`: the pattern it found, or none.
fn internet(name: &str, code: &str) -> Option<String> {
    let dir = std::env::temp_dir().join(format!("sv-empty-host-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("server.py"), code).unwrap();
    let sigs = Signatures::load_all(&[
        &data("tech-signatures.json"),
        &data("claim-corroborators.json"),
    ])
    .expect("signatures load");
    let report = scan(&dir, &sigs).expect("scan");
    std::fs::remove_dir_all(&dir).ok();
    let answer = report
        .answers
        .iter()
        .find(|a| a.condition == Condition::Internet)
        .expect("internet answered");
    match (&answer.value, &answer.evidence) {
        (Some(true), Evidence::Source { pattern, .. }) => Some(pattern.clone()),
        _ => None,
    }
}

#[test]
fn each_empty_host_is_a_sign_of_listening_on_every_address() {
    for (name, code) in [
        (
            "tuple",
            "from http.server import HTTPServer\nHTTPServer((\"\", 8080), H).serve_forever()\n",
        ),
        (
            "tuple-single",
            "import socket\nsocket.socket().bind(('', 8080))\n",
        ),
        (
            "default",
            "import os\nHOST = os.environ.get(\"HOST\", \"\")\n",
        ),
        (
            "default-single",
            "import os\nHOST = os.environ.get('HOST', '')\n",
        ),
        ("ipv6", "import socket\ns.bind((\"::\", 8080))\n"),
    ] {
        assert!(internet(name, code).is_some(), "{name}: {code}");
    }
}

#[test]
fn an_app_on_loopback_shows_no_sign() {
    // The setup: `0.0.0.0` was a sign before this change too.
    assert!(internet("before", "serve(('0.0.0.0', 8080))\n").is_some());
    assert_eq!(
        internet(
            "loopback",
            "NAME = \"\"\nprint(\"\", 1)\nserve(('127.0.0.1', 8080))\n"
        ),
        None
    );
}
