//! The Docker backend.
//!
//! Shape of a run, and why each step is the way it is:
//!
//! 1. Create a per-run network with `--internal`. Measured: no outbound, no DNS, and unreachable
//!    from this computer. That last part is the reason there is no published port anywhere below.
//! 2. Start the app on it, with its folder mounted read-only and no credentials in its environment.
//! 3. Wait for it to answer its health path — from a **sidecar container on the same network**,
//!    because the host cannot reach it. The sidecar is started once, and the requests after go
//!    through it: a container started per request cost about half a second each. Single requests
//!    go through one `exec` kept open for the run (`exec_session`, backlog 0341), since a signed-in
//!    run sends a few hundred and an `exec` each was estimated at 100 to 300 milliseconds.
//! 4. Ask the probes, anonymous and signed in, through the same sidecar; then remove it.
//! 5. Run the declared test command inside the app container.
//! 6. Tear everything down, whatever happened.

use crate::{Backend, CannotRun, Fence, RunFailed, RunOutcome, RunPlan, TestResult, output_of};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Duration;
use sv_check::text::first_line;

mod fence;
// Its free functions are used by its own methods, and reached from the tests through `docker`.
#[cfg(test)]
use fence::*;
// Public as it was when it lived here.
pub use fence::network_create_args;

mod probes;
use probes::*;

mod helpers;

mod install;

mod containers;
use containers::*;
// Public as it was when it lived here.
pub use containers::read_only_bind;

/// How long to wait for the app to answer before calling it not assessed.
const READY_TIMEOUT_SECONDS: u64 = 60;
/// The helper images, each named by its digest as well as its tag, so a moved tag cannot change what runs (backlog
/// 0238). They are written in one file, `images/Dockerfile`, which CI reads to fetch them and Dependabot moves (backlog
/// 0317); each is found there when `sv` is compiled, so a name missing from the file stops the build.
const IMAGES: &str = include_str!("../images/Dockerfile");

/// The image on the line `FROM <image> AS <stage>` of `images/Dockerfile`.
const fn helper_image(stage: &str) -> &'static str {
    let bytes = IMAGES.as_bytes();
    let mut start = 0;
    while start < bytes.len() {
        let mut end = start;
        while end < bytes.len() && bytes[end] != b'\n' {
            end += 1;
        }
        let (_, rest) = bytes.split_at(start);
        let (line, _) = rest.split_at(end - start);
        if let Some(image) = image_of_line(line, stage.as_bytes()) {
            return match std::str::from_utf8(image) {
                Ok(image) => image,
                Err(_) => panic!("images/Dockerfile: an image name that is not UTF-8"),
            };
        }
        start = end + 1;
    }
    panic!("images/Dockerfile: no line FROM <image> AS <stage> for one of the helper images")
}

/// The image of one line, when the line is exactly `FROM <image> AS <stage>` (a trailing `\r` or space allowed).
const fn image_of_line<'a>(line: &'a [u8], stage: &[u8]) -> Option<&'a [u8]> {
    let mut line = line;
    while let [rest @ .., b'\r' | b' ' | b'\t'] = line {
        line = rest;
    }
    let Some(rest) = strip(line, b"FROM ") else {
        return None;
    };
    let mut at = 0;
    while at < rest.len() && rest[at] != b' ' {
        at += 1;
    }
    let (image, tail) = rest.split_at(at);
    let Some(name) = strip(tail, b" AS ") else {
        return None;
    };
    if image.is_empty() || name.len() != stage.len() {
        return None;
    }
    let mut i = 0;
    while i < name.len() {
        if name[i] != stage[i] {
            return None;
        }
        i += 1;
    }
    Some(image)
}

/// `bytes` without `prefix`, if it starts with it.
const fn strip<'a>(bytes: &'a [u8], prefix: &[u8]) -> Option<&'a [u8]> {
    if bytes.len() < prefix.len() {
        return None;
    }
    let mut i = 0;
    while i < prefix.len() {
        if bytes[i] != prefix[i] {
            return None;
        }
        i += 1;
    }
    Some(bytes.split_at(prefix.len()).1)
}

/// The image the probes run from. Tiny, and already needed for the health check. The digest is the multi-platform one
/// the tag points to.
pub const PROBE_IMAGE: &str = helper_image("probe");

/// The one place a container that talks to the app may write: in memory, where nothing written can
/// be run, and only big enough for the request it is about to send.
const PROBE_TMPFS: &str = "/tmp:rw,noexec,nosuid,size=16m";
/// Ways of giving the fenced network's bridge no address of its own on the host, tried in order.
/// `--internal` stops traffic leaving for the internet, but the bridge's gateway address is the host
/// itself (on Docker Desktop and Colima, the virtual machine), so without one of these the app could
/// reach anything listening there (the deep review of 4 October 2026, S2). Containers on the network
/// still reach one another. The first asks Docker (28 and later) for no gateway over IPv4 or IPv6:
/// a daemon set to give new networks IPv6 gives the internal network an IPv6 gateway too, which is
/// the host just the same (backlog 0300). The second asks it of IPv4 alone, the third is the older
/// IPv4 option. Which was used is said if the check fails, and the gateway check knocks on every
/// gateway of either family, so a daemon that took an option and left an IPv6 gateway open is still
/// caught.
const NO_GATEWAY: &[&[&str]] = &[
    &[
        "com.docker.network.bridge.gateway_mode_ipv4=isolated",
        "com.docker.network.bridge.gateway_mode_ipv6=isolated",
    ],
    &["com.docker.network.bridge.gateway_mode_ipv4=isolated"],
    &["com.docker.network.bridge.inhibit_ipv4=true"],
];
/// A port the gateway check knocks on: whatever answers there, open or refused, is the host's stack
/// answering, which is what the fence must not allow.
const GATEWAY_PORT: &str = "9";
/// The mail server the app is given when the probes need to read its email: Mailpit, which keeps
/// every message it is sent and answers questions about them over HTTP. Pinned to a minor release,
/// as the probe image is, so a run does not change under the owner because a new one came out.
const MAIL_IMAGE: &str = helper_image("mail");
/// The test OpenID Connect provider runs in a stock Node image: its script uses built-in modules
/// only, because the fence has no route to a package registry.
pub(crate) const PROVIDER_IMAGE: &str = helper_image("provider");
const PROVIDER_PORT: u16 = 9000;
const PROVIDER_SCRIPT: &str = include_str!("../assets/oidc-provider.mjs");
/// The test model the app's AI feature is pointed at, in the same stock Node image. See
/// `assets/model-provider.mjs`.
const MODEL_PORT: u16 = 9100;
const MODEL_SCRIPT: &str = include_str!("../assets/model-provider.mjs");
/// What the app is given as its API keys for the test model: something to send, and nothing that
/// would work anywhere else.
const MODEL_KEY: &str = "sv-test-model-key-not-a-real-key";
/// The client id the app is told to use. Not a secret: the provider checks it only to refuse a
/// sign-in the app did not ask for with its own configuration.
const PROVIDER_CLIENT_ID: &str = "sv-test-client";
/// The headless browser, pinned to one version so a run today and a run next month draw pages the
/// same way. Its DevTools port is reached only from the driver, which shares its network.
const BROWSER_IMAGE: &str = helper_image("browser");
/// Every helper image `sv` runs, by name and digest, in the run record so a report says what ran (backlog 0238).
pub const HELPER_IMAGES: &[&str] = &[PROBE_IMAGE, MAIL_IMAGE, PROVIDER_IMAGE, BROWSER_IMAGE];

/// The digest the owner's app image was pulled by, from the local Docker, for the run record (backlog 0238): the one
/// its registry names when it was pulled, otherwise its own image ID. Local only: `docker image inspect` asks no
/// registry. `None` when Docker cannot say.
pub fn image_digest(image: &str) -> Option<String> {
    let inspect = |format: &str| -> Option<String> {
        let out = std::process::Command::new("docker")
            .args(["image", "inspect", "--format", format, image])
            .output()
            .ok()?;
        out.status
            .success()
            .then(|| String::from_utf8_lossy(&out.stdout).trim().to_owned())
            .filter(|s| !s.is_empty())
    };
    inspect("{{index .RepoDigests 0}}")
        .as_deref()
        .and_then(digest_of)
        .or_else(|| inspect("{{.Id}}").as_deref().and_then(digest_of))
}

/// The `sha256:` digest in a repository digest such as `repo/name@sha256:…`, or an image ID; `None` for anything
/// else. A digest is 64 lowercase hexadecimal characters after `sha256:`.
pub fn digest_of(text: &str) -> Option<String> {
    let digest = text.rsplit_once('@').map_or(text, |(_, d)| d);
    let hex = digest.strip_prefix("sha256:")?;
    (hex.len() == 64
        && hex
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()))
    .then(|| digest.to_owned())
}

#[cfg(test)]
#[path = "docker_images_file_tests.rs"]
mod images_file_tests;
#[cfg(test)]
#[path = "docker_images_tests.rs"]
mod images_tests;

#[cfg(test)]
#[path = "docker_ipv6_advice_tests.rs"]
mod ipv6_advice_tests;
/// What drives it: a script of `sv`'s own, run in the same stock Node image as the test provider.
const DRIVER_SCRIPT: &str = include_str!("../assets/browser-driver.mjs");
/// Where the app sends its mail on the mail server, and where the probes read it.
const SMTP_PORT: u16 = 1025;
const MAIL_API_PORT: u16 = 8025;
/// How long to wait for an email the app may send after it has already answered.
const MAIL_WAIT_SECONDS: u64 = 10;
/// What every container `sv` starts may use at most: memory (with no swap beyond it), processes, and
/// processors. The app is code `sv` was asked to check, not code it trusts, and without these an app
/// that leaks memory or starts processes without end could take this computer down with it (the deep
/// review of 4 October 2026, S9). Its in-memory folders count against the same memory. Two gigabytes
/// and 512 processes are far more than an app answering a few hundred requests and running its tests
/// needs; the processors are two, or fewer when Docker has fewer, since Docker refuses more.
const MEMORY_LIMIT: &str = "2g";
const PROCESS_LIMIT: &str = "512";
const MOST_CPUS: u64 = 2;

