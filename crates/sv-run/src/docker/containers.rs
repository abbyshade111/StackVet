//! What every container `sv` starts is given: the hardening and limits they all share, and each one's arguments (the
//! app, the helpers, the browser and its driver, the mail server), with how the mail is read back. Moved out of
//! `docker.rs` unchanged (backlog 0324).

use super::*;

/// `args`, a `docker run` or `docker create`, with the limits every container is started under
/// inserted after the command.
/// What every container this backend starts carries, whichever path starts it: a read-only root
/// (each place it may write is a tmpfs the caller names, with a size), no capabilities, and no
/// way to gain any. Put on in `prepared` beside the limits (ADR-019, 4 October and 8 October
/// 2026), so that a path which forgets it cannot exist: until 8 October 2026 these five arguments
/// were written out at eleven places in this file and one in `install.rs`, and the fallback for
/// a run without a sidecar once had none of them.
pub(crate) const HARDENING: [&str; 5] = [
    "--read-only",
    "--cap-drop",
    "ALL",
    "--security-opt",
    "no-new-privileges",
];

/// The `--mount` value that puts `src`, a folder or file of the owner's, read-only at `dst` in a
/// container (backlog 0329). Docker's short `-v src:dst:ro` splits at every colon, so a folder such
/// as `Notes: 2026/app` could not be mounted and the run failed without saying why; `--mount` is read
/// as comma-separated fields, and a field holding a comma or a double quote is quoted the way Docker's
/// reader expects, with any double quote doubled.
pub fn read_only_bind(src: &std::path::Path, dst: &str) -> String {
    let source = format!("src={}", src.display());
    let source = if source.contains([',', '"']) {
        format!("\"{}\"", source.replace('"', "\"\""))
    } else {
        source
    };
    format!("type=bind,{source},dst={dst},readonly")
}

/// `args` with `HARDENING` just after the command word.
pub(super) fn hardened(args: Vec<String>) -> Vec<String> {
    let mut out = args;
    let at = 1.min(out.len());
    out.splice(at..at, HARDENING.iter().map(|s| (*s).to_owned()));
    out
}

pub(super) fn limited(args: Vec<String>, cpus: Option<&str>) -> Vec<String> {
    let mut limits = vec![
        "--memory",
        MEMORY_LIMIT,
        "--memory-swap",
        MEMORY_LIMIT,
        "--pids-limit",
        PROCESS_LIMIT,
    ];
    if let Some(cpus) = cpus {
        limits.extend(["--cpus", cpus]);
    }
    let mut out = args;
    let at = 1.min(out.len());
    out.splice(at..at, limits.into_iter().map(str::to_owned));
    out
}

/// The processor limit for a Docker that says it has `ncpu` processors: `MOST_CPUS`, or all of them
/// when there are fewer. `None` when the answer is not a count.
pub(super) fn cpus_for(ncpu: &str) -> Option<String> {
    let n: u64 = ncpu.trim().parse().ok().filter(|n| *n > 0)?;
    Some(n.min(MOST_CPUS).to_string())
}

/// The app's own `/tmp`: in memory, and with a size, so the app has somewhere to keep its data
/// while it runs (`sv init` tells it to use `/tmp`) and cannot fill the machine's memory through it.
pub(super) const APP_TMP: &str = "/tmp:size=256m";

/// The folder a test runner writes its report to, at `REPORT_DIR`, in memory and with a size.
/// `/app` is read-only on purpose, so a test runner has nowhere to put its report unless something
/// is provided — which is how the first version of `test-report` failed: the runner could not
/// write the file and the report read as "no report", correctly but uselessly. Findings are still
/// only ever read out with `exec`.
pub(super) const REPORT_TMPFS: &str = "/sv-reports:size=16m";

/// How the app itself is started: on the fenced network, and hardened like every helper — a
/// read-only file system, no capabilities, no way to gain any (ADR-019, "Later, 30 September
/// 2026"). Its writable places are two in-memory folders with a size each, `/tmp` and the report
/// folder. Its own folder is mounted read-only: `sv` reads code, it does not let the code it is
/// checking rewrite itself mid-check. No port is published — nothing on this computer could reach
/// it anyway, and saying so in the arguments keeps that honest. Separate so a test can read it.
pub(super) fn app_args<'a>(
    name: &'a str,
    network: &'a str,
    mount: &'a str,
    port_env: &'a str,
) -> Vec<&'a str> {
    vec![
        "run",
        "-d",
        "--name",
        name,
        "--network",
        network,
        "--tmpfs",
        APP_TMP,
        "--mount",
        mount,
        "--tmpfs",
        REPORT_TMPFS,
        "-w",
        "/app",
        "-e",
        port_env,
        // Nothing of the owner's reaches the app: no API keys, no home directory.
        "--env-file",
        "/dev/null",
    ]
}

/// How a helper that runs a script of `sv`'s own is started, the test provider (`PROVIDER_SCRIPT`) and the test
/// model (`MODEL_SCRIPT`) alike: fenced and hardened like the mail server, with its settings in `env` and its script
/// passed on the command line so nothing is written to the owner's disk (one function since backlog 0461).
pub(super) fn helper_args<'a>(
    network: &'a str,
    name: &'a str,
    env: &[&'a str],
    script: &'a str,
) -> Vec<&'a str> {
    let mut args = vec!["run", "-d", "--rm", "--name", name, "--network", network];
    for pair in env {
        args.extend(["-e", pair]);
    }
    args.extend([PROVIDER_IMAGE, "node", "--input-type=module", "-e", script]);
    args
}

