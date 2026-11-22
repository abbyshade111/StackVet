//! The note kept outside the app's folder (ADR-159, decision 12, as the owner chose it on 22 November 2026; backlog
//! 0716, part 3): each time `sv`'s MCP server writes a line of the build-loop record, it also writes down, outside the
//! app's folder, where that record's chain has got to. A tool confined to the app's folder can rewrite the record and
//! its chain; it cannot change the note. A report then looks at whether the record still passes through every place
//! noted.
//!
//! This took the place of giving the chain a key. A key would have changed how every line is written, needed a new
//! "could not be checked" state wherever the key was missing, and missed a record rewritten whole in the old way.
//! The note changes no line of the record, and a record rewritten whole does not pass through what was noted.
//!
//! What it does not do, said wherever it is shown: a tool that can write anywhere the owner can, can change the note
//! too. Only a checkpoint the owner signed (`checkpoint`) shows a change made by such a tool, and only to lines
//! signed for.
//!
//! The notes are one file per app beside the hook records, a line for each place noted: the time, how many chained
//! lines the record held, and its chain's last link. Nothing of what the record holds.

use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

/// Whether this process writes notes. Off unless the program turns it on: `sv mcp` does, since it is what writes the
/// record for an app someone is building; a test that writes a record does not, and so writes nothing into the
/// folder of whoever runs the tests.
pub static ON: AtomicBool = AtomicBool::new(false);

/// The most a file of notes is let grow to; past it, nothing more is noted, and a report says how far the notes go.
pub const MAX_BYTES: u64 = 4 * 1024 * 1024;

/// Where the notes of the app at `app` (its full, real path) are kept: beside the hook records, in `loop-notes`.
pub fn file_of(app: &Path) -> Option<PathBuf> {
    let key = crate::plain::sha256(app.display().to_string().as_bytes());
    crate::hook_record::folder()
        .and_then(|hooks| Some(hooks.parent()?.join("loop-notes")))
        .map(|f| f.join(format!("{}.jsonl", &key[..16])))
}

/// Notes where the build-loop record of the app at `app_dir` has got to, when this process writes notes. Never an
/// error: a note that cannot be written is one place less noted, which a report shows as the notes stopping short.
pub fn note(app_dir: &Path, at: &str) {
    if !ON.load(Ordering::Relaxed) {
        return;
    }
    write(app_dir, at);
}

/// `note`, whether or not this process writes notes: what `note` does, for the tests of it.
pub fn write(app_dir: &Path, at: &str) {
    let Ok(real) = sv_frameworks::paths::canonical(app_dir) else {
        return;
    };
    let Some(path) = file_of(&real) else {
        return;
    };
    if std::fs::metadata(&path).is_ok_and(|m| m.len() >= MAX_BYTES) {
        return;
    }
    let seen = crate::build_loop::read(&real);
    // A record whose chain does not check out is not noted: the note would vouch for the change.
    let (None, Some(chain)) = (seen.chain_broken_at, seen.chain_head) else {
        return;
    };
    if seen.chained == 0 {
        return;
    }
    let _ = crate::hook_record::append(
        &path,
        json!({ "at": at, "lines": seen.chained, "head": chain }),
    );
}

/// One place noted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Noted {
    pub at: String,
    pub lines: usize,
    pub chain: String,
}

/// The places noted for the app at `app`, oldest first. A line that is not a note is left out.
pub fn read(app: &Path) -> Vec<Noted> {
    let Some(path) = file_of(app) else {
        return Vec::new();
    };
    if !std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_file()) {
        return Vec::new();
    }
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    text.lines()
        .filter_map(|raw| serde_json::from_str::<Value>(raw).ok())
        .filter_map(|v| {
            Some(Noted {
                at: v.get("at")?.as_str()?.to_owned(),
                lines: usize::try_from(v.get("lines")?.as_u64()?).ok()?,
                chain: v.get("head")?.as_str()?.to_owned(),
            })
        })
        .collect()
}

/// How the build-loop record stands against the notes kept outside its folder.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "standing")]
pub enum Standing {
    /// Nothing was ever noted for this app on this computer.
    None,
    /// The record passes through every place noted: `noted` places, the last at `lines` lines. `after` lines of the
    /// record came after the last note.
    Kept {
        noted: usize,
        lines: usize,
        after: usize,
    },
    /// The record does not pass through a place noted at `at`, when it held `lines` lines: lines up to there were
    /// changed or cut since.
    NotKept { at: String, lines: usize },
}

/// The chain of each chained line of a record, in order, up to the first line whose chain does not check out.
fn links(bytes: &[u8]) -> Vec<String> {
    let whole = crate::build_loop::check_chain(bytes);
    let mut out = Vec::new();
    for (number, raw) in String::from_utf8_lossy(bytes).lines().enumerate() {
        if whole.broken_at.is_some_and(|broken| number + 1 >= broken) {
            break;
        }
        if let Some(chain) = serde_json::from_str::<Value>(raw)
            .ok()
            .and_then(|v| v.get("chain")?.as_str().map(str::to_owned))
        {
            out.push(chain);
        }
    }
    out
}

/// How the record of the app at `app` (its full, real path) stands against its notes.
pub fn standing(app: &Path) -> Standing {
    let notes = read(app);
    if notes.is_empty() {
        return Standing::None;
    }
    let record = crate::checkpoint::record_path(app, crate::checkpoint::Kind::BuildLoop)
        .and_then(|path| std::fs::read(path).ok())
        .unwrap_or_default();
    let links = links(&record);
    for noted in &notes {
        let passes = noted.lines >= 1 && links.get(noted.lines - 1) == Some(&noted.chain);
        if !passes {
            return Standing::NotKept {
                at: noted.at.clone(),
                lines: noted.lines,
            };
        }
    }
    let last = notes.last().map_or(0, |n| n.lines);
    Standing::Kept {
        noted: notes.len(),
        lines: last,
        after: links.len().saturating_sub(last),
    }
}

#[cfg(test)]
#[path = "outside_note_tests.rs"]
mod tests;
