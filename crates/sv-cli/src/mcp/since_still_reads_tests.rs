//! A check asked for again names, among the findings that went, those whose line still reads as it did (backlog
//! 0401), as written or but for a comment telling a scanner to look away, and tells the AI tool to tell the person.
//!
//! `sv`'s own rules have no comment that hides a finding, so the two that went are put in the session's memory of the
//! first check, as a check that ran Bandit would leave them: fingerprinted from the app's code as it was, before
//! the code changed.
//!
//! A finding `sv` still finds on its line, under a new fingerprint because a comment was added there, did not go, and
//! nothing is said of it.

use super::since_tests::{check, fingerprints, recipes};
use super::tests::text;
use super::*;

fn bandit(file: &str, line: usize) -> sv_check::Finding {
    sv_check::Finding {
        evidence: Vec::new(),
        rule_id: "bandit.B104".into(),
        title: "binds every address".into(),
        severity: sv_check::Severity::Medium,
        confidence: sv_check::Confidence::Medium,
        location: sv_check::Location {
            file: file.into(),
            line,
        },
        secret: None,
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
    }
}

/// One weak hash, on a line like no other in the app.
const HASHES: &str = "import hashlib\ndef h(x):\n    return hashlib.md5(x).hexdigest()\n";

#[test]
fn findings_that_went_while_their_lines_still_read_the_same_are_named() {
    let root = recipes("still-reads", 1);
    let app_dir = root.join("app");
    std::fs::write(
        app_dir.join("serve.py"),
        "import app\nhost = \"0.0.0.0\"\nrun(host)\n",
    )
    .unwrap();
    std::fs::write(
        app_dir.join("other.py"),
        "import app\nport = \"0.0.0.0\"\nrun(port)\n",
    )
    .unwrap();
    std::fs::write(app_dir.join("hashes.py"), HASHES).unwrap();
    let planted: Vec<check_text::Shown> = [bandit("serve.py", 2), bandit("other.py", 2)]
        .into_iter()
        .map(|mut f| {
            f.fingerprint = sv_check::review::fingerprint(&app_dir, &f);
            check_text::Shown::of(&f)
        })
        .collect();
    assert!(
        planted.iter().all(|s| s.fingerprint.starts_with("v2-")),
        "the setup: {planted:?}"
    );
    let server = Server::new(&root).unwrap();
    let app = json!({ "path": "app" });
    check(&server, app.clone());
    {
        let mut seen = server.seen.lock().unwrap();
        assert_eq!(seen.len(), 1, "the setup: the first check is remembered");
        for s in seen.values_mut() {
            s.findings.extend(planted.iter().cloned());
        }
    }

    // One line gains a comment that tells Bandit to look away; the other moves down a line, unchanged.
    std::fs::write(
        app_dir.join("serve.py"),
        "import app\nhost = \"0.0.0.0\"  # nosec\nrun(host)\n",
    )
    .unwrap();
    std::fs::write(
        app_dir.join("other.py"),
        "import app\n\nport = \"0.0.0.0\"\nrun(port)\n",
    )
    .unwrap();
    // And one of `sv`'s own findings gains such a comment: `sv` still finds it, under a new fingerprint.
    std::fs::write(
        app_dir.join("hashes.py"),
        HASHES.replace("()\n", "()  # nosec\n"),
    )
    .unwrap();

    let second = check(&server, app.clone());
    let data = &second["structuredContent"];
    std::fs::remove_dir_all(&root).ok();
    let gone = fingerprints(&data["gone"], "fingerprint");
    for s in &planted {
        assert!(
            gone.contains(&s.fingerprint),
            "the setup: {} went: {data:#}",
            s.file
        );
    }
    let still = |file: &str| {
        data["gone"]
            .as_array()
            .unwrap()
            .iter()
            .find(|g| g["file"] == file)
            .map(|g| g["stillReads"].clone())
            .unwrap()
    };
    assert_eq!(
        still("serve.py"),
        json!({ "how": "look-away", "scanner": "Bandit", "line": 2 })
    );
    assert_eq!(still("other.py"), json!({ "how": "unchanged", "line": 3 }));
    // The weak hash went by its old fingerprint, and is found again on its line: nothing is said of it.
    let hashes = data["gone"]
        .as_array()
        .unwrap()
        .iter()
        .find(|g| g["file"] == "hashes.py")
        .unwrap_or_else(|| {
            panic!("the setup: the weak hash went by its old fingerprint: {data:#}")
        });
    assert_eq!(hashes["stillReads"], Value::Null, "{hashes:#}");
    assert_eq!(gone.len(), 3, "{data:#}");
    let said = text(&second);
    assert!(
        said.contains(
            "its line, now line 2, reads as it did but for a comment telling Bandit to look away"
        ),
        "{said}"
    );
    assert!(
        said.contains("its line, now line 3, still reads as it did"),
        "{said}"
    );
}