#[cfg(all(test, unix))]
mod container_record_tests;
#[cfg(all(test, unix))]
mod resource_reading_tests;
#[cfg(test)]
mod short_body_tests;
#[cfg(all(test, unix))]
mod stand_in_ready_tests;

/// How the wait for the app to answer ended: whether it answered, after how many seconds, and how
/// it ended when it stopped of its own accord first.
struct Readiness {
    answered: bool,
    after_seconds: u64,
    exited: Option<crate::Exited>,
}

/// How a container ended, from `docker inspect`'s "status exit-code out-of-memory", or `None` while
/// it has not ended.
fn exited_from(state: &str) -> Option<crate::Exited> {
    match state.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["exited" | "dead", code, oom] => Some(crate::Exited {
            code: code.parse().unwrap_or(-1),
            out_of_memory: *oom == "true",
        }),
        _ => None,
    }
}

/// How long the sidecar may live if nothing removes it. It is removed as soon as the probes are
/// done, and by the teardown whatever happens; this is the bound for a run that dies without either,
/// so a crash cannot leave a container behind on the owner's machine for longer than this. Made of
/// what the run's questions take and the whole of the waiting a run may do for a rate limiter
/// (`sv_check::signed_in::MOST_WAITING`, one budget for every suite since 8 October 2026), so a
/// run that waits as long as it may still has its sidecar at the end (ADR-025, Later, 8 October
/// 2026; until then the two numbers were set apart, and a run could outlive its sidecar, after
/// which every request read as "no answer" and nothing said why).
const SIDECAR_SECONDS: u64 = SIDECAR_QUESTIONS_SECONDS + sv_check::signed_in::MOST_WAITING;

/// What the questions themselves are allowed, before any waiting for a limiter.
const SIDECAR_QUESTIONS_SECONDS: u64 = 600;

pub struct DockerBackend {
    binary: String,
    /// What everything this backend starts is labeled with: this machine and this process.
    owner: String,
    /// The processors each container may use, asked of Docker once, when the first is started.
    /// `None` when Docker would not say, and then no processor limit is set.
    cpus: OnceLock<Option<String>>,
    /// The run under way, if one is: everything started while it is set carries it as `RUN_LABEL`,
    /// and its teardown removes exactly what carries it.
    run: std::sync::Mutex<Option<String>>,
    /// How far the containers' clock is ahead of this computer's, in seconds, measured once (`clock_offset`).
    clock_offset: OnceLock<i64>,
    /// The first request that found the sidecar gone, when one did: the request's id and what
    /// Docker said. Every request after it reads as "no answer", and the run says why.
    sidecar_lost: std::sync::Mutex<Option<String>>,
    /// The answers that came with less body than their `Content-Length` said, as "id (status, N of M bytes)": each is
    /// read as no answer, and the run says so once (backlog 0598, ADR-021).
    short_answers: std::sync::Mutex<Vec<String>>,
    /// What the teardown of the last run that failed could not remove, each with what Docker said:
    /// that teardown runs as the run unwinds (`Teardown`'s drop), after the run's own answer is
    /// gone, so it is kept here for the failure to say (backlog 226, part 2, item 18).
    left_behind: std::sync::Mutex<Vec<String>>,
    /// How long each request to the app took, in milliseconds, in the order sent: one entry for a
    /// request sent alone, one for several sent in one call (backlog 226, part 2, item 13).
    request_times: std::sync::Mutex<Vec<(String, u64)>>,
    /// The one `docker exec` into the sidecar that single requests go through, with the sidecar's name, while it
    /// works (backlog 0341).
    session: std::sync::Mutex<Option<(String, crate::exec_session::Session)>>,
    /// The sidecar a session would not start in or broke down in: its requests go one exec each, as before.
    no_session: std::sync::Mutex<Option<String>>,
    /// How many sessions were started, and how many requests were answered through one, for the tests that hold
    /// the count of execs down.
    sessions_started: std::sync::atomic::AtomicUsize,
    answered_in_session: std::sync::atomic::AtomicUsize,
}

/// The label naming the one run a container or network belongs to. The owner label says which
/// process made it, for the next run's cleanup after a crash; this one says which run, so a
/// teardown removes only that run's and never another's (the deep review of 4 October 2026, S10).
const RUN_LABEL: &str = sv_frameworks::names::RUN_LABEL;

