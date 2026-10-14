//! Backlog 0485: how `sv` decides what applies (the claims, the code, the level, and the four bins) was told in four
//! places in prose and drawn in none. It is drawn once, in `docs/ARCHITECTURE.md`, in the order
//! `sv_frameworks::applicability::bucket` applies its rules; the README and the guide link there. A bin dropped, the
//! rules drawn in another order, or a page that stops linking, fails here.

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
fn the_picture_draws_the_four_bins_in_the_order_bucket_decides() {
    let architecture = read("docs/ARCHITECTURE.md");
    let start = architecture
        .find("\n## How `sv` decides what applies\n")
        .expect("ARCHITECTURE.md has the section");
    let rest = &architecture[start + 1..];
    let section = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
    let picture = section
        .split("```text\n")
        .nth(1)
        .and_then(|p| p.split("```").next())
        .expect("the section has its picture");
    // The order `bucket` tests them in: the level, no rule, a yes, an unknown, and only then a no.
    let mut at = 0;
    for step in [
        "above the level",
        "no rule ties it to a condition",
        "is yes",
        "is unknown",
        "every condition that brings it in is no",
    ] {
        let found = picture[at..].find(step).unwrap_or_else(|| {
            panic!("{step:?} is missing, or out of the order bucket decides in")
        });
        at += found + step.len();
    }
    for bin in [
        "ABOVE THE APP'S LEVEL",
        "APPLIES",
        "NOT ASSESSED",
        "DOES NOT APPLY",
    ] {
        assert!(picture.contains(bin), "the picture has no bin {bin}");
    }
    for input in [
        "[app] audience",
        "[data] categories",
        "sv review",
        "never remove",
    ] {
        assert!(
            picture.contains(input),
            "the picture does not show {input:?}"
        );
    }
    for (page, link) in [
        (
            "README.md",
            "(docs/ARCHITECTURE.md#how-sv-decides-what-applies)",
        ),
        (
            "docs/GETTING-STARTED.md",
            "(ARCHITECTURE.md#how-sv-decides-what-applies)",
        ),
    ] {
        assert!(
            read(page).contains(link),
            "{page} does not link the picture"
        );
    }
}
