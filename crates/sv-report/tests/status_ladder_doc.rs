//! Backlog 0438: the nine statuses are drawn once, as a ladder in `docs/ARCHITECTURE.md` with who can grant each, and
//! the README, the guide, and the glossary link there. Held here to `Status::ALL` and each status's own label, in
//! order: a status added, renamed, or moved in the code without the drawing fails, and so does a page that stops
//! linking.

use std::path::Path;
use sv_report::Status;

fn read(relative: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative),
    )
    .unwrap_or_else(|e| panic!("{relative}: {e}"))
}

/// Whether each status's row, made by `row` from its label, appears in `text` in `Status::ALL`'s order.
fn in_order(text: &str, row: impl Fn(&str) -> String) -> Result<(), String> {
    let mut at = 0;
    for status in Status::ALL {
        let wanted = row(status.label());
        match text[at..].find(&wanted) {
            Some(found) => at += found + wanted.len(),
            None => return Err(format!("{wanted:?} is missing, or out of order")),
        }
    }
    Ok(())
}

#[test]
fn the_ladder_and_its_table_hold_every_status_in_the_codes_order() {
    let architecture = read("docs/ARCHITECTURE.md");
    let start = architecture
        .find("\n## The nine statuses, strongest evidence first\n")
        .expect("ARCHITECTURE.md has its statuses section");
    let rest = &architecture[start + 1..];
    let section = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
    assert_eq!(Status::ALL.len(), 9, "the heading says nine");
    let picture = section
        .split("```text\n")
        .nth(1)
        .and_then(|p| p.split("```").next())
        .expect("the section has its picture");
    in_order(picture, |label| format!("│ {label} ")).unwrap_or_else(|e| panic!("the ladder: {e}"));
    in_order(section, |label| format!("| {label} |")).unwrap_or_else(|e| panic!("the table: {e}"));
    assert!(
        picture.contains("who can grant it"),
        "the ladder does not say who can grant each tier"
    );
    for (page, link) in [
        (
            "README.md",
            "(docs/ARCHITECTURE.md#the-nine-statuses-strongest-evidence-first)",
        ),
        (
            "docs/GETTING-STARTED.md",
            "(ARCHITECTURE.md#the-nine-statuses-strongest-evidence-first)",
        ),
        (
            "docs/GLOSSARY.md",
            "(ARCHITECTURE.md#the-nine-statuses-strongest-evidence-first)",
        ),
    ] {
        assert!(
            read(page).contains(link),
            "{page} does not link to the ladder"
        );
    }
}
