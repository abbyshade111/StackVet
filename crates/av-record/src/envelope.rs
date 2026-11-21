//! The task envelope (ADR-159, decision 8; backlog 0715, part 1): a file the owner writes, `agentvet.toml`, saying
//! what an AI coding tool's task is and what it may touch. Drift is what falls outside it.
//!
//! ```toml
//! envelope-version = 1
//!
//! [task]
//! goal = "Add a sign-out link to the header."
//!
//! [may]
//! write = ["src/", "tests/"]        # a folder ends in a slash; anything else is one file
//! run = ["pytest", "npm"]           # the programs it may start, by name
//! reach = ["pypi.org"]              # the hosts it may connect to
//!
//! [guards]
//! files = ["deploy/", "Makefile"]   # what it may not change, added to the list av always guards
//!
//! ask-first = ["delete", "push"]    # of: delete, push, install, send, spend
//! ```
//!
//! Three rules, each `sv`'s for `stackvet.toml` too:
//!
//! - **A line left out is a question, never a pass.** `[may] write` left out does not mean the tool may write
//!   anywhere, or nowhere: it means nobody said, and the report asks (`Envelope::questions`).
//! - **A file that cannot be read holds nothing.** A key this does not know, a path that leaves the folder, or a
//!   version from the future is said, with its line where TOML gives one, and nothing is judged against the file.
//! - **The file does not say who wrote it.** Until the owner's word is recorded, the report says it may be the
//!   tool's own (`MAY_BE_THE_TOOLS`).
//!
//! Nothing here judges what a tool did: that is the detectors' work (backlog 0715, parts 2 and 3). This reads the
//! file, and answers what an entry covers.

use serde::Deserialize;
use std::path::Path;

/// The envelope's name, beside the app's own files.
pub const FILE: &str = "agentvet.toml";

/// The one version of the format there is.
pub const VERSION: u32 = 1;

/// The most of the file that is read: an envelope is a page, and a larger file is not one.
pub const MAX_BYTES: u64 = 256 * 1024;

/// What the report says of an envelope it read, every time: the file is not evidence of whose word it is.
pub const MAY_BE_THE_TOOLS: &str = "The file does not say who wrote it. Your AI coding tool can write agentvet.toml as \
                                    well as you can, so until your word for it is recorded, it may be the tool's own \
                                    account of its task.";

/// The actions an envelope can say must ask first.
pub const ASK_FIRST: [&str; 5] = ["delete", "push", "install", "send", "spend"];

/// The files that always guard oversight, whatever the envelope names: the tool's own instructions and permissions,
/// the hooks and servers it is started with, what runs in CI and who must review, and the envelope itself. A folder
/// ends in a slash. An envelope's own list adds to these and takes none away (the owner's choice, 21 November 2026):
/// an envelope the tool wrote cannot name a short list of its own to unguard them.
pub const DEFAULT_GUARDS: [&str; 15] = [
    FILE,
    "AGENTS.md",
    "CLAUDE.md",
    "GEMINI.md",
    ".claude/",
    ".cursor/",
    ".cursorrules",
    ".windsurf/",
    ".gemini/",
    ".roo/",
    ".continue/",
    ".mcp.json",
    ".github/workflows/",
    ".github/CODEOWNERS",
    ".gitlab-ci.yml",
];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Envelope {
    pub envelope_version: u32,
    pub task: Task,
    #[serde(default)]
    pub may: May,
    #[serde(default)]
    pub guards: Guards,
    /// The actions that must ask first, of `ASK_FIRST`. Left out, nobody said: a question.
    pub ask_first: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Task {
    /// The task, in a sentence.
    pub goal: String,
}

/// What the tool may touch. Each left out is a question; each given as an empty list is "nothing".
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct May {
    /// Folders (ending in a slash) and files it may write, from the envelope's folder.
    pub write: Option<Vec<String>>,
    /// Programs it may start, each by its name alone.
    pub run: Option<Vec<String>>,
    /// Hosts it may connect to.
    pub reach: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Guards {
    /// Folders and files the tool may not change, besides `DEFAULT_GUARDS`, which are always guarded.
    pub files: Option<Vec<String>>,
}

/// What reading a folder's envelope came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Read {
    /// No `agentvet.toml`: nothing is held to a task.
    NotThere,
    /// One is there and cannot be read as an envelope, and why, in `av`'s words with the file's own text on one
    /// line. Nothing is judged against it.
    Unreadable(String),
    Held(Box<Envelope>),
}

/// The envelope of the app at `folder`. Never read through a link, and never past `MAX_BYTES`.
pub fn read(folder: &Path) -> Read {
    let path = folder.join(FILE);
    let meta = match std::fs::symlink_metadata(&path) {
        Ok(meta) => meta,
        Err(_) => return Read::NotThere,
    };
    if meta.file_type().is_symlink() {
        return Read::Unreadable(format!(
            "{FILE} is a link to somewhere else, and av does not read through one"
        ));
    }
    if !meta.is_file() {
        return Read::Unreadable(format!("{FILE} is not an ordinary file"));
    }
    if meta.len() > MAX_BYTES {
        return Read::Unreadable(format!(
            "{FILE} is larger than {} KB, more than av reads as an envelope",
            MAX_BYTES / 1024
        ));
    }
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(why) => return Read::Unreadable(format!("{FILE} could not be read: {why}")),
    };
    match parse(&text) {
        Ok(envelope) => Read::Held(Box::new(envelope)),
        Err(why) => Read::Unreadable(why),
    }
}