impl Default for DockerBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerBackend {
    pub fn new() -> Self {
        Self {
            binary: "docker".to_owned(),
            owner: crate::cleanup::owner(),
            cpus: OnceLock::new(),
            run: std::sync::Mutex::new(None),
            clock_offset: OnceLock::new(),
            sidecar_lost: std::sync::Mutex::new(None),
            short_answers: std::sync::Mutex::new(Vec::new()),
            left_behind: std::sync::Mutex::new(Vec::new()),
            request_times: std::sync::Mutex::new(Vec::new()),
            session: std::sync::Mutex::new(None),
            no_session: std::sync::Mutex::new(None),
            sessions_started: std::sync::atomic::AtomicUsize::new(0),
            answered_in_session: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Whether `out`, from a request that failed with `code`, says the container it was sent
    /// through is gone (the sidecar ended, or was removed), and if so writes it down once: the
    /// difference between the app not answering and nothing being there to ask it (ADR-025,
    /// Later, 8 October 2026).
    fn note_if_sidecar_lost(&self, request_id: &str, code: i32, out: &str) -> bool {
        if code == 0 || !sidecar_gone(out) {
            return false;
        }
        if let Ok(mut lost) = self.sidecar_lost.lock()
            && lost.is_none()
        {
            // In `sv`'s own words, whatever the backend said and however it cased it (backlog 0519).
            *lost = Some(format!(
                "the container the questions are sent from was gone when `{request_id}` was sent: \
                 it had stopped, or been removed, before the request reached it"
            ));
        }
        true
    }

    /// What `note_if_sidecar_lost` wrote down, if anything, for the run's outcome.
    fn sidecar_lost(&self) -> Option<String> {
        self.sidecar_lost.lock().ok().and_then(|l| l.clone())
    }

    /// Writes down how long the request (or the requests sent in one call) named `what` took.
    fn note_request_time(&self, what: String, started: std::time::Instant) {
        let took = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
        if let Ok(mut times) = self.request_times.lock() {
            times.push((what, took));
        }
    }

    /// The request times written down since the last call, which empties them.
    fn take_request_times(&self) -> Vec<(String, u64)> {
        self.request_times
            .lock()
            .map(|mut t| std::mem::take(&mut *t))
            .unwrap_or_default()
    }

    /// How far the clock the app's containers read is ahead of this computer's, in seconds, asked of the
    /// fence's container once. On a Mac, Docker runs in a virtual machine whose clock can fall behind
    /// after the computer sleeps, and a two-factor code made by this computer's clock is then one the
    /// app, reading the machine's, refuses (the deep review's improvement 5). 0 when it cannot be read.
    fn clock_offset(&self, via: &Via) -> i64 {
        *self
            .clock_offset
            .get_or_init(|| self.read_clock_offset(via).unwrap_or(0))
    }

    /// One reading of the containers' clock against this computer's: `None` when it could not be read.
    fn read_clock_offset(&self, via: &Via) -> Option<i64> {
        let before = host_now();
        let read = self.inside_fence(via, &["date", "+%s"]);
        let after = host_now();
        match read {
            Ok((0, out)) => out
                .trim()
                .parse::<u64>()
                .ok()
                .map(|theirs| offset_from(before, theirs, after)),
            _ => None,
        }
    }

    /// The last 2000 lines the app wrote, to its output and its errors, in the order it wrote them.
    ///
    /// `docker logs` gives the two streams apart, and they were read one after the other, so a
    /// window between two markers on one stream held nothing the app wrote to the other in between
    /// (item 15 of the review of 1 to 4 October). Each line now comes with the time Docker took it,
    /// and the two are put back in that order.
    fn app_log(&self, app: &str) -> String {
        self.docker(&["logs", "--timestamps", "--tail", "2000", app])
            .map(|(_, out)| interleaved(&out))
            .unwrap_or_default()
    }

    fn docker(&self, args: &[&str]) -> Result<(i32, String), String> {
        let mut c = Command::new(&self.binary);
        c.args(self.prepared(args));
        output_of(&mut c)
    }

    /// What the engine says of the app's container now: one `docker stats` reading and the limits it started
    /// with (backlog 0243). `None` when the engine would not say, or what it said is not the format
    /// `crate::resources` reads; the report then says so rather than showing a guess.
    fn resource_readings(&self, app: &str) -> Option<crate::resources::Taken> {
        let (code, stats) = self
            .docker(&[
                "stats",
                "--no-stream",
                "--format",
                crate::resources::STATS_FORMAT,
                app,
            ])
            .ok()?;
        if code != 0 {
            return None;
        }
        let reading = crate::resources::parse_stats(stats.lines().find(|l| !l.trim().is_empty())?)?;
        let (code, limits) = self
            .docker(&["inspect", "--format", crate::resources::LIMITS_FORMAT, app])
            .ok()?;
        if code != 0 {
            return None;
        }
        let limits = crate::resources::parse_limits(limits.trim())?;
        Some(crate::resources::Taken { reading, limits })
    }

    /// As `docker`, with `secrets` in the Docker program's own environment, for arguments that name
    /// them with `-e NAME` and no value: Docker then copies each from its own environment. A value on
    /// the command line can be read by any other user of the computer while the command runs; a
    /// process's environment only by its own user (the deep review's improvement 5).
    fn docker_with_secrets(
        &self,
        args: &[&str],
        secrets: &[(&str, String)],
    ) -> Result<(i32, String), String> {
        let mut c = Command::new(&self.binary);
        c.args(self.prepared(args));
        c.envs(secrets.iter().map(|(k, v)| (*k, v.as_str())));
        output_of(&mut c)
    }

    /// A Docker call's arguments as they are sent: labeled, and, for one that starts a container,
    /// limited. Every container this backend starts goes through here, so none is missed.
    fn prepared(&self, args: &[&str]) -> Vec<String> {
        let mut labeled = crate::cleanup::labeled(args, &self.owner);
        let at = match args {
            ["run", ..] | ["create", ..] => Some(1),
            ["network", "create", ..] => Some(2),
            _ => None,
        };
        if let Some(at) = at
            && let Some(run) = self.run.lock().ok().and_then(|run| run.clone())
        {
            labeled.splice(at..at, ["--label".to_owned(), format!("{RUN_LABEL}={run}")]);
        }
        if !matches!(args, ["run", ..] | ["create", ..]) {
            return labeled;
        }
        let cpus = self.cpus.get_or_init(|| {
            let mut c = Command::new(&self.binary);
            c.args(["info", "--format", "{{.NCPU}}"]);
            match output_of(&mut c) {
                Ok((0, out)) => cpus_for(&out),
                _ => None,
            }
        });
        limited(hardened(labeled), cpus.as_deref())
    }

    /// Removes the containers, then the networks, that runs on this machine left behind when their
    /// process was killed outright. What it removed, by name.
    fn remove_leftovers(&self) -> Vec<String> {
        let machine = crate::cleanup::this_machine();
        let left = |args: &[&str]| -> Vec<String> {
            match self.docker(args) {
                Ok((0, out)) => out
                    .lines()
                    .filter_map(|line| line.split_once('\t'))
                    .filter(|(_, label)| {
                        crate::cleanup::is_leftover(label.trim(), &machine, crate::cleanup::alive)
                    })
                    .map(|(name, _)| name.trim().to_owned())
                    .collect(),
                _ => Vec::new(),
            }
        };
        let mut removed = Vec::new();
        // Under the label a run writes now, and under the one runs wrote before the rename
        // (ADR-062): a leftover is a leftover under either.
        for label in [crate::cleanup::OWNER_LABEL, crate::cleanup::OLD_OWNER_LABEL] {
            let filter = format!("label={label}");
            let format = format!("{{{{.Names}}}}\t{{{{.Label \"{label}\"}}}}");
            let network_format = format.replace(".Names", ".Name");
            for name in left(&["ps", "-a", "--filter", &filter, "--format", &format]) {
                if matches!(self.docker_cleanup(&["rm", "-f", &name]), Ok((0, _))) {
                    removed.push(name);
                }
            }
            for name in left(&[
                "network",
                "ls",
                "--filter",
                &filter,
                "--format",
                &network_format,
            ]) {
                if matches!(self.docker_cleanup(&["network", "rm", &name]), Ok((0, _))) {
                    removed.push(name);
                }
            }
        }
        removed
    }

    /// A Docker call that removes what the run made. It still runs after Ctrl-C, since removing
    /// things is the point of catching it, and gets two minutes rather than twenty.
    fn docker_cleanup(&self, args: &[&str]) -> Result<(i32, String), String> {
        let mut c = Command::new(&self.binary);
        c.args(args);
        crate::bounded_output(&mut c, Duration::from_secs(120), true)
    }
}

/// What `docker info --format {{.OSType}}` answered, read: a daemon that runs Linux containers is a
/// backend, and one that runs Windows containers is not. Every container `sv` starts is a Linux one
/// (the app's image, the probe sidecar, the install step), and Docker on Windows set to Windows
/// containers refuses each in a way that reads like a fault of the app's (`could not find plugin
/// bridge`, `read-only mode is not supported`). Said here instead, with what to change (backlog 0120).
pub(crate) fn linux_containers(code: i32, answer: &str) -> Result<(), CannotRun> {
    if code != 0 {
        // Podman answers `docker info` too, through its `docker` command, but its info has no `OSType`, and the
        // template it is asked for fails on its own type (`*define.Info`). `sv` does not run the app under Podman yet:
        // its network, as `sv` builds it, keeps a way to this computer that `sv` cannot close (backlogs 0508, 0520).
        if answer.contains("define.Info") {
            return Err(CannotRun::NoBackend {
                checked: "the `docker` command here is Podman, which `sv` does not run the app under yet; \
                          the running-app checks need Docker"
                    .to_owned(),
                unreachable: None,
            });
        }
        return Err(CannotRun::NoBackend {
            checked: format!("`docker info` failed: {}", first_line(answer)),
            unreachable: unreachable_socket(answer),
        });
    }
    match answer.trim() {
        "linux" => Ok(()),
        "windows" => Err(CannotRun::NoBackend {
            checked: "Docker here runs Windows containers, and every container `sv` starts is a \
                      Linux one; in Docker Desktop, choose \"Switch to Linux containers\""
                .to_owned(),
            unreachable: None,
        }),
        other => Err(CannotRun::NoBackend {
            checked: format!(
                "`docker info` did not say it runs Linux containers (it said `{}`)",
                first_line(other)
            ),
            unreachable: None,
        }),
    }
}

/// Docker's socket, when `docker info` was refused it (backlog 0544): the socket is there and this shell may not open
/// it, as inside an AI coding tool's sandbox, so Docker is installed and only out of reach. Docker says "permission
/// denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock" (or "to the docker API
/// at"); a macOS sandbox makes the connection fail with "operation not permitted". `None` for anything else, among it
/// "Cannot connect to the Docker daemon", where nothing is listening.
pub(crate) fn unreachable_socket(answer: &str) -> Option<String> {
    let lower = answer.to_ascii_lowercase();
    if !(lower.contains("permission denied") || lower.contains("operation not permitted")) {
        return None;
    }
    let at = ["unix://", "npipe://"]
        .iter()
        .find_map(|scheme| answer.find(scheme).map(|i| (i, scheme.len())));
    Some(match at {
        Some((i, scheme)) => {
            let rest = &answer[i + scheme..];
            let end = rest
                .find(|c: char| c == ':' || c == '"' || c.is_whitespace())
                .unwrap_or(rest.len());
            // A sentence may end on it: "at unix:///var/run/docker.sock. Is the docker daemon running?"
            match rest[..end].trim_end_matches('.') {
                "" => "Docker's socket".to_owned(),
                path => path.to_owned(),
            }
        }
        None => "Docker's socket".to_owned(),
    })
}

impl Backend for DockerBackend {
    fn name(&self) -> String {
        "Docker".to_owned()
    }

    fn available(&self) -> Result<(), CannotRun> {
        // `docker info` and not `docker --version`: the version prints happily with no daemon
        // behind it, and a backend that cannot run anything is not a backend.
        match self.docker(&["info", "--format", "{{.OSType}}"]) {
            Ok((code, answer)) => linux_containers(code, &answer),
            Err(e) => Err(CannotRun::NoBackend {
                checked: format!("`docker` could not be started: {e}"),
                unreachable: None,
            }),
        }
    }

    fn run(
        &self,
        plan: &RunPlan,
        probes: &[sv_check::probes::ProbeRequest],
    ) -> Result<RunOutcome, RunFailed> {
        // Before anything is started, so Ctrl-C from here on removes what was.
        crate::catch_interrupts();
        // Each run's request times are its own.
        self.take_request_times();
        // First, and outside the run proper, so that a run that then fails still says what it
        // removed.
        let left_over_removed = self.remove_leftovers();
        if let Ok(mut left) = self.left_behind.lock() {
            left.clear();
        }
        self.run_after_cleanup(plan, probes, left_over_removed.clone())
            .map_err(|reason| RunFailed {
                reason,
                left_over_removed,
                not_removed: self
                    .left_behind
                    .lock()
                    .map(|mut left| std::mem::take(&mut *left))
                    .unwrap_or_default(),
            })
    }
}

impl DockerBackend {
    /// The run itself, once what earlier runs left has been removed.
    fn run_after_cleanup(
        &self,
        plan: &RunPlan,
        probes: &[sv_check::probes::ProbeRequest],
        left_over_removed: Vec<String>,
    ) -> Result<RunOutcome, CannotRun> {
        let run_id = run_id(std::process::id(), next_run_number(), &crate::random_hex(4));
        if let Ok(mut run) = self.run.lock() {
            *run = Some(run_id.clone());
        }
        let network = format!("{run_id}-net");
        let app = format!("{run_id}-app");
        let sidecar = format!("{run_id}-probe");
        let mail_name = format!("{run_id}-mail");
        let provider_name = format!("{run_id}-idp");
        let browser_name = format!("{run_id}-browser");
        let model_name = format!("{run_id}-model");
        let name_server_name = format!("{run_id}-dns");
        let switched_off = format!("{run_id}-app-off");
        let installer = |e: crate::install::Ecosystem| format!("{run_id}-install-{}", e.short());
        let guard = Teardown {
            backend: self,
            run: run_id.clone(),
            network: network.clone(),
            containers: vec![
                app.clone(),
                sidecar.clone(),
                mail_name.clone(),
                provider_name.clone(),
                browser_name.clone(),
                model_name.clone(),
                name_server_name.clone(),
                switched_off.clone(),
                installer(crate::install::Ecosystem::Python),
                installer(crate::install::Ecosystem::Node),
            ],
            done: false,
        };

        // 0. The app's packages, when stackvet.toml asks for them (ADR-052): before anything else
        //    starts, in a container of their own that can reach the internet and is given only the
        //    dependency files. A refusal or a failed install stops the run before the app starts.
        let installs = if plan.install {
            crate::install::plan(&plan.app_dir, &plan.image)
                .map_err(|why| CannotRun::InstallRefused { why })?
        } else {
            Vec::new()
        };
        let mut installed = Vec::new();
        let mut container = crate::ContainerRecord::default();
        for install in &installs {
            let reused = self.install(install, &installer(install.ecosystem), &plan.image)?;
            installed.push((install.ecosystem, reused));
            container.volumes_kept.push(install.volume.clone());
        }

        // 1. The fence. Without a gateway address when the daemon allows one of the ways; a daemon that
        //    refuses them all gets the plain internal network, which the gateway check below refuses.
        let made_with = self.make_fenced_network(&network)?;

        // 1b. Do not take the flag's word for it. Asking Docker whether the network really is
        // internal costs one call and turns "we passed --internal" into "the fence is there".
        // If a future edit drops the flag, or a daemon ignores it, this stops the run before any
        // untrusted code starts, rather than running it unfenced and reporting a clean result.
        self.verify_fenced(&network)?;
        let network_made = made_with.map_or(
            "plain internal: Docker refused both ways of leaving out a gateway".to_owned(),
            |options| {
                let named: Vec<String> = options.iter().map(|o| format!("`{o}`")).collect();
                format!("with {}", named.join(" and "))
            },
        );
        // 1b'. And that nothing on it can reach the host through the bridge's gateway, which
        //      `--internal` alone leaves open.
        container.network_made = Some(network_made.clone());
        self.verify_gateway_closed(&network, made_with)
            .map_err(|e| match e {
                CannotRun::BackendFailed { detail } => CannotRun::BackendFailed {
                    detail: format!("{detail} (the network was made {network_made})"),
                },
                other => other,
            })?;

        // 1c. A mail server, when a check needs to read what the app emails. Started before the app so
        //     it is there to be sent to, on the same fenced network, and nowhere else: mail sent to it
        //     goes no further. If it cannot be started the run goes on without it, and the checks
        //     that needed it say so.
        let wants_mail = plan
            .users
            .as_ref()
            .is_some_and(|u| u.reset.is_some() || u.email_code.is_some() || u.activation.is_some());
        let mail =
            (wants_mail && self.start_mail(&network, &mail_name)).then_some(mail_name.as_str());

        // 1d. A test OpenID Connect provider, when the app signs in through another service. Also
        //     before the app, which may read the provider's details as it starts. The secret is new
        //     every run and goes only to the provider and the app.
        let client_secret = crate::random_hex(16);
        // The access token the app's MCP server is told to accept, when it takes one fixed token.
        // New every run, and given only to the app and to the questions that send it.
        let mcp_token = plan
            .mcp_server
            .as_ref()
            .and_then(|m| m.token_env.as_ref())
            .map(|_| crate::random_hex(24));
        let provider = (plan.oidc.is_some()
            && self.start_provider(&network, &provider_name, &client_secret))
        .then_some(provider_name.as_str());

        // 1d½. A test model, when the app has an AI feature to ask. Before the app, which may read
        //      the model's address as it starts.
        // The test model's server also records what a feature that fetches addresses fetches, and
        // whether the app fetches the key a sign-in token names (V9.1.3): `sv` learns whether the
        // app's tokens are JWTs only after signing in, so any run that signs in starts it.
        let model = ((plan.ai.is_some() || plan.fetch.is_some() || plan.users.is_some())
            && self.start_model(&network, &model_name))
        .then_some(model_name.as_str());

        // 1e. A headless browser, when stackvet.toml asks for checks made in one. On the same
        //     fenced network, so the pages it draws can reach nothing the app could not. If it
        //     cannot be started the checks that needed it say so.
        let wants_browser = plan.users.as_ref().is_some_and(|u| u.browser.is_some());
        let browser = (wants_browser && self.start_browser(&network, &browser_name))
            .then_some(browser_name.as_str());

        // 1f. The stand-in name server (ADR-085, backlog 0240), on the same fenced network and before
        //     the app. The app's name lookups go to it through `--dns`: it answers each with SERVFAIL,
        //     as a name is answered today, and writes each question down. Its address is read back from
        //     Docker once it runs. If it cannot be started, or its address cannot be read, the app runs
        //     as before, and the record says no names were asked.
        let name_server_dns = if self.start_name_server(&network, &name_server_name) {
            self.address_on(&name_server_name, &network)
        } else {
            None
        };
        let name_server = name_server_dns.as_ref().map(|_| name_server_name.as_str());

        // 2. The app, fenced and hardened like every helper (`app_args`).
        let mount = read_only_bind(&plan.app_dir, "/app");
        let port_env = format!("PORT={}", plan.port);
        // The installed packages, read-only, with their commands first on the PATH and Python's on
        // its import path. Set on the container, so the start command, `seed`, and the tests all
        // find them.
        let install_args: Vec<String> = if installs.is_empty() {
            Vec::new()
        } else {
            let mut out = Vec::new();
            for install in &installs {
                out.extend(["-v".to_owned(), install.app_mount()]);
            }
            if installs
                .iter()
                .any(|i| i.ecosystem == crate::install::Ecosystem::Python)
            {
                out.extend([
                    "-e".to_owned(),
                    format!("PYTHONPATH={}", crate::install::PYTHON_DEPS),
                ]);
            }
            let image_path = self.image_path(&plan.image);
            out.extend([
                "-e".to_owned(),
                format!("PATH={}", crate::install::app_path(&installs, &image_path)),
            ]);
            out
        };
        let mail_env: Vec<String> = mail
            .map(|host| {
                vec![
                    format!("SMTP_HOST={host}"),
                    format!("SMTP_PORT={SMTP_PORT}"),
                    format!("SMTP_URL=smtp://{host}:{SMTP_PORT}"),
                ]
            })
            .unwrap_or_default();
        let command = match &plan.build {
            Some(build) => format!("cd /app && {build} && {}", plan.start),
            None => format!("cd /app && {}", plan.start),
        };
        let mut args: Vec<&str> = app_args(&app, &network, &mount, &port_env);
        // The app's own name lookups go to the name server, when it runs (ADR-085).
        if let Some(address) = &name_server_dns {
            args.extend(["--dns", address.as_str()]);
        }
        args.extend(install_args.iter().map(String::as_str));
        for pair in &mail_env {
            args.extend(["-e", pair.as_str()]);
        }
        let provider_env: Vec<String> = provider
            .map(|host| {
                vec![
                    format!("OIDC_ISSUER=http://{host}:{PROVIDER_PORT}"),
                    format!("OIDC_CLIENT_ID={PROVIDER_CLIENT_ID}"),
                    format!("OIDC_CLIENT_SECRET={client_secret}"),
                ]
            })
            .unwrap_or_default();
        for pair in &provider_env {
            args.extend(["-e", pair.as_str()]);
        }
        let model_env: Vec<String> = match (model, &plan.ai) {
            (Some(host), Some(section)) => {
                model_env(host, &section.base_url_env, section.mcp_url_env.as_deref())
            }
            _ => Vec::new(),
        };
        for pair in &model_env {
            args.extend(["-e", pair.as_str()]);
        }
        let mcp_env: Vec<String> = match (
            plan.mcp_server.as_ref().and_then(|m| m.token_env.as_ref()),
            &mcp_token,
        ) {
            (Some(name), Some(token)) if sv_manifest::is_variable_name(name) => {
                vec![format!("{name}={token}")]
            }
            _ => Vec::new(),
        };
        for pair in &mcp_env {
            args.extend(["-e", pair.as_str()]);
        }
        args.extend([plan.image.as_str(), "sh", "-c", command.as_str()]);
        // Kept for the copy of the app the kill-switch check starts, which differs only in its
        // name and one more setting.
        let app_args: Vec<String> = args.iter().map(|a| (*a).to_owned()).collect();
        let (code, out) = self
            .docker(&args)
            .map_err(|e| CannotRun::BackendFailed { detail: e })?;
        if code != 0 {
            return Err(CannotRun::BackendFailed {
                detail: first_line(&out),
            });
        }

        // 3. Ready, judged from inside the fence, by the sidecar every request goes through. If it
        //    cannot be started, each request starts a container of its own instead, as before:
        //    slower, and the same answers.
        let via = if self.start_sidecar(&network, &sidecar, plan) {
            Via::Sidecar(&sidecar)
        } else {
            Via::FreshContainer(&network)
        };
        let ready = self.wait_until_ready(&via, &app, plan);
        let healthy = ready.answered;
        container.ready_after_seconds = healthy.then_some(ready.after_seconds);
        // The browser reaches the app at http://localhost:<port>, as a person running it on their
        // own computer would: an app that trusts its own origin for forms trusts that one, and a
        // browser treats localhost as secure, so `Secure` cookies work without HTTPS. A forwarder
        // inside the browser's container carries it across. Without it there is no browser.
        let browser = browser.filter(|name| healthy && self.forward_browser(name, &app, plan.port));
        if !healthy {
            let logs = self
                .docker(&["logs", "--tail", "20", &app])
                .map(|(_, o)| o)
                .unwrap_or_default();
            // Whether the app had anything to start: the same folder, mounted the same way into a
            // container of the small image the probes already use, listed. Asked only here, so a
            // run that works pays nothing for it.
            let inside = self
                .docker(&[
                    "run",
                    "--rm",
                    "--network",
                    "none",
                    "--mount",
                    &mount,
                    PROBE_IMAGE,
                    "ls",
                    "-A",
                    "/app",
                ])
                .ok()
                .filter(|(code, _)| *code == 0)
                .map(|(_, listed)| listed);
            drop(guard);
            if let Some(unseen) = crate::unseen_folder(&plan.app_dir, inside.as_deref()) {
                return Err(unseen);
            }
            let (detail, crashed) = never_ready_detail(&logs, plan.build.as_deref());
            return Err(CannotRun::NeverReady {
                waited_seconds: ready.after_seconds,
                detail,
                loopback: crate::loopback_named_in(&plan.start),
                crashed,
                exited: ready.exited,
            });
        }

        // 4. The script (`sv_check::script`): the anonymous questions, the signed-in suites, the
        //    sign-in through the test provider, the app as an MCP server, the fetch, the AI feature
        //    with its kill switch, and whether the app is still up; in that order, written against
        //    what only this harness can do (`DockerRun`). Since 8 October 2026; before, it was here.
        let accounts = plan.users.as_ref().map(crate::accounts_for);
        let harness = DockerRun {
            backend: self,
            via: &via,
            app: &app,
            switched_off: &switched_off,
            app_args: &app_args,
            plan,
            mail,
            provider,
            browser,
            model,
            model_up: std::sync::OnceLock::new(),
            provider_up: std::sync::OnceLock::new(),
        };
        let sv_check::script::Outcome {
            probe_responses,
            probes_rate_limited,
            mut signed_in,
            oidc,
            mcp_server,
            fetch,
            mut ai,
            liveness,
            timings: mut suite_timings,
        } = sv_check::script::run(
            &harness,
            &sv_check::script::Plan {
                users: plan.users.as_ref(),
                policy: &plan.policy,
                oidc: plan.oidc.as_ref(),
                ai: plan.ai.as_ref(),
                mcp_server: plan.mcp_server.as_ref(),
                fetch: plan.fetch.as_ref(),
                health_path: &plan.health_path,
                slow: plan.slow,
                accounts: accounts.as_ref(),
                mcp_token: mcp_token.as_deref(),
            },
            probes,
        );

        // What the stand-ins received, read before they go (ADR-082), with the run's test secrets
        // blanked by value.
        let mut secrets = accounts.as_ref().map(test_secrets).unwrap_or_default();
        secrets.push(client_secret.clone());
        secrets.extend(mcp_token.clone());
        let stand_ins = self.stand_ins(&via, mail, provider, model, name_server, &secrets);
        // What the engine says of the app's own container, read while it still runs (backlog 0243). Nothing
        // after this point sends a request, so these are the app's numbers as the probes left it.
        let resources = self.resource_readings(&app);
        // And from the lines of the app's own output the log checks read, kept the same way: an
        // app that logs a sign-in with its password logs the test account's.
        if let Some(asked) = signed_in.as_mut() {
            crate::stand_ins::blank_log(asked, &secrets);
        }
        // The AI suite's the same way (backlog 0381): its exchanges are kept now, and the account it signs in as is
        // the test account.
        if let Some(asked) = ai.as_mut() {
            crate::stand_ins::blank_log(asked, &secrets);
        }

        // Nothing after this point sends a request, so the sidecar goes now rather than waiting on
        // the tests, which can take as long as they like. The mail server with it: nothing reads it
        // after the probes.
        self.end_session();
        let mut helpers = vec![&sidecar];
        helpers.extend(mail.is_some().then_some(&mail_name));
        helpers.extend(provider.is_some().then_some(&provider_name));
        helpers.extend(browser.is_some().then_some(&browser_name));
        helpers.extend(model.is_some().then_some(&model_name));
        helpers.extend(name_server.is_some().then_some(&name_server_name));
        for helper in helpers {
            container.not_removed.extend(not_removed(
                "container",
                helper,
                self.docker(&["rm", "-f", helper]),
            ));
        }

        // 5. The declared tests, inside the app container so they see what the app sees.
        let tests_began = std::time::Instant::now();
        let tests = plan.test.as_ref().and_then(|test_command| {
            if let Some(crate::OnStep(say)) = plan.on_step {
                say("the app's own tests");
            }
            // A report left over from a previous run — committed into the repository, or baked into
            // the image — would be read as this run's result and credit tests that never ran here.
            // So it is removed first, and after the run the file must be there or nothing is read.
            // This is the same rule as a missing tool in the adapters: absent never reads as clean.
            let report_path = plan.test_report.as_ref().map(|p| crate::report_path(p));
            let removed_stale = report_path.as_ref().map(|path| {
                self.docker(&["exec", &app, "sh", "-c", &format!("rm -f -- '{path}'")])
                    .map(|(code, _)| code == 0)
                    .unwrap_or(false)
            });
            // At most `TEST_LIMIT`: a suite that hangs would otherwise hang the whole run. Stopping
            // the `docker exec` leaves the suite running in the app's container, which the
            // teardown then removes.
            let ran = crate::run_bounded(
                Command::new(&self.binary).args(["exec", &app, "sh", "-c", test_command]),
                plan.test_limit,
                false,
            )
            .ok()?;
            if ran.stopped {
                return Some(TestResult {
                    exit_code: ran.code,
                    output: ran.text,
                    report: None,
                    report_note: None,
                    stopped_after: Some(plan.test_limit),
                });
            }
            let (exit_code, output) = (ran.code, ran.text);
            let (report, report_note) = match (&report_path, removed_stale) {
                (None, _) => (
                    None,
                    Some(
                        "stackvet.toml declares no test-report, so only the exit code is known and a suite with one failing test credits nothing"
                            .to_owned(),
                    ),
                ),
                (Some(path), Some(false)) => (
                    None,
                    Some(format!(
                        "a report left over from an earlier run could not be removed from {path}, so anything found there now cannot be trusted to be this run's"
                    )),
                ),
                (Some(path), _) => match self.docker(&["exec", &app, "cat", "--", path]) {
                    Ok((0, xml)) if !xml.trim().is_empty() => (Some(xml), None),
                    Ok((0, _)) => (
                        None,
                        Some(format!("the test runner wrote nothing to {path}")),
                    ),
                    _ => (
                        None,
                        Some(format!(
                            "the test runner wrote no report to {path}; only the exit code is known"
                        )),
                    ),
                },
            };
            Some(TestResult {
                exit_code,
                output,
                report,
                report_note,
                stopped_after: None,
            })
        });
        if plan.test.is_some() {
            let took = tests_began.elapsed().as_millis();
            suite_timings.push((
                "the app's own tests",
                u64::try_from(took).unwrap_or(u64::MAX),
            ));
        }

        // The app's own word on an answer that came short, read while its container is still there (backlog 0598).
        let short_answers = self.short_answers();
        let short_answers_log = if short_answers.is_empty() {
            None
        } else {
            content_length_line(&self.app_log(&app))
        };
        for left in guard.finish() {
            if !container.not_removed.contains(&left) {
                container.not_removed.push(left);
            }
        }
        Ok(RunOutcome {
            healthy,
            tests,
            fence: Fence::DockerInternalNetwork,
            probe_responses,
            probes_rate_limited,
            signed_in,
            oidc,
            ai,
            mcp_server,
            fetch,
            left_over_removed,
            liveness,
            installed,
            sidecar_lost: self.sidecar_lost(),
            short_answers,
            short_answers_log,
            container,
            stand_ins,
            resources,
            suite_timings,
            request_timings: self.take_request_times(),
        })
    }
}

/// What only this harness can do for the run's script (`sv_check::script::Services`): a way to the
/// app with the helpers the run has, the helpers' readiness, the app's log, the seed, the second
/// copy of the app for the kill-switch check, and a look at whether the app is still up. Each is
/// a Docker call; the order they are made in is the script's.
struct DockerRun<'a> {
    backend: &'a DockerBackend,
    via: &'a Via<'a>,
    app: &'a str,
    /// The name the kill-switch copy is started under.
    switched_off: &'a str,
    /// The app's own arguments, which the copy differs from only in its name and one setting.
    app_args: &'a [String],
    plan: &'a RunPlan,
    mail: Option<&'a str>,
    provider: Option<&'a str>,
    browser: Option<&'a str>,
    model: Option<&'a str>,
    /// Whether the test model answered, asked once for the run (backlog 0369): asked on each of the script's calls, a
    /// model that never came up cost ten seconds of waiting every time. The AI suite asks the model itself before it
    /// relies on it, so one that stops answering later is still said.
    model_up: std::sync::OnceLock<bool>,
    /// The same, for the test sign-in provider.
    provider_up: std::sync::OnceLock<bool>,
}

impl DockerRun<'_> {
    fn model_answering(&self) -> Option<&str> {
        self.model.filter(|host| {
            *self
                .model_up
                .get_or_init(|| self.backend.model_ready(self.via, host))
        })
    }

