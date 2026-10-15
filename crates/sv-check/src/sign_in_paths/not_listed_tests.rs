//! Which ways in an entry of `[stack.auth] pathways` names, and which are left over (backlog 0491; ADR-123).

use super::{Pathway, named_by, not_listed};

fn way(key: &str) -> Pathway {
    Pathway {
        what: format!("something, {key}"),
        key: key.to_owned(),
        file: "app.py".to_owned(),
        line: 1,
    }
}

#[test]
fn an_entry_names_its_own_key_a_path_below_it_and_a_strategy_by_its_short_name() {
    assert!(named_by(&way("/login"), "/login"));
    assert!(named_by(&way("/login"), " /LOGIN/ "));
    assert!(named_by(&way("/auth/google/callback"), "/auth/google"));
    assert!(named_by(&way("GoogleStrategy"), "google"));
    assert!(named_by(&way("GoogleStrategy"), "GoogleStrategy"));
    assert!(named_by(&way("x-api-key"), "X-API-Key"));
}

#[test]
fn an_entry_does_not_name_a_neighbor_or_a_prefix_of_a_word() {
    assert!(!named_by(&way("/login-legacy"), "/login"));
    assert!(!named_by(&way("/auth"), "/auth/google"));
    assert!(!named_by(&way("/admin/login"), "/login"));
    assert!(!named_by(&way("GoogleStrategy"), "goo"));
    assert!(!named_by(&way("/login"), ""));
    assert!(!named_by(&way("/login"), "/"));
}

#[test]
fn what_no_entry_names_is_left_over() {
    let ways = [
        way("/login"),
        way("/auth/google/callback"),
        way("x-api-key"),
    ];
    let listed = ["/login".to_owned(), "/auth/google".to_owned()];
    let left: Vec<&str> = not_listed(&ways, &listed)
        .iter()
        .map(|w| w.key.as_str())
        .collect();
    assert_eq!(left, vec!["x-api-key"]);
    // An empty list names nothing: the owner said there is no way in.
    assert_eq!(not_listed(&ways, &[]).len(), 3);
}