/// An envelope from its text, or why it is not one.
pub fn parse(text: &str) -> Result<Envelope, String> {
    let envelope: Envelope = toml::from_str(text).map_err(|why| {
        format!(
            "{FILE} is not an envelope av can read: {}",
            one_line(&why.to_string())
        )
    })?;
    if envelope.envelope_version != VERSION {
        return Err(format!(
            "{FILE} says envelope-version = {}, and this av reads version {VERSION} only",
            envelope.envelope_version
        ));
    }
    if envelope.task.goal.trim().is_empty() {
        return Err(format!(
            "{FILE} has an empty [task] goal: say the task in a sentence"
        ));
    }
    for (key, entries) in [
        ("[may] write", envelope.may.write.as_deref()),
        ("[guards] files", envelope.guards.files.as_deref()),
    ] {
        for entry in entries.unwrap_or_default() {
            if let Some(why) = not_a_place(entry) {
                return Err(format!(
                    "{FILE}: {key} holds \"{}\", {why}",
                    one_line(entry)
                ));
            }
        }
    }
    for (key, entries) in [
        ("[may] run", envelope.may.run.as_deref()),
        ("[may] reach", envelope.may.reach.as_deref()),
    ] {
        for entry in entries.unwrap_or_default() {
            if entry.is_empty() || entry.chars().any(|c| c.is_whitespace() || c.is_control()) {
                return Err(format!(
                    "{FILE}: {key} holds \"{}\", which is not one name: write each as a single word, with no spaces",
                    one_line(entry)
                ));
            }
        }
    }
    for entry in envelope.ask_first.as_deref().unwrap_or_default() {
        if !ASK_FIRST.contains(&entry.as_str()) {
            return Err(format!(
                "{FILE}: ask-first holds \"{}\", which is not one of {}",
                one_line(entry),
                ASK_FIRST.join(", ")
            ));
        }
    }
    Ok(envelope)
}

/// Why `entry` is not a folder or file below the envelope's own folder, when it is not.
fn not_a_place(entry: &str) -> Option<&'static str> {
    if entry.is_empty() {
        return Some("which names nothing");
    }
    if entry.chars().any(char::is_control) {
        return Some("which holds a control character");
    }
    if entry.starts_with('/') || entry.starts_with('\\') || entry.contains(':') {
        return Some(
            "which is not a place below this folder: write it from the folder agentvet.toml is in",
        );
    }
    if entry.contains('\\') {
        return Some("which uses a backslash: write folders with a forward slash");
    }
    if entry.split('/').any(|part| part == "..") {
        return Some("which leaves this folder (`..`)");
    }
    None
}

/// `text` on one line, as a report shows the file's own words.
fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Whether `entries` cover `path`, a file named from the envelope's folder with forward slashes. An entry ending in
/// a slash is a folder and covers everything under it; `./` is the whole folder; any other entry is one file. Names
/// are compared as written: nothing is matched by pattern, and case is not folded.
pub fn covers(entries: &[String], path: &str) -> bool {
    let path = path.strip_prefix("./").unwrap_or(path);
    entries.iter().any(|entry| {
        let entry = entry.strip_prefix("./").unwrap_or(entry);
        if entry.is_empty() || entry == "." {
            // `./` and `.`: the whole folder.
            true
        } else if entry.ends_with('/') {
            path.starts_with(entry)
        } else {
            path == entry
        }
    })
}

impl Envelope {
    /// The files that guard oversight: `DEFAULT_GUARDS`, then the envelope's own list, each once.
    pub fn guards(&self) -> Vec<String> {
        let mut all: Vec<String> = DEFAULT_GUARDS.iter().map(|g| (*g).to_owned()).collect();
        for own in self.guards.files.as_deref().unwrap_or_default() {
            if !all.contains(own) {
                all.push(own.clone());
            }
        }
        all
    }

    /// Whether `path` is one the tool may not change.
    pub fn guards_file(&self, path: &str) -> bool {
        covers(&self.guards(), path)
    }

    /// Whether the tool may write `path`: `None` when the envelope does not say what it may write.
    pub fn may_write(&self, path: &str) -> Option<bool> {
        self.may.write.as_ref().map(|entries| covers(entries, path))
    }

    /// What the envelope leaves unsaid, each as the question the report asks. Never read as a yes or a no.
    pub fn questions(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.may.write.is_none() {
            out.push(
                "Which folders and files may the tool write? ([may] write is left out.)".to_owned(),
            );
        }
        if self.may.run.is_none() {
            out.push("Which programs may the tool start? ([may] run is left out.)".to_owned());
        }
        if self.may.reach.is_none() {
            out.push("Which hosts may the tool connect to? ([may] reach is left out.)".to_owned());
        }
        if self.ask_first.is_none() {
            out.push(format!(
                "Which actions must the tool ask you about first, of {}? (ask-first is left out.)",
                ASK_FIRST.join(", ")
            ));
        }
        out
    }
}

#[cfg(test)]
#[path = "envelope_tests.rs"]
mod tests;
