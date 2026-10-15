//! An ordinary user's 200 for an admin page is read beside the admin's own page (backlog 0560, ADR-119): a 403
//! page served with 200 is a 2xx too.

use super::super::fake_app::*;
use super::*;

fn why_not(o: &Outcome) -> Vec<&String> {
    o.not_assessed
        .iter()
        .filter(|(ids, _)| ids == "V8.2.1")
        .map(|(_, why)| why)
        .collect()
}

#[test]
fn a_denial_served_with_200_is_neither_a_finding_nor_a_refusal() {
    let o = run_against(
        Flaws {
            admin_denied_on_200: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        !rule_ids(&o).contains(&ADMIN_PAGE.rule_id),
        "a denial on a 200 page was read as the page opening: {:?}",
        o.findings
    );
    assert!(!verified_ids(&o).contains(&ADMIN_PAGE.rule_id));
    assert!(
        why_not(&o)
            .iter()
            .any(|w| w
                .contains("/admin answered 200 with a page that shows little of what the admin")),
        "{:?}",
        why_not(&o)
    );
}

#[test]
fn the_admin_page_itself_served_to_an_ordinary_user_is_still_found() {
    let o = run_against(
        Flaws {
            admin_open: true,
            ..Default::default()
        },
        &users(),
    );
    assert!(
        rule_ids(&o).contains(&ADMIN_PAGE.rule_id),
        "{:?}",
        o.findings
    );
}

#[test]
fn a_page_not_judged_keeps_the_credit_from_resting_on_the_pages_that_were() {
    // Two admin pages: one refused with 403, which alone would be credited, and one denied on a 200 page.
    let mut u = users();
    u.admin = vec!["/admin/people".into(), "/admin".into()];
    let alone = {
        let mut only = u.clone();
        only.admin = vec!["/admin/people".into()];
        run_against(
            Flaws {
                admin_denied_on_200: true,
                ..Default::default()
            },
            &only,
        )
    };
    assert!(
        verified_ids(&alone).contains(&ADMIN_PAGE.rule_id),
        "the page refused with 403 is credited alone: {:?}",
        alone.not_assessed
    );
    let both = run_against(
        Flaws {
            admin_denied_on_200: true,
            ..Default::default()
        },
        &u,
    );
    assert!(!verified_ids(&both).contains(&ADMIN_PAGE.rule_id));
    assert!(!rule_ids(&both).contains(&ADMIN_PAGE.rule_id));
}
