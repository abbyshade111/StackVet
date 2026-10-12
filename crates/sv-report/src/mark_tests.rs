//! The StackVet mark in `report.html` and the dashboard (backlog 0219 part 5, ADR-107): drawn inside
//! the page, the brand file's own shape and colors, and nothing fetched for it.

use crate::html::{CSP_META, MARK, STYLE};

fn brand(file: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/brand")
        .join(file);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Every `name="..."` value in `svg`, in order.
fn values<'a>(svg: &'a str, name: &str) -> Vec<&'a str> {
    let key = format!("{name}=\"");
    svg.split(key.as_str())
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap())
        .collect()
}

#[test]
fn the_mark_is_the_brand_files_shape_and_colors() {
    let light = brand("stackvet-mark.svg");
    let dark = brand("stackvet-mark-dark.svg");
    assert_eq!(values(MARK, "d"), values(&light, "d"), "the paths drawn");
    assert_eq!(values(&light, "d").len(), 3, "the setup: three paths read");
    // Light: brackets, brackets, check; and the dark file's, in the dark rule.
    let (l, d) = (values(&light, "stroke"), values(&dark, "stroke"));
    assert_eq!(l.len(), 3, "the setup: {l:?}");
    assert!(
        STYLE.contains(&format!(".mark-bracket {{ stroke: {};", l[0])),
        "{STYLE}"
    );
    assert!(
        STYLE.contains(&format!(".mark-check {{ stroke: {}; }}", l[2])),
        "{STYLE}"
    );
    let dark_rule = STYLE
        .split("@media (prefers-color-scheme: dark) { .mark-bracket")
        .nth(1)
        .expect("a dark rule for the mark");
    assert!(
        dark_rule.starts_with(&format!(" {{ stroke: {}; }}", d[0])),
        "{dark_rule}"
    );
    assert!(
        dark_rule.contains(&format!(".mark-check {{ stroke: {}; }}", d[2])),
        "{dark_rule}"
    );
}

#[test]
fn both_pages_draw_it_in_their_heading_and_fetch_nothing_for_it() {
    let report = crate::html::page(&crate::next_step_tests::empty_report(Vec::new()));
    let dashboard = crate::dashboard::page(
        &[],
        "2026-10-12",
        None,
        &crate::dashboard::Health::default(),
    );
    for page in [&report, &dashboard] {
        let h1 = page
            .split("<h1>")
            .nth(1)
            .and_then(|rest| rest.split("</h1>").next())
            .expect("a heading");
        assert!(h1.starts_with(MARK), "{h1}");
        assert_eq!(page.matches("class=\"mark\"").count(), 1, "once");
        // Nothing for it is loaded: no image, no link, and the policy as it was.
        assert!(!page.contains("<img") && !page.contains("<link"), "{page}");
        assert!(page.contains(CSP_META.trim_end()));
    }
    assert!(!MARK.contains("href") && !MARK.contains("url("), "{MARK}");
    assert!(MARK.contains("aria-hidden=\"true\""));
}
