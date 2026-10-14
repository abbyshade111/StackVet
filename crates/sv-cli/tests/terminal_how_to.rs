//! A person who has never opened a terminal is told how (backlog 0473): the glossary's Terminal entry says how to open
//! one on each system, and how to open one in the app's folder, and the guide's connect step has Docker's path copied
//! from `which docker` rather than given by a shell substitution.

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
fn the_glossary_says_how_to_open_a_terminal_and_one_in_a_folder() {
    let glossary = read("docs/GLOSSARY.md");
    let start = glossary.find("## Terminal").expect("a Terminal entry");
    let end = start
        + 3
        + glossary[start + 3..]
            .find("\n## ")
            .unwrap_or(glossary.len() - start - 3);
    // As one line, since the entry wraps where it likes.
    let entry = glossary[start..end]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for words in [
        "on a Mac, press Command and the space bar",
        "On Windows, open the Start menu, type `PowerShell`",
        "On Linux, look for Terminal",
        "To open one in your app's folder",
        "drag the folder",
        "open the folder in File Explorer",
    ] {
        assert!(
            entry.contains(words),
            "the Terminal entry lacks {words:?}: {entry}"
        );
    }
}

#[test]
fn the_guide_has_dockers_path_copied_not_substituted() {
    let guide = read("docs/GETTING-STARTED.md");
    assert!(
        !guide.contains("$(command -v docker)"),
        "a shell substitution is back"
    );
    assert!(
        guide.contains("`which docker` prints"),
        "the guide no longer says how to find Docker's path"
    );
    assert!(
        guide.contains("this is just `sv review .`"),
        "the long review line no longer says what replaces it"
    );
}
