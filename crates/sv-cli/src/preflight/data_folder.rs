//! Whether the app keeps its data under /tmp by default (backlog 0659, ADR-141). `sv`'s starter once told the AI
//! coding tool to "keep its data somewhere like /tmp", since the app's folder is read-only while `sv` runs it, and with
//! no way to set a variable for `sv`'s copy alone, apps made /tmp their own default: in 8 of 12 apps surveyed on 23
//! October 2026 (backlog 0648). Read from the text, like the rest of the preflight; it decides nothing.

use super::{Answer, Item, Manifest, Source, app, sv};

/// The first line of the app's own code, test code aside, that writes a path under /tmp, as `(file, line, text)`.
fn first_tmp_path(source: &Source) -> Option<(String, usize, String)> {
    source
        .files
        .iter()
        .filter(|(file, _)| !sv_scan::files::is_test_path(file))
        .find_map(|(file, text)| {
            text.lines().enumerate().find_map(|(i, line)| {
                ["\"/tmp/", "'/tmp/", "\"/tmp\"", "'/tmp'"]
                    .iter()
                    .any(|p| line.contains(p))
                    .then(|| (file.clone(), i + 1, line.trim().to_owned()))
            })
        })
}

/// The preflight's word on where the app keeps its data, or `None` when its code names no path under /tmp.
pub(super) fn item(manifest: &Manifest, source: &Source) -> Option<Item> {
    let (file, line, text) = first_tmp_path(source)?;
    let shown: String = text.chars().take(120).collect();
    let mut says = vec![
        sv(format!("`{file}` line {line} writes under /tmp: ")),
        app(format!("`{shown}`")),
        sv(
            ". If that is where the real app keeps its data, give it a default of its own instead: /tmp is shared with every user of the machine and emptied on restart. ",
        ),
    ];
    says.push(sv(if manifest.stack.run.env.is_empty() {
        "Point the copy `sv` runs at /tmp under `[stack.run]` with `env = { DATABASE_PATH = \"/tmp/app.db\" }` (with the variable your app reads), since its folder is read-only there."
    } else {
        "`[stack.run] env` already sets variables for the copy `sv` runs; have the app read where its data lives from one of them."
    }));
    Some(Item::new("data-folder", Answer::Look, says))
}

#[cfg(test)]
#[path = "data_folder_tests.rs"]
mod tests;