    fn provider_answering(&self) -> Option<&str> {
        self.provider.filter(|host| {
            *self
                .provider_up
                .get_or_init(|| self.backend.provider_ready(self.via, host))
        })
    }
}

impl sv_check::script::Services for DockerRun<'_> {
    fn http<'s>(
        &'s self,
        target: sv_check::script::Target,
        with: sv_check::script::With,
    ) -> Box<dyn sv_check::signed_in::Http + 's> {
        use sv_check::script::{Model, Target};
        Box::new(DockerHttp {
            backend: self.backend,
            via: self.via,
            app: match target {
                Target::App => self.app,
                Target::SwitchedOff => self.switched_off,
            },
            port: self.plan.port,
            mail: self.mail.filter(|_| with.mail),
            provider: if with.provider {
                self.provider_answering()
            } else {
                None
            },
            browser: self.browser.filter(|_| with.browser),
            model: match with.model {
                Model::None => None,
                Model::IfAnswering => self.model_answering(),
                Model::AsStarted => self.model,
            },
            on_step: self.plan.on_step,
        })
    }

    fn model_canary(&self) -> Option<String> {
        self.model_answering()
            .map(|host| format!("http://{host}:{MODEL_PORT}"))
    }

    fn app_log(&self) -> String {
        self.backend.app_log(self.app)
    }

    fn seed(
        &self,
        target: sv_check::script::Target,
        seed: &str,
        accounts: &sv_check::signed_in::Accounts,
    ) -> Result<std::collections::BTreeMap<String, String>, String> {
        let container = match target {
            sv_check::script::Target::App => self.app,
            sv_check::script::Target::SwitchedOff => self.switched_off,
        };
        self.backend.seed(container, seed, accounts)
    }

    fn start_switched_off(&self, setting: &str) -> bool {
        self.backend.start_switched_off(
            self.app_args,
            self.app,
            self.switched_off,
            &self.plan.image,
            setting,
        ) && self
            .backend
            .wait_until_ready(self.via, self.switched_off, self.plan)
            .answered
    }

    fn remove_switched_off(&self) {
        let _ = self.backend.docker(&["rm", "-f", self.switched_off]);
    }

    fn liveness(&self, after: &str) -> sv_check::running::Liveness {
        self.backend.liveness(self.via, self.app, self.plan, after)
    }
    fn starting(&self, suite: &str) {
        if let Some(crate::OnStep(say)) = self.plan.on_step {
            say(suite);
        }
    }
}

