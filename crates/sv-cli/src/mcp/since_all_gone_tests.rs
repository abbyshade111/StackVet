//! A check asked for again after every finding was fixed names the ones that went (backlog 0452): the answer with none
//! left and the GONE lines is longer than the plain "No findings" one, and was dropped for it, so the confirmation
//! that the fix took, the case backlog 0336 was for, never came.

use super::since_tests::{check, fingerprints, recipes};
use super::tests::text;
use super::*;

/// Where to report a problem, which the app otherwise lacks (`config.security-contact`).
fn security_contact(root: &std::path::Path) {
    std::fs::write(
        root.join("app/SECURITY.md"),
        "# Security\n\nReport a problem to the owner by email; expect a reply within a week.\n",
    )
    .unwrap();
}

#[test]
fn when_every_finding_is_fixed_the_next_check_names_the_ones_that_went() {
    let root = recipes("all-gone", 2);
    let server = Server::new(&root).unwrap();
    let app = json!({ "path": "app" });

    let first = check(&server, app.clone());
    let before = fingerprints(&first["structuredContent"]["findings"], "fingerprint");
    assert!(before.len() >= 6, "the setup: {before:?}");

    // Every finding fixed: the weak hashes, and somewhere to report a problem.
    security_contact(&root);
    for n in 0..2 {
        let path = root.join(format!("app/recipes_{n}.py"));
        let code = std::fs::read_to_string(&path).unwrap();
        std::fs::write(&path, code.replace("hashlib.md5", "hashlib.sha256")).unwrap();
    }

    let second = check(&server, app.clone());
    let data = &second["structuredContent"];
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(
        data["findings"],
        json!([]),
        "the setup: none left: {data:#}"
    );
    assert_eq!(
        fingerprints(&data["gone"], "fingerprint"),
        before,
        "{data:#}"
    );
    assert_eq!(data["unchanged"], json!([]), "{data:#}");
    assert!(
        data["since"].as_str().is_some_and(|s| !s.is_empty()),
        "{data:#}"
    );
    let said = text(&second);
    assert!(said.contains("No findings"), "{said}");
    assert!(said.contains("GONE since your check at"), "{said}");
    for fingerprint in &before {
        assert!(said.contains(fingerprint.as_str()), "{fingerprint}: {said}");
    }
}

#[test]
fn with_nothing_found_and_nothing_gone_the_answer_is_the_plain_one() {
    // The rule kept: a check with nothing to say about what went is the ordinary answer.
    let root = recipes("none-gone", 0);
    security_contact(&root);
    let server = Server::new(&root).unwrap();
    let app = json!({ "path": "app" });
    let first = check(&server, app.clone());
    assert_eq!(
        first["structuredContent"]["findings"],
        json!([]),
        "the setup: nothing found"
    );
    let second = check(&server, app);
    std::fs::remove_dir_all(&root).ok();
    let data = &second["structuredContent"];
    assert!(data.get("since").is_none(), "{data:#}");
    assert!(!text(&second).contains("GONE"), "{}", text(&second));
}
