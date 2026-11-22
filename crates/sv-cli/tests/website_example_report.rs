//! The reports page's example report is whole and current (backlog 0698; the whole-site review, backlog 0696): every
//! file the page's table lists is in `docs/website/reports/notes-with-users/` and linked from the page, and the `sv`
//! that made it is not older, in its major or minor number, than this workspace's. On 21 November 2026 the example on
//! the site had been made by `sv` 0.1.0 with 8 of the 14 files, and nothing noticed until a person read the page.
//!
//! A release that moves the version turns this red on purpose: the example is to be made again, with the command the
//! page's own notes give, which the failure prints.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

const FOLDER: &str = "docs/website/reports/notes-with-users";

/// The command the page's notes say makes the report again.
fn remake(page: &str) -> String {
    page.split('`')
        .find(|span| span.starts_with("sv report examples/notes-with-users"))
        .expect("the reports page's notes say how the example report was made")
        .to_owned()
}

/// The files in the first column of the page's table: `| `name` | ... |`.
fn listed(page: &str) -> Vec<String> {
    page.lines()
        .filter_map(|l| l.strip_prefix("| `"))
        .filter_map(|l| l.split_once('`'))
        .map(|(name, _)| name.to_owned())
        .collect()
}

fn major_minor(version: &str) -> (u32, u32) {
    let mut parts = version.split('.').map(|p| p.parse::<u32>().unwrap_or(0));
    (parts.next().unwrap_or(0), parts.next().unwrap_or(0))
}

#[test]
fn every_file_the_table_lists_is_there_and_linked() {
    let page = read("docs/website/reports-page.md");
    let files = listed(&page);
    assert!(
        files.len() >= 14,
        "the setup: the table's files were read: {files:?}"
    );
    let again = remake(&page);
    for file in &files {
        assert!(
            root().join(FOLDER).join(file).is_file(),
            "{file} is in the reports page's table but not in {FOLDER}. Make the report again, from the repository's \
             top folder: {again}"
        );
        assert!(
            page.contains(&format!("](reports/notes-with-users/{file})")),
            "{file} is in the reports page's table but the page does not link it"
        );
    }
}

#[test]
fn the_example_was_made_by_this_release_or_a_later_one() {
    let page = read("docs/website/reports-page.md");
    let workspace = read("Cargo.toml");
    let ours = workspace
        .lines()
        .skip_while(|l| l.trim() != "[workspace.package]")
        .find_map(|l| l.trim().strip_prefix("version = "))
        .expect("the setup: the workspace's version")
        .trim_matches('"')
        .to_owned();
    let made = read(&format!("{FOLDER}/security.md"));
    let by = made
        .lines()
        .find_map(|l| l.split("Produced by `sv` ").nth(1))
        .and_then(|rest| rest.split_whitespace().next())
        .expect("the example report says which sv made it");
    assert!(
        major_minor(by) >= major_minor(&ours),
        "the example report was made by sv {by}, older than this workspace's {ours}. Make it again, from the \
         repository's top folder, with sv built from this commit: {}",
        remake(&page)
    );
}