/// Requests to the app, made from the sidecar on the fenced network — the same way the anonymous
/// probes are made, so signing in happens inside the fence too.
struct DockerHttp<'a> {
    backend: &'a DockerBackend,
    via: &'a Via<'a>,
    app: &'a str,
    port: u16,
    /// The mail server's name on the fenced network, when the run has one.
    mail: Option<&'a str>,
    /// The test provider's name on the fenced network, when the run has one that answered.
    provider: Option<&'a str>,
    /// The headless browser's container, when the run has one.
    browser: Option<&'a str>,
    /// The test model's name on the fenced network, when the run has one that answered.
    model: Option<&'a str>,
    /// Where a long wait says how far it is: the run's own `on_step`, none in a test (backlog 0247).
    on_step: Option<crate::OnStep>,
}

/// Now, in seconds since 1970, by this computer's clock.
fn host_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// The containers' clock against this computer's, from a reading taken between `before` and `after`: the
/// difference from the middle of the two, and none when it is a second or less, which is the reading's
/// own uncertainty.
fn offset_from(before: u64, theirs: u64, after: u64) -> i64 {
    let middle = (i128::from(before) + i128::from(after)) / 2;
    let offset = i128::from(theirs) - middle;
    if offset.abs() <= 1 {
        0
    } else {
        i64::try_from(offset).unwrap_or(0)
    }
}

