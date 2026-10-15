//! Backlog 0579: the line on the bill of materials said "some of the app's package lists could not be read" whenever
//! the bill was not complete, though most often it is not complete because there is no lockfile, and then every list
//! was read. It says which now.

use crate::next_step_tests::empty_report;

fn line(unread_lists: usize, at_asked_versions: usize) -> String {
    let mut report = empty_report(Vec::new());
    report.sbom = Some(crate::SbomFile {
        json: "{}".into(),
        components: 13,
        complete: false,
        unread_lists,
        at_asked_versions,
    });
    crate::sbom_line(&report)
}

#[test]
fn a_missing_lockfile_is_not_said_as_a_list_unread() {
    let said = line(0, 13);
    assert!(
        said.contains(
            "13 packages, not complete: 13 are listed at the version asked for, since no lockfile says which \
             version is installed."
        ),
        "{said}"
    );
    assert!(!said.contains("could not be read"), "{said}");
}

#[test]
fn a_list_unread_is_said_so_and_both_together() {
    let said = line(1, 0);
    assert!(
        said.contains("not complete: one of the app's package lists could not be read, and the report says which."),
        "{said}"
    );
    assert!(!said.contains("version asked for"), "{said}");
    let said = line(2, 1);
    assert!(
        said.contains(
            "not complete: some of the app's package lists could not be read, and the report says which, and 1 is \
             listed at the version asked for"
        ),
        "{said}"
    );
}
