//! Which findings a folder left out sets aside, and what the report says of each entry (ADR-099).

use super::*;
use sv_check::{Confidence, Location, Severity};

fn finding(rule: &str, file: &str, key: bool) -> Finding {
    sv_check::finding::found(Finding {
        evidence: Vec::new(),
        rule_id: rule.into(),
        title: format!("found by {rule}"),
        severity: Severity::High,
        confidence: Confidence::Medium,
        location: Location {
            file: file.into(),
            line: 3,
        },
        secret: key.then(|| sv_check::finding::Secret::redact("a-value-long-enough-to-cut")),
        requirement_ids: Vec::new(),
        cwe: Vec::new(),
        description: String::new(),
        impact: String::new(),
        fix: String::new(),
        also_reported_by: Vec::new(),
        fingerprint: format!("fp-{rule}-{file}"),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
    })
}

fn files(found: &[Finding]) -> Vec<&str> {
    found.iter().map(|f| f.location.file.as_str()).collect()
}

#[test]
fn a_key_counts_in_a_folder_left_out_and_nothing_outside_it_is_set_aside() {
    let found = vec![
        finding("ast.weak-hash", "fixtures/old.py", false),
        finding("secrets.anthropic", "fixtures/settings.py", true),
        finding("ast.weak-hash", "app.py", false),
        // A name that starts the same is not inside the folder.
        finding("ast.weak-hash", "fixtures-old/app.py", false),
    ];
    let (counted, set_aside) = split(found.clone(), &["fixtures".to_owned()]);
    assert_eq!(
        files(&counted),
        ["fixtures/settings.py", "app.py", "fixtures-old/app.py"]
    );
    assert_eq!(files(&set_aside), ["fixtures/old.py"]);
    // No folder left out: nothing is set aside.
    let (counted, set_aside) = split(found, &[]);
    assert_eq!(counted.len(), 4);
    assert!(set_aside.is_empty());
}

fn entry(confirmed: Result<&str, &str>) -> LeftOut {
    LeftOut {
        entries: vec![Entry {
            folder: "fixtures".into(),
            why: "Recorded answers, kept to replay in tests.".into(),
            confirmed: confirmed.map(str::to_owned).map_err(str::to_owned),
        }],
        refused: Vec::new(),
    }
}

#[test]
fn each_entry_is_named_with_its_reason_and_whether_it_counts() {
    let used = ["fixtures".to_owned()];
    let app_files = ["app.py", "fixtures/old.py", "fixtures/settings.py"];
    let set_aside = [finding("ast.weak-hash", "fixtures/old.py", false)];

    let confirmed = entry(Ok("the owner on 2026-10-12 on this computer"));
    assert_eq!(confirmed.counted_out(&used), ["fixtures"]);
    let said = &confirmed.gaps(&used, &app_files, &set_aside)[0];
    assert_eq!(said.what, "the folder `fixtures`, left out of the checks");
    for part in [
        "confirmed by the owner on 2026-10-12",
        "Recorded answers, kept to replay in tests.",
        "It holds 2 files.",
        "1 finding there is not counted",
        "keys and passwords are still looked for there",
    ] {
        assert!(said.why.contains(part), "{part} in {}", said.why);
    }

    // The scan did not use the folders set apart (they held all the code): nothing is left out.
    assert!(confirmed.counted_out(&[]).is_empty());
    let said = &confirmed.gaps(&[], &app_files, &[])[0];
    assert!(said.why.contains("not left out"), "{}", said.why);

    let waiting = entry(Err("nobody has confirmed it through `sv review`"));
    assert!(waiting.counted_out(&used).is_empty());
    let said = &waiting.gaps(&used, &app_files, &[])[0];
    for part in [
        "nobody has confirmed it through `sv review`",
        "its 2 files are checked and its findings count",
        "Run `sv review`",
    ] {
        assert!(said.why.contains(part), "{part} in {}", said.why);
    }

    let refused = LeftOut {
        entries: Vec::new(),
        refused: vec!["`[[left-out]]` `.`: it names the whole app, so it is not left out".into()],
    };
    let said = refused.gaps(&used, &app_files, &[]);
    assert_eq!(said.len(), 1);
    assert!(
        said[0].why.contains("it names the whole app"),
        "{}",
        said[0].why
    );
}