impl sv_check::signed_in::Http for DockerHttp<'_> {
    /// Now by the clock the app reads: this computer's, moved by how far the containers' differs, so a
    /// two-factor code is made for the time the app checks it against.
    fn now(&mut self) -> u64 {
        host_now().saturating_add_signed(self.backend.clock_offset(self.via))
    }

    /// The waits `--slow` makes can last an hour and a half, so they are taken in short steps that
    /// end at Ctrl-C, when the run goes on to remove its containers instead of waiting them out.
    fn wait(&mut self, seconds: u64) {
        let until = std::time::Instant::now() + Duration::from_secs(seconds);
        while !crate::interrupted() {
            let left = until.saturating_duration_since(std::time::Instant::now());
            if left.is_zero() {
                break;
            }
            std::thread::sleep(left.min(Duration::from_millis(200)));
        }
    }

    fn say(&mut self, line: &str) {
        if let Some(crate::OnStep(say)) = self.on_step {
            say(line);
        }
    }

    fn send(
        &mut self,
        request: &sv_check::probes::ProbeRequest,
    ) -> Option<sv_check::probes::ProbeResponse> {
        self.backend.probe(self.via, self.app, self.port, request)
    }

    fn send_together(
        &mut self,
        requests: &[sv_check::probes::ProbeRequest],
    ) -> Option<Vec<Option<sv_check::probes::ProbeResponse>>> {
        self.backend
            .probe_together(self.via, self.app, self.port, requests)
    }

    fn send_in_turn(
        &mut self,
        requests: &[sv_check::probes::ProbeRequest],
    ) -> Option<Vec<Option<sv_check::probes::ProbeResponse>>> {
        self.backend
            .probe_in_turn(self.via, self.app, self.port, requests)
    }

    fn provider(
        &mut self,
        request: &sv_check::probes::ProbeRequest,
    ) -> Option<sv_check::probes::ProbeResponse> {
        let host = self.provider?;
        self.backend.probe(self.via, host, PROVIDER_PORT, request)
    }

    fn model(
        &mut self,
        request: &sv_check::probes::ProbeRequest,
    ) -> Option<sv_check::probes::ProbeResponse> {
        let host = self.model?;
        self.backend.probe(self.via, host, MODEL_PORT, request)
    }

    fn model_address(&mut self) -> Option<String> {
        self.model.map(|host| format!("http://{host}:{MODEL_PORT}"))
    }

    fn browser(&mut self, job: &sv_check::browser::Job) -> Option<Vec<serde_json::Value>> {
        let container = self.browser?;
        let job = serde_json::json!({
            "app": format!("http://localhost:{}", self.port),
            "actions": job.actions.iter().map(|a| a.to_json()).collect::<Vec<_>>(),
        });
        // Handed over in base64, so that nothing in the job can end the shell command it travels in.
        let env = format!(
            "SV_JOB={}",
            sv_check::encoding::base64(
                job.to_string().as_bytes(),
                sv_check::encoding::Alphabet::Standard,
                true,
            )
        );
        let network = format!("container:{container}");
        let (code, out) = self.backend.docker(&driver_args(&network, &env)).ok()?;
        if code != 0 {
            return None;
        }
        // The driver prints one line of JSON last; anything Node said before it is not the answer.
        let line = out.lines().rev().find(|l| l.starts_with('['))?;
        serde_json::from_str::<Vec<serde_json::Value>>(line).ok()
    }

    fn mail(&mut self, to: &str, at_least: usize) -> Option<Vec<String>> {
        let host = self.mail?;
        let deadline =
            std::time::Instant::now() + std::time::Duration::from_secs(MAIL_WAIT_SECONDS);
        let ids = loop {
            // A mail server that does not answer is not an empty mailbox: the check is told there is
            // nothing to read, not that nothing was sent.
            let listing =
                self.backend
                    .fetch(self.via, host, MAIL_API_PORT, "/api/v1/messages?limit=500")?;
            let ids = mail_ids_to(&listing, to)?;
            if ids.len() >= at_least || std::time::Instant::now() >= deadline {
                break ids;
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        };
        Some(
            ids.iter()
                .filter_map(|id| {
                    let path = format!("/api/v1/message/{id}");
                    let message = self.backend.fetch(self.via, host, MAIL_API_PORT, &path)?;
                    mail_text(&message)
                })
                .collect(),
        )
    }
}

/// What an app that never answered last said, and, when it had a build step, why such a step so
/// often fails here. The step runs inside the fence like the app, where nothing can be downloaded
/// and nothing outside `/tmp` written, so one that installs packages fails whatever it prints:
/// `pip install` says its package folder "is not writeable", and before the app ran read-only it
/// said the network was unreachable. Neither line names the cause, so this does.
fn never_ready_detail(logs: &str, build: Option<&str>) -> (String, bool) {
    // An app that crashed says what went wrong in its error line, which a Python traceback writes
    // last; one that is still running, or waiting, says the most in its last line (the review of
    // the loop's item 6, 6 October 2026: three crashes were quoted as "Traceback (most recent call
    // last):").
    let crash = crash_line(logs);
    let last = match &crash {
        Some(line) => format!("It stopped with an error: {line}"),
        None => format!("Its last output was: {}", last_line(logs)),
    };
    let crashed = crash.is_some();
    let detail = match build {
        Some(step) => format!(
            "{last} Its build step (`{step}`) ran inside the fence, where nothing can be \
             downloaded and the file system is read-only apart from /tmp, so a step that installs \
             packages cannot work there: set `install = true` under `[stack.run]` to have `sv` \
             install them before the run, or install them into the image."
        ),
        None => last,
    };
    (detail, crashed)
}

/// The line that says why the app stopped, when its output shows it crashed: the last line that
/// names an error (`KeyError: 'PORT'`, `Error: Cannot find module 'express'`, `panic: …`,
/// `thread 'main' panicked at …`, Ruby's `… (NameError)`), read from the end past stack frames and
/// what a runtime prints after them. `None` when nothing in the output reads as a crash.
fn crash_line(logs: &str) -> Option<String> {
    logs.lines()
        .map(str::trim)
        .rev()
        .filter(|l| !l.is_empty())
        .find(|l| names_an_error(l))
        .map(str::to_owned)
}

/// Whether a line of output names an error: it starts with an error's name (`KeyError:`,
/// `sqlite3.OperationalError:`, `Error:`, `TypeError [ERR_…]:`), or with Go's or Rust's panic, or ends
/// with one in brackets, as Ruby writes it. An error's name ends in `Error`, `Exception`, or `Exit`, and
/// its last part is capitalized, so `server.onExit:` is not one. Read by hand rather than with a pattern library, which
/// `sv-run` does not otherwise need.
fn names_an_error(line: &str) -> bool {
    let is_error_name = |name: &str| {
        ["Error", "Exception", "Exit"]
            .iter()
            .any(|end| name.ends_with(end))
            && name
                .rsplit(['.', ':'])
                .next()
                .and_then(|last| last.chars().next())
                .is_some_and(|c| c.is_ascii_uppercase())
    };
    let first: String = line
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '$'))
        .collect();
    let after = line[first.len()..].chars().next();
    if is_error_name(&first) && matches!(after, None | Some(':' | ' ' | '[')) {
        return true;
    }
    if line.starts_with("panic:")
        || (line.starts_with("thread '") && line.contains("' panicked"))
        || line.starts_with("Uncaught ")
        || line.to_ascii_lowercase().starts_with("fatal error")
    {
        return true;
    }
    line.strip_suffix(')')
        .and_then(|l| l.rsplit_once('('))
        .is_some_and(|(_, inner)| {
            inner
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == ':' || c == '_')
                && is_error_name(inner.rsplit("::").next().unwrap_or(inner))
        })
}

/// The last line of the app's output that says anything.
fn last_line(text: &str) -> String {
    text.lines()
        .map(str::trim)
        .rev()
        .find(|l| !l.is_empty())
        .unwrap_or("no detail")
        .to_owned()
}

impl DockerBackend {
    /// What each stand-in that ran received, read from it while it is still up.
    fn stand_ins(
        &self,
        via: &Via,
        mail: Option<&str>,
        provider: Option<&str>,
        model: Option<&str>,
        name_server: Option<&str>,
        secrets: &[String],
    ) -> crate::stand_ins::StandIns {
        use crate::stand_ins::{StandIns, mail_of, read_json};
        let mut out = StandIns::default();
        // The name server writes each question to its output, which `docker logs` keeps until the
        // container is removed. Read before it goes, like the others (ADR-085).
        if let Some(name) = name_server {
            out.names = self
                .docker(&["logs", name])
                .ok()
                .filter(|(code, _)| *code == 0)
                .map(|(_, log)| crate::name_server::parse_log(&log));
            if out.names.is_none() {
                out.unread.push("the name server");
            }
        }
        if let Some(host) = model {
            out.model = self
                .fetch(via, host, MODEL_PORT, "/_sv/seen")
                .and_then(|text| read_json(&text, secrets));
            if out.model.is_none() {
                out.unread.push("the test model");
            }
        }
        if let Some(host) = provider {
            out.provider = self
                .fetch(via, host, PROVIDER_PORT, "/_sv/requests")
                .and_then(|text| read_json(&text, secrets));
            if out.provider.is_none() {
                out.unread.push("the test sign-in provider");
            }
        }
        if let Some(host) = mail {
            out.mail = self
                .fetch(via, host, MAIL_API_PORT, "/api/v1/messages?limit=500")
                .and_then(|text| mail_of(&text, secrets));
            if out.mail.is_none() {
                out.unread.push("the mail catcher");
            }
        }
        out
    }
}

