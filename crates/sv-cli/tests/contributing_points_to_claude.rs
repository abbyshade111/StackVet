//! Backlog 0484: CLAUDE.md and CONTRIBUTING.md carried the same rules in two wordings, and they had drifted (the
//! network rule's exceptions, 0437). The owner's decision of 13 October 2026: CLAUDE.md keeps the rules in full, and
//! CONTRIBUTING.md names each in one sentence under the same bold lead and points there. A lead renamed or dropped in
//! CLAUDE.md, or a rule grown back to a paragraph in CONTRIBUTING.md, fails here.

use std::path::Path;

fn read(relative: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative),
    )
    .unwrap_or_else(|e| panic!("{relative}: {e}"))
}

/// CONTRIBUTING.md's "The rules every change keeps", up to the next heading.
fn rules_section() -> String {
    let contributing = read("CONTRIBUTING.md");
    let heading = "## The rules every change keeps\n";
    let start = contributing
        .find(heading)
        .expect("CONTRIBUTING.md has its rules section")
        + heading.len();
    let rest = &contributing[start..];
    rest[..rest.find("\n## ").unwrap_or(rest.len())].to_owned()
}

/// Each bullet of the section, its wrapped lines joined.
fn bullets(section: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in section.lines() {
        if let Some(text) = line.strip_prefix("- ") {
            out.push(text.to_owned());
        } else if line.starts_with("  ") && !out.is_empty() {
            let last = out.last_mut().unwrap();
            last.push(' ');
            last.push_str(line.trim());
        } else if line.trim().is_empty() || !line.starts_with(' ') {
            // A paragraph or a blank line ends the bullet before it.
            if !out.is_empty() && !line.trim().is_empty() {
                break;
            }
        }
    }
    out
}

#[test]
fn every_rule_in_contributing_is_one_sentence_under_a_lead_claude_md_has() {
    let claude = read("CLAUDE.md");
    let section = rules_section();
    assert!(
        section.contains("[`CLAUDE.md`](CLAUDE.md)"),
        "CONTRIBUTING.md's rules do not point to CLAUDE.md"
    );
    let rules = bullets(&section);
    assert!(
        rules.len() >= 6,
        "CONTRIBUTING.md's rules section lists {} rules; the reading of it is broken or rules were dropped: {rules:#?}",
        rules.len()
    );
    for rule in &rules {
        let lead = rule
            .strip_prefix("**")
            .and_then(|r| r.split_once("**"))
            .map(|(lead, _)| format!("**{lead}**"))
            .unwrap_or_else(|| panic!("a rule with no bold lead: {rule}"));
        assert!(
            claude.contains(&lead),
            "CONTRIBUTING.md's rule {lead} is not a lead in CLAUDE.md, so the two have drifted"
        );
        assert!(
            rule.len() <= 220,
            "CONTRIBUTING.md's rule {lead} is {} characters: one sentence, and the full text in CLAUDE.md",
            rule.len()
        );
    }
}
