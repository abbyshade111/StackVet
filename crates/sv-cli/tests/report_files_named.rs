//! Backlog 0279: the README, `docs/ARCHITECTURE.md` and the description the AI coding tool reads of
//! `stackvet_write_report` name every file a report is written as. They said five while `sv` wrote seven, and then
//! eight; a file added to `report_files::REPORT_FILES` and left out of any of them fails here.

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
fn every_report_file_is_named_where_the_report_is_described() {
    let catalog = read("crates/sv-cli/src/mcp/catalog.rs");
    let write_report = catalog
        .lines()
        .find(|line| line.contains("\"description\": \"Write the full reports"))
        .expect("stackvet_write_report's description");
    let places = [
        ("README.md", read("README.md")),
        ("docs/ARCHITECTURE.md", read("docs/ARCHITECTURE.md")),
        (
            "stackvet_write_report's description",
            write_report.to_owned(),
        ),
    ];
    for (place, text) in &places {
        for name in sv_cli::report_files::NAMES {
            assert!(text.contains(name), "{place} does not name {name}");
        }
    }
}
