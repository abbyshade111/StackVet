//! Backlog 0440: the guide carried three readers' material in one 6,300-word file. It now keeps the first afternoon
//! (steps 1 to 5, what you get, and where the report is) and links the rest: installing for `--run` and keeping up to
//! date (`docs/INSTALL.md`), several apps over time (`docs/ALL-YOUR-APPS.md`), and the advisory downloads
//! (`docs/KNOWN-VULNERABILITIES.md`). An install step that drifts back into the guide, a page it stops linking, or a
//! guide grown back past its budget fails here.

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
fn the_guide_keeps_the_first_afternoon_and_links_the_later_days() {
    let guide = read("docs/GETTING-STARTED.md");
    for page in ["INSTALL.md", "ALL-YOUR-APPS.md", "KNOWN-VULNERABILITIES.md"] {
        assert!(
            guide.contains(&format!("]({page})")),
            "the guide does not link {page}"
        );
        assert!(
            read(&format!("docs/{page}")).len() > 500,
            "docs/{page} is missing or empty"
        );
    }
    for later in [
        "https://sh.rustup.rs",
        "sv dashboard ~/code/app-one",
        "https://osv-vulnerabilities.storage.googleapis.com/",
    ] {
        assert!(
            !guide.contains(later),
            "the guide holds {later:?} again, which belongs on its own page"
        );
    }
    let words = guide.split_whitespace().count();
    assert!(
        words <= 4_200,
        "the guide is {words} words; its first afternoon was under 4,000 when it was split"
    );
}
