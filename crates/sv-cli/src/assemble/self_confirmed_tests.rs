//! Backlog 0543: each line where the AI coding tool vouches for its own work is named, and nothing else is.

use super::*;
use std::path::Path;

fn manifest(tables: &str) -> sv_manifest::Manifest {
    let toml = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack.run]\nimage = \"python:3.12-slim\"\n\
         start = \"python app.py\"\nhealth = \"/\"\n{tables}"
    );
    sv_manifest::Manifest::parse(&toml, Path::new("stackvet.toml")).expect("the manifest parses")
}

#[test]
fn the_trial_lines_are_each_named() {
    // Antigravity's design confirmations and VS Code's check by hand, as the trial wrote them.
    let m = manifest(
        r#"
[design]
"V4.1.3" = { answer = "yes", by = "ai-tool", confirmed = { by = "ai-tool", how = "Read the code." } }
"V6.2.1" = { answer = "yes", by = "ai-tool", confirmed = { by = "AI coding tool", on = "2026-10-14", how = "Tried it." } }

[checked-by-hand]
"V2.2.1" = { result = "done", on = "2026-10-14", by = "ai-tool", how = "Sent a long name; it was refused." }
"V3.4.1" = { result = "done", on = "2026-10-14", by = "ai-tool", how = "Looked.", confirmed = { by = "ai-tool", on = "2026-10-14", how = "Looked again." } }
"#,
    );
    assert_eq!(m.design.len(), 2, "the setup reached the manifest");
    assert_eq!(m.checked_by_hand.len(), 2, "the setup reached the manifest");
    let found = of(&m);
    assert_eq!(
        found.lines,
        [
            "[design] V4.1.3, confirmed by the AI coding tool",
            "[design] V6.2.1, confirmed by the AI coding tool",
            "[checked-by-hand] V2.2.1, checked by hand by the AI coding tool",
            "[checked-by-hand] V3.4.1, checked by hand and confirmed by the AI coding tool",
        ]
    );
    assert_eq!(found.design.len(), 2);
    assert!(found.hand.contains("V3.4.1") && found.hand.len() == 1);
    let gap = found.gap().expect("a gap names them");
    assert!(
        gap.what.starts_with("4 lines in stackvet.toml"),
        "{}",
        gap.what
    );
    assert!(
        gap.why
            .starts_with("A confirmation by the AI coding tool counts for nothing")
            && gap.why.contains("Only `sv review`")
            && gap.why.contains("counts only as the tool's word")
            && gap.why.contains("[checked-by-hand] V2.2.1"),
        "{}",
        gap.why
    );
}

#[test]
fn what_counts_or_is_named_elsewhere_is_not_named() {
    // The control: the tool's answers themselves, a problem it reports, a check not yet made, the owner's own, and
    // a check the tool made that a person confirmed.
    let m = manifest(
        r#"
[design]
"V4.1.3" = { answer = "yes", by = "ai-tool" }
"V6.2.1" = { answer = "yes", by = "ai-tool", confirmed = { by = "owner", on = "2026-10-14", how = "Tried it." } }

[checked-by-hand]
"V2.2.1" = { result = "problem", on = "2026-10-14", by = "ai-tool", how = "A long name was taken." }
"V2.2.2" = { result = "not-yet", by = "ai-tool" }
"V3.4.1" = { result = "done", on = "2026-10-14", by = "owner", how = "Looked." }
"V12.2.2" = { result = "done", on = "2026-10-14", by = "ai-tool", how = "Looked.", confirmed = { by = "Sam Lee", on = "2026-10-14", how = "Looked again." } }
"#,
    );
    assert_eq!(m.design.len(), 2, "the setup reached the manifest");
    assert_eq!(m.checked_by_hand.len(), 4, "the setup reached the manifest");
    let found = of(&m);
    assert!(found.lines.is_empty(), "{:?}", found.lines);
    assert!(found.gap().is_none());
}

#[test]
fn each_kind_is_said_only_when_there_is_one() {
    let confirmations = of(&manifest(
        "\n[design]\n\"V4.1.3\" = { answer = \"yes\", by = \"ai-tool\", confirmed = { by = \"ai-tool\" } }\n",
    ))
    .gap()
    .expect("a gap names it");
    assert!(
        confirmations.why.contains("counts for nothing")
            && !confirmations.why.contains("counts only as the tool's word"),
        "{}",
        confirmations.why
    );
    // A check the person described, written down as setup prompt 6 asks: the tool's word, never "nothing".
    let relayed = of(&manifest(
        "\n[checked-by-hand]\n\"V2.2.1\" = { result = \"done\", on = \"2026-10-14\", by = \"ai-tool\", how = \"They tried it.\" }\n",
    ))
    .gap()
    .expect("a gap names it");
    assert!(
        relayed.why.contains("counts only as the tool's word")
            && !relayed.why.contains("counts for nothing"),
        "{}",
        relayed.why
    );
}
