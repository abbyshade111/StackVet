//! What the hook record shows of the permissions a person was asked for (backlog 0718; ADR-159, "As built", part 12;
//! ADR-122, Later, 22 November 2026): each time the AI coding tool asked, whether the tool it asked about then ran,
//! and, for one that did not, whether the same call was asked for again or ran later in the same session.
//!
//! **What a hook is told, and so what this can say** (read in Claude Code's hooks reference, and tried on the
//! owner's computer on 22 November 2026). No hook fires when a person refuses. One fires when Claude Code is about
//! to ask (`PermissionRequest`), and one after a tool ran (`PostToolUse`, `PostToolUseFailure`); a refused call gets
//! the first and neither of the others. So a refusal is never seen: what is seen is **asked, and did not run**. A
//! prompt closed without an answer, and a session that ended with the prompt open, look the same, and the report
//! says so every time. The trial showed the refusal; it did not show the other two.
//!
//! **Which asking a run answers.** A run answers the open asking for the same tool with the same fingerprint of
//! what it was given, in the same session; with no such asking, the newest open one for that tool, since a tool
//! asks and is answered before the next thing happens. A tool's input can be changed between the asking and the
//! run, which the second rule is for.
//!
//! The record is a file the tool could rewrite, and a hook the tool could take out. Every word of it shown is the
//! tool's text.

use crate::hook_record::{ASKED, MAX_BYTES, RAN};
use serde_json::Value;
use std::io::Read as _;
use std::path::Path;
use sv_frameworks::paths::Canonical as _;

/// The most askings that did not run a report is handed; the rest are counted.
pub const MOST_LINES: usize = 20;

/// One asking that no run answered.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NotRun {
    pub at: String,
    pub tool: String,
    /// What the tool was to be given, as the record kept it: cut short, and with any key cut.
    pub summary: String,
    /// How many times the same call (the same tool, given the same thing) was asked for again later in the session.
    pub asked_again: usize,
    /// How many times the same call ran later in the session.
    pub ran_later: usize,
    /// The next thing the record holds for that session, in a few words: what the tool did after it was not let.
    pub next: Option<String>,
    /// Whether nothing follows it in its session: the prompt may still have been open when the record was read.
    pub last_of_session: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Seen {
    /// Why nothing was read, when nothing was.
    pub not_read: Option<String>,
    /// How many times the person was asked.
    pub asked: usize,
    /// How many of those were followed by the tool running.
    pub ran: usize,
    /// Those that were not, oldest first, at most `MOST_LINES`.
    pub not_run: Vec<NotRun>,
    pub not_run_more: usize,
}

struct Asking {
    line: usize,
    session: String,
    tool: String,
    print: String,
    at: String,
    summary: String,
    ran: bool,
}

/// What a line is, in the few words `NotRun::next` uses.
fn said(value: &Value) -> Option<String> {
    let tool = value["tool"].as_str().unwrap_or("a tool");
    match value["event"].as_str()? {
        ASKED => Some(format!(
            "asked for {tool}: {}",
            value["summary"].as_str().unwrap_or_default()
        )),
        RAN => Some(format!("{tool} ran")),
        "PostToolUse" | "PostToolUseFailure" => Some(match value["command"].as_str() {
            Some(command) => format!("ran: {command}"),
            None => format!("wrote {}", value["file"].as_str().unwrap_or("a file")),
        }),
        "Stop" => Some("the turn ended".to_owned()),
        _ => None,
    }
}

/// What the hook record's `bytes` show of the permissions asked for.
pub fn of(bytes: &[u8]) -> Seen {
    let lines: Vec<Value> = String::from_utf8_lossy(bytes)
        .lines()
        .filter_map(|raw| serde_json::from_str::<Value>(raw).ok())
        .collect();
    let text = |value: &Value, key: &str| value[key].as_str().unwrap_or_default().to_owned();
    let mut askings: Vec<Asking> = Vec::new();
    for (number, value) in lines.iter().enumerate() {
        let (session, tool, print) = (
            text(value, "session"),
            text(value, "tool"),
            text(value, "print"),
        );
        match value["event"].as_str() {
            Some(ASKED) => askings.push(Asking {
                line: number,
                session,
                tool,
                print,
                at: text(value, "at"),
                summary: text(value, "summary"),
                ran: false,
            }),
            Some(RAN | "PostToolUse" | "PostToolUseFailure") => {
                let open = |a: &&mut Asking| !a.ran && a.session == session && a.tool == tool;
                let same = askings
                    .iter_mut()
                    .rev()
                    .filter(open)
                    .find(|a| !print.is_empty() && a.print == print);
                match same {
                    Some(asking) => asking.ran = true,
                    None => {
                        if let Some(asking) = askings.iter_mut().rev().find(|a| open(a)) {
                            asking.ran = true;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    let mut seen = Seen {
        asked: askings.len(),
        ran: askings.iter().filter(|a| a.ran).count(),
        ..Default::default()
    };
    for asking in askings.iter().filter(|a| !a.ran) {
        let later: Vec<&Value> = lines[asking.line + 1..]
            .iter()
            .filter(|v| v["session"].as_str().unwrap_or_default() == asking.session)
            .collect();
        let same_call = |v: &&&Value| {
            v["tool"].as_str() == Some(asking.tool.as_str())
                && !asking.print.is_empty()
                && v["print"].as_str() == Some(asking.print.as_str())
        };
        seen.not_run.push(NotRun {
            at: asking.at.clone(),
            tool: asking.tool.clone(),
            summary: asking.summary.clone(),
            asked_again: later
                .iter()
                .filter(same_call)
                .filter(|v| v["event"] == ASKED)
                .count(),
            ran_later: later
                .iter()
                .filter(same_call)
                .filter(|v| v["event"] != ASKED)
                .count(),
            next: later.first().and_then(|v| said(v)),
            last_of_session: later.is_empty(),
        });
    }
    seen.not_run_more = seen.not_run.len().saturating_sub(MOST_LINES);
    seen.not_run.truncate(MOST_LINES);
    seen
}

/// What the hook record of the app at `app_dir` shows of the permissions asked for. Read only for a report the
/// person asked to read the hook record for: `hook_record::read_for`'s consent holds here too.
pub fn read_for(app_dir: &Path) -> Seen {
    let not_read = |why: &str| Seen {
        not_read: Some(why.to_owned()),
        ..Default::default()
    };
    let Some(path) = app_dir
        .canonical()
        .ok()
        .and_then(|app| crate::hook_record::record_of(&app))
    else {
        return not_read(
            "this computer has no folder for sv's records (neither HOME nor XDG_STATE_HOME is set)",
        );
    };
    if !std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_file()) {
        return not_read("there is no hook record for this app on this computer");
    }
    let mut bytes = Vec::new();
    match std::fs::File::open(&path).and_then(|f| f.take(MAX_BYTES + 1).read_to_end(&mut bytes)) {
        Ok(_) => of(&bytes),
        Err(_) => not_read("the hook record could not be read"),
    }
}

#[cfg(test)]
#[path = "asked_tests.rs"]
mod tests;
