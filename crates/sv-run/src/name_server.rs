//! The stand-in name server on the fenced network, and what it was asked (ADR-085, backlog 0240).
//!
//! It runs for the length of a run, on the same network as the app, and answers every name the app
//! looks up with SERVFAIL, the answer a name gets on the fenced network today. The app's own name
//! lookups go to it (`--dns`), so the app sees what it saw before. The name server writes one JSON
//! line per question to its output, and `sv` reads that output with `docker logs` before the
//! container goes. Names are kept as the app asked them, and `sv-cli` redacts them before they reach
//! `seen.json`.

use crate::docker::PROVIDER_IMAGE;
use serde_json::Value;

/// The script the name server runs: the same text as `assets/name-server.mjs`, passed on the command
/// line so nothing is written to the owner's disk, as the test provider's is.
pub const SCRIPT: &str = include_str!("../assets/name-server.mjs");

/// One question the name server was asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookup {
    /// The name as the app asked for it, without a final dot.
    pub name: String,
    /// The DNS query type as a number: 1 for an IPv4 address, 28 for an IPv6 address, and so on.
    pub kind: u16,
    /// When the question arrived, as the name server wrote it (UTC, to the millisecond).
    pub at: String,
}

/// The arguments that start the name server on `network` under `name`: no published port, and the
/// hardening every helper gets. Node runs the script as an ES module, with its built-in modules only.
pub fn start_args(network: &str, name: &str) -> Vec<String> {
    let args = [
        "run",
        "-d",
        "--rm",
        "--name",
        name,
        "--network",
        network,
        PROVIDER_IMAGE,
        "node",
        "--input-type=module",
        "-e",
        SCRIPT,
    ];
    crate::docker::hardened(args.iter().map(|arg| (*arg).to_owned()).collect())
}

/// The lines the name server wrote, read back. Each question is a JSON object with a `name`, a
/// `type` and a `time`; any other line (its own `ready` line, a stack trace, a line cut short) is
/// skipped, so one bad line cannot hide the rest. Repeats are kept: the same name is often asked for
/// twice, once for an IPv4 address and once for an IPv6 one, and `sv-cli` counts them.
pub fn parse_log(text: &str) -> Vec<Lookup> {
    text.lines()
        .filter_map(|line| serde_json::from_str::<Value>(line.trim()).ok())
        .filter_map(|value| {
            let name = value.get("name")?.as_str()?;
            let kind = u16::try_from(value.get("type")?.as_u64()?).ok()?;
            let at = value.get("time")?.as_str()?;
            Some(Lookup {
                name: name.to_owned(),
                kind,
                at: at.to_owned(),
            })
        })
        .collect()
}

/// The name a query type has in plain words, for the record: `A` and `AAAA` for the two address
/// types an app asks for, otherwise the number.
pub fn kind_name(kind: u16) -> String {
    match kind {
        1 => "A".to_owned(),
        28 => "AAAA".to_owned(),
        other => format!("type {other}"),
    }
}

#[cfg(test)]
#[path = "name_server_tests.rs"]
mod tests;
