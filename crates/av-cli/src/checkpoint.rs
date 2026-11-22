//! `av checkpoint` (ADR-159, decision 12; backlog 0716, part 3): the owner, at their own terminal, signs where each
//! record of an app has got to, with the key `sv review` signs with. `av` never makes that key and never keeps its
//! passphrase: `sv review` makes the key, and the passphrase is typed here each time and used once.
//!
//! It signs the records as they stand: the build-loop record, the hook record, and the pass-through's record. A
//! record whose chain does not check out is not signed, since that would be signing for a record already changed.
//! Signing reads where each record's chain ends, never what the record holds.

use av_record::checkpoint::{self, Checkpoint, Kind, NoHead};
use av_record::plain::quiet;
use std::io::{BufRead, Write};
use std::path::Path;
use sv_check::signed::{SigningKey, Stored};

/// How many times a passphrase is asked for before `av checkpoint` gives up.
const TRIES: usize = 3;

const WHAT_IT_SHOWS: &str = "A checkpoint shows, in a later report, whether the lines you signed for are still as they \
                             were. It says nothing of lines written after it, and nothing of whether what a record \
                             holds was true when it was written.";

fn now() -> String {
    av_record::plain::utc_time(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs()),
    )
}

/// Reads a passphrase from `input` without the terminal showing it, where the terminal can be told not to.
fn passphrase(input: &mut dyn BufRead, out: &mut dyn Write) -> std::io::Result<Option<String>> {
    let line = {
        let _shown = quiet::hide_typing();
        if !quiet::CAN_HIDE {
            writeln!(out, "{}", quiet::CANNOT_HIDE)?;
        }
        write!(out, "The passphrase of your signing key: ")?;
        out.flush()?;
        let mut line = String::new();
        if input.read_line(&mut line)? == 0 {
            None
        } else {
            Some(line.trim_end_matches(['\r', '\n']).to_owned())
        }
    };
    writeln!(out)?;
    Ok(line)
}

/// The signing key in `keys`, unlocked by the person when it has a passphrase. `Ok(None)` when there is none to
/// sign with, having said why.
fn signing_key(
    keys: &Path,
    input: &mut dyn BufRead,
    out: &mut dyn Write,
) -> Result<Option<SigningKey>, String> {
    let said = |out: &mut dyn Write, text: &str| writeln!(out, "{text}").map_err(|e| e.to_string());
    match SigningKey::load_from(keys)? {
        None => {
            said(
                out,
                "This computer has no signing key yet, so nothing was signed. `sv review` makes one the first time \
                 you run it, and asks whether to protect it with a passphrase; av never makes a key.",
            )?;
            Ok(None)
        }
        Some(Stored::Ready(key)) => {
            said(
                out,
                "Your signing key has no passphrase, so anything that runs as you could have made this checkpoint, \
                 your AI coding tool included. A later report says so beside it.",
            )?;
            Ok(Some(key))
        }
        Some(Stored::Locked(locked)) => {
            for left in (0..TRIES).rev() {
                let Some(typed) = passphrase(input, out).map_err(|e| e.to_string())? else {
                    break;
                };
                match locked.unlock(&typed) {
                    Ok(key) => return Ok(Some(key)),
                    Err(_) if left > 0 => {
                        said(out, "That is not the key's passphrase. Try again.")?
                    }
                    Err(_) => {}
                }
            }
            said(
                out,
                "The signing key was not unlocked, so nothing was signed.",
            )?;
            Ok(None)
        }
    }
}

/// Signs where each record of the app at `app` (its full, real path) stands, with the signing key in `keys`,
/// asking for its passphrase on `input`. `Ok(true)` when at least one record was signed.
pub fn run(
    app: &Path,
    keys: &Path,
    input: &mut dyn BufRead,
    out: &mut dyn Write,
) -> Result<bool, String> {
    let io = |e: std::io::Error| e.to_string();
    let mut to_sign = Vec::new();
    for kind in Kind::ALL {
        match checkpoint::head(app, kind) {
            Ok(head) => to_sign.push((kind, head)),
            Err(NoHead::Nothing) => {
                writeln!(
                    out,
                    "Nothing to sign for {}: there is no record, or it holds no line.",
                    kind.said()
                )
                .map_err(io)?;
            }
            Err(NoHead::Broken { at }) => {
                writeln!(
                    out,
                    "Not signed: the chain of {} does not check out at line {at}, so a line was changed, removed, \
                     or added after it was written. Signing it would be signing for that change. `av report` says \
                     more.",
                    kind.said()
                )
                .map_err(io)?;
            }
        }
    }
    if to_sign.is_empty() {
        writeln!(out, "No record of this app has anything to sign.").map_err(io)?;
        return Ok(false);
    }
    let Some(key) = signing_key(keys, input, out)? else {
        return Ok(false);
    };
    let app_id = sv_check::seal::App::of(app)?;
    // The key is named as trusted for this app on this computer, as `sv review` names it, so a report here can
    // check what it signs.
    sv_check::signed::trust_here(keys, &key, &app_id)?;
    let signer = key.for_app(&app_id);
    let at = now();
    for (kind, head) in &to_sign {
        let fields = checkpoint::fields(*kind, head);
        let fields: Vec<&str> = fields.iter().map(String::as_str).collect();
        let seal = signer.seal(&fields)?;
        checkpoint::write(
            app,
            &Checkpoint {
                at: at.clone(),
                kind: *kind,
                head: head.clone(),
                seal,
                key: key.fingerprint(),
            },
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            out,
            "Signed: {}, as it stands at {}.",
            kind.said(),
            match head.lines {
                1 => "1 line".to_owned(),
                n => format!("{n} lines"),
            }
        )
        .map_err(io)?;
    }
    writeln!(out, "Signed with key {}.", key.fingerprint()).map_err(io)?;
    writeln!(out, "{WHAT_IT_SHOWS}").map_err(io)?;
    Ok(true)
}

#[cfg(test)]
#[path = "checkpoint_tests.rs"]
mod tests;
