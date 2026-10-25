//! Backlog 0629: the storage check after the password sign-out keeps its own words now that a sign-in through the
//! test provider shares its core: "In a real browser:" and no way in named, where the provider's says which way.

use super::*;

#[test]
fn the_password_sign_outs_notes_name_no_other_way_in() {
    let (o, _) = run_sign_out(App {
        storage: "unreadable",
        ..App::default()
    });
    let said: Vec<&str> = o
        .not_assessed
        .iter()
        .filter(|(ids, _)| ids == "V14.3.1")
        .map(|(_, why)| why.as_str())
        .collect();
    // The setup: the storage could not be read, so a note was written.
    assert!(!said.is_empty(), "{:?}", o.not_assessed);
    for why in said {
        assert!(why.starts_with("In a real browser: "), "{why}");
        assert!(!why.contains("test provider"), "{why}");
    }
}
