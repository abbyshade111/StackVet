//! Backlog 0439: `sv review` and its seals were explained in full twice, in the README and the guide, each with a step
//! the other lacked (the README's GitHub path for `SV_TRUSTED_SEALS`, the guide's `touch` so Docker does not make a
//! folder in the list's place). `docs/REVIEW.md` is now the one full account; the README and the guide say it short
//! and link there. A step dropped from the home, or a page that stops linking, fails here.

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
fn the_review_page_holds_every_step_and_the_other_pages_link_to_it() {
    let home = read("docs/REVIEW.md");
    for (what, words) in [
        ("the picture", "```text"),
        (
            "the GitHub path",
            "SV_TRUSTED_SEALS: ${{ vars.SV_TRUSTED_SEALS }}",
        ),
        (
            "the touch step",
            &format!(
                "touch ~/.config/stackvet/{}",
                sv_check::signed::TRUSTED_FILE
            ),
        ),
        ("what a seal cannot show", "## What it cannot show"),
        (
            "an unrecorded owner line",
            "never recorded this way still counts as the tool's word",
        ),
    ] {
        assert!(home.contains(words), "docs/REVIEW.md has lost {what}");
    }
    for (page, link) in [
        ("README.md", "(docs/REVIEW.md)"),
        ("docs/GETTING-STARTED.md", "(REVIEW.md)"),
    ] {
        assert!(
            read(page).contains(link),
            "{page} does not link to docs/REVIEW.md"
        );
    }
}
