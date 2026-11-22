//! Checkpoints (ADR-159, decision 12; backlog 0716, part 3): the owner, at their own terminal, signs where each
//! record's chain has got to, with the key `sv review` signs with. A later report can then say which lines of a
//! record stand as the owner signed for them, and which rest on the chain alone.
//!
//! Why it is the owner's key and not the chain's: a tool that runs as the owner can rewrite a record and its chain.
//! It cannot make the owner's signature, when that key has a passphrase. So a checkpoint is what shows a rewrite of
//! lines the owner already signed for; it shows nothing about lines written since, and the report says so.
//!
//! A checkpoint is one line in a file of its own, outside the app's folder: the time, which record, how many lines
//! its chain held, the chain's last link, the signature, and the signing key's fingerprint. The signature is over
//! the record's kind, the line count, and the link, for this app (the seal names the app, as `sv review`'s do).

use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use sv_check::seal::{Checker, Sealed};
use sv_report::hook::{Earlier, Tip};

/// The records a checkpoint is made of, by the names a checkpoint's line and signature use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    BuildLoop,
    Hook,
    Proxy,
}

impl Kind {
    pub const ALL: [Kind; 3] = [Kind::BuildLoop, Kind::Hook, Kind::Proxy];

    pub fn name(self) -> &'static str {
        match self {
            Kind::BuildLoop => "build-loop",
            Kind::Hook => "hook",
            Kind::Proxy => "proxy",
        }
    }

    /// The record, as a report names it.
    pub fn said(self) -> &'static str {
        match self {
            Kind::BuildLoop => "the build-loop record",
            Kind::Hook => "the hook record",
            Kind::Proxy => "the pass-through's record",
        }
    }

    fn named(name: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.name() == name)
    }
}

/// Where a record's chain has got to: how many chained lines, and the last link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Head {
    pub lines: usize,
    pub chain: String,
}

/// Why a record has no head to sign.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoHead {
    /// There is no record, or it holds no chained line.
    Nothing,
    /// Its chain stops checking out at this line: signing would be signing for a record already changed.
    Broken { at: usize },
}

/// Where the `kind` record of the app at `app` is kept, when this computer has a place for it.
pub fn record_path(app: &Path, kind: Kind) -> Option<PathBuf> {
    match kind {
        Kind::BuildLoop => Some(sv_scan::ecosystems::build_loop_record_in(app)),
        Kind::Hook => crate::hook_record::record_of(app),
        Kind::Proxy => crate::passthrough::record_of(app),
    }
}

/// The head of the `kind` record of the app at `app` (its full, real path), as it stands now.
pub fn head(app: &Path, kind: Kind) -> Result<Head, NoHead> {
    match kind {
        Kind::BuildLoop => {
            let seen = crate::build_loop::read(app);
            if let Some(at) = seen.chain_broken_at {
                return Err(NoHead::Broken { at });
            }
            match (seen.chained, seen.chain_head) {
                (lines, Some(chain)) if lines > 0 => Ok(Head { lines, chain }),
                _ => Err(NoHead::Nothing),
            }
        }
        Kind::Hook | Kind::Proxy => {
            let path = record_path(app, kind).ok_or(NoHead::Nothing)?;
            let seen = crate::hook_record::read(&path, None);
            if let Some(at) = seen.broken_at {
                return Err(NoHead::Broken { at });
            }
            match seen.tip {
                Some(Tip { lines, chain }) if lines > 0 => Ok(Head { lines, chain }),
                _ => Err(NoHead::Nothing),
            }
        }
    }
}

/// Whether the `kind` record still passes through `at`: its first `at.lines` chained lines are there, in order, and
/// end at `at.chain`. `None` when there is no record to read now.
pub fn still_passes(app: &Path, kind: Kind, at: &Head) -> Option<bool> {
    let path = record_path(app, kind)?;
    if !std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_file()) {
        return None;
    }
    match kind {
        Kind::BuildLoop => {
            let bytes = std::fs::read(&path).ok()?;
            let whole = crate::build_loop::check_chain(&bytes);
            // The line the checkpoint ended at: the `at.lines`-th line that carries a chain.
            let mut chained = 0;
            for (number, raw) in String::from_utf8_lossy(&bytes).lines().enumerate() {
                let Some(chain) = serde_json::from_str::<Value>(raw)
                    .ok()
                    .and_then(|v| v.get("chain")?.as_str().map(str::to_owned))
                else {
                    continue;
                };
                chained += 1;
                if chained == at.lines {
                    let before_any_break = whole.broken_at.is_none_or(|broken| broken > number + 1);
                    return Some(chain == at.chain && before_any_break);
                }
            }
            // Fewer chained lines than were signed for: lines were cut.
            Some(false)
        }
        Kind::Hook | Kind::Proxy => {
            let tip = Tip {
                lines: at.lines,
                chain: at.chain.clone(),
            };
            match crate::hook_record::read(&path, Some(&tip)).earlier {
                Some(Earlier::Kept { .. }) => Some(true),
                Some(Earlier::NotKept { .. }) => Some(false),
                None => None,
            }
        }
    }
}

