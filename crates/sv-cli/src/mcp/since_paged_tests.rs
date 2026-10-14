//! A finding the AI coding tool was only listed, on a page of a first check that came in parts, is not named as
//! "unchanged since your check" in the next one (backlog 0453): the server recorded every finding as given, so one the
//! tool never read in full became a line with no fix from then on.

use super::since_tests::{check, fingerprints, recipes};
use super::tests::text;
use super::*;

#[test]
fn a_finding_only_listed_in_a_paged_check_is_not_called_unchanged_next_time() {
    // Ten files of three weak hashes: a first check too long to come whole, and a second short enough to name the
    // unchanged in a line each. Before the fix it named all 31 as unchanged, 14 of them given in full.
    let root = recipes("since-paged", 10);
    let every = {
        let server = Server::new(&root).unwrap();
        let all = check(&server, json!({ "path": "app", "section": "all" }));
        fingerprints(&all["structuredContent"]["findings"], "fingerprint")
    };

    let server = Server::new(&root).unwrap();
    let app = json!({ "path": "app" });
    let first = check(&server, app.clone());
    let given = fingerprints(&first["structuredContent"]["findings"], "fingerprint");
    assert!(
        first["structuredContent"].get("part").is_some(),
        "the setup: the first check comes in parts"
    );
    assert!(
        !given.is_empty() && given.len() < every.len(),
        "the setup: some findings were only listed ({} of {})",
        given.len(),
        every.len()
    );

    let second = check(&server, app);
    std::fs::remove_dir_all(&root).ok();
    let data = &second["structuredContent"];
    match data.get("unchanged") {
        Some(unchanged) => {
            let unchanged = fingerprints(unchanged, "fingerprint");
            for fingerprint in &unchanged {
                assert!(
                    given.contains(fingerprint),
                    "{fingerprint} is called unchanged, though the first answer only listed it: {}",
                    text(&second)
                );
            }
            // Every finding is still accounted for: unchanged, or given in full.
            let again = fingerprints(&data["findings"], "fingerprint");
            assert_eq!(unchanged.len() + again.len(), every.len(), "{data:#}");
        }
        // Too long to come whole with the rest in full: the ordinary answer, in parts, as the first was.
        None => assert!(data.get("part").is_some(), "{data:#}"),
    }
}

#[test]
fn a_whole_check_still_names_every_finding_as_unchanged_next_time() {
    // The rule kept: a first check that came whole gave every finding in full.
    let root = recipes("since-whole", 4);
    let server = Server::new(&root).unwrap();
    let app = json!({ "path": "app" });
    let first = check(&server, app.clone());
    assert!(
        first["structuredContent"].get("part").is_none(),
        "the setup: it comes whole"
    );
    let given = fingerprints(&first["structuredContent"]["findings"], "fingerprint");
    let second = check(&server, app);
    std::fs::remove_dir_all(&root).ok();
    assert_eq!(
        fingerprints(&second["structuredContent"]["unchanged"], "fingerprint"),
        given
    );
}
