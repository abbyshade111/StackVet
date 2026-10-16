//! Probing the running app: the scripts the sidecar runs to send a request and read the whole answer, one at a time
//! or several at once, what is kept of each answer, and Docker's own words when the sidecar is gone. Moved out of
//! `docker.rs` unchanged (backlog 0324), but for `request_bytes`'s doc comment, which sat above `exchange_script`.

use super::*;

/// The shell that sends one request and reads the whole answer, run in the sidecar.
///
/// Until 26 September 2026 this was `echo … | nc`, and it lost every answer a Node server was
/// still working on. When `echo` finishes, BusyBox's `nc` half-closes the connection, and Node's
/// HTTP server drops a connection whose client has stopped sending before the reply is written —
/// so a route that waited on anything (a database, a fetch, OpenID Connect discovery) came back
/// as "no answer", while a route that replied at once worked. Found by the first real sign-in
/// through the test provider: the app answered `/` and never `/login/google`. `nc -e` hands the
/// connected socket to a small script that writes the request and then reads until the server
/// closes, so the sending side stays open. `timeout` stands in for `nc -w`, which no longer
/// applies once the script has the socket, so a server that never closes cannot stall the run.
pub(super) fn exchange_script(host: &str, port: u16) -> String {
    // The request arrives as what the container reads, is written to a file in its memory, and is
    // read from there by the script `nc -e` runs. It once went in as an argument, as base64, and
    // every request over about 96 KB failed: Linux refuses a single argument over 128 KiB, and the
    // failure read as the app not answering. `nc -e` closes every file but the socket before it
    // starts the script, so the file is how the request gets there.
    format!(
        "f=$(mktemp) && cat > \"$f\" && timeout 15 nc -w 5 {host} {port} -e sh -c \"cat $f; cat 1>&2\" 2>&1; rm -f \"$f\""
    )
}

/// What starts each copy's answer in the output of `at_once_script`, before a part made fresh for each
/// call (`at_once_mark`). Printed on a line of its own before each, so an answer that never came still
/// has its place.
pub(super) const AT_ONCE_MARK: &str = "@@sv-at-once-";

/// The marker for one call of `at_once_script`: `AT_ONCE_MARK` and a random part. With the marker
/// fixed, an app could print `@@sv-at-once-2@@` and an answer of its own inside its first answer, and
/// that would be read as the second copy's: a race check fooled into a pass (the deep review's
/// improvement 5). The app cannot know a marker made after it started.
pub(super) fn at_once_mark() -> String {
    format!("{AT_ONCE_MARK}{}-", crate::random_hex(8))
}

/// What `at_once_script` reads for `requests`: each different request once, one after another;
/// their sizes; and, for each copy in the order given, which of them it is, counting from 1. Two
/// users' copies of one action are two requests, and sending each once keeps the input small.
pub(super) fn together_input(
    requests: &[sv_check::probes::ProbeRequest],
    host: &str,
) -> Option<(Vec<u8>, Vec<usize>, Vec<usize>)> {
    let mut distinct: Vec<Vec<u8>> = Vec::new();
    let mut which = Vec::with_capacity(requests.len());
    for request in requests {
        let raw = request_bytes(request, host)?;
        let k = match distinct.iter().position(|d| *d == raw) {
            Some(k) => k,
            None => {
                distinct.push(raw);
                distinct.len() - 1
            }
        };
        which.push(k + 1);
    }
    let sizes = distinct.iter().map(Vec::len).collect();
    Some((distinct.concat(), sizes, which))
}

/// The two parts `at_once_script` and `in_turn_script` share: the commands that cut the input in
/// `$d/all` into a file for each request (`$d/r1`, `$d/r2`, …), each ending in `&&`, and the
/// "copy:request" pairs that say which file each copy sends.
pub(super) fn cut_and_pairs(sizes: &[usize], which: &[usize]) -> (String, String) {
    let mut offset = 0;
    let mut cut = String::new();
    for (k, size) in sizes.iter().enumerate() {
        cut.push_str(&format!(
            "tail -c +{} \"$d/all\" | head -c {size} > \"$d/r{}\" && ",
            offset + 1,
            k + 1
        ));
        offset += size;
    }
    let pairs: Vec<String> = which
        .iter()
        .enumerate()
        .map(|(i, k)| format!("{}:{k}", i + 1))
        .collect();
    let pairs = pairs.join(" ");
    (cut, pairs)
}

