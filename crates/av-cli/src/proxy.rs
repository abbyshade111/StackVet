//! `av proxy` (ADR-159, decision 10; backlog 0716, part 1): the pass-through. It starts the tool server it is given,
//! relays every line the agent sends it and every line it answers, unchanged, and writes down each call to a tool
//! and how it was answered (`av_record::passthrough`).
//!
//! Three things it holds to:
//!
//! - **It changes nothing it relays.** Every byte goes on as it came, whether or not `av` could read it: a line that
//!   is not a message, or is too long to read, is relayed and not recorded.
//! - **It never stops the agent's work.** A record that cannot be written is said on standard error, counted, and
//!   the relaying goes on.
//! - **It opens no connection.** It starts one program, the server named on its command line, and talks to it over
//!   that program's standard input and output. This is the one place `av` starts a program.

use av_record::passthrough::{self, Answer, Call};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

/// The calls asked and not yet answered, and where to write them down.
struct Record {
    path: Option<PathBuf>,
    waiting: HashMap<String, Call>,
    /// Lines that could not be written, said once and counted.
    unwritten: usize,
}

impl Record {
    fn write(&mut self, call: &Call, answer: Option<Answer>) {
        let Some(path) = &self.path else {
            self.unwritten += 1;
            return;
        };
        let line = passthrough::line_of(&now(), call, answer);
        if let Err(why) = passthrough::append(path, line) {
            if self.unwritten == 0 {
                eprintln!(
                    "av proxy: a call could not be written down ({}); relaying goes on, and the record is short \
                     from here.",
                    sv_report::one_line(&why.to_string())
                );
            }
            self.unwritten += 1;
        }
    }
}

fn now() -> String {
    av_record::plain::utc_time(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs()),
    )
}

/// Reads `from` a line at a time and writes each to `to` as it came, handing each to `seen` first. Ends when `from`
/// ends or `to` is closed.
fn relay(from: impl std::io::Read, mut to: impl Write, mut seen: impl FnMut(&[u8])) {
    let mut from = BufReader::new(from);
    let mut line = Vec::new();
    loop {
        line.clear();
        match from.read_until(b'\n', &mut line) {
            Ok(0) | Err(_) => return,
            Ok(_) => {}
        }
        seen(line.strip_suffix(b"\n").unwrap_or(&line));
        if to.write_all(&line).and_then(|()| to.flush()).is_err() {
            return;
        }
    }
}

/// Runs `server` with its arguments, relays between this program's standard input and output and the server's, and
/// writes the record for the app at `app`. Gives the server's exit code, or why it could not be started.
pub fn run(app: &Path, server: &[String]) -> Result<u8, String> {
    let (program, arguments) = server
        .split_first()
        .ok_or("av proxy needs the tool server's command after `--`")?;
    let rules = sv_check::secrets::SecretRules::shared()
        .map_err(|why| format!("the rules for cutting keys could not be loaded: {why}"))?;
    let path = passthrough::record_of(app);
    if path.is_none() {
        eprintln!(
            "av proxy: this computer has no folder for av's records (neither HOME nor XDG_STATE_HOME is set), so \
             nothing is written down; relaying goes on."
        );
    }
    let mut child = Command::new(program)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|why| format!("{program} could not be started: {why}"))?;
    let (Some(to_server), Some(from_server)) = (child.stdin.take(), child.stdout.take()) else {
        return Err(format!("{program} was started without a way to talk to it"));
    };
    let record = Arc::new(Mutex::new(Record {
        path,
        waiting: HashMap::new(),
        unwritten: 0,
    }));

    // The agent's lines, to the server: a call to a tool is kept until its answer comes.
    let asked = Arc::clone(&record);
    let agent = std::thread::spawn(move || {
        relay(std::io::stdin().lock(), to_server, |line| {
            if let Some((id, call)) = passthrough::call_in(line, rules)
                && let Ok(mut record) = asked.lock()
            {
                record.waiting.insert(id, call);
            }
        });
        // The agent closed its side: dropping `to_server` here tells the server so.
    });

    // The server's lines, to the agent: an answer is written down with the call it answers.
    let answered = Arc::clone(&record);
    relay(from_server, std::io::stdout().lock(), |line| {
        if let Some((id, answer)) = passthrough::answer_in(line)
            && let Ok(mut record) = answered.lock()
            && let Some(call) = record.waiting.remove(&id)
        {
            record.write(&call, Some(answer));
        }
    });

    let status = child
        .wait()
        .map_err(|why| format!("waiting for {program}: {why}"))?;
    // The server has ended, so nothing more will be answered; the agent's side may still be open, and is left.
    drop(agent);
    if let Ok(mut record) = record.lock() {
        let unanswered: Vec<Call> = record.waiting.drain().map(|(_, call)| call).collect();
        for call in &unanswered {
            record.write(call, None);
        }
        if record.unwritten > 0 {
            eprintln!(
                "av proxy: {} not written down.",
                match record.unwritten {
                    1 => "1 call was".to_owned(),
                    n => format!("{n} calls were"),
                }
            );
        }
    }
    Ok(status
        .code()
        .and_then(|c| u8::try_from(c).ok())
        .unwrap_or(1))
}
