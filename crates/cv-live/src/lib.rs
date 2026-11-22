//! ConfigVet's live reading of a hosting account (ADR-157, decision 5; backlog 0717). Everything here keeps to that
//! decision's fence:
//!
//! - **Off until asked for by name:** only `cv live <provider>` calls in, never `cv check`.
//! - **The token from the environment,** handed to `curl` on its standard input as a header, so it is on no command
//!   line, in no file, and in no report; the report shows it only as `Secret::redact` shows a key.
//! - **A fixed list of `GET` requests,** to the provider's named host only, over HTTPS with certificate checks
//!   (`curl`'s own), no redirect followed unless it stays on that host, and limits on the number of requests, the
//!   size of each answer, and the time each may take.
//! - **Every request listed in the report:** method, host, path, and status.
//!
//! The providers are modules of their own: [`github`] is the first (ADR-157, Later, 21 November 2026), and
//! [`cloudflare`] the second (ADR-157, Later, 22 November 2026; backlog 0732). Both send their requests through one
//! [`Client`], which holds the limits and checks each redirect.

pub mod cloudflare;
pub mod cloudflare_judge;
pub mod github;
pub mod judge;

use serde::Serialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// No answer larger than this is read: a settings answer is a few kilobytes.
pub const MOST_ANSWER_BYTES: u64 = 1_000_000;
/// No request takes longer than this, in seconds.
pub const MOST_SECONDS: u64 = 20;
/// No reading makes more requests than this, redirects included.
pub const MOST_REQUESTS: usize = 40;
/// No request follows more redirects than this, each to the same host.
pub const MOST_REDIRECTS: usize = 2;

/// One request as the report lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sent {
    pub method: &'static str,
    pub host: String,
    pub path: String,
    /// The answer's HTTP status; 0 when no answer came.
    pub status: u16,
}

/// What came back for one request.
#[derive(Debug, Clone, Default)]
pub struct Answer {
    pub status: u16,
    /// Each header's name in lower case, and its value.
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    /// Where a redirect pointed, when it was one.
    pub redirect: Option<String>,
}

impl Answer {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

/// One `GET` for one address, with the token handed over out of sight: `curl` for real, a stand-in in a test.
pub trait Transport {
    fn get(&mut self, url: &str, token: &str) -> Result<Answer, String>;
}

/// The program that sends the requests: `curl`, found on the PATH. Only in a debug build, `CV_CURL_FOR_TEST` names the
/// program to run instead, as `SV_CURL_FOR_TEST` does for `sv`, so a test can put a stand-in in its place. A released
/// `cv` has no such switch.
fn curl_program() -> std::ffi::OsString {
    #[cfg(debug_assertions)]
    if let Some(program) = std::env::var_os("CV_CURL_FOR_TEST") {
        return program;
    }
    "curl".into()
}

/// The real requests: `curl`, with the flags that make the limits true rather than intended.
pub struct Curl {
    folder: PathBuf,
    count: usize,
    /// The provider's own headers, sent beside the token: `github::HEADERS` or `cloudflare::HEADERS`.
    headers: &'static [&'static str],
}

impl Curl {
    /// A `curl` that writes each answer into a folder of its own, which only this user can open, removed when done, and
    /// sends `headers`, the provider's own, beside the token.
    pub fn new(headers: &'static [&'static str]) -> Result<Self, String> {
        let folder = std::env::temp_dir().join(format!(
            "cv-live-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let mut builder = std::fs::DirBuilder::new();
        #[cfg(unix)]
        std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
        builder
            .create(&folder)
            .map_err(|e| format!("cv could not make a private folder for the answers ({e})"))?;
        Ok(Curl {
            folder,
            count: 0,
            headers,
        })
    }

    /// Every flag but the address and the two files: `--disable` first, the only place curl reads it, so no
    /// `.curlrc` adds anything; globbing off; HTTPS only; no redirect followed by curl itself, since each is checked
    /// here first; the limits; and the headers, the token's among them, read from standard input (`@-`).
    pub fn args(body: &Path, headers: &Path) -> Vec<String> {
        [
            "--disable",
            "--globoff",
            "--proto",
            "=https",
            "--max-redirs",
            "0",
            "--silent",
            "--show-error",
            "--max-filesize",
            &MOST_ANSWER_BYTES.to_string(),
            "--max-time",
            &MOST_SECONDS.to_string(),
            "--user-agent",
            "cv (ConfigVet, reading settings read-only)",
            "--header",
            "@-",
            "--write-out",
            "%{http_code} %{redirect_url}",
            "--dump-header",
            &headers.display().to_string(),
            "--output",
            &body.display().to_string(),
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
}

impl Drop for Curl {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.folder);
    }
}

impl Transport for Curl {
    fn get(&mut self, url: &str, token: &str) -> Result<Answer, String> {
        use std::io::Write;
        self.count += 1;
        let body = self.folder.join(format!("{}.body", self.count));
        let headers = self.folder.join(format!("{}.headers", self.count));
        let mut child = Command::new(curl_program())
            .args(Curl::args(&body, &headers))
            .arg(url)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                format!("curl could not be started ({e}); cv live needs curl on the PATH")
            })?;
        {
            let mut stdin = child
                .stdin
                .take()
                .ok_or("curl's input could not be opened")?;
            // The token goes no further than this pipe: not into an argument, a file, or anything printed.
            let mut sent = writeln!(stdin, "Authorization: Bearer {token}");
            for header in self.headers {
                sent = sent.and_then(|_| writeln!(stdin, "{header}"));
            }
            sent.map_err(|e| format!("curl's input could not be written ({e})"))?;
        }
        let out = child
            .wait_with_output()
            .map_err(|e| format!("curl did not finish ({e})"))?;
        let said = String::from_utf8_lossy(&out.stdout).into_owned();
        let mut parts = said.trim().splitn(2, ' ');
        let status: u16 = parts.next().unwrap_or("0").parse().unwrap_or(0);
        let redirect = parts
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned);
        if !out.status.success() {
            // Any failure of curl's own, an answer cut short at the size limit among them, is no answer: what came is
            // never read as if it were whole. curl's own words name the address and the failure, never a header.
            let why = String::from_utf8_lossy(&out.stderr);
            return Err(format!(
                "curl failed: {}",
                why.trim().chars().take(200).collect::<String>()
            ));
        }
        let headers = std::fs::read_to_string(&headers).unwrap_or_default();
        let body = std::fs::read(&body).unwrap_or_default();
        Ok(Answer {
            status,
            headers: parse_headers(&headers),
            body,
            redirect,
        })
    }
}

