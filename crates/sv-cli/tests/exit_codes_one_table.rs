//! Backlog 0479: the exit codes were explained three times, in three scopes, with different lists of what 2 covers.
//! They are now one table, in `docs/ARCHITECTURE.md`, with a row for each command that has a status of its own; the
//! README and the guide say them in a sentence and link there. A row dropped, or a page that stops linking, fails here.

use std::path::Path;

fn read(relative: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative),
    )
    .unwrap_or_else(|e| panic!("{relative}: {e}"))
}

#[test]
fn the_exit_codes_are_one_table_and_the_other_pages_link_to_it() {
    let architecture = read("docs/ARCHITECTURE.md");
    let start = architecture
        .find("\n## The exit codes\n")
        .expect("ARCHITECTURE.md has its exit-code section");
    let rest = &architecture[start + 1..];
    let section = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
    assert!(
        section.contains("| Command | 0 | 1 | 2 | 3 |"),
        "the exit codes are not a table: {section}"
    );
    for row in [
        "| `sv check`, `sv report` |",
        "| `sv audit` |",
        "| `sv run` |",
        "| `sv probe` |",
        "| Every other command |",
    ] {
        assert!(
            section.contains(row),
            "the exit-code table has no row {row}"
        );
    }
    for (page, link) in [
        ("README.md", "(docs/ARCHITECTURE.md#the-exit-codes)"),
        (
            "docs/GETTING-STARTED.md",
            "(ARCHITECTURE.md#the-exit-codes)",
        ),
    ] {
        assert!(
            read(page).contains(link),
            "{page} does not link to the exit-code table"
        );
    }
}