/// What a checkpoint's signature is over, with the app, which the seal itself names.
pub fn fields(kind: Kind, at: &Head) -> [String; 4] {
    [
        "av checkpoint".to_owned(),
        kind.name().to_owned(),
        at.lines.to_string(),
        at.chain.clone(),
    ]
}

/// Where the checkpoints of the app at `app` are kept: beside the hook records, in `checkpoints`.
pub fn file_of(app: &Path) -> Option<PathBuf> {
    let key = crate::plain::sha256(app.display().to_string().as_bytes());
    crate::hook_record::folder()
        .and_then(|hooks| Some(hooks.parent()?.join("checkpoints")))
        .map(|f| f.join(format!("{}.jsonl", &key[..16])))
}

/// One checkpoint, as its line holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub at: String,
    pub kind: Kind,
    pub head: Head,
    pub seal: String,
    /// The fingerprint of the key that signed it, as the signer said it; the report takes the key from the
    /// signature, never from this.
    pub key: String,
}

/// Writes one checkpoint for the app at `app`.
pub fn write(app: &Path, checkpoint: &Checkpoint) -> anyhow::Result<()> {
    let path = file_of(app).ok_or_else(|| {
        anyhow::anyhow!(
            "this computer has no folder for av's records (neither HOME nor XDG_STATE_HOME is set)"
        )
    })?;
    crate::hook_record::append(
        &path,
        json!({
            "at": checkpoint.at,
            "record": checkpoint.kind.name(),
            "lines": checkpoint.head.lines,
            "head": checkpoint.head.chain,
            "seal": checkpoint.seal,
            "key": checkpoint.key,
        }),
    )
}

/// The checkpoints of the app at `app`, oldest first. A line that is not a checkpoint is left out.
pub fn read(app: &Path) -> Vec<Checkpoint> {
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
            Some(Checkpoint {
                at: v.get("at")?.as_str()?.to_owned(),
                kind: Kind::named(v.get("record")?.as_str()?)?,
                head: Head {
                    lines: usize::try_from(v.get("lines")?.as_u64()?).ok()?,
                    chain: v.get("head")?.as_str()?.to_owned(),
                },
                seal: v.get("seal")?.as_str()?.to_owned(),
                key: v.get("key")?.as_str().unwrap_or_default().to_owned(),
            })
        })
        .collect()
}

/// How a record stands against its checkpoints.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "standing")]
pub enum Standing {
    /// No checkpoint was ever made for it: every line rests on the chain alone.
    None,
    /// Its newest checkpoint whose signature holds, and the record still passes through it.
    Kept {
        at: String,
        lines: usize,
        /// The key that signed it, and what this computer knows of it, in the report's words.
        signed: String,
    },
    /// Its newest checkpoint whose signature holds, and the record no longer passes through it: lines the owner
    /// signed for were changed or cut.
    NotKept { at: String, lines: usize },
    /// Checkpoints are there and none could be checked: why, for the newest.
    Unchecked { at: String, why: String },
}

/// How the `kind` record of the app at `app` stands, checking each checkpoint's signature with `checker`.
pub fn standing(app: &Path, kind: Kind, checker: &Checker) -> Standing {
    let mut unchecked = None;
    for checkpoint in read(app).into_iter().rev().filter(|c| c.kind == kind) {
        let fields = fields(kind, &checkpoint.head);
        let fields: Vec<&str> = fields.iter().map(String::as_str).collect();
        match checker.recorded(Some(&checkpoint.seal), &fields) {
            Ok(Sealed::Signed { key, from, lock }) => {
                return match still_passes(app, kind, &checkpoint.head) {
                    Some(true) => Standing::Kept {
                        at: checkpoint.at,
                        lines: checkpoint.head.lines,
                        signed: format!(
                            "signed with key {key}, which {} trusts for this app, {}",
                            from.named(),
                            lock.said()
                        ),
                    },
                    // No record at all now is the record cut to nothing.
                    Some(false) | None => Standing::NotKept {
                        at: checkpoint.at,
                        lines: checkpoint.head.lines,
                    },
                };
            }
            // A checkpoint is always signed; a seal of the older kind is not one of these.
            Ok(Sealed::Here) => {
                unchecked.get_or_insert((checkpoint.at, "its seal is not a signature".to_owned()));
            }
            Err(why) => {
                unchecked.get_or_insert((checkpoint.at, why.why()));
            }
        }
    }
    match unchecked {
        Some((at, why)) => Standing::Unchecked { at, why },
        None => Standing::None,
    }
}

#[cfg(test)]
#[path = "checkpoint_tests.rs"]
mod tests;
