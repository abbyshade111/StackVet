//! No page of stackvet.dev says `sv` makes fewer network connections than "The fence" allows (backlog 0700; the
//! whole-site review, backlog 0696). Two pages did: How it works showed the paper's "the one network connection sv
//! makes of its own", and the check-a-project page said that, while a project was fetched, "nothing else leaves your
//! computer", though Semgrep fetches its rules in the same run.
//!
//! A refused phrase may stand in a paragraph that also names what it leaves out. The paper's own description, which
//! How it works shows as the paper has it, may say it only below the page's note that says what has changed since.
//! The build itself holds the home page's count of exceptions to the fence table (`tools/website.py`,
//! `with_exceptions`).

use std::path::{Path, PathBuf};
use std::process::Command;

const REFUSED: [&str; 2] = [
    "the one network connection",
    "nothing else leaves your computer",
];
/// Words that name an exception the refused phrase would leave out.
const NAMES_ONE: [&str; 3] = ["Semgrep", "semgrep.dev", "exceptions"];
const NOTE: &str = "no longer StackVet's only network connection";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn build(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
    let out = Command::new("python3")
        .arg("-I")
        .arg(root().join("tools/website.py"))
        .arg("--out")
        .arg(dir)
        .output()
        .expect("python3 runs");
    assert!(
        out.status.success(),
        "the site did not build: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Each paragraph or list item a page shows, as text.
fn paragraphs(page: &str) -> Vec<String> {
    page.split(|c| c == '\n')
        .collect::<Vec<_>>()
        .join(" ")
        .split("</p>")
        .flat_map(|chunk| chunk.split("</li>"))
        .map(|chunk| {
            let mut text = String::new();
            let mut in_tag = false;
            for c in chunk.chars() {
                match c {
                    '<' => in_tag = true,
                    '>' => in_tag = false,
                    _ if !in_tag => text.push(c),
                    _ => {}
                }
            }
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .filter(|p| !p.is_empty())
        .collect()
}

fn refused_in(page: &str) -> Vec<String> {
    paragraphs(page)
        .into_iter()
        .filter(|p| {
            REFUSED.iter().any(|r| p.contains(r)) && !NAMES_ONE.iter().any(|n| p.contains(n))
        })
        .collect()
}

#[test]
fn no_page_says_fewer_network_connections_than_the_fence() {
    let dir = std::env::temp_dir().join(format!("sv-website-fence-{}", std::process::id()));
    build(&dir);
    let mut pages: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "html"))
        .collect();
    pages.sort();
    assert!(
        pages.len() >= 8,
        "the setup: the pages were built: {pages:?}"
    );
    let mut said = Vec::new();
    for path in &pages {
        let mut page = std::fs::read_to_string(path).unwrap();
        if path.ends_with("how-it-works.html") {
            // The paper's description, shown as the paper has it, comes last, in the page's last Markdown block, and
            // may say what the paper said only under the note that says what changed since.
            let note = page
                .find(NOTE)
                .expect("How it works shows the note on what changed since the paper");
            let paper = page
                .rfind(r#"<div class="md">"#)
                .expect("the setup: the paper's description is shown");
            assert!(
                note < paper,
                "the note comes after the paper's description, so it does not lead into it"
            );
            page.truncate(paper);
        }
        for p in refused_in(&page) {
            said.push(format!(
                "{}: {p}",
                path.file_name().unwrap().to_string_lossy()
            ));
        }
    }
    assert!(
        said.is_empty(),
        "a page says fewer network connections than \"The fence\" allows:\n{}",
        said.join("\n")
    );
}

#[test]
fn the_refusal_reads_a_paragraph_whole() {
    let page = "<p>It fetches one commit, and nothing else leaves your computer.</p>\
                <li>The one download, and nothing else leaves your computer, but Semgrep fetches its rules.</li>";
    let found = refused_in(page);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].starts_with("It fetches one commit"));
}
