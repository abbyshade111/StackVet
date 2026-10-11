//! A data file that did not load is named once for the run's record, by its file name and not its path (backlog
//! 0227, ADR-086). The list is taken, and starts again empty.

use super::*;

#[test]
fn a_data_file_that_did_not_load_is_named_once_by_file_name() {
    // The setup: the same file fails twice, from a folder with the owner's own path in it. The name is unique to
    // this test, since other tests in this binary may fail a data file too.
    let name = format!("not-loaded-probe-{}.json", std::process::id());
    let path = Path::new("/Users/owner/private/project").join(&name);
    let _ = take_not_loaded();
    let message = not_understood(&path);
    not_understood(&path);
    let taken = take_not_loaded();
    assert!(taken.contains(&name), "{taken:?}");
    assert_eq!(taken.iter().filter(|n| **n == name).count(), 1, "{taken:?}");
    assert!(
        taken.iter().all(|n| !n.contains('/')),
        "a path was kept: {taken:?}"
    );
    assert!(message.contains(&name), "{message}");
    // Taken, the list starts again empty for this file.
    assert!(!take_not_loaded().contains(&name));
}
