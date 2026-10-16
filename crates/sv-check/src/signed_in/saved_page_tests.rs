//! Backlog 0535 (ADR-126): a member's own page, saved through a `creates` entry marked `page = true` and fetched
//! where it is read back, judged as an uploaded page is (V3.2.1).

use super::fake_app::*;
use super::tests::with_page;
use super::*;

/// Where the fake app serves a member's page: under the record's own address.
const FRAME: Option<&str> = Some("/pages/{id}/frame");

fn run(flaws: Flaws, read: Option<&str>) -> Outcome {
    run_against(flaws, &with_page(users(), read))
}

fn credited(o: &Outcome) -> Option<String> {
    o.verified
        .iter()
        .find(|v| v.check_id == PAGE_RENDERED.rule_id)
        .map(|v| v.scope.clone())
}

fn gap(o: &Outcome) -> Option<&str> {
    o.not_assessed
        .iter()
        .find(|(id, why)| id == "V3.2.1" && why.contains("/pages"))
        .map(|(_, why)| why.as_str())
}

/// The setup: the page was saved and fetched back, so what is judged is how it was served.
fn fetched(o: &Outcome) -> bool {
    o.steps
        .iter()
        .any(|s| s.starts_with("fetched the saved page from /pages/") && s.contains("/frame:"))
}

#[test]
fn a_sandboxed_page_is_credited_in_part_and_not_reported() {
    let o = run(Flaws::default(), FRAME);
    assert!(fetched(&o), "the setup: the saved page was fetched back");
    let scope = credited(&o).expect("V3.2.1 not credited for a sandboxed page");
    assert!(scope.contains("with a sandbox policy"), "{scope}");
    assert!(
        credited_in_part(&o, PAGE_RENDERED.rule_id),
        "credited whole"
    );
    assert!(!rule_ids(&o).contains(&PAGE_RENDERED.rule_id), "reported");
}

#[test]
fn a_page_served_as_part_of_the_app_is_found_and_not_credited() {
    let o = run(
        Flaws {
            pages_unsandboxed: true,
            ..Default::default()
        },
        FRAME,
    );
    assert!(fetched(&o), "the setup: the saved page was fetched back");
    assert!(rule_ids(&o).contains(&PAGE_RENDERED.rule_id));
    assert!(credited(&o).is_none());
}

#[test]
fn a_page_with_its_script_escaped_is_neither_credited_nor_found() {
    let o = run(
        Flaws {
            pages_unsandboxed: true,
            pages_escaped: true,
            ..Default::default()
        },
        FRAME,
    );
    assert!(fetched(&o), "the setup: the saved page was fetched back");
    assert!(!rule_ids(&o).contains(&PAGE_RENDERED.rule_id));
    assert!(credited(&o).is_none());
    assert!(
        gap(&o).is_some_and(|why| why.contains("taken out or escaped")),
        "the gap does not say why"
    );
}

#[test]
fn a_page_read_on_a_host_of_its_own_is_not_fetched_and_says_so() {
    let o = run(
        Flaws {
            pages_unsandboxed: true,
            ..Default::default()
        },
        Some("https://pages.example.net/{id}"),
    );
    assert!(
        !o.steps.iter().any(|s| s.contains("a marked script")),
        "the page was saved"
    );
    assert!(credited(&o).is_none());
    assert!(
        gap(&o).is_some_and(|why| why.contains("a host of its own")),
        "the gap does not say why"
    );
}

#[test]
fn a_read_address_without_the_page_is_not_judged() {
    let o = run(Flaws::default(), Some("/pages/99/frame"));
    assert!(credited(&o).is_none());
    assert!(!rule_ids(&o).contains(&PAGE_RENDERED.rule_id));
    assert!(
        gap(&o).is_some_and(|why| why.contains("was not found at /pages/99/frame")),
        "the gap does not say why"
    );
}