/// The first copy's `docker run` arguments, renamed, with one more setting before the image.
pub(super) fn switched_off_args(
    app_args: &[String],
    app: &str,
    name: &str,
    image: &str,
    setting: &str,
) -> Option<Vec<String>> {
    let mut args: Vec<String> = app_args
        .iter()
        .map(|a| if a == app { name.to_owned() } else { a.clone() })
        .collect();
    let at = args.iter().rposition(|a| a == image)?;
    args.splice(at..at, ["-e".to_owned(), setting.to_owned()]);
    Some(args)
}

/// What the app is told about the test model: the addresses the OpenAI, Anthropic, and Google Gemini
/// libraries read, a key for each that works nowhere else, and the OpenAI-style address in any other
/// variables stackvet.toml names. Google's library reads its key from `GEMINI_API_KEY` or
/// `GOOGLE_API_KEY` and its address from `GOOGLE_GEMINI_BASE_URL` (ADR-019, Later, 9 October 2026).
pub(super) fn model_env(host: &str, others: &[String], mcp: Option<&str>) -> Vec<String> {
    let openai = format!("http://{host}:{MODEL_PORT}/v1");
    let mut env = vec![
        format!("OPENAI_BASE_URL={openai}"),
        format!("OPENAI_API_KEY={MODEL_KEY}"),
        format!("ANTHROPIC_BASE_URL=http://{host}:{MODEL_PORT}"),
        format!("ANTHROPIC_API_KEY={MODEL_KEY}"),
        format!("GOOGLE_GEMINI_BASE_URL=http://{host}:{MODEL_PORT}"),
        format!("GEMINI_API_KEY={MODEL_KEY}"),
        format!("GOOGLE_API_KEY={MODEL_KEY}"),
    ];
    env.extend(others.iter().map(|name| format!("{name}={openai}")));
    // The test MCP server is the same container, at `/mcp`.
    env.extend(mcp.map(|name| format!("{name}=http://{host}:{MODEL_PORT}/mcp")));
    env
}

/// The browser's `/tmp`, where Chromium keeps its profile, and the mail server's, where Mailpit keeps
/// its messages: each in memory, and each with a size, as the app's is.
pub(super) const BROWSER_TMP: &str = "/tmp:size=512m";
pub(super) const MAIL_TMP: &str = "/tmp:size=64m";

/// How the browser is started. Hardened like the sidecar, with somewhere in memory to write, since
/// Chromium keeps its profile under `/tmp`; it runs with its own sandbox off, as it must in a
/// container, so the container is the sandbox and everything it may not do is taken away.
///
/// Chromium is started directly, not through the image's own script, which forwards port 9222 on
/// every address to DevTools and has Chromium listen on every address too: on the fenced network,
/// the app could reach DevTools and drive the browser that checks it (deep review S11). Here
/// DevTools listens on 127.0.0.1 alone, where only the driver, sharing the browser's network,
/// reaches it.
pub(super) fn browser_args<'a>(network: &'a str, name: &'a str) -> Vec<&'a str> {
    vec![
        "run",
        "-d",
        "--rm",
        "--name",
        name,
        "--network",
        network,
        "--tmpfs",
        BROWSER_TMP,
        "--entrypoint",
        "/headless-shell/headless-shell",
        BROWSER_IMAGE,
        "--no-sandbox",
        "--use-gl=angle",
        "--use-angle=swiftshader",
        "--remote-debugging-address=127.0.0.1",
        "--remote-debugging-port=9223",
    ]
}

/// How the driver is started: inside the browser's network, where the DevTools port is on
/// 127.0.0.1 and the app is reached by its name on the fenced network, and nothing else is.
pub(super) fn driver_args<'a>(network: &'a str, job: &'a str) -> Vec<&'a str> {
    vec![
        "run",
        "--rm",
        "--network",
        network,
        "-e",
        job,
        PROVIDER_IMAGE,
        "node",
        "--input-type=module",
        "-e",
        DRIVER_SCRIPT,
    ]
}

/// How the mail server is started. Separate so its hardening can be checked without starting it.
pub(super) fn mail_args<'a>(network: &'a str, name: &'a str) -> Vec<&'a str> {
    vec![
        "run",
        "-d",
        "--rm",
        "--name",
        name,
        "--network",
        network,
        "--tmpfs",
        MAIL_TMP,
        MAIL_IMAGE,
        "--smtp-auth-accept-any",
        "--smtp-auth-allow-insecure",
    ]
}

/// The messages sent to this address, oldest first, from Mailpit's list of messages.
///
/// Matched on every recipient field, since an app may put its user in `Bcc`, and without regard to
/// case, since an address's domain has none. `None` when the answer is not a list at all.
pub(super) fn mail_ids_to(listing: &str, to: &str) -> Option<Vec<String>> {
    let value: serde_json::Value = serde_json::from_str(listing).ok()?;
    let messages = value.get("messages")?.as_array()?;
    let mut ids: Vec<String> = messages
        .iter()
        .filter(|m| {
            ["To", "Cc", "Bcc"].iter().any(|field| {
                m.get(field).and_then(|v| v.as_array()).is_some_and(|list| {
                    list.iter().any(|r| {
                        r.get("Address")
                            .and_then(|a| a.as_str())
                            .is_some_and(|a| a.eq_ignore_ascii_case(to))
                    })
                })
            })
        })
        .filter_map(|m| m.get("ID")?.as_str().map(str::to_owned))
        .collect();
    // Mailpit lists the newest first.
    ids.reverse();
    Some(ids)
}

/// One message's text: the plain part and the HTML part both, since a link may be in either.
pub(super) fn mail_text(message: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(message).ok()?;
    let part = |name: &str| value.get(name).and_then(|v| v.as_str()).unwrap_or("");
    Some(format!("{}\n{}", part("Text"), part("HTML")))
}
