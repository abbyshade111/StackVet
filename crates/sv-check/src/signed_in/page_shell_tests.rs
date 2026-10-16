//! Tests of `signed_in/mod.rs`: a private page that is only the page shell of a single-page app, and the addresses it
//! fetches judged beside it. Moved out of `mod.rs` unchanged (backlog 0324).

use super::fake_app::*;
use super::*;

fn spa() -> Flaws {
    Flaws {
        page_shell: true,
        ..Default::default()
    }
}

fn with_private(private: &[&str]) -> UsersSection {
    UsersSection {
        private: private.iter().map(|p| (*p).to_owned()).collect(),
        ..users()
    }
}

fn answer(status: u16, body: &str) -> Option<ProbeResponse> {
    Some(ProbeResponse {
        id: String::new(),
        status,
        headers: Vec::new(),
        body: body.to_owned(),
    })
}

#[test]
fn a_private_page_that_is_only_the_page_shell_is_not_judged_and_says_what_to_list() {
    let o = run_against(spa(), &with_private(&["/account"]));
    assert!(
        !rule_ids(&o).contains(&PRIVATE_PAGE.rule_id),
        "the page shell was taken for the private page: {:#?}",
        o.findings
    );
    assert!(
        !verified_ids(&o).contains(&PRIVATE_PAGE.rule_id),
        "nothing was judged, so nothing is credited"
    );
    let (_, why) = o
        .not_assessed
        .iter()
        .find(|(ids, _)| ids.contains("V8.2.1"))
        .expect("V8.2.1 says why it was not judged");
    assert!(why.contains("/account") && why.contains("/api/me"), "{why}");
    // The checks after it are given no shell to compare with either: a shell answered after
    // signing out is not a session that still works.
    assert!(o.findings.is_empty(), "{:#?}", o.findings);
}

#[test]
fn the_address_the_page_fetches_is_judged_beside_the_shell() {
    let o = run_against(spa(), &with_private(&["/account", "/api/me"]));
    assert!(o.findings.is_empty(), "{:#?}", o.findings);
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == PRIVATE_PAGE.rule_id)
        .expect("/api/me refused a stranger and opened for A");
    assert!(
        credit.scope.starts_with("1 private page,"),
        "{}",
        credit.scope
    );
    assert!(
        credit
            .scope
            .contains("/account answered as the front page does"),
        "{}",
        credit.scope
    );
    assert!(
        o.steps.iter().any(|s| s.contains("page shell")),
        "{:?}",
        o.steps
    );
}

#[test]
fn an_open_address_behind_the_shell_is_still_found() {
    let o = run_against(
        Flaws {
            private_open: true,
            ..spa()
        },
        &with_private(&["/account", "/api/me"]),
    );
    let found = o
        .findings
        .iter()
        .find(|f| f.rule_id == PRIVATE_PAGE.rule_id)
        .expect("/api/me opens for anybody");
    assert!(
        found.description.contains("/api/me"),
        "{}",
        found.description
    );
    assert!(
        !found.description.contains("/account"),
        "{}",
        found.description
    );
}

#[test]
fn only_a_whole_answer_exactly_the_front_pages_is_a_shell() {
    let front = answer(200, PAGE_SHELL);
    assert!(is_page_shell(&answer(200, PAGE_SHELL), &front));
    // Another page, a refusal, an empty answer, or no front page to compare with.
    assert!(!is_page_shell(&answer(200, "your account"), &front));
    assert!(!is_page_shell(&answer(302, PAGE_SHELL), &front));
    assert!(!is_page_shell(
        &answer(200, PAGE_SHELL),
        &answer(404, PAGE_SHELL)
    ));
    assert!(!is_page_shell(&answer(200, ""), &answer(200, "")));
    assert!(!is_page_shell(&answer(200, PAGE_SHELL), &None));
    // A server-rendered page as long as the kept part may differ from the front page only
    // past the cut, so matching that far is not the same page.
    let long = "<div>layout</div>".repeat(crate::probes::KEPT_CHARS);
    let cut: String = long.chars().take(crate::probes::KEPT_CHARS).collect();
    assert!(!is_page_shell(&answer(200, &cut), &answer(200, &cut)));
}