/// `exchange_script`, for requests sent together. The input, the requests of `sizes` one after
/// another, is cut into a file each; then every connection is started in the background before any
/// is waited for, so they reach the app together rather than one after another, copy `i` sending
/// request `which[i]`. Each answer is kept in its own file and printed in order after all have
/// finished. The cutting reads a file, not the pipe, so no request takes bytes of the next.
pub(super) fn at_once_script(
    host: &str,
    port: u16,
    sizes: &[usize],
    which: &[usize],
    mark: &str,
) -> String {
    let (cut, pairs) = cut_and_pairs(sizes, which);
    let numbers: Vec<String> = (1..=which.len()).map(|i| i.to_string()).collect();
    let numbers = numbers.join(" ");
    format!(
        "d=$(mktemp -d) && cat > \"$d/all\" && {cut}\
         for p in {pairs}; do i=${{p%:*}}; k=${{p#*:}}; timeout 15 nc -w 5 {host} {port} -e sh -c \"cat $d/r$k; cat 1>&2\" > \"$d/$i\" 2>&1 & done; \
         wait; for i in {numbers}; do printf '\\n{mark}%s@@\\n' \"$i\"; cat \"$d/$i\"; done; rm -rf \"$d\""
    )
}

/// `exchange_script`, for requests sent one after another in one call: the input cut as
/// `at_once_script` cuts it, and then each request sent and its answer read before the next starts,
/// the marker printed before each answer as `at_once_script` prints it, so `parse_at_once` reads the
/// output. Each answer goes straight out rather than to a file, so a large one cannot fill the
/// container's small memory folder and cut the next short.
pub(super) fn in_turn_script(
    host: &str,
    port: u16,
    sizes: &[usize],
    which: &[usize],
    mark: &str,
) -> String {
    let (cut, pairs) = cut_and_pairs(sizes, which);
    format!(
        "d=$(mktemp -d) && cat > \"$d/all\" && {cut}\
         for p in {pairs}; do i=${{p%:*}}; k=${{p#*:}}; printf '\\n{mark}%s@@\\n' \"$i\"; \
         timeout 15 nc -w 5 {host} {port} -e sh -c \"cat $d/r$k; cat 1>&2\" 2>&1; done; rm -rf \"$d\""
    )
}

#[cfg(test)]
/// The answers in the output of `at_once_script`, in order, each under its request's id: `None`
/// for a copy that got none.
pub(super) fn parse_at_once(
    ids: &[&str],
    out: &str,
    mark: &str,
) -> Vec<Option<sv_check::probes::ProbeResponse>> {
    at_once_raws(ids.len(), out, mark)
        .into_iter()
        .zip(ids)
        .map(|(raw, id)| raw.and_then(|raw| parse_response(id, raw)))
        .collect()
}

/// Each copy's raw answer in the output of `at_once_script`, in order: `None` for a copy that got none.
pub(super) fn at_once_raws<'a>(copies: usize, out: &'a str, mark: &str) -> Vec<Option<&'a str>> {
    (1..=copies)
        .map(|i| {
            let start = format!("\n{mark}{i}@@\n");
            let next = format!("\n{mark}{}@@\n", i + 1);
            let from = out.find(&start)? + start.len();
            let to = out[from..].find(&next).map_or(out.len(), |n| from + n);
            let raw = &out[from..to];
            (!raw.trim().is_empty()).then_some(raw)
        })
        .collect()
}

