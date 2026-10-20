//! V2.2.3 and V13.2.5 each have a hand check that says why nothing in the app's folder can show them
//! (backlog 0506). Before it they were in no catalog, so the report listed them with nothing to do.

use std::path::PathBuf;

use sv_check::human::HumanChecks;

fn human() -> HumanChecks {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/human-checks.json");
    HumanChecks::load(&path).expect("human-checks.json loads")
}

/// V7.6.1 (backlog 0621): an app that signs people in through another service is asked to check by hand that
/// signing out there, or its time running out, is felt in the app. 0500 proposed it; until 20 October 2026 it was in
/// no catalog, so the report counted it among the requirements with nothing to do.
#[test]
fn signing_in_through_another_service_gets_a_check_by_hand_that_says_why() {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let notes = sv_check::notes::Catalog::load(&data.join("security-notes.json")).unwrap();
    let design = sv_check::design::Questions::load(&data.join("design-questions.json")).unwrap();
    let human = human();
    let wanted = std::collections::BTreeSet::from(["V7.6.1".to_owned()]);
    let items = sv_check::human::checklist(&notes, &design, &human, &wanted);
    assert_eq!(items.len(), 1, "{items:?}");
    assert_eq!(items[0].route, sv_check::human::Route::GoAndLook);
    for words in ["sign out of that service", "wrote down", "checked by hand"] {
        assert!(items[0].how.contains(words), "{words:?}: {}", items[0].how);
    }
    assert!(sv_check::human::without_instructions(&notes, &design, &human, &wanted).is_empty());
}

#[test]
fn business_rules_and_the_hosts_outbound_list_say_why_sv_cannot_check_them() {
    let checks = human();
    assert!(checks.checks.len() > 20, "the catalog did not load");
    for (id, why) in [
        ("V2.2.3", "a rule of your own business"),
        ("V13.2.5", "where the app is hosted"),
    ] {
        let check = checks
            .checks
            .iter()
            .find(|c| c.id == id)
            .unwrap_or_else(|| panic!("{id} has no hand check"));
        assert!(
            check.how.starts_with("`sv` does not check this:"),
            "{id}: {}",
            check.how
        );
        assert!(check.how.contains(why), "{id}: {}", check.how);
        assert!(check.how.contains("Write down"), "{id}: {}", check.how);
    }
}
