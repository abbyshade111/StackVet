//! Backlog 0437: the network rule is written out in full in one place, `docs/ARCHITECTURE.md`'s "The fence", and the
//! other pages that state it say "three exceptions" and point there. Six pages stated it with two different lists once
//! ADR-111 added the outside tools; a page that goes back to two, or a home that loses one of the three, fails here.

use std::path::Path;

const HOME: &str = "## The fence: what can reach the internet, and what turns it on";
const ANCHOR: &str =
    "docs/ARCHITECTURE.md#the-fence-what-can-reach-the-internet-and-what-turns-it-on";

fn read(relative: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative),
    )
    .unwrap_or_else(|e| panic!("{relative}: {e}"))
}

#[test]
fn the_home_draws_and_lists_all_three_exceptions() {
    let architecture = read("docs/ARCHITECTURE.md");
    let start = architecture
        .find(HOME)
        .expect("ARCHITECTURE.md has the fence's own section");
    let rest = &architecture[start + HOME.len()..];
    let section = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
    let picture_start = section
        .find("```text\n")
        .expect("the fence's section has its picture")
        + 8;
    let picture_len = section[picture_start..]
        .find("```")
        .expect("the picture ends");
    let picture = &section[picture_start..picture_start + picture_len];
    let words = section[..picture_start].to_owned() + &section[picture_start + picture_len..];
    for what in [
        "sv probe <address>",
        "install = true",
        "--no-tools",
        "semgrep.dev",
    ] {
        assert!(
            picture.contains(what),
            "the fence's picture does not name {what}"
        );
        assert!(
            words.contains(what),
            "the fence's table does not name {what}"
        );
    }
    for what in ["**off**", "**on**"] {
        assert!(
            words.contains(what),
            "the fence's table does not say which are {what}"
        );
    }
    for adr in ["ADR-027", "ADR-052", "ADR-111"] {
        assert!(
            section.contains(adr),
            "the fence's section does not cite {adr}"
        );
    }
}

#[test]
fn every_page_that_states_the_rule_says_three_and_points_home() {
    for page in ["README.md", "SECURITY.md", "CONTRIBUTING.md"] {
        let text = read(page);
        assert!(
            text.contains(ANCHOR),
            "{page} does not link to the fence's section"
        );
        assert!(
            text.contains("three exceptions"),
            "{page} does not say three exceptions"
        );
        assert!(
            !text.contains("two exceptions"),
            "{page} still says two exceptions"
        );
    }
    let architecture = read("docs/ARCHITECTURE.md");
    assert!(
        architecture.contains("(#the-fence-what-can-reach-the-internet-and-what-turns-it-on)"),
        "the rules' own fence bullet does not point to the section below it"
    );
    assert!(
        !architecture.contains("but `sv probe` and the install step"),
        "ARCHITECTURE.md's old list of two is back"
    );
}