/// Writes out the request to send, or refuses to send one at all.
///
/// The path comes from the app's own manifest (`health_path`), so it is not something `sv` wrote. A
/// newline anywhere in a request line or a header lets that text add headers, or a second request,
/// of its own. There is no safe repair for that — a stripped path is a different request from the
/// one asked for — so the whole request is refused, and a probe with no answer is already reported
/// as unanswered rather than as a pass.
pub(super) fn request_bytes(
    request: &sv_check::probes::ProbeRequest,
    host: &str,
) -> Option<Vec<u8>> {
    let unsafe_text = |s: &str| s.contains(['\r', '\n', ' ', '\t']);
    if unsafe_text(&request.method) || unsafe_text(&request.path) || unsafe_text(host) {
        return None;
    }
    if request
        .headers
        .iter()
        .any(|(name, value)| name.contains([':', '\r', '\n']) || value.contains(['\r', '\n']))
    {
        return None;
    }
    // A request may name its own `Host`, to ask what the app does with a name that is not its
    // own; it then replaces this one rather than being sent beside it, since two would be refused
    // for being two.
    let own_host = request
        .headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("host"));
    let mut raw = if own_host {
        format!(
            "{} {} HTTP/1.0\r\nConnection: close\r\n",
            request.method, request.path
        )
    } else {
        format!(
            "{} {} HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\n",
            request.method, request.path
        )
    };
    for (name, value) in &request.headers {
        raw.push_str(&format!("{name}: {value}\r\n"));
    }
    // A body is framed by its length, so nothing in it can be read as a second request: the server
    // stops at the byte count, whatever the body contains.
    let mut raw = raw.into_bytes();
    if let Some(body) = &request.body {
        raw.extend_from_slice(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
        raw.extend_from_slice(body);
    } else {
        raw.extend_from_slice(b"\r\n");
    }
    Some(raw)
}

impl DockerBackend {
    /// Fetches a whole body from a service inside the fence other than the app: the mail server.
    ///
    /// Not through `probe`, which keeps only the start of a body — enough to judge an error page, and
    /// not enough to hold a list of messages or an HTML email.
    pub(super) fn fetch(&self, via: &Via, host: &str, port: u16, path: &str) -> Option<String> {
        let request = sv_check::probes::ProbeRequest {
            id: "mail".to_owned(),
            method: "GET".to_owned(),
            path: path.to_owned(),
            headers: Vec::new(),
            body: None,
        };
        let raw = request_bytes(&request, host)?;
        let script = exchange_script(host, port);
        let (_, out) = self
            .inside_fence_with_input(via, &["sh", "-c", &script], &raw)
            .ok()?;
        let (head, body) = out.split_once("\r\n\r\n")?;
        head.split_whitespace()
            .nth(1)
            .is_some_and(|status| status == "200")
            .then(|| body.to_owned())
    }
}

/// Whether the backend's answer to an `exec` says the container is not there to run it in, in any case: Docker says
/// "No such container", Podman "no such container" (backlog 0519).
pub(super) fn sidecar_gone(out: &str) -> bool {
    let out = out.to_lowercase();
    out.contains("no such container") || out.contains("is not running")
}

/// The first line of the app's output that speaks of a `Content-Length`, cut to a line's worth: what a server says
/// when an answer it sent came short (uvicorn's "Response content longer than Content-Length"), for the run to put
/// beside the answers it read as none (backlog 0598).
pub(super) fn content_length_line(log: &str) -> Option<String> {
    log.lines()
        .find(|l| {
            let l = l.to_lowercase();
            l.contains("content-length") || l.contains("content length")
        })
        .map(|l| l.trim().chars().take(300).collect())
}

/// An answer that came with less body than its `Content-Length` said: (bytes that came, bytes it said). A server
/// that writes its headers and then fails sends this, and uvicorn does when a handler's body outgrows the length it
/// set (backlog 0598): the page is blank in a browser, so it is no answer to read. Never for an answer that has no
/// body by HTTP's rules: one to `HEAD`, or 1xx, 204, or 304.
pub(super) fn short_body(raw: &str, head: bool) -> Option<(usize, usize)> {
    let (head_part, body) = raw
        .split_once("\r\n\r\n")
        .or_else(|| raw.split_once("\n\n"))?;
    let (status, headers) = sv_check::probes::read_head(head_part);
    let status = status?;
    if head || status < 200 || status == 204 || status == 304 {
        return None;
    }
    let said: usize = sv_check::probes::header_in(&headers, "content-length")?
        .trim()
        .parse()
        .ok()?;
    (body.len() < said).then_some((body.len(), said))
}

