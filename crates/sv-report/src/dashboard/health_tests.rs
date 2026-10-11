//! The dashboard's section on `sv`'s own health says what it kept, and says when it kept nothing (backlog 0227,
//! ADR-086).

use super::*;

fn crash(when: &str, place: &str) -> Crash {
    Crash {
        when: when.to_owned(),
        version: "0.9.5".to_owned(),
        command: "run".to_owned(),
        place: place.to_owned(),
    }
}

#[test]
fn the_section_says_when_no_crash_was_kept() {
    let section = health_section(&Health::default(), 0);
    assert!(
        section.contains("sv's own health on this computer"),
        "{section}"
    );
    assert!(
        section.contains("No crashes are kept on this computer."),
        "{section}"
    );
    assert!(
        !section.contains("kept on this computer. Each"),
        "{section}"
    );
}

#[test]
fn each_crash_is_listed_with_its_time_version_command_and_place_escaped() {
    let health = Health {
        crashes: vec![
            crash("2026-10-11T03:00:00Z", "outside_tools"),
            crash("2026-10-11T04:00:00Z", "<b>x</b>"),
        ],
        unreadable_crash_files: 0,
    };
    let section = health_section(&health, 0);
    assert!(
        section.contains("2 crashes kept on this computer"),
        "{section}"
    );
    assert!(section.contains("2026-10-11T03:00:00Z: sv 0.9.5, running <code>run</code>, stopped at <code>outside_tools</code>"), "{section}");
    assert!(
        !section.contains("<b>x</b>"),
        "a place was written unescaped: {section}"
    );
    assert!(section.contains("&lt;b&gt;x&lt;/b&gt;"), "{section}");
}

#[test]
fn unreadable_crash_files_and_kept_runs_are_counted_when_there_are_any() {
    let health = Health {
        crashes: Vec::new(),
        unreadable_crash_files: 1,
    };
    let section = health_section(&health, 2);
    assert!(
        section.contains("1 crash file in the history folder could not be read."),
        "{section}"
    );
    assert!(
        section.contains("2 kept runs in all could not be read"),
        "{section}"
    );
    let quiet = health_section(&Health::default(), 0);
    assert!(!quiet.contains("could not be read"), "{quiet}");
}
