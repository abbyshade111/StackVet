//! Backlog 0496: the sites the signed-in pages tried to reach, compared with `[stack] external-apis` (V13.1.1). Only
//! ever a finding, and nothing compared when the owner has not filled the list in.

use super::*;

fn hosts(list: &[&str]) -> Vec<String> {
    list.iter().map(|h| (*h).to_owned()).collect()
}

#[test]
fn a_site_the_list_does_not_name_is_a_finding_naming_only_that_site() {
    let seen = hosts(&[
        "js.stripe.com",
        "www.google-analytics.com",
        "api.example.com",
    ]);
    let declared = hosts(&["stripe.com", "API.example.com"]);
    let (found, step) = undeclared_outside_services(Some(&seen), Some(&declared));
    assert_eq!(found.len(), 1, "one finding for the one site not named");
    let f = &found[0];
    assert_eq!(f.rule_id, "probe.undeclared-outside-service");
    assert_eq!(f.requirement_ids, ["V13.1.1"]);
    assert!(
        f.description.contains("www.google-analytics.com"),
        "{}",
        f.description
    );
    assert!(
        !f.description.contains("js.stripe.com") && !f.description.contains("api.example.com"),
        "a site named, or under one named, is not reported: {}",
        f.description
    );
    let step = step.expect("the run says what it compared");
    assert!(step.contains("1 site that"), "{step}");
}

#[test]
fn a_parent_domain_counts_only_for_the_sites_under_it() {
    // `stripe.com` is not a suffix match for `evilstripe.com`: only a whole label counts.
    let seen = hosts(&["evilstripe.com"]);
    let declared = hosts(&["stripe.com"]);
    let (found, _) = undeclared_outside_services(Some(&seen), Some(&declared));
    assert_eq!(found.len(), 1);
}

#[test]
fn every_site_named_is_no_finding_and_says_so() {
    let seen = hosts(&["api.example.com"]);
    let declared = hosts(&["api.example.com"]);
    let (found, step) = undeclared_outside_services(Some(&seen), Some(&declared));
    assert!(found.is_empty());
    assert!(step.unwrap().starts_with("every site"));
}

#[test]
fn an_empty_list_the_owner_wrote_names_nothing() {
    let seen = hosts(&["cdn.example.net"]);
    let (found, _) = undeclared_outside_services(Some(&seen), Some(&[]));
    assert_eq!(
        found.len(),
        1,
        "`external-apis = []` says the app calls nothing"
    );
}

#[test]
fn no_list_is_not_compared_and_the_run_says_why() {
    let seen = hosts(&["cdn.example.net"]);
    let (found, step) = undeclared_outside_services(Some(&seen), None);
    assert!(
        found.is_empty(),
        "an owner who said nothing is not found wanting"
    );
    let step = step.expect("the run says why nothing was compared");
    assert!(step.contains("not filled in"), "{step}");
}

#[test]
fn nothing_seen_compares_nothing_and_says_nothing() {
    let declared = hosts(&["api.example.com"]);
    for seen in [None, Some(Vec::new())] {
        let (found, step) = undeclared_outside_services(seen.as_deref(), Some(&declared));
        assert!(found.is_empty());
        assert!(step.is_none());
    }
}

#[test]
fn the_browser_check_keeps_the_hosts_the_pages_tried_once_each() {
    // A made-up account: the request carries none of its details, so only the hosts are at stake here.
    let account = Account {
        user: "first@example.test".to_owned(),
        password: ["not", "-", "sent"].concat(),
    };
    let sent = json!({ "requests": [
        { "url": "https://cdn.example.net/app.js", "body": "", "headers": "" },
        { "url": "https://user:pw@stats.example.org:8443/hit?x=1", "body": "", "headers": "" },
        { "url": "https://cdn.example.net/other.css", "body": "", "headers": "" },
    ]});
    let mut out = Outcome::default();
    details_sent_elsewhere(&account, &[], &sent, &mut out);
    assert_eq!(
        out.outside_hosts,
        Some(hosts(&["cdn.example.net", "stats.example.org"]))
    );

    // The browser did not say: nothing is kept, so nothing is compared.
    let mut out = Outcome::default();
    details_sent_elsewhere(&account, &[], &json!({}), &mut out);
    assert_eq!(out.outside_hosts, None);
}
