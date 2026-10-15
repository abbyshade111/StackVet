//! A contact on a name no mail reaches is a stand-in too (backlog 0594): in the trial of 15 October 2026 an AI coding
//! tool cleared the finding with an address on `.local`.

use super::*;
use crate::config::check_dir;

fn notes(text: &str) -> Vec<String> {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("sv-contact-unreachable-{}-{n}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("SECURITY.md"), text).unwrap();
    let report = check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    // The setup: the file was read, so a pass below is the address passing and not the file missed.
    assert!(
        report
            .passed
            .iter()
            .any(|p| p.check_id == "config.security-contact"),
        "the security file was not found"
    );
    report
        .findings
        .into_iter()
        .filter(|f| f.rule_id == PLACEHOLDER)
        .map(|f| f.description)
        .collect()
}

#[test]
fn an_address_on_a_name_no_mail_reaches_is_a_stand_in() {
    for (contact, why) in [
        (
            "Email abigail@familygames.local.",
            "found only on the network it is on",
        ),
        ("Email security@app.invalid.", "reaches no one"),
        ("Email security@example.de.", "made for examples"),
        ("Email security@example.co.uk.", "made for examples"),
    ] {
        let got = notes(&format!("# Security\n\n{contact}\n"));
        assert_eq!(got.len(), 1, "{contact}: {got:?}");
        assert!(got[0].contains(why), "{contact}: {}", got[0]);
    }
}

#[test]
fn a_name_that_only_looks_like_one_is_left_alone() {
    for contact in [
        "Email security@localgames.dev.",
        "Email security@my-local.com.",
        "Email security@examples-of-games.com.",
        "Email security@counterexample.org.",
    ] {
        assert_eq!(
            notes(&format!("# Security\n\n{contact}\n")),
            Vec::<String>::new(),
            "{contact}"
        );
    }
}
