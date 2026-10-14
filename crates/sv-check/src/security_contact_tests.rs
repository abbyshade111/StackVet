//! Each form of placeholder contact is noted at its line; a real address and an app with no security file are not.

use super::*;
use crate::config::check_dir;

/// A folder of its own for each call, since the tests run side by side.
fn scratch(name: &str) -> PathBuf {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "sv-security-contact-{name}-{}-{n}",
        std::process::id()
    ));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The placeholder notes `check_dir` makes for an app holding `text` at `place`, as (file, line, description).
fn notes(place: &str, text: &str) -> Vec<(String, usize, String)> {
    let dir = scratch(&place.replace(['/', '.'], "-"));
    let path = dir.join(place);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, text).unwrap();
    let report = check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    // The file was there to be read: the presence check credits it.
    assert!(
        report
            .passed
            .iter()
            .any(|p| p.check_id == "config.security-contact"),
        "the setup did not put a security file where sv looks: {place}"
    );
    report
        .findings
        .into_iter()
        .filter(|f| f.rule_id == PLACEHOLDER)
        .map(|f| (f.location.file, f.location.line, f.description))
        .collect()
}

#[test]
fn each_form_of_placeholder_contact_is_noted_at_its_line() {
    let report = "# Security\n\nPlease report a problem privately.\n";
    for (contact, why) in [
        ("Email security@example.com.", "kept for examples"),
        ("Email security@example.org.", "kept for examples"),
        ("Email security@mail.example.net.", "kept for examples"),
        ("Email security@app.test.", "kept for examples and tests"),
        ("Email root@localhost.", "`localhost`"),
        ("Email security@yourdomain.com.", "stand-in"),
        ("Email you@ with the details.", "stand-in"),
        ("Email <your-email> with the details.", "stand-in"),
        (
            "Email [your security address] with the details.",
            "stand-in",
        ),
        ("Email your_email with the details.", "stand-in"),
        ("Email changeme with the details.", "stand-in"),
    ] {
        let got = notes(
            "SECURITY.md",
            &format!("{report}{contact}\nWe answer within 48 hours.\n"),
        );
        assert_eq!(got.len(), 1, "{contact}: {got:?}");
        assert_eq!(
            (got[0].0.as_str(), got[0].1),
            ("SECURITY.md", 4),
            "{contact}"
        );
        assert!(got[0].2.contains(why), "{contact}: {}", got[0].2);
    }
    // security.txt's own form, where it is served from, with a web address on its Contact line.
    let got = notes(
        ".well-known/security.txt",
        "Contact: https://example.com/security\nExpires: 2027-10-14T00:00:00.000Z\n",
    );
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(
        (got[0].0.as_str(), got[0].1),
        (".well-known/security.txt", 1)
    );
    // Two lines: one note, at the first, saying there is another.
    let got = notes(
        "SECURITY.md",
        "Email security@example.com\nor call the team at changeme.\n",
    );
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(got[0].1, 1);
    assert!(got[0].2.contains("One more line"), "{}", got[0].2);
}

#[test]
fn a_real_contact_and_an_example_that_is_not_one_pass() {
    for text in [
        // A real address, written several ways.
        "Email security@stackvet-owner.dev.\n",
        "Contact: mailto:security@stackvet-owner.dev\nContact: https://stackvet-owner.dev/report\n",
        // Words a policy uses that are not stand-ins.
        "Send your email to security@stackvet-owner.dev; we will change me nothing.\n",
        // An `example.com` web address that is an example of something else, not the contact.
        "Email security@stackvet-owner.dev. A report like https://example.com/?q=<script> helps.\n",
    ] {
        assert_eq!(notes("SECURITY.md", text), vec![], "{text}");
    }
    let dir = scratch("real-passes");
    std::fs::write(
        dir.join("SECURITY.md"),
        "Email security@stackvet-owner.dev.\n",
    )
    .unwrap();
    let report = check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    assert!(report.passed.iter().any(|p| p.check_id == PLACEHOLDER));
}

#[test]
fn with_no_security_file_the_check_says_it_could_not_run() {
    let dir = scratch("none");
    let report = check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.rule_id == "config.security-contact")
    );
    let (_, why) = report
        .not_assessed
        .iter()
        .find(|(id, _)| id == PLACEHOLDER)
        .expect("not assessed, never passed");
    assert!(why.contains("no SECURITY.md"), "{why}");
    assert!(!report.passed.iter().any(|p| p.check_id == PLACEHOLDER));
}
