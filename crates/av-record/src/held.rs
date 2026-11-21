//! What a tool did, held to the task envelope (ADR-159, decision 8; backlog 0715, part 2): what falls outside what
//! the envelope allows, and what touched a file that guards oversight. Read from git's history and, when the person
//! asked for it, the hook record. It lists; it decides nothing.
//!
//! Three things hold everywhere here:
//!
//! - **What the envelope does not say is not held either way.** `[may] write` left out lists no write as outside,
//!   and says that it could not.
//! - **What no record shows is said, never passed over:** the hosts a tool reached, what a script it started went on
//!   to do, an action it should have asked about first.
//! - **A path that leaves the folder is outside it, however it is written.** `src/../../home/x` begins with `src/`
//!   and is not in it.

use crate::envelope::Envelope;
use crate::hook_record::Done;
use sv_check::git::changes::Commit;

/// The most lines listed; the rest are counted.
pub const MOST_LINES: usize = 60;

/// Which of the design's groups a line belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    /// A file that guards oversight was changed.
    Oversight,
    /// A file written, or a program started, that the envelope does not allow.
    OutsideEnvelope,
}

impl Group {
    pub fn label(self) -> &'static str {
        match self {
            Group::Oversight => "a file that guards oversight was changed",
            Group::OutsideEnvelope => "outside what the task file allows",
        }
    }
}

/// Which record a line was read from, so the report can say how far to rely on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Witness {
    /// A commit whose message names a tool.
    Git,
    /// The hook record.
    Hook,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Seen {
    pub group: Group,
    /// What was seen, in `av`'s words around the file's or program's own name.
    pub what: String,
    /// Where: the commit, or the time the hook wrote the line.
    pub place: String,
    pub witness: Witness,
}

/// What holding the records to an envelope came to.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Held {
    pub seen: Vec<Seen>,
    /// Lines past `MOST_LINES`, counted and not listed.
    pub more: usize,
    /// What could not be held to the envelope, and why, each a sentence.
    pub not_held: Vec<String>,
}

/// A path as the envelope's entries are written: forward slashes, no leading `./`. `None` for a path that is not
/// below the folder: one from a root or a drive, or one that climbs out with `..`.
fn below(path: &str) -> Option<String> {
    let path = path.replace('\\', "/");
    let path = path.strip_prefix("./").unwrap_or(&path);
    if path.is_empty() || path.starts_with('/') || path.contains(':') {
        return None;
    }
    if path.split('/').any(|part| part == "..") {
        return None;
    }
    Some(path.to_owned())
}

/// The programs a command line starts, as far as its text shows: the first word of each part between `&&`, `||`,
/// `;`, `|`, and `&`, past any `NAME=value` set before it, each by its name alone. A word that only passes the
/// next one on (`sudo`, `env`, `time`, `nohup`, `xargs`) is listed, and so is the word after it. Nothing a quoted
/// script, a subshell, or a started program goes on to do is seen, and the caller says so.
pub fn programs(command: &str) -> Vec<String> {
    const PASSES_ON: [&str; 6] = ["sudo", "env", "time", "nohup", "xargs", "exec"];
    let mut out: Vec<String> = Vec::new();
    for part in command.split(['&', '|', ';', '\n']) {
        let mut words = part
            .split_whitespace()
            .skip_while(|w| is_assignment(w))
            .peekable();
        while let Some(word) = words.next() {
            let name = word.rsplit(['/', '\\']).next().unwrap_or(word);
            let name = name.trim_matches(['"', '\'', '(', ')', '{', '}', '`', '$']);
            if name.is_empty() {
                break;
            }
            if !out.iter().any(|seen| seen == name) {
                out.push(name.to_owned());
            }
            if !PASSES_ON.contains(&name) {
                break;
            }
            // Past the options and settings of the word that passes on, to the program it starts.
            while words
                .peek()
                .is_some_and(|w| w.starts_with('-') || is_assignment(w))
            {
                words.next();
            }
        }
    }
    out
}

/// `NAME=value`, set before a command.
fn is_assignment(word: &str) -> bool {
    word.split_once('=').is_some_and(|(name, _)| {
        !name.is_empty()
            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            && !name.starts_with(|c: char| c.is_ascii_digit())
    })
}