/// Turns a raw HTTP response into the shape the probes read; `None` when it has no status line.
pub(super) fn parse_response(id: &str, raw: &str) -> Option<sv_check::probes::ProbeResponse> {
    // A header block ends at the first blank line; tolerate a server that uses bare newlines.
    let (head, body) = raw
        .split_once("\r\n\r\n")
        .or_else(|| raw.split_once("\n\n"))
        .unwrap_or((raw, ""));
    let (status, headers) = sv_check::probes::read_head(head);
    let status = status?;
    Some(sv_check::probes::ProbeResponse {
        id: id.to_owned(),
        status,
        headers,
        body: kept_answer(status, body),
    })
}

/// How many characters of an error answer's visible words are kept past the cut (backlog 0527).
pub(super) const ERROR_WORDS: usize = 1000;

/// What is kept of an answer: [`kept_body`], and, for an error answer (400 and up) longer than the cut, the words a
/// person would read on it, after the cut and up to [`ERROR_WORDS`]. In the three-tool trial of 14 October 2026 a
/// sign-in answered 400, and the page's inline styles ran past the cut, so the one sentence that said why (a refused
/// anti-forgery token) was in neither the run's steps nor seen.json.
pub(super) fn kept_answer(status: u16, body: &str) -> String {
    let mut kept = kept_body(body);
    if status >= 400 && body.chars().count() > KEPT_CHARS {
        let words = sv_check::text::visible_words(body);
        if !words.is_empty() {
            kept.push_str("\n[…]\n");
            kept.extend(words.chars().take(ERROR_WORDS));
        }
    }
    kept
}

/// How much of a body is kept (`sv_check::probes::KEPT_CHARS`).
pub(super) const KEPT_CHARS: usize = sv_check::probes::KEPT_CHARS;
/// How much is kept on each side of the reflection probes' value, when it comes back further down.
pub(super) const AROUND_ECHO: usize = 200;
/// How many times it is kept further down.
pub(super) const MOST_ECHOES: usize = 5;

/// The start of a body, and, when the reflection probes' value comes back past it, the text around
/// each time it does. A page often repeats a search term well down, past its head and navigation,
/// and the value is one only `sv` sends, so nothing else's answer keeps more than it did.
///
/// The same for the signs of a stack trace (`sv_check::probes::TRACE_MARKERS`): the whole answer is
/// searched, and the text around the first of each that comes past the cut is kept. Until 5 October
/// 2026 only the start was read, so an error page whose trace began below a long page of markup was
/// credited as saying nothing it should not (H17 of the deep review).
pub(super) fn kept_body(body: &str) -> String {
    let mut kept: String = body.chars().take(KEPT_CHARS).collect();
    let from = kept.len();
    let mark = sv_check::probes::REFLECTION_MARK;
    // Searched from a little before the cut, so a value that starts inside the kept part and runs
    // past it is kept whole.
    let search = body.ceil_char_boundary(from.saturating_sub(mark.len() + AROUND_ECHO));
    let mut after = from;
    for (at, _) in body[search..].match_indices(mark).take(MOST_ECHOES) {
        let at = search + at;
        if at + mark.len() + AROUND_ECHO <= from {
            continue;
        }
        let start = body.ceil_char_boundary(if at < after {
            at
        } else {
            at.saturating_sub(AROUND_ECHO).max(after)
        });
        let end = body.floor_char_boundary(at + mark.len() + AROUND_ECHO);
        kept.push_str("\n[…]\n");
        kept.push_str(&body[start..end]);
        after = end;
    }
    for marker in sv_check::probes::TRACE_MARKERS {
        let Some(at) = body.find(marker) else {
            continue;
        };
        if at + marker.len() <= from {
            continue;
        }
        let start = body.ceil_char_boundary(at.saturating_sub(AROUND_ECHO));
        let end = body.floor_char_boundary(at + marker.len() + AROUND_ECHO);
        kept.push_str("\n[…]\n");
        kept.push_str(&body[start..end]);
    }
    kept
}

