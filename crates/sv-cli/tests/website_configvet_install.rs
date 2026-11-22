//! The ConfigVet page's install commands are `docs/INSTALL.md`'s, word for word (backlog 0701; the whole-site review,
//! backlog 0696). The guide is where install commands are decided and tried; the page said "not packaged yet" after the
//! tap had a `cv` formula, and its lines were rewritten by hand when v0.3.0's packages moved.

use std::path::PathBuf;

fn read(path: &str) -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path),
    )
    .unwrap()
}

/// Each line of a fenced code block that runs `brew`, `scoop`, or `cargo`.
fn install_lines(markdown: &str) -> Vec<String> {
    let mut inside = false;
    let mut out = Vec::new();
    for line in markdown.lines() {
        if line.trim_start().starts_with("```") {
            inside = !inside;
            continue;
        }
        let line = line.trim();
        if inside
            && ["brew ", "scoop ", "cargo "]
                .iter()
                .any(|p| line.starts_with(p))
        {
            out.push(line.to_owned());
        }
    }
    out
}

#[test]
fn the_configvet_pages_install_lines_are_the_guides() {
    let page = install_lines(&read("docs/website/configvet-page.md"));
    assert!(
        page.iter().any(|l| l.starts_with("brew install"))
            && page.iter().any(|l| l.starts_with("scoop install")),
        "the setup: the page's install lines were read: {page:?}"
    );
    let guide = install_lines(&read("docs/INSTALL.md"));
    for line in &page {
        assert!(
            guide.contains(line),
            "the ConfigVet page says `{line}`, which docs/INSTALL.md does not: the guide decides the install \
             commands, so change it there first, then here"
        );
    }
}