impl DockerBackend {
    /// Runs the owner's `seed` command inside the app's container, with the run's accounts in its
    /// environment; or says, for the report, why it could not.
    fn seed(
        &self,
        app: &str,
        seed: &str,
        accounts: &sv_check::signed_in::Accounts,
    ) -> Result<std::collections::BTreeMap<String, String>, String> {
        let env = seed_env(accounts);
        let names: Vec<&str> = env.iter().map(|(k, _)| *k).collect();
        let args = seed_args(app, seed, &names);
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        match self.docker_with_secrets(&args, &env) {
            // What it printed as `NAME=value` lines, for the manifest's `{seed:NAME}` (backlog 0536, ADR-124).
            Ok((0, out)) => Ok(seed_values(&out)),
            Ok((code, out)) => Err(seed_failed(code, &out, accounts)),
            Err(e) => Err(format!("The seed command could not be started: {e}.")),
        }
    }

    /// Whether the test model answers yet, for the same reason as the provider below.
    fn model_ready(&self, via: &Via, host: &str) -> bool {
        let health = sv_check::probes::ProbeRequest {
            id: "model-health".to_owned(),
            method: "GET".to_owned(),
            path: sv_check::stand_in::HEALTH.to_owned(),
            headers: Vec::new(),
            body: None,
        };
        (0..20).any(|_| {
            let up = self
                .probe(via, host, MODEL_PORT, &health)
                .is_some_and(|r| r.status == 200);
            if !up {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            up
        })
    }

    /// Whether the provider answers yet: Node takes a moment, and a check that starts before it
    /// is up would report the app's sign-in as broken when the provider was.
    fn provider_ready(&self, via: &Via, host: &str) -> bool {
        let health = sv_check::probes::ProbeRequest {
            id: "provider-health".to_owned(),
            method: "GET".to_owned(),
            path: sv_check::stand_in::HEALTH.to_owned(),
            headers: Vec::new(),
            body: None,
        };
        for _ in 0..20 {
            if self
                .probe(via, host, PROVIDER_PORT, &health)
                .is_some_and(|r| r.status == 200)
            {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        false
    }

    /// Runs a command where requests to the app are made from: in the sidecar, or in a throw-away
    /// container on the same internal network when there is no sidecar.
    fn inside_fence(&self, via: &Via, command: &[&str]) -> Result<(i32, String), String> {
        self.inside_fence_with_input(via, command, &[])
    }

    /// `inside_fence`, with `input` given to the command as what it reads. How a request reaches
    /// the container: an argument cannot carry one, since Linux refuses any single argument over
    /// 128 KiB, which an upload passes easily.
    fn inside_fence_with_input(
        &self,
        via: &Via,
        command: &[&str],
        input: &[u8],
    ) -> Result<(i32, String), String> {
        let mut args = self.fence_args(via);
        args.extend_from_slice(command);
        let mut c = Command::new(&self.binary);
        c.args(self.prepared(&args));
        crate::output_with_input(&mut c, input)
    }

    /// How a container that talks to the app is started, either way. Separate so the two paths can
    /// be compared without starting anything.
    fn fence_args<'a>(&self, via: &Via<'a>) -> Vec<&'a str> {
        match via {
            Via::Sidecar(name) => vec!["exec", "-i", name],
            // Hardened and limited by `prepared`, like the sidecar. Before 8 October 2026 each path
            // carried its own copy of the flags, and the fallback once had none of them: they were
            // added where the fast path was written and not where the fallback already lived, so
            // a run that could not start a sidecar quietly made every request from a container with
            // its capabilities and a writable file system.
            Via::FreshContainer(network) => vec![
                "run",
                "-i",
                "--rm",
                "--network",
                network,
                "--tmpfs",
                PROBE_TMPFS,
                PROBE_IMAGE,
            ],
        }
    }

    /// Whether the app is still running and answering, after `after`.
    ///
    /// Read from `docker inspect` and one request to the health path, tried three times two seconds
    /// apart so a moment of slowness is not taken for a stopped app.
    fn liveness(
        &self,
        via: &Via,
        app: &str,
        plan: &RunPlan,
        after: &str,
    ) -> sv_check::running::Liveness {
        let state = self
            .docker(&[
                "inspect",
                "-f",
                "{{.State.Status}} {{.RestartCount}} {{.State.ExitCode}} {{.State.OOMKilled}}",
                app,
            ])
            .ok()
            .filter(|(code, _)| *code == 0)
            .map(|(_, out)| out)
            .unwrap_or_default();
        let words: Vec<&str> = state.split_whitespace().collect();
        let (status, restarts, exit_code, out_of_memory) = match words.as_slice() {
            [status, restarts, exit_code, oom] => (
                (*status).to_owned(),
                restarts.parse().unwrap_or(0),
                exit_code.parse().unwrap_or(0),
                *oom == "true",
            ),
            _ => (String::new(), 0, 0, false),
        };
        let url = format!("http://{app}:{}{}", plan.port, plan.health_path);
        let answered = status == "running"
            && (0..3).any(|attempt| {
                if attempt > 0 {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
                matches!(
                    self.inside_fence(via, &["wget", "-q", "-T", "5", "-O", "/dev/null", &url]),
                    Ok((0, _))
                )
            });
        sv_check::running::Liveness {
            after: after.to_owned(),
            status,
            restarts,
            exit_code,
            out_of_memory,
            answered,
        }
    }

    /// Polls the health path from inside the fence.
    ///
    /// This is the part that could not be done from the host. An `--internal` network is
    /// unreachable from this computer whether or not a port is published, so the probe has to live
    /// inside the fence with the app.
    fn wait_until_ready(&self, via: &Via, app: &str, plan: &RunPlan) -> Readiness {
        let url = format!("http://{app}:{}{}", plan.port, plan.health_path);
        let started = std::time::Instant::now();
        let deadline = started + std::time::Duration::from_secs(READY_TIMEOUT_SECONDS);
        let ended = |answered, exited| Readiness {
            answered,
            after_seconds: started.elapsed().as_secs(),
            exited,
        };
        let mut tries = 0;
        while std::time::Instant::now() < deadline {
            // If the app has already given up, waiting the full minute tells nobody anything; how
            // it ended is kept, so the person is told the wait was short and why (backlog 226,
            // part 2, item 18).
            if let Ok((_, state)) = self.docker(&[
                "inspect",
                "-f",
                "{{.State.Status}} {{.State.ExitCode}} {{.State.OOMKilled}}",
                app,
            ]) && let Some(exited) = exited_from(&state)
            {
                return ended(false, Some(exited));
            }
            if let Ok((0, _)) =
                self.inside_fence(via, &["wget", "-q", "-T", "3", "-O", "/dev/null", &url])
            {
                return ended(true, None);
            }
            std::thread::sleep(ready_pause(tries));
            tries += 1;
        }
        ended(false, None)
    }
}

/// The pause after the readiness try numbered `tries` (from 0): a quarter of a second, doubling to two
/// seconds. A fixed two seconds saw an app up in one second at about four (backlog 0355); the same
/// checks, sooner, and no more often than every two seconds once an app is slow to start.
fn ready_pause(tries: u32) -> std::time::Duration {
    std::time::Duration::from_millis((250u64 << tries.min(3)).min(2_000))
}

/// How long the sidecar may live: the usual limit, and with `--slow` long enough to wait out the
/// timeouts the owner states and the ten minutes an emailed sign-in code is kept.
fn sidecar_seconds(plan: &RunPlan) -> u64 {
    // The AI feature's rate check waits a minute before its burst, whether or not the run is slow.
    let rate = if plan.ai.is_some() && plan.policy.ai_requests_per_minute.is_some() {
        120
    } else {
        0
    };
    if !plan.slow {
        return SIDECAR_SECONDS + rate;
    }
    let minutes = plan.policy.idle_timeout_minutes.unwrap_or(0)
        + plan.policy.session_lifetime_minutes.unwrap_or(0);
    // An emailed sign-in code is kept ten minutes before it is used.
    let code_minutes = if plan.users.as_ref().is_some_and(|u| u.email_code.is_some()) {
        12
    } else {
        0
    };
    SIDECAR_SECONDS + u64::from(minutes.min(180) + code_minutes) * 60 + 120 + rate
}

/// Where requests to the app are sent from.
enum Via<'a> {
    /// The run's sidecar, by name: each request is an `exec` into it.
    Sidecar(&'a str),
    /// A container started for the one request, on this network, when there is no sidecar.
    FreshContainer(&'a str),
}

/// A run's name: `sv-<process>-<run in this process>-<random>`. The process id alone repeats
/// between two copies of `sv` in two containers sharing one Docker daemon (each is often process 1
/// or 7 in its own container), and then each run's teardown removed the other's containers by
/// name (S10). The random part makes the names, and so the label, this run's alone.
fn run_id(process: u32, number: u64, random: &str) -> String {
    format!("sv-{process}-{number}-{random}")
}

/// Removes the containers and the network however the run ended, including on an early return.
///
/// It removes what carries this run's label, by id, and nothing else. Only if Docker will not list
/// them does it fall back to the run's own names, which hold its random part.
struct Teardown<'a> {
    backend: &'a DockerBackend,
    run: String,
    network: String,
    containers: Vec<String>,
    /// Set once the removal has run, so dropping after `finish` does not run it again.
    done: bool,
}

impl Teardown<'_> {
    /// Removes what the run started, now, and names what could not be removed (backlog 226, part
    /// 2, item 18): until 10 October 2026 each failure was dropped.
    fn finish(mut self) -> Vec<String> {
        self.remove()
    }

    fn remove(&mut self) -> Vec<String> {
        self.done = true;
        let mut left = Vec::new();
        let filter = format!("label={RUN_LABEL}={}", self.run);
        let listed = |args: &[&str]| match self.backend.docker_cleanup(args) {
            Ok((0, out)) => Some(out),
            _ => None,
        };
        let containers = listed(&["ps", "-aq", "--filter", &filter]);
        for container in to_remove(containers.as_deref(), &self.containers) {
            left.extend(not_removed(
                "container",
                &container,
                self.backend.docker_cleanup(&["rm", "-f", &container]),
            ));
        }
        let networks = listed(&["network", "ls", "-q", "--filter", &filter]);
        for network in to_remove(networks.as_deref(), std::slice::from_ref(&self.network)) {
            left.extend(not_removed(
                "network",
                &network,
                self.backend.docker_cleanup(&["network", "rm", &network]),
            ));
        }
        if let Ok(mut run) = self.backend.run.lock()
            && run.as_deref() == Some(self.run.as_str())
        {
            *run = None;
        }
        left
    }
}

impl Drop for Teardown<'_> {
    /// The teardown of a run that ended early, on a failure or a panic: what it could not remove is
    /// kept on the backend for the failure to say.
    fn drop(&mut self) {
        if !self.done {
            let left = self.remove();
            if let Ok(mut kept) = self.backend.left_behind.lock() {
                *kept = left;
            }
        }
    }
}