fn commit_place(c: &Commit) -> String {
    let subject: String = c.subject.chars().take(60).collect();
    format!(
        "commit {} \"{subject}\", by {}",
        c.hash.chars().take(8).collect::<String>(),
        c.tool.as_deref().unwrap_or("a tool not named")
    )
}

fn changed(status: char) -> &'static str {
    match status {
        'A' => "added",
        'D' => "deleted",
        _ => "changed",
    }
}

/// What `commits` (`None` when git's history could not be set against the envelope, with `no_commits` saying why)
/// and `done` (the hook record's lines, `None` when it was not read) show against `envelope`.
pub fn against(
    envelope: &Envelope,
    commits: Result<&[Commit], &str>,
    done: Option<&[Done]>,
) -> Held {
    let mut held = Held::default();
    let mut seen: Vec<Seen> = Vec::new();
    // One file, judged the same whichever record names it.
    let mut file = |path: &str, how: &str, place: String, witness: Witness| {
        let Some(inside) = below(path) else {
            seen.push(Seen {
                group: Group::OutsideEnvelope,
                what: format!("{path} was {how}, which is not below this folder"),
                place,
                witness,
            });
            return;
        };
        if envelope.guards_file(&inside) {
            seen.push(Seen {
                group: Group::Oversight,
                what: format!("{inside} was {how}"),
                place: place.clone(),
                witness,
            });
        }
        if envelope.may_write(&inside) == Some(false) {
            seen.push(Seen {
                group: Group::OutsideEnvelope,
                what: format!("{inside} was {how}, and [may] write does not cover it"),
                place,
                witness,
            });
        }
    };
    match commits {
        Ok(commits) => {
            for c in commits.iter().rev().filter(|c| c.tool.is_some()) {
                for (status, path) in &c.files {
                    file(path, changed(*status), commit_place(c), Witness::Git);
                }
            }
        }
        Err(why) => held
            .not_held
            .push(format!("What the tool's commits changed: {why}.")),
    }
    for d in done.unwrap_or_default() {
        if let Done::Wrote { at, file: path } = d {
            file(
                path,
                "written",
                format!("the hook record, {at}"),
                Witness::Hook,
            );
        }
    }
    match (&envelope.may.run, done) {
        (Some(allowed), Some(done)) => {
            for d in done {
                let Done::Ran { at, command } = d else {
                    continue;
                };
                for program in programs(command) {
                    if !allowed.contains(&program) {
                        seen.push(Seen {
                            group: Group::OutsideEnvelope,
                            what: format!("{program} was started, and [may] run does not name it"),
                            place: format!("the hook record, {at}"),
                            witness: Witness::Hook,
                        });
                    }
                }
            }
            held.not_held.push(
                "What a program the tool started went on to do: a command is read as far as its text shows, so a \
                 script, a quoted command, or a program that starts another is not seen. The hook record also cuts a \
                 long command short."
                    .to_owned(),
            );
        }
        (Some(_), None) => held.not_held.push(
            "The programs the tool started: only the hook record shows them, and it is read only when you ask, \
             with --hook-record."
                .to_owned(),
        ),
        (None, _) => held.not_held.push(
            "The programs the tool started: the task file does not say which it may start ([may] run is left out)."
                .to_owned(),
        ),
    }
    if envelope.may.write.is_none() {
        held.not_held.push(
            "The files the tool wrote, beyond the guarded ones: the task file does not say what it may write \
             ([may] write is left out)."
                .to_owned(),
        );
    }
    if done.is_none() {
        held.not_held.push(
            "The files the tool wrote and did not commit: only the hook record shows them, and it is read only when \
             you ask, with --hook-record."
                .to_owned(),
        );
    }
    held.not_held.push(
        "The hosts the tool reached: no record read here shows them, whatever [may] reach says."
            .to_owned(),
    );
    held.not_held.push(
        "Whether the tool asked first: no record read here shows what it asked, whatever ask-first says."
            .to_owned(),
    );
    seen.sort_by_key(|s| s.group);
    held.more = seen.len().saturating_sub(MOST_LINES);
    seen.truncate(MOST_LINES);
    held.seen = seen;
    held
}

#[cfg(test)]
#[path = "held_tests.rs"]
mod tests;
