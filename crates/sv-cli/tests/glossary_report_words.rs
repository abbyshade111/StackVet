//! The words the guide uses for a non-programmer, explained (backlog 0478): the glossary has "needs attention",
//! "checked in part", "the fence", and "seal", and its Report entry names `report.html` first and points at the
//! README's table of the eight files, rather than calling `compliance.md` the one to read. The guide's exit codes are a
//! pointer, and its "Known problems" section, true of a build in September, is gone.

use std::path::Path;

fn read(relative: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative),
    )
    .unwrap_or_else(|e| panic!("{relative}: {e}"))
}

/// The glossary entry under `heading`, as one line.
fn entry(glossary: &str, heading: &str) -> String {
    let start = glossary
        .find(&format!("\n## {heading}\n"))
        .unwrap_or_else(|| panic!("no glossary entry {heading:?}"));
    let rest = &glossary[start + 1..];
    let end = rest[3..].find("\n## ").map_or(rest.len(), |e| e + 3);
    rest[..end].split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn the_glossary_explains_the_words_the_guide_uses() {
    let glossary = read("docs/GLOSSARY.md");
    for heading in ["Needs attention", "Checked in part", "The fence", "Seal"] {
        assert!(
            entry(&glossary, heading).len() > heading.len() + 60,
            "the entry {heading:?} says too little"
        );
    }
    let report = entry(&glossary, "Report");
    let html = report
        .find("`report.html`")
        .expect("the Report entry names report.html");
    let markdown = report.find("`compliance.md`").expect("and compliance.md");
    assert!(html < markdown, "report.html comes first: {report}");
    assert!(
        report.contains("(../README.md#what-a-report-holds)"),
        "{report}"
    );
    assert!(!report.contains("`compliance.md` to read"), "{report}");
}

#[test]
fn the_guide_points_at_the_exit_codes_and_has_no_stale_known_problems() {
    let guide = read("docs/GETTING-STARTED.md");
    assert!(
        !guide.contains("## Known problems"),
        "the September section is back"
    );
    assert!(
        !guide.contains("`sv check . --fail-on attention:high`"),
        "the exit codes are spelled out in the guide again"
    );
    assert!(guide.contains("(ARCHITECTURE.md#the-exit-codes)"));
}
