//! What the pass-through's record holds, for a report (ADR-159, decision 10; backlog 0716, part 2): which tools the
//! agent called through `av proxy` and how often, which calls the server turned down, a call turned down and then
//! made again with the same arguments, and, when the task file says which tools may be called (`[may] call`), the
//! calls outside it.
//!
//! What this cannot show, said by the report wherever it is read (found in the owner's trial of 21 November 2026):
//!
//! - **A no the person gives at their AI coding tool's own permission prompt.** A call the person refuses there is
//!   never sent to the server, so the pass-through writes no line for it. Seeing those needs the tool's own hooks
//!   (backlog 0718).
//! - **Why the server said no.** A server that will not do a thing may answer with an error of the protocol
//!   (`refused`, with a number) or with an ordinary result marked as failed (`tool-error`); `sv`'s own server does
//!   the second. So the two are counted together as "turned down", and a `tool-error` may as well be a tool that
//!   ran and failed, such as a test that did not pass. Making such a call again is often right.
//!
//! The record is a file the tool could rewrite. Every word of it shown is the tool's text.

use serde_json::Value;
use std::io::Read as _;
use std::path::Path;
use sv_frameworks::paths::Canonical as _;

/// The most lines of one kind a report is handed; the rest are counted.
pub const MOST_LINES: usize = 20;

/// One tool, and how often it was called.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Tool {
    pub name: String,
    pub calls: usize,
    /// How many of those the server turned down (`tool-error` or `refused`).
    pub turned_down: usize,
}

/// A call the server turned down, made again with the same arguments.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Again {
    pub tool: String,
    /// The summary the record kept of what the tool was given.
    pub summary: String,
    /// When the server first turned it down.
    pub at: String,
    /// How it was turned down: `tool-error` or `refused`.
    pub answer: String,
    /// How many times it was made again after that.
    pub times: usize,
    /// How the last of those was answered.
    pub last_answer: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Seen {
    /// Why nothing was read, when nothing was.
    pub not_read: Option<String>,
    /// Lines that are a call.
    pub calls: usize,
    /// Lines whose chain checks out, from the first.
    pub chained: usize,
    /// The first line whose chain does not check out.
    pub broken_at: Option<usize>,
    pub first: Option<String>,
    pub last: Option<String>,
    pub ok: usize,
    pub tool_error: usize,
    pub refused: usize,
    pub unanswered: usize,
    /// By name, in order of name.
    pub tools: Vec<Tool>,
    /// Oldest first, at most `MOST_LINES`.
    pub again: Vec<Again>,
    pub again_more: usize,
    /// The tools called that `[may] call` does not name, with how often: `None` when the task file does not say
    /// which tools may be called, or there is none.
    pub outside: Option<Vec<Tool>>,
}

impl Seen {
    /// The calls the server turned down, however it said so.
    pub fn turned_down(&self) -> usize {
        self.tool_error + self.refused
    }
}

fn turned_down(answer: &str) -> bool {
    matches!(answer, "tool-error" | "refused")
}