impl DockerBackend {
    /// Makes one request to the app from inside the fence.
    ///
    /// HTTP is spoken directly over a socket rather than through a client, for two reasons found by
    /// trying the alternative: `wget` returns no body at all for a 404 or a 500, which is exactly the
    /// response the error-page probe needs to read, and it cannot send a method other than GET or POST.
    pub(super) fn probe(
        &self,
        via: &Via,
        app: &str,
        port: u16,
        request: &sv_check::probes::ProbeRequest,
    ) -> Option<sv_check::probes::ProbeResponse> {
        let raw = request_bytes(request, app)?;
        let started = std::time::Instant::now();
        let sent = match self.in_session(via, app, port, &raw) {
            Some(Ok(answered)) => Ok(answered),
            Some(Err(said)) => Err(said),
            None => {
                let script = exchange_script(app, port);
                self.inside_fence_with_input(via, &["sh", "-c", &script], &raw)
                    .map_err(Lost::from)
            }
        };
        self.note_request_time(request.id.clone(), started);
        let (code, out) = match sent {
            Ok(answered) => answered,
            Err(Lost::Said(code, said)) => {
                self.note_if_sidecar_lost(&request.id, code, &said);
                return None;
            }
            Err(Lost::Unsaid) => return None,
        };
        if self.note_if_sidecar_lost(&request.id, code, &out) {
            return None;
        }
        if code != 0 && out.trim().is_empty() {
            return None;
        }
        self.whole_answer(
            &request.id,
            &out,
            request.method.eq_ignore_ascii_case("HEAD"),
        )
    }

    /// `parse_response`, with an answer shorter than its `Content-Length` said written down and read as no answer
    /// (backlog 0598, ADR-021).
    pub(super) fn whole_answer(
        &self,
        id: &str,
        raw: &str,
        head: bool,
    ) -> Option<sv_check::probes::ProbeResponse> {
        let answer = parse_response(id, raw)?;
        if let Some((came, said)) = short_body(raw, head) {
            if let Ok(mut short) = self.short_answers.lock() {
                short.push(format!("{id} ({}, {came} of {said} bytes)", answer.status));
            }
            return None;
        }
        Some(answer)
    }

    /// What `whole_answer` wrote down, for the run's outcome.
    pub(super) fn short_answers(&self) -> Vec<String> {
        self.short_answers
            .lock()
            .map(|s| s.clone())
            .unwrap_or_default()
    }

    /// Sends one request through the session kept open in the sidecar (`exec_session`), starting it when there is
    /// none. `None` when there is no sidecar, or no session would start, or nothing was sent: the request then goes
    /// the old way, an exec of its own. A session that breaks down is dropped, and the sidecar's requests after it go
    /// that way too, so a sidecar that has gone is told from a silent app as before (`note_if_sidecar_lost`).
    pub(super) fn in_session(
        &self,
        via: &Via,
        host: &str,
        port: u16,
        raw: &[u8],
    ) -> Option<Result<(i32, String), Lost>> {
        let Via::Sidecar(sidecar) = via else {
            return None;
        };
        // After Ctrl-C nothing new is sent this way; the exec path refuses to start, and says so.
        if crate::interrupted() {
            return None;
        }
        let mut session = self.session.lock().ok()?;
        if session.as_ref().is_some_and(|(name, _)| name != sidecar) {
            *session = None;
        }
        let refused = |no_session: &std::sync::Mutex<Option<String>>| {
            if let Ok(mut no) = no_session.lock() {
                *no = Some((*sidecar).to_owned());
            }
        };
        if session.is_none() {
            if self
                .no_session
                .lock()
                .ok()
                .is_some_and(|no| no.as_deref() == Some(*sidecar))
            {
                return None;
            }
            let mark = format!("@@sv-session-{}-", crate::random_hex(8));
            let script = crate::exec_session::loop_script(&mark);
            let mut command = Command::new(&self.binary);
            command.args(self.prepared(&["exec", "-i", sidecar, "sh", "-c", &script]));
            match crate::exec_session::Session::start(command, &mark) {
                Some(started) => {
                    self.sessions_started
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    *session = Some(((*sidecar).to_owned(), started));
                }
                None => {
                    refused(&self.no_session);
                    return None;
                }
            }
        }
        let (_, open) = session.as_mut()?;
        match open.send(host, port, raw) {
            Ok(answered) => {
                self.answered_in_session
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Some(Ok(answered))
            }
            Err(crate::exec_session::Failed::NotSent) => {
                *session = None;
                refused(&self.no_session);
                None
            }
            Err(crate::exec_session::Failed::Lost { said, code }) => {
                *session = None;
                refused(&self.no_session);
                Some(Err(Lost::Said(if code == 0 { -1 } else { code }, said)))
            }
        }
    }

