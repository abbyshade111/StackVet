//! The crash files `sv` keeps under the history folder (backlog 0237), read for the dashboard's health section
//! (backlog 0227, ADR-086). A file is read only when it has the three lines `write_crash_file` writes; one
//! without them is counted as unreadable, not guessed at.

use std::path::Path;
use sv_report::dashboard::{Crash, Health};

/// One crash file, by its name and its text: `crash-<seconds>.txt`, with `version`, `command`, and `place` lines.
pub fn parse(name: &str, text: &str) -> Option<Crash> {
    let seconds: u64 = name
        .strip_prefix("crash-")?
        .strip_suffix(".txt")?
        .parse()
        .ok()?;
    let line = |key: &str| {
        text.lines()
            .find_map(|l| l.strip_prefix(key))
            .map(|v| v.trim().to_owned())
    };
    Some(Crash {
        when: crate::bundle::utc_time(seconds),
        version: line("version: ")?,
        command: line("command: ")?,
        place: line("place: ")?,
    })
}

/// The crashes kept in `folder`, oldest first, and how many crash files in it could not be read.
pub fn read(folder: &Path) -> Health {
    let mut health = Health::default();
    let Ok(entries) = std::fs::read_dir(folder) else {
        return health;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !(name.starts_with("crash-") && name.ends_with(".txt")) {
            continue;
        }
        match std::fs::read_to_string(entry.path())
            .ok()
            .and_then(|text| parse(&name, &text))
        {
            Some(crash) => health.crashes.push(crash),
            None => health.unreadable_crash_files += 1,
        }
    }
    health.crashes.sort_by(|a, b| a.when.cmp(&b.when));
    health
}

#[cfg(test)]
#[path = "crash_notes_tests.rs"]
mod tests;
