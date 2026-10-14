//! V2.2.3 and V13.2.5 each have a hand check that says why nothing in the app's folder can show them
//! (backlog 0506). Before it they were in no catalog, so the report listed them with nothing to do.

use std::path::PathBuf;

use sv_check::human::HumanChecks;

fn human() -> HumanChecks {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/human-checks.json");
    HumanChecks::load(&path).expect("human-checks.json loads")
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
