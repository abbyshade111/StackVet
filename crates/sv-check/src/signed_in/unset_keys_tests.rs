//! Backlog 0533: each row of `UNSET` names only requirements the run lists as not assessed because its key was left
//! out: gaps the run with the key does not have. An id that is not assessed for another reason (V14.3.1 is, when
//! signing out sends no `Clear-Site-Data`) does not count for the row.

use super::fake_app::{Flaws, run_against, users, users_full};
use super::tests::with_signup;
use super::unset_keys::{UNSET, unset_in};
use sv_manifest::UsersSection;

/// The fixture with the row's key, and the same with it taken out.
fn with_and_without(key: &str) -> (UsersSection, UsersSection) {
    let with = users_full();
    let mut without = with.clone();
    match key {
        "owned" => without.owned = None,
        "update` or `delete` under `owned" => {
            let owned = without.owned.as_mut().unwrap();
            owned.update = None;
            owned.delete = None;
        }
        "logout" => without.logout = None,
        "admin" => without.admin = Vec::new(),
        "admin-actions" => without.admin_actions = Vec::new(),
        "once" => without.once = None,
        // Seeded, with and without a sign-up beside the seed.
        "signup" => {
            let mut with = users();
            with.signup = with_signup().signup;
            return (with, users());
        }
        other => panic!("no way to take out {other}"),
    }
    (with, without)
}

#[test]
fn every_row_names_only_what_leaving_its_key_out_makes_not_assessed() {
    for row in UNSET {
        let (with, without) = with_and_without(row.key);
        // The setup: the key is in the one and out of the other, as `unset_in` sees them.
        assert!(
            !unset_in(&with).iter().any(|r| r.key == row.key),
            "{}: the fixture with it lacks it",
            row.key
        );
        assert!(
            unset_in(&without).iter().any(|r| r.key == row.key),
            "{}: the fixture without it still has it",
            row.key
        );
        let before = run_against(Flaws::default(), &with).not_assessed;
        let after = run_against(Flaws::default(), &without).not_assessed;
        let added: Vec<&str> = after
            .iter()
            .filter(|gap| !before.contains(gap))
            .flat_map(|(ids, _)| ids.split(", "))
            .collect();
        for id in row.ids {
            assert!(
                added.contains(id),
                "{}: names {id}, which leaving it out did not make not assessed: {added:?}",
                row.key
            );
        }
    }
}

#[test]
fn a_full_fixture_leaves_out_only_the_sign_up_it_has_no_need_of_beside_seed() {
    let keys: Vec<&str> = unset_in(&users_full()).iter().map(|r| r.key).collect();
    assert_eq!(keys, ["signup"]);
}