/// "container <name> (what Docker said)" when removing it failed, or `None` when it went, or was
/// already gone: a helper that never started is not one left behind.
fn not_removed(kind: &str, name: &str, removed: Result<(i32, String), String>) -> Option<String> {
    let said = match removed {
        Ok((0, _)) => return None,
        Ok((_, out)) => first_line(&out),
        Err(e) => first_line(&e),
    };
    if said.contains("No such") || said.contains("not found") {
        return None;
    }
    Some(format!("{kind} {name} ({said})"))
}

/// What a teardown removes: the ids Docker listed under this run's label, or, when it would not
/// list them, the names this run gave.
fn to_remove(listed: Option<&str>, names: &[String]) -> Vec<String> {
    match listed {
        Some(listed) => listed
            .lines()
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .map(str::to_owned)
            .collect(),
        None => names.to_vec(),
    }
}

/// How the test provider is started, and the secret Docker hands it from its own environment: the
/// client secret by name only on the command line, as the seed's passwords are.
fn provider_start(
    network: &str,
    name: &str,
    secret: &str,
) -> (Vec<String>, Vec<(&'static str, String)>) {
    let issuer = format!("ISSUER=http://{name}:{PROVIDER_PORT}");
    let port = format!("PORT={PROVIDER_PORT}");
    let client = format!("CLIENT_ID={PROVIDER_CLIENT_ID}");
    let args = helper_args(
        network,
        name,
        &[&issuer, &port, &client, "CLIENT_SECRET"],
        PROVIDER_SCRIPT,
    )
    .into_iter()
    .map(str::to_owned)
    .collect();
    (args, vec![("CLIENT_SECRET", secret.to_owned())])
}

/// `docker exec` for the owner's `seed`, naming each of `names` with `-e` and no value, so the values
/// come from Docker's own environment (`docker_with_secrets`) and never stand on its command line.
fn seed_args(app: &str, seed: &str, names: &[&str]) -> Vec<String> {
    let mut args: Vec<String> = vec!["exec".into()];
    for name in names {
        args.push("-e".into());
        args.push((*name).to_owned());
    }
    args.extend([
        app.to_owned(),
        "sh".into(),
        "-c".into(),
        format!("cd /app && {seed}"),
    ]);
    args
}

/// What `seed` is given: the run's accounts, their passwords, and the two-factor secrets in base32.
fn seed_env(accounts: &sv_check::signed_in::Accounts) -> Vec<(&'static str, String)> {
    let mut env = vec![
        ("SV_USER_A", accounts.a.user.clone()),
        ("SV_PASSWORD_A", accounts.a.password.clone()),
        ("SV_USER_B", accounts.b.user.clone()),
        ("SV_PASSWORD_B", accounts.b.password.clone()),
    ];
    if let Some(admin) = &accounts.admin {
        env.push(("SV_ADMIN", admin.user.clone()));
        env.push(("SV_ADMIN_PASSWORD", admin.password.clone()));
    }
    if let Some(totp) = &accounts.totp {
        env.push(("SV_USER_TOTP", totp.account.user.clone()));
        env.push(("SV_PASSWORD_TOTP", totp.account.password.clone()));
        env.push(("SV_TOTP_SECRET", sv_check::totp::base32(&totp.secret)));
    }
    if let Some(secret) = &accounts.admin_totp_secret {
        env.push(("SV_ADMIN_TOTP_SECRET", sv_check::totp::base32(secret)));
    }
    env
}

/// What the report says when `seed` failed: its exit code and the first line it wrote, with every
/// password and two-factor secret the run gave it taken out. A seed that echoes its environment, or
/// a stack trace that prints the value it choked on, would otherwise carry them into the report.
/// Every test secret of `sv`'s that `accounts` holds, as the text it could appear as: each password,
/// and each two-factor secret in base32, upper and lower case.
pub(crate) fn test_secrets(accounts: &sv_check::signed_in::Accounts) -> Vec<String> {
    let mut secrets: Vec<String> = [&accounts.a, &accounts.b]
        .into_iter()
        .chain(accounts.admin.as_ref())
        .chain(accounts.totp.as_ref().map(|t| &t.account))
        .map(|account| account.password.clone())
        .collect();
    for secret in accounts
        .totp
        .as_ref()
        .map(|t| &t.secret)
        .into_iter()
        .chain(accounts.admin_totp_secret.as_ref())
    {
        let encoded = sv_check::totp::base32(secret);
        secrets.push(encoded.to_lowercase());
        secrets.push(encoded);
    }
    secrets
}

/// The `NAME=value` lines `seed` printed, each a value the manifest may cite as `{seed:NAME}` (backlog 0536, ADR-124):
/// a name of letters, digits, and `_` not starting with a digit, an optional `export ` before it, and the rest of the
/// line, quotes around it taken off. Any other line is the seed's own talk and is not read.
pub(crate) fn seed_values(out: &str) -> std::collections::BTreeMap<String, String> {
    let mut values = std::collections::BTreeMap::new();
    for line in out.lines() {
        let line = line.trim();
        let line = line.strip_prefix("export ").map_or(line, str::trim_start);
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        let named = name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        if !named {
            continue;
        }
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .or_else(|| value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
            .unwrap_or(value);
        values.insert(name.to_owned(), value.to_owned());
    }
    values
}

fn seed_failed(code: i32, out: &str, accounts: &sv_check::signed_in::Accounts) -> String {
    let mut line = first_line(out);
    let mut secrets = test_secrets(accounts);
    // And any value it printed for `{seed:NAME}` before it failed (backlog 0536).
    secrets.extend(
        seed_values(out)
            .into_values()
            .filter(|v| v.chars().count() >= 4),
    );
    // `sv`'s own test secrets first, by value, as a blank the redaction below leaves alone; then
    // every other credential the line carries, the app's own included (backlog 0226, part 1,
    // item 1): a seed that fails on its database prints the database's address, password and all.
    const BLANK: &str = "{sv_test_secret}";
    for secret in secrets.iter().filter(|s| !s.is_empty()) {
        line = line.replace(secret.as_str(), BLANK);
    }
    line = match sv_check::secrets::SecretRules::shared() {
        Ok(rules) => sv_check::secrets::redact_text(rules, &line).0,
        Err(_) => format!("what it printed is {}", sv_check::secrets::RULES_UNREAD),
    };
    let line = line.replace(BLANK, "[a test secret, left out]");
    format!(
        "The seed command in stackvet.toml failed (exit {code}): {line}. With no accounts there \
         is nobody to sign in as."
    )
}

/// The lines of `docker logs --timestamps`, each `2026-10-06T05:12:04.123456789Z text`, in the
/// order of their times and without them. The times are all UTC, written the same way, so they
/// sort as text; lines with the same time keep the order they came in. A line with no time (a note
/// that output was cut) stays after the line before it.
fn interleaved(text: &str) -> String {
    let mut lines: Vec<(String, usize, &str)> = Vec::new();
    let mut last = String::new();
    for (n, line) in text.lines().enumerate() {
        match line.split_once(' ') {
            Some((time, rest)) if is_docker_time(time) => {
                last = time.to_owned();
                lines.push((last.clone(), n, rest));
            }
            _ => lines.push((last.clone(), n, line)),
        }
    }
    lines.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut out = lines
        .into_iter()
        .map(|(_, _, l)| l)
        .collect::<Vec<_>>()
        .join("\n");
    if text.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// `2026-10-06T05:12:04.123456789Z`: the form Docker stamps a log line with.
fn is_docker_time(word: &str) -> bool {
    let b = word.as_bytes();
    b.len() >= 20
        && b[4] == b'-'
        && b[7] == b'-'
        && b[10] == b'T'
        && b[13] == b':'
        && b[16] == b':'
        && word.ends_with('Z')
        && b[..4].iter().all(u8::is_ascii_digit)
}

#[cfg(test)]
#[path = "docker_session_tests.rs"]
mod session_tests;

#[cfg(test)]
#[path = "seed_failure_tests.rs"]
mod seed_failure_tests;

#[cfg(test)]
mod tests;

/// A number that is different for every run in this process.
///
/// The names were the process id alone, which is unique between processes and constant within one.
/// Two runs in the same process therefore asked the daemon for a network that already existed, and
/// the second failed — found by running the fence tests without `--test-threads=1`, where three of
/// them went red at once with `network with name sv-37867-net already exists`. It is not only a test
/// problem: a single process that checks two apps, or rebuilds one, hits it the same way.
fn next_run_number() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[cfg(test)]
mod probe_tests;

#[cfg(test)]
mod gateway_tests;

#[cfg(test)]
mod at_once_tests;

#[cfg(test)]
mod backend_tests;

#[cfg(test)]
mod podman_words_tests;

#[cfg(test)]
mod unreachable_tests;

#[cfg(test)]
mod error_words_tests;

#[cfg(test)]
mod gateway_said_tests;

#[cfg(test)]
mod ready_pause_tests;
