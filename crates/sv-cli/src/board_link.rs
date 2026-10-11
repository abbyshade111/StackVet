//! Where the backlog overview's file is, for the dashboard to link (backlog 0227, ADR-086). The owner says
//! where the board is by writing its path, as the first line of the file `backlog-board` in the `stackvet`
//! config folder. `sv` only reads that file; it never writes to the config folder itself.

use std::path::PathBuf;

/// The path the owner's `backlog-board` names: its first line, trimmed. An empty first line names nothing.
pub fn named_in(text: &str) -> Option<PathBuf> {
    let line = text.lines().next()?.trim();
    (!line.is_empty()).then(|| PathBuf::from(line))
}

/// The board the dashboard links, when the owner's `backlog-board` names a file that exists.
pub fn current() -> Option<PathBuf> {
    let config = config_root()?
        .join(sv_frameworks::names::CONFIG_DIR)
        .join("backlog-board");
    let path = named_in(&std::fs::read_to_string(config).ok()?)?;
    path.is_file().then_some(path)
}

/// `$XDG_CONFIG_HOME`, or `~/.config`, as the seal's key folder uses.
fn config_root() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
}

#[cfg(test)]
#[path = "board_link_tests.rs"]
mod tests;