/// The headers of the last answer in a `--dump-header` file, each name in lower case.
pub fn parse_headers(text: &str) -> Vec<(String, String)> {
    let last = text
        .split("\r\n\r\n")
        .filter(|block| !block.trim().is_empty())
        .last()
        .unwrap_or("");
    last.lines()
        .skip(1)
        .filter_map(|line| line.split_once(':'))
        .map(|(n, v)| (n.trim().to_ascii_lowercase(), v.trim().to_owned()))
        .collect()
}

/// One provider's requests: every one a `GET` to `host`, a redirect followed only when it stays on that host, and no
/// more than [`MOST_REQUESTS`] of them, redirects included.
pub(crate) struct Client<'t> {
    transport: &'t mut dyn Transport,
    token: &'t str,
    host: &'static str,
    /// The provider's name, for what a person reads: "GitHub", "Cloudflare".
    name: &'static str,
    /// The path each request is listed under begins after this: the host, and the API's own folder.
    base: &'static str,
    pub(crate) requests: Vec<Sent>,
}

/// An answer: its JSON, when it was a 200 that held JSON; otherwise its status.
pub(crate) enum Got {
    Json(Value, Answer),
    Status(u16, Answer),
}

impl<'t> Client<'t> {
    pub(crate) fn new(
        host: &'static str,
        base: &'static str,
        name: &'static str,
        transport: &'t mut dyn Transport,
        token: &'t str,
    ) -> Self {
        Client {
            transport,
            token,
            host,
            name,
            base,
            requests: Vec::new(),
        }
    }

    /// A `GET` of `path`, under `https://<host><base>`, following a redirect only when it stays on that host.
    pub(crate) fn get(&mut self, path: &str) -> Result<Got, String> {
        let (host, name) = (self.host, self.name);
        let mut url = format!("https://{host}{}{path}", self.base);
        for _ in 0..=MOST_REDIRECTS {
            if self.requests.len() >= MOST_REQUESTS {
                return Err(format!(
                    "cv stopped at its limit of {MOST_REQUESTS} requests"
                ));
            }
            let shown = url
                .trim_start_matches(&format!("https://{host}"))
                .to_owned();
            let answer = match self.transport.get(&url, self.token) {
                Ok(answer) => answer,
                Err(why) => {
                    self.requests.push(Sent {
                        method: "GET",
                        host: host.to_owned(),
                        path: shown,
                        status: 0,
                    });
                    return Err(why);
                }
            };
            self.requests.push(Sent {
                method: "GET",
                host: host.to_owned(),
                path: shown,
                status: answer.status,
            });
            if (300..400).contains(&answer.status) {
                match &answer.redirect {
                    Some(next) if next.starts_with(&format!("https://{host}/")) => {
                        url = next.clone();
                        continue;
                    }
                    Some(next) => {
                        return Err(format!(
                            "{name} redirected to {next}, which is not {host}, so it was not followed"
                        ));
                    }
                    None => return Ok(Got::Status(answer.status, answer)),
                }
            }
            if answer.status == 200 {
                return match serde_json::from_slice::<Value>(&answer.body) {
                    Ok(value) => Ok(Got::Json(value, answer)),
                    Err(_) => Err(format!("{name}'s answer to {path} was not JSON")),
                };
            }
            return Ok(Got::Status(answer.status, answer));
        }
        Err(format!(
            "{name} redirected {path} more than {MOST_REDIRECTS} times"
        ))
    }
}

/// The `curl` flags as a test reads them.
pub fn curl_args_for_test() -> Vec<String> {
    Curl::args(Path::new("BODY"), Path::new("HEADERS"))
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;
