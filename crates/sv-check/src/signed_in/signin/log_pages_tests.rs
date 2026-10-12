//! A run plants the authorization-failure markers on every private page stackvet.toml lists (backlog 0232, part 5).

use super::super::fake_app::*;

#[test]
fn a_run_plants_a_marker_on_each_listed_private_page() {
    let mut users = users();
    users.private = vec!["/account".into(), "/nowhere".into(), "/notes/{id}".into()];
    let o = run_against(Flaws::default(), &users);
    let pages: Vec<(usize, &str)> = o
        .log_markers
        .refused_requests
        .iter()
        .map(|(m, _, page)| (*page, m.as_str()))
        .collect();
    // The setup: the first page's marker is planted as before.
    assert!(
        pages
            .iter()
            .any(|(p, m)| *p == 0 && m.starts_with("sv-log-refused-")),
        "{pages:?}"
    );
    // The second page has its own, numbered marker; the page with a placeholder has none.
    assert!(
        pages
            .iter()
            .any(|(p, m)| *p == 1 && m.starts_with("sv-log-refused1-")),
        "{pages:?}"
    );
    assert!(pages.iter().all(|(p, _)| *p < 2), "{pages:?}");
}