/// What the record at `path` holds. `may_call` is the task file's `[may] call`, when it has one.
pub fn read(path: &Path, may_call: Option<&[String]>) -> Seen {
    let mut seen = Seen::default();
    if !std::fs::symlink_metadata(path).is_ok_and(|m| m.is_file()) {
        seen.not_read = Some(
            "there is no pass-through record for this app on this computer: av proxy has not been put in front of a \
             tool server for this folder, or no tool was called through it yet"
                .to_owned(),
        );
        return seen;
    }
    let mut bytes = Vec::new();
    if let Err(e) = std::fs::File::open(path).and_then(|f| {
        f.take(crate::hook_record::MAX_BYTES + 1)
            .read_to_end(&mut bytes)
    }) {
        seen.not_read = Some(format!("the pass-through's record could not be read ({e})"));
        return seen;
    }
    let mut previous = crate::hook_record::GENESIS.to_owned();
    let mut lines = 0;
    // Each call turned down, by tool and fingerprint, as the place in `again` it would be listed at.
    let mut noes: std::collections::HashMap<(String, String), Again> =
        std::collections::HashMap::new();
    let mut order: Vec<(String, String)> = Vec::new();
    let mut tools: std::collections::BTreeMap<String, Tool> = std::collections::BTreeMap::new();
    for raw in String::from_utf8_lossy(&bytes).lines() {
        if raw.trim().is_empty() {
            continue;
        }
        lines += 1;
        let Ok(mut value) = serde_json::from_str::<Value>(raw) else {
            seen.broken_at.get_or_insert(lines);
            continue;
        };
        if seen.broken_at.is_none() {
            let stored = value
                .get("chain")
                .and_then(Value::as_str)
                .map(str::to_owned);
            if let Some(object) = value.as_object_mut() {
                object.remove("chain");
            }
            match stored {
                Some(stored)
                    if crate::hook_record::chain_of(&previous, &value.to_string()) == stored =>
                {
                    seen.chained += 1;
                    previous = stored;
                }
                _ => seen.broken_at = Some(lines),
            }
        }
        let (Some(tool), Some(answer)) = (value["tool"].as_str(), value["answer"].as_str()) else {
            continue;
        };
        // The file may not be the pass-through's own any more: nothing in it is taken at more than the length the
        // pass-through writes.
        let tool = sv_check::text::cut_with_count(tool, crate::passthrough::NAME_CHARS);
        let at = value["at"].as_str().unwrap_or_default().to_owned();
        seen.calls += 1;
        if seen.first.is_none() {
            seen.first = Some(at.clone());
        }
        seen.last = Some(at.clone());
        match answer {
            "ok" => seen.ok += 1,
            "tool-error" => seen.tool_error += 1,
            "refused" => seen.refused += 1,
            _ => seen.unanswered += 1,
        }
        let entry = tools.entry(tool.clone()).or_insert_with(|| Tool {
            name: tool.clone(),
            calls: 0,
            turned_down: 0,
        });
        entry.calls += 1;
        if turned_down(answer) {
            entry.turned_down += 1;
        }
        // The same call is the same tool given the same thing: the fingerprint is of the arguments whole.
        let Some(print) = value["args_sha256"].as_str() else {
            continue;
        };
        let key = (tool.clone(), print.to_owned());
        match noes.get_mut(&key) {
            Some(no) => {
                no.times += 1;
                no.last_answer = answer.to_owned();
            }
            None if turned_down(answer) => {
                order.push(key.clone());
                noes.insert(
                    key,
                    Again {
                        tool,
                        summary: sv_check::text::cut_with_count(
                            value["summary"].as_str().unwrap_or_default(),
                            crate::passthrough::SUMMARY_CHARS,
                        ),
                        at,
                        answer: answer.to_owned(),
                        times: 0,
                        last_answer: answer.to_owned(),
                    },
                );
            }
            None => {}
        }
    }
    let mut again: Vec<Again> = order
        .into_iter()
        .filter_map(|key| noes.remove(&key))
        .filter(|no| no.times > 0)
        .collect();
    seen.again_more = again.len().saturating_sub(MOST_LINES);
    again.truncate(MOST_LINES);
    seen.again = again;
    seen.tools = tools.into_values().collect();
    seen.outside = may_call.map(|allowed| {
        seen.tools
            .iter()
            .filter(|t| !allowed.contains(&t.name))
            .cloned()
            .collect()
    });
    seen
}

/// What the record of the app at `app_dir` holds, for a report the person asked to read it for.
pub fn read_for(app_dir: &Path, may_call: Option<&[String]>) -> Seen {
    match app_dir
        .canonical()
        .ok()
        .and_then(|app| crate::passthrough::record_of(&app))
    {
        Some(path) => read(&path, may_call),
        None => Seen {
            not_read: Some(
                "this computer has no folder for av's records (neither HOME nor XDG_STATE_HOME is set)"
                    .to_owned(),
            ),
            ..Default::default()
        },
    }
}

#[cfg(test)]
#[path = "proxy_seen_tests.rs"]
mod tests;
