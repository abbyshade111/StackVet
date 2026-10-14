//! The roadmap at the top of `docs/BACKLOG.md` is a table whose status column is read from each item's own status line
//! (backlog 0486), so it cannot say an item is open when its file says done. `tools/backlog.py roadmap` compares the two
//! and fails on a stale row; this runs it on the real files, and checks the table is there to be compared.

use std::process::Command;

fn root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn the_roadmap_is_a_table_between_its_markers() {
    let text =
        std::fs::read_to_string(root().join("docs/BACKLOG.md")).expect("docs/BACKLOG.md reads");
    let start = text
        .find("<!-- roadmap:start -->")
        .expect("the roadmap's start marker");
    let end = text
        .find("<!-- roadmap:end -->")
        .expect("the roadmap's end marker");
    assert!(start < end, "the markers are in order");
    let table = &text[start..end];
    assert!(
        table.contains("| Order | Phase | Item | Status | Asked for |"),
        "the roadmap's header row:\n{table}"
    );
    let rows = table
        .lines()
        .filter(|l| l.starts_with("| ") && l.contains(" | 0"))
        .count();
    assert!(rows > 20, "the roadmap holds its rows, read {rows}");
}

#[test]
fn the_roadmaps_statuses_are_the_items_own() {
    let out = Command::new("python3")
        .arg("-I")
        .arg(root().join("tools/backlog.py"))
        .arg("roadmap")
        .output()
        .expect("python3 runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    assert!(
        said.contains("the roadmap's statuses are the items' own"),
        "{said}"
    );
}
