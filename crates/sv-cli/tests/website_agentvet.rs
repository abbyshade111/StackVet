//! The AgentVet page of stackvet.dev (backlog 0725, ADR-159): it is made, labeled preview, shows its words and none
//! of the owner's notes on them, prints the install commands of its published packages, and the home page points to it. And its
//! install commands are `docs/INSTALL.md`'s, word for word, as the ConfigVet page's are (backlog 0701).

use std::path::{Path, PathBuf};
use std::process::Command;
use sv_frameworks::paths::Canonical;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonical()
        .unwrap()
}

fn read(path: &str) -> String {
    std::fs::read_to_string(repo().join(path)).unwrap()
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
fn the_agentvet_page_is_made_with_its_words_and_without_its_notes_and_the_home_page_points_to_it() {
    let dir = std::env::temp_dir().join(format!("sv-website-agentvet-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    let out = Command::new("python3")
        .arg("-I")
        .arg(repo().join("tools/website.py"))
        .arg("--out")
        .arg(&dir)
        .output()
        .expect("python3 runs");
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let page = std::fs::read_to_string(dir.join("agentvet.html")).expect("agentvet.html is made");
    for said in [
        "AgentVet: what the record shows your AI coding tool did. A preview.",
        "It decides nothing about the tool, and it never says a tool can be trusted.",
        // A list that is the page's own words is kept: only each section's last list, its notes, is left out.
        "A line left out is a question, never a yes or a no.",
        "asked, and did not run",
        "An empty one is not a clean one.",
        "cargo build --release --locked -p av-cli",
    ] {
        assert!(page.contains(said), "the page does not say: {said}");
    }
    assert!(
        page.contains("<table>"),
        "the records are not drawn as a table"
    );
    // The notes say what each line rests on, for the owner; none of them is shown.
    for note in ["ADR-159", "ADR-122", "As built", "crates/av-", "backlog 07"] {
        assert!(!page.contains(note), "a note is shown on the page: {note}");
    }
    // From v0.3.1, av's own packages: the tap's formula and the bucket's manifest, printed once each had merged
    // (backlog 0724). The page installs av, and leaves sv's and cv's lines to their own pages.
    assert!(page.contains("brew install abbyshade111/stackvet/av"));
    assert!(page.contains("scoop install stackvet/av"));
    assert!(
        !page.contains("scoop install stackvet/sv")
            && !page.contains("brew install abbyshade111/stackvet/sv")
            && !page.contains("stackvet/cv"),
        "the AgentVet page installs another program; that is its own page's"
    );
    let home = std::fs::read_to_string(dir.join("index.html")).unwrap();
    assert!(
        // The menu on every page links to it too, so this looks for the home page's own line.
        home.contains("href=\"agentvet.html\">AgentVet</a>"),
        "the home page does not point to the AgentVet page"
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_agentvet_pages_install_lines_are_the_guides() {
    let page = install_lines(&read("docs/website/agentvet-page.md"));
    assert!(
        page.iter().any(|l| l.starts_with("brew install"))
            && page.iter().any(|l| l.starts_with("scoop install"))
            && page.iter().any(|l| l.starts_with("cargo build")),
        "the setup: the page's install lines were read: {page:?}"
    );
    let guide = install_lines(&read("docs/INSTALL.md"));
    for line in &page {
        assert!(
            guide.contains(line),
            "the AgentVet page says `{line}`, which docs/INSTALL.md does not: the guide decides the install \
             commands, so change it there first, then here"
        );
    }
}
