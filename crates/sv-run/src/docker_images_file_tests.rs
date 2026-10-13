//! The helper images are named in one file, `images/Dockerfile`, which the code compiles in, CI fetches from, and
//! Dependabot moves (backlog 0317). These hold the three readers to the same file.

use super::*;

const WORKFLOW: &str = include_str!("../../../.github/workflows/rust.yml");

#[test]
fn a_line_is_read_only_when_it_is_from_an_image_as_that_stage() {
    let image = |line: &str, stage: &str| {
        image_of_line(line.as_bytes(), stage.as_bytes())
            .map(|i| String::from_utf8(i.to_vec()).unwrap())
    };
    assert_eq!(
        image("FROM a:1@sha256:00 AS probe", "probe").as_deref(),
        Some("a:1@sha256:00")
    );
    assert_eq!(
        image("FROM a:1 AS probe\r", "probe").as_deref(),
        Some("a:1")
    );
    assert_eq!(
        image("FROM a:1 AS probe  ", "probe").as_deref(),
        Some("a:1")
    );
    // Another stage, a stage that only starts the same, no stage, a comment, and no image.
    assert_eq!(image("FROM a:1 AS mail", "probe"), None);
    assert_eq!(image("FROM a:1 AS probes", "probe"), None);
    assert_eq!(image("FROM a:1 AS pro", "probe"), None);
    assert_eq!(image("FROM a:1", "probe"), None);
    assert_eq!(image("# FROM a:1 AS probe", "probe"), None);
    assert_eq!(image("FROM  AS probe", "probe"), None);
}

/// Every `FROM` line of the file, by the same reading CI's `awk '/^FROM/ {print $2}'` does.
fn listed() -> Vec<&'static str> {
    IMAGES
        .lines()
        .filter(|line| line.starts_with("FROM"))
        .filter_map(|line| line.split_whitespace().nth(1))
        .collect()
}

#[test]
fn the_file_lists_the_helper_images_and_nothing_else() {
    let mut listed = listed();
    let mut helpers = HELPER_IMAGES.to_vec();
    // The setup: the file was read, and each helper was found in it by its own stage.
    assert_eq!(listed.len(), 4, "{listed:?}");
    assert_eq!(PROBE_IMAGE, helper_image("probe"));
    assert!(PROBE_IMAGE.starts_with("busybox:"), "{PROBE_IMAGE}");
    assert!(MAIL_IMAGE.starts_with("axllent/mailpit:"), "{MAIL_IMAGE}");
    assert!(PROVIDER_IMAGE.starts_with("node:"), "{PROVIDER_IMAGE}");
    assert!(
        BROWSER_IMAGE.starts_with("chromedp/headless-shell:"),
        "{BROWSER_IMAGE}"
    );
    listed.sort_unstable();
    helpers.sort_unstable();
    assert_eq!(
        listed, helpers,
        "CI would fetch a different list than sv runs"
    );
}

#[test]
fn ci_fetches_the_helper_images_from_the_file_and_names_none_by_tag() {
    let fetches = WORKFLOW
        .matches("$(awk '/^FROM/ {print $2}' crates/sv-run/images/Dockerfile)")
        .count();
    // Once through the mirror, and once more signed in for what the mirror missed.
    assert_eq!(
        fetches, 2,
        "the suite's fetches no longer read images/Dockerfile"
    );
    for image in HELPER_IMAGES {
        let name = image.split(':').next().unwrap();
        // busybox and node are also the apps' own images in the tests, named there by tag; the two only sv starts
        // are named nowhere else in the workflow.
        if name == "busybox" || name == "node" {
            continue;
        }
        assert!(
            !WORKFLOW.contains(name),
            "rust.yml names {name} itself rather than reading it from images/Dockerfile"
        );
    }
}
