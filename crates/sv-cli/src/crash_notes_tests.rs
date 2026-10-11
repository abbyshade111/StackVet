//! The crash files kept under the history folder are read for the dashboard, and one that lacks a line is
//! counted as unreadable, not guessed at (backlog 0227, ADR-086).

use super::*;

const WRITTEN: &str = "version: 0.9.5\ncommand: run\nplace: outside_tools\n";

#[test]
fn a_crash_file_with_its_three_lines_is_read_with_its_time() {
    let crash = parse("crash-1791686211.txt", WRITTEN).expect("a file sv wrote is read");
    assert_eq!(crash.when, crate::bundle::utc_time(1791686211));
    assert_eq!(crash.version, "0.9.5");
    assert_eq!(crash.command, "run");
    assert_eq!(crash.place, "outside_tools");
}

#[test]
fn a_crash_file_missing_a_line_or_not_named_as_one_is_not_read() {
    assert_eq!(
        parse("crash-1791686211.txt", "version: 0.9.5\ncommand: run\n"),
        None
    );
    assert_eq!(parse("notes-1791686211.txt", WRITTEN), None);
    assert_eq!(parse("crash-soon.txt", WRITTEN), None);
}

#[test]
fn the_folder_gives_the_crashes_oldest_first_and_counts_those_it_cannot_read() {
    // The setup: a folder with two readable crashes, one unreadable, and a file that is not a crash at all.
    let folder = std::env::temp_dir().join(format!("sv-crash-notes-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("crash-1791686300.txt"), WRITTEN).unwrap();
    std::fs::write(folder.join("crash-1791686211.txt"), WRITTEN).unwrap();
    std::fs::write(folder.join("crash-1791686400.txt"), "version: 0.9.5\n").unwrap();
    std::fs::write(folder.join("history.jsonl"), "{}").unwrap();

    let health = read(&folder);
    let _ = std::fs::remove_dir_all(&folder);

    assert_eq!(health.crashes.len(), 2, "{health:?}");
    assert!(
        health.crashes[0].when < health.crashes[1].when,
        "{health:?}"
    );
    assert_eq!(health.unreadable_crash_files, 1, "{health:?}");
}

#[test]
fn a_folder_that_does_not_exist_has_no_crashes_and_no_unreadable_files() {
    let missing = std::env::temp_dir().join("sv-crash-notes-no-such-folder-here");
    assert_eq!(read(&missing), Health::default());
}
