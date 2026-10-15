//! Sweep 3 (backlog 0561, ADR-119), inventory item 4: every private page listed is opened as the signed-in user, not
//! only the first, before the private-page credit says they were. A listed address that refuses everybody is refused
//! to a stranger as well, and that shows nothing about whether it is private.

use super::fake_app::*;
use super::*;

#[test]
fn a_listed_page_that_refuses_everybody_is_not_credited_as_private() {
    let mut u = users_full();
    u.private = vec!["/account".into(), "/nowhere".into()];
    let (o, _) = run_seeded(FakeApp::new(Flaws::default()), &u);
    let credit = o
        .verified
        .iter()
        .find(|v| v.check_id == PRIVATE_PAGE.rule_id)
        .unwrap_or_else(|| panic!("{:#?}", o.steps));
    assert!(
        credit.scope.starts_with("1 private page,"),
        "{}",
        credit.scope
    );
    assert!(credit.in_part, "{credit:?}");
    assert!(
        o.not_assessed
            .iter()
            .any(|(_, why)| why.starts_with("/nowhere (")
                && why.contains("refused the signed-in user")),
        "{:?}",
        o.not_assessed
    );
}

// Two pages that each open are credited in full: `in_part_tests::two_pages_listed_are_each_asked_and_credited_in_full`.
