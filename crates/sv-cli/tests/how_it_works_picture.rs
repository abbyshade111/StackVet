//! Backlog 0436: the path from "I built an app" to "I have a report" is drawn once, as
//! `docs/website/diagrams/how-it-works.svg`, near the top of the README and of the guide. The picture is made by the
//! script beside it; a picture edited by hand, or a script changed without the picture made again, fails here, and so
//! does a page that stops showing it.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo().join(relative)).unwrap_or_else(|e| panic!("{relative}: {e}"))
}

#[test]
fn the_picture_is_what_its_script_draws_and_both_first_pages_show_it() {
    let dir = std::env::temp_dir().join(format!("sv-how-it-works-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let fresh = dir.join("how-it-works.svg");
    let out = Command::new("python3")
        .arg("-I")
        .arg(repo().join("docs/website/diagrams/make_how_it_works.py"))
        .arg(&fresh)
        .output()
        .expect("python3 runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let drawn = std::fs::read_to_string(&fresh).expect("the script wrote the picture");
    std::fs::remove_dir_all(&dir).ok();
    let committed = read("docs/website/diagrams/how-it-works.svg");
    assert_eq!(
        committed, drawn,
        "how-it-works.svg is not what make_how_it_works.py draws: run the script and commit its picture"
    );
    for words in [
        "You",
        "Your AI coding tool",
        ">sv<",
        "stackvet_check",
        "sv review",
        "sv report --run",
        "<title",
        "<desc",
    ] {
        assert!(
            committed.contains(words),
            "the picture does not show {words}"
        );
    }
    for (page, src) in [
        ("README.md", "(docs/website/diagrams/how-it-works.svg)"),
        (
            "docs/GETTING-STARTED.md",
            "(website/diagrams/how-it-works.svg)",
        ),
    ] {
        let text = read(page);
        let at = text
            .find(src)
            .unwrap_or_else(|| panic!("{page} does not show the picture"));
        assert!(
            text[..at].matches("\n## ").count() == 0,
            "{page} shows the picture below its first section, not near its top"
        );
    }
}