    /// Ends the session kept open in the sidecar, before the sidecar goes.
    pub(super) fn end_session(&self) {
        if let Ok(mut session) = self.session.lock() {
            *session = None;
        }
    }

    /// `probe`, for requests sent one after another in one call (`in_turn_script`). `None` when
    /// one of them cannot be sent at all or the container that sends them did not run.
    pub(super) fn probe_in_turn(
        &self,
        via: &Via,
        app: &str,
        port: u16,
        requests: &[sv_check::probes::ProbeRequest],
    ) -> Option<Vec<Option<sv_check::probes::ProbeResponse>>> {
        let (input, sizes, which) = together_input(requests, app)?;
        let mark = at_once_mark();
        let script = in_turn_script(app, port, &sizes, &which, &mark);
        let started = std::time::Instant::now();
        let sent = self.inside_fence_with_input(via, &["sh", "-c", &script], &input);
        self.note_request_time(several(requests, "one after another"), started);
        let (code, out) = sent.ok()?;
        let first = requests.first().map_or("", |r| r.id.as_str());
        if self.note_if_sidecar_lost(first, code, &out) || !out.contains(&mark) {
            return None;
        }
        let ids: Vec<&str> = requests.iter().map(|r| r.id.as_str()).collect();
        Some(
            at_once_raws(ids.len(), &out, &mark)
                .into_iter()
                .zip(requests)
                .map(|(raw, r)| {
                    raw.and_then(|raw| {
                        self.whole_answer(&r.id, raw, r.method.eq_ignore_ascii_case("HEAD"))
                    })
                })
                .collect(),
        )
    }

    /// `probe`, for requests sent together. `None` when one of them cannot be sent at all or the
    /// container that sends them did not run.
    pub(super) fn probe_together(
        &self,
        via: &Via,
        app: &str,
        port: u16,
        requests: &[sv_check::probes::ProbeRequest],
    ) -> Option<Vec<Option<sv_check::probes::ProbeResponse>>> {
        let (input, sizes, which) = together_input(requests, app)?;
        let mark = at_once_mark();
        let script = at_once_script(app, port, &sizes, &which, &mark);
        let started = std::time::Instant::now();
        let sent = self.inside_fence_with_input(via, &["sh", "-c", &script], &input);
        self.note_request_time(several(requests, "together"), started);
        let (code, out) = sent.ok()?;
        let first = requests.first().map_or("", |r| r.id.as_str());
        if self.note_if_sidecar_lost(first, code, &out) || !out.contains(&mark) {
            return None;
        }
        let ids: Vec<&str> = requests.iter().map(|r| r.id.as_str()).collect();
        Some(
            at_once_raws(ids.len(), &out, &mark)
                .into_iter()
                .zip(requests)
                .map(|(raw, r)| {
                    raw.and_then(|raw| {
                        self.whole_answer(&r.id, raw, r.method.eq_ignore_ascii_case("HEAD"))
                    })
                })
                .collect(),
        )
    }
}

/// Why a request sent alone has no answer to read: the session it went through broke down, with what it said and
/// its exit code, or the exec it went through could not be run.
pub(super) enum Lost {
    Said(i32, String),
    Unsaid,
}

impl From<String> for Lost {
    fn from(_: String) -> Self {
        Lost::Unsaid
    }
}

/// How several requests sent in one call are named in the request times: the first, how many more,
/// and how they were sent.
pub(super) fn several(requests: &[sv_check::probes::ProbeRequest], how: &str) -> String {
    match requests {
        [] => format!("no requests, sent {how}"),
        [one] => one.id.clone(),
        [first, rest @ ..] => format!("{} and {} more, sent {how}", first.id, rest.len()),
    }
}
