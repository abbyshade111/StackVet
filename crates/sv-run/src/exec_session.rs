//! One `docker exec` into the sidecar, kept open for a run's requests (backlog 0341; ADR-019, Later, 14 October
//! 2026). Each request used to start a `docker exec` of its own, estimated at 100 to 300 milliseconds before the app is
//! asked anything, and a signed-in run sends a few hundred. Now one shell in the sidecar reads the requests one
//! after another from what it is given and answers each in turn, and every request still goes in from inside the
//! fence, through the same `nc` exchange as before.
//!
//! Each request is handed over as two lines, `<host> <port>` and the request in base64, so no byte of it can end a
//! line early; the shell reads a line with `read`, which takes no more than the line, and writes the request to a
//! file in the sidecar's memory as `exchange_script` does. After each answer it prints a line holding a marker made
//! fresh for the session, which the app cannot know, and the exchange's exit code. Before any real request, one
//! `-` exchange sends bytes of every value and must get them back unchanged, so a sidecar whose shell or tools read
//! the framing differently is found before it can garble a request, and the run goes on one exec per request.

use std::io::{Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Longest a request is waited for: the exchange itself stops at 15 seconds (`timeout 15` in the loop), so this is
/// reached only when the session itself has stalled.
const ANSWER_LIMIT: Duration = Duration::from_secs(60);

/// Longest the check exchange is waited for: a few hundred bytes handed straight back, after the exec has started.
const CHECK_LIMIT: Duration = Duration::from_secs(10);

/// How much of the session's own error output is kept, for saying why it ended.
const ERRORS_KEPT: usize = 64 * 1024;

/// The shell the session runs in the sidecar, answering requests until what it reads ends. The code after each answer
/// is `rm`'s, as the exit code of `exchange_script`, whose last command it is, always was: a request through the
/// session reads exactly as one through an exec of its own.
pub(crate) fn loop_script(mark: &str) -> String {
    format!(
        "while IFS=' ' read -r host port; do \
           IFS= read -r body || break; \
           f=$(mktemp) || break; \
           printf '%s' \"$body\" | base64 -d > \"$f\"; \
           if [ \"$host\" = - ]; then cat \"$f\"; \
           else timeout 15 nc -w 5 \"$host\" \"$port\" -e sh -c \"cat $f; cat 1>&2\" 2>&1 < /dev/null; fi; \
           rm -f \"$f\"; code=$?; \
           printf '\\n%s%s\\n' '{mark}' \"$code\"; \
         done"
    )
}

/// Why a request through the session has no answer.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Failed {
    /// Nothing reached the sidecar, so the request may be sent another way.
    NotSent,
    /// It was sent, and no whole answer came back: what came, the session's own error output, and its exit code.
    Lost { said: String, code: i32 },
}

/// The open session.
pub(crate) struct Session {
    child: Child,
    stdin: ChildStdin,
    out: Receiver<Vec<u8>>,
    errors: Arc<Mutex<Vec<u8>>>,
    mark: String,
    /// What came after the last answer's end and is not yet read: nothing, while one request is in flight at a time.
    left: Vec<u8>,
}

