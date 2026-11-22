//! The pass-through's record (ADR-159, decision 10; backlog 0716, part 1): one line for each call an agent made to a
//! tool through `av proxy`, and how it was answered. This module reads the lines that pass and makes the record's
//! lines; `av`'s `proxy` does the relaying and starts the server.
//!
//! What a line holds: the time by `av`'s clock; the tool's name; how the call was answered (`ok`, `tool-error`,
//! `refused` with the server's error number, or `unanswered`); a SHA-256 of the arguments and their size; and a
//! summary of the arguments, cut short and with any key cut as the secrets scan cuts one (the owner's choice, 21
//! November 2026: "redacted summary too"). Never an answer's text, and never the arguments whole.
//!
//! The record is kept where the hook record is, outside the app's folder, one file per app named by a hash of its
//! path, readable only by the person, and chained the same way: a tool clearing the app's folder does not take it.

use serde_json::{Value, json};
use std::path::{Path, PathBuf};

/// The most of a call's arguments a line keeps, in characters.
pub const SUMMARY_CHARS: usize = 200;

/// The most of a tool's name a line keeps.
pub const NAME_CHARS: usize = 80;

/// A line longer than this is relayed and not read: no message of a call's size should be, and reading one is not
/// worth holding it twice.
pub const MAX_LINE: usize = 4 * 1024 * 1024;

/// Where the records are kept: beside the hook records, in `proxy-records`.
pub fn folder() -> Option<PathBuf> {
    crate::hook_record::folder().and_then(|hooks| Some(hooks.parent()?.join("proxy-records")))
}

/// The record of the app at `app` (a full path): one file per app, named by a hash of its path.
pub fn record_of(app: &Path) -> Option<PathBuf> {
    let key = crate::plain::sha256(app.display().to_string().as_bytes());
    folder().map(|f| f.join(format!("{}.jsonl", &key[..16])))
}

/// One call to a tool, as it was asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub tool: String,
    pub args_sha256: String,
    pub args_bytes: usize,
    pub summary: String,
}

/// How a call was answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    /// The tool ran and gave a result.
    Ok,
    /// The tool ran and said it failed (`isError`).
    ToolError,
    /// The server answered with an error rather than a result: the call was not carried out. `code` is the
    /// server's own number for why.
    Refused { code: i64 },
}

impl Answer {
    pub fn name(self) -> &'static str {
        match self {
            Answer::Ok => "ok",
            Answer::ToolError => "tool-error",
            Answer::Refused { .. } => "refused",
        }
    }
}

/// A message's id, as a key: a call and its answer carry the same one.
fn id_of(message: &Value) -> Option<String> {
    match message.get("id")? {
        Value::Null => None,
        id @ (Value::String(_) | Value::Number(_)) => Some(id.to_string()),
        _ => None,
    }
}

fn parsed(line: &[u8]) -> Option<Value> {
    if line.len() > MAX_LINE {
        return None;
    }
    serde_json::from_slice(line).ok()
}

/// The call a line from the agent makes, with its id, when the line is a call to a tool (`tools/call`): `None` for
/// any other message, and for a line that is not one message.
pub fn call_in(line: &[u8], rules: &sv_check::secrets::SecretRules) -> Option<(String, Call)> {
    let message = parsed(line)?;
    if message.get("method")?.as_str()? != "tools/call" {
        return None;
    }
    let id = id_of(&message)?;
    let params = message.get("params")?;
    let tool = params.get("name")?.as_str()?;
    let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);
    let text = if arguments.is_null() {
        String::new()
    } else {
        arguments.to_string()
    };
    let (redacted, _) = sv_check::secrets::redact_text(rules, &text);
    let one_line: String = redacted
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    Some((
        id,
        Call {
            tool: sv_check::text::cut_with_count(&one_line_of(tool), NAME_CHARS),
            args_sha256: crate::plain::sha256(text.as_bytes()),
            args_bytes: text.len(),
            summary: sv_check::text::cut_with_count(one_line.trim(), SUMMARY_CHARS),
        },
    ))
}

fn one_line_of(text: &str) -> String {
    text.chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect()
}

/// The answer a line from the server gives, with the id of the call it answers: `None` for a line that answers
/// nothing.
pub fn answer_in(line: &[u8]) -> Option<(String, Answer)> {
    let message = parsed(line)?;
    if message.get("method").is_some() {
        // A request or a notice of the server's own, not an answer.
        return None;
    }
    let id = id_of(&message)?;
    if let Some(error) = message.get("error") {
        let code = error.get("code").and_then(Value::as_i64).unwrap_or(0);
        return Some((id, Answer::Refused { code }));
    }
    let result = message.get("result")?;
    let failed = result.get("isError").and_then(Value::as_bool) == Some(true);
    Some((
        id,
        if failed {
            Answer::ToolError
        } else {
            Answer::Ok
        },
    ))
}

/// The record's line for `call`, at `at`: answered as `answer`, or never answered before the pass-through ended.
pub fn line_of(at: &str, call: &Call, answer: Option<Answer>) -> Value {
    let mut line = json!({
        "at": at,
        "tool": call.tool,
        "answer": answer.map_or("unanswered", Answer::name),
        "args_sha256": call.args_sha256,
        "args_bytes": call.args_bytes,
        "summary": call.summary,
    });
    if let Some(Answer::Refused { code }) = answer {
        line["code"] = json!(code);
    }
    line
}

/// The line that says `av proxy` started, at `at` (backlog 0721, part 1): what follows it, up to the next such line,
/// is one run of the pass-through, which is one session of the agent with the tool server. Without it, a report could
/// not tell one session's calls from another's.
pub fn started_line(at: &str) -> Value {
    json!({ "at": at, "event": STARTED })
}

/// What a start line's `event` says.
pub const STARTED: &str = "started";

/// Appends `line` to the record at `path`, chained to the line before it, as the hook record's lines are.
pub fn append(path: &Path, line: Value) -> anyhow::Result<()> {
    crate::hook_record::append(path, line)
}

#[cfg(test)]
#[path = "passthrough_tests.rs"]
mod tests;