impl Session {
    /// Starts `command`, which must run `loop_script(mark)`, and checks the framing with one `-` exchange. `None`
    /// when it would not start or the check failed; the session is then stopped.
    pub(crate) fn start(mut command: Command, mark: &str) -> Option<Session> {
        #[cfg(unix)]
        std::os::unix::process::CommandExt::process_group(&mut command, 0);
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()?;
        let stdin = child.stdin.take()?;
        let mut stdout = child.stdout.take()?;
        let mut stderr = child.stderr.take()?;
        let (send, out) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut buf = vec![0u8; 64 * 1024];
            while let Ok(n) = stdout.read(&mut buf) {
                if n == 0 || send.send(buf[..n].to_vec()).is_err() {
                    break;
                }
            }
        });
        let errors = Arc::new(Mutex::new(Vec::new()));
        let kept = Arc::clone(&errors);
        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            while let Ok(n) = stderr.read(&mut buf) {
                if n == 0 {
                    break;
                }
                if let Ok(mut kept) = kept.lock()
                    && kept.len() < ERRORS_KEPT
                {
                    kept.extend_from_slice(&buf[..n]);
                }
            }
        });
        let mut session = Session {
            child,
            stdin,
            out,
            errors,
            mark: mark.to_owned(),
            left: Vec::new(),
        };
        let every_byte: Vec<u8> = (0..=255u8)
            .chain(b"\n\r\n'\"$`\\ end".iter().copied())
            .collect();
        match session.exchange("-", 0, &every_byte, CHECK_LIMIT) {
            Ok((0, back)) if back == every_byte => Some(session),
            _ => None,
        }
    }

    /// Sends `request` to `host:port` from the sidecar, and the exchange's exit code and everything it printed.
    pub(crate) fn send(
        &mut self,
        host: &str,
        port: u16,
        request: &[u8],
    ) -> Result<(i32, String), Failed> {
        self.exchange(host, port, request, ANSWER_LIMIT)
            .map(|(code, out)| (code, String::from_utf8_lossy(&out).into_owned()))
    }

    fn exchange(
        &mut self,
        host: &str,
        port: impl std::fmt::Display,
        request: &[u8],
        limit: Duration,
    ) -> Result<(i32, Vec<u8>), Failed> {
        let line =
            sv_check::encoding::base64(request, sv_check::encoding::Alphabet::Standard, true);
        let framed = format!("{host} {port}\n{line}\n");
        if self.stdin.write_all(framed.as_bytes()).is_err() || self.stdin.flush().is_err() {
            return Err(Failed::NotSent);
        }
        let started = Instant::now();
        let mut reading = Reading::new(&self.mark, std::mem::take(&mut self.left));
        loop {
            if let Some((code, answer, left)) = reading.answer() {
                self.left = left;
                return Ok((code, answer));
            }
            if reading.got.len() as u64 > crate::OUTPUT_LIMIT || crate::interrupted() {
                return Err(self.lost(reading.got));
            }
            match self.out.recv_timeout(
                Duration::from_millis(200).min(limit.saturating_sub(started.elapsed())),
            ) {
                Ok(chunk) => reading.got.extend_from_slice(&chunk),
                Err(RecvTimeoutError::Timeout) if started.elapsed() < limit => {}
                Err(_) => return Err(self.lost(reading.got)),
            }
        }
    }

    /// The session stopped, and what it said: what came of the answer, then its own error output.
    fn lost(&mut self, got: Vec<u8>) -> Failed {
        let waited = Instant::now();
        let code = loop {
            match self.child.try_wait() {
                Ok(Some(status)) => break status.code().unwrap_or(-1),
                Ok(None) if waited.elapsed() < Duration::from_secs(2) => {
                    std::thread::sleep(Duration::from_millis(20));
                }
                _ => {
                    self.stop();
                    break -1;
                }
            }
        };
        let mut said = String::from_utf8_lossy(&got).into_owned();
        if let Ok(errors) = self.errors.lock() {
            said.push_str(&String::from_utf8_lossy(&errors));
        }
        Failed::Lost { said, code }
    }

    fn stop(&mut self) {
        #[cfg(unix)]
        if let Ok(pid) = libc::pid_t::try_from(self.child.id()) {
            // SAFETY: a plain system call; the group is the one `start` made for this child.
            unsafe { libc::kill(-pid, libc::SIGKILL) };
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stop();
    }
}

/// One answer as it comes in, in pieces of any size, until the line that ends it.
struct Reading {
    end: Vec<u8>,
    got: Vec<u8>,
    /// How much of `got` has been searched for `end`, so each piece is searched once, with room for an end that
    /// arrived split across two pieces.
    searched: usize,
    found: Option<usize>,
}

impl Reading {
    fn new(mark: &str, got: Vec<u8>) -> Reading {
        Reading {
            end: format!("\n{mark}").into_bytes(),
            got,
            searched: 0,
            found: None,
        }
    }

    /// The exit code, the answer, and whatever came after its end line, once the end line has come whole.
    fn answer(&mut self) -> Option<(i32, Vec<u8>, Vec<u8>)> {
        let from = self.searched.saturating_sub(self.end.len() - 1);
        self.searched = self.got.len();
        self.found = self
            .found
            .or_else(|| find(&self.got[from..], &self.end).map(|at| from + at));
        let at = self.found?;
        let after = at + self.end.len();
        let newline = self.got[after..].iter().position(|b| *b == b'\n')?;
        let code = std::str::from_utf8(&self.got[after..after + newline])
            .ok()
            .and_then(|c| c.trim().parse::<i32>().ok())
            .unwrap_or(-1);
        let left = self.got.split_off(after + newline + 1);
        let mut answer = std::mem::take(&mut self.got);
        answer.truncate(at);
        Some((code, answer, left))
    }
}

/// Where `needle` first starts in `haystack`.
fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
#[path = "exec_session_tests.rs"]
mod tests;
