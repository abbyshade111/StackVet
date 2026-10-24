//! V17.1.1 and V17.1.2, read from a TURN relay's own settings (backlog 0625, part 3; ADR-146).
//!
//! A TURN relay carries a call's audio and video through when two people cannot reach each other directly. Whoever
//! uses it chooses the address it sends to, so a relay that does not refuse internal addresses is a way into the network
//! it sits in (V17.1.1), and one with no limit on what a user may open can be worn out (V17.1.2).
//!
//! The most used relay is coturn, set up by a `turnserver.conf` file or by the same settings given as flags to its
//! container. Its example configuration (`examples/etc/turnserver.conf`, read 24 October 2026) says what decides:
//! `denied-peer-ip` takes an address or a range `a-b`, and an address both denied and allowed is allowed;
//! `no-multicast-peers` refuses 224.0.0.0 and above and FFXX; loopback needs `allow-loopback-peers` to be let through;
//! `user-quota` and `total-quota` default to 0, no limit; and nothing refuses a private range unless the file does.
//!
//! What is read is every `turnserver.conf`, and the `command` of a compose service whose image is coturn. What it
//! credits is in part: the file, never the running relay. With no file, nothing is said and nothing is credited.

use crate::config::ConfigReport;
use crate::finding::{Confidence, Finding, Location, Severity};
use crate::verified::Verified;
use std::net::IpAddr;
use sv_scan::files::Listing;

pub const PEERS: &str = "config.turn-relay-reaches-reserved-addresses";
pub const QUOTA: &str = "config.turn-relay-unlimited";

/// The reserved ranges a relay must refuse, as `(written, first, last)`: "this network", the three private ranges,
/// shared address space, link-local, and IPv6's unique-local and link-local ranges. Loopback is refused by coturn
/// unless `allow-loopback-peers` is set, and broadcast by `no-multicast-peers`; both are read as flags.
const RESERVED: &[(&str, &str, &str)] = &[
    ("0.0.0.0/8", "0.0.0.0", "0.255.255.255"),
    ("10.0.0.0/8", "10.0.0.0", "10.255.255.255"),
    ("100.64.0.0/10", "100.64.0.0", "100.127.255.255"),
    ("169.254.0.0/16", "169.254.0.0", "169.254.255.255"),
    ("172.16.0.0/12", "172.16.0.0", "172.31.255.255"),
    ("192.168.0.0/16", "192.168.0.0", "192.168.255.255"),
    (
        "fc00::/7",
        "fc00::",
        "fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
    ),
    (
        "fe80::/10",
        "fe80::",
        "febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
    ),
];

/// One relay's settings, from one file: each option with its value, if any, and the line it is on.
#[derive(Debug, Default)]
struct Settings {
    file: String,
    first_line: usize,
    options: Vec<(String, Option<String>, usize)>,
}

impl Settings {
    fn values(&self, key: &str) -> impl Iterator<Item = &str> {
        self.options
            .iter()
            .filter(move |(k, _, _)| k == key)
            .filter_map(|(_, v, _)| v.as_deref())
    }

    fn has(&self, key: &str) -> bool {
        self.options.iter().any(|(k, _, _)| k == key)
    }

    fn number(&self, key: &str) -> Option<u64> {
        self.values(key).last().and_then(|v| v.trim().parse().ok())
    }
}

/// An address range as two numbers in one space: IPv4 below 2^32, IPv6 in its own range above it.
fn number(ip: IpAddr) -> u128 {
    match ip {
        IpAddr::V4(v4) => u128::from(u32::from(v4)),
        IpAddr::V6(v6) => u128::from(v6) | (1u128 << 127),
    }
}

/// A `denied-peer-ip` or `allowed-peer-ip` value: one address, or `first-last`. `None` when it is neither.
fn range(value: &str) -> Option<(u128, u128)> {
    let value = value.trim().trim_matches(['"', '\'']);
    let (first, last) = value.split_once('-').unwrap_or((value, value));
    let (first, last): (IpAddr, IpAddr) = (first.trim().parse().ok()?, last.trim().parse().ok()?);
    if first.is_ipv4() != last.is_ipv4() || number(first) > number(last) {
        return None;
    }
    Some((number(first), number(last)))
}

/// A `turnserver.conf`: one option a line, `name=value`, `name value`, or `name` alone, `#` beginning a comment.
fn from_conf(file: &str, text: &str) -> Settings {
    let mut settings = Settings {
        file: file.to_owned(),
        first_line: 1,
        ..Settings::default()
    };
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = match line.split_once(['=', ' ', '\t']) {
            Some((k, v)) => (k, Some(v.trim().to_owned())),
            None => (line, None),
        };
        settings
            .options
            .push((key.trim_start_matches('-').to_lowercase(), value, n + 1));
    }
    settings
}

/// A compose file's coturn services, each read from its `command` flags: `--name=value`, `--name value`, or `--name`.
fn from_compose(file: &str, text: &str) -> Vec<Settings> {
    let Ok(document) = crate::yaml::parse(text) else {
        return Vec::new();
    };
    let Some(crate::yaml::Value::Map(services)) = document.get("services") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (_, service) in services {
        let image = service.get("image").and_then(|v| v.text()).unwrap_or("");
        if !image.to_lowercase().contains("coturn") {
            continue;
        }
        let Some(command) = service.get("command") else {
            continue;
        };
        let mut words: Vec<(String, usize)> = Vec::new();
        let mut scalars = Vec::new();
        command.scalars_at(&mut scalars);
        for (scalar, line) in scalars {
            words.extend(scalar.split_whitespace().map(|w| (w.to_owned(), line)));
        }
        let mut settings = Settings {
            file: file.to_owned(),
            first_line: command.line().unwrap_or(1),
            ..Settings::default()
        };
        let mut i = 0;
        while i < words.len() {
            let (word, line) = &words[i];
            i += 1;
            let Some(option) = word.strip_prefix("--") else {
                continue;
            };
            let (key, value) = match option.split_once('=') {
                Some((k, v)) => (k.to_owned(), Some(v.to_owned())),
                None => match words.get(i) {
                    Some((next, _)) if !next.starts_with('-') => {
                        i += 1;
                        (option.to_owned(), Some(next.clone()))
                    }
                    _ => (option.to_owned(), None),
                },
            };
            settings.options.push((key.to_lowercase(), value, *line));
        }
        out.push(settings);
    }
    out
}

/// Every relay's settings in the app: each `turnserver.conf`, and each coturn service in a compose file.
fn relays(listing: &Listing) -> Vec<Settings> {
    let mut out = Vec::new();
    for entry in listing.app_files() {
        let name = entry
            .relative
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(&entry.relative);
        let compose = matches!(
            name,
            "docker-compose.yml" | "docker-compose.yaml" | "compose.yml" | "compose.yaml"
        );
        if name != "turnserver.conf" && !compose {
            continue;
        }
        let Ok(text) = entry.read_text() else {
            continue;
        };
        if compose {
            out.extend(from_compose(&entry.relative, &text));
        } else {
            out.push(from_conf(&entry.relative, &text));
        }
    }
    out
}

/// What a relay's settings leave open of the reserved ranges, in words; empty when every one is refused.
fn open_to(settings: &Settings) -> Vec<String> {
    let denied: Vec<(u128, u128)> = settings
        .values("denied-peer-ip")
        .filter_map(range)
        .collect();
    let allowed: Vec<(u128, u128)> = settings
        .values("allowed-peer-ip")
        .filter_map(range)
        .collect();
    let mut open = Vec::new();
    for (written, first, last) in RESERVED {
        let (first, last) = (
            number(first.parse().expect("a fixed address")),
            number(last.parse().expect("a fixed address")),
        );
        let let_back_in = allowed.iter().any(|&(a, b)| a <= last && b >= first);
        if !covered(&denied, first, last) {
            open.push(format!("{written} is not in any `denied-peer-ip`"));
        } else if let_back_in {
            open.push(format!("{written} is let back in by an `allowed-peer-ip`"));
        }
    }
    if !settings.has("no-multicast-peers") {
        open.push(
            "`no-multicast-peers` is not set, so broadcast addresses are reachable".to_owned(),
        );
    }
    if settings.has("allow-loopback-peers") {
        open.push(
            "`allow-loopback-peers` is set, so the relay's own machine is reachable".to_owned(),
        );
    }
    open
}

/// Whether the ranges, together, hold every address from `first` to `last`.
fn covered(ranges: &[(u128, u128)], first: u128, last: u128) -> bool {
    let mut sorted = ranges.to_vec();
    sorted.sort_unstable();
    let mut next = first;
    for (a, b) in sorted {
        if a > next {
            break;
        }
        if b >= next {
            if b >= last {
                return true;
            }
            next = b + 1;
        }
    }
    false
}

fn finding(
    id: &str,
    title: &str,
    requirement: &str,
    settings: &Settings,
    description: String,
    fix: &str,
) -> Finding {
    crate::finding::found(Finding {
        evidence: Vec::new(),
        also_reported_by: Vec::new(),
        fingerprint: String::new(),
        earlier_fingerprints: Vec::new(),
        marked_test_code: false,
        bundled_library: None,
        outranked: None,
        also_on_this_line: Vec::new(),
        help_uri: None,
        rule_id: id.into(),
        title: title.into(),
        severity: if requirement == "V17.1.1" {
            Severity::High
        } else {
            Severity::Medium
        },
        confidence: Confidence::Medium,
        location: Location {
            file: settings.file.clone(),
            line: settings.first_line,
        },
        secret: None,
        requirement_ids: vec![requirement.into()],
        cwe: vec![if requirement == "V17.1.1" { "CWE-918" } else { "CWE-770" }.into()],
        description,
        impact: if requirement == "V17.1.1" {
            "Anyone who can use the relay can send through it to the addresses it does not refuse: the servers, \
             databases, and admin pages on the network it runs in, which nobody outside should reach."
        } else {
            "One person can open as many relayed connections as they like and keep them, until the relay has none left \
             for anyone else's calls."
        }
        .into(),
        fix: fix.into(),
    })
}

pub fn check(listing: &Listing, report: &mut ConfigReport) {
    let relays = relays(listing);
    let mut peers_ok = !relays.is_empty();
    let mut quota_ok = !relays.is_empty();
    for settings in &relays {
        let open = open_to(settings);
        if !open.is_empty() {
            peers_ok = false;
            report.findings.push(finding(
                PEERS,
                "The TURN relay can be used to reach internal addresses",
                "V17.1.1",
                settings,
                format!(
                    "The TURN relay's settings in {} leave reserved addresses open: {}.",
                    settings.file,
                    open.join("; ")
                ),
                "Add a `denied-peer-ip` line for each of 0.0.0.0-0.255.255.255, 10.0.0.0-10.255.255.255, \
                 100.64.0.0-100.127.255.255, 169.254.0.0-169.254.255.255, 172.16.0.0-172.31.255.255, \
                 192.168.0.0-192.168.255.255, fc00::-fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff, and \
                 fe80::-febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff; add `no-multicast-peers`; take out \
                 `allow-loopback-peers`; and allow back only the one address the relay must reach, if any.",
            ));
        }
        let quotas = ["user-quota", "total-quota"];
        let unset: Vec<&str> = quotas
            .into_iter()
            .filter(|q| settings.number(q).is_none_or(|n| n == 0))
            .collect();
        if !unset.is_empty() {
            quota_ok = false;
            report.findings.push(finding(
                QUOTA,
                "The TURN relay does not limit how many connections a user may open",
                "V17.1.2",
                settings,
                format!(
                    "The TURN relay's settings in {} do not set {} above 0, and 0, coturn's default, means no limit.",
                    settings.file,
                    unset.iter().map(|q| format!("`{q}`")).collect::<Vec<_>>().join(" or ")
                ),
                "Set `user-quota` (the connections one user may have at once, such as 12) and `total-quota` (all users \
                 together) to numbers that fit the calls the app expects.",
            ));
        }
    }
    let read = relays
        .iter()
        .map(|s| s.file.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    if peers_ok {
        report.passed.push(
            Verified::new(
                PEERS,
                &["V17.1.1"],
                format!(
                    "the TURN relay's settings in {read} refuse every reserved range (the private, shared, link-local, and \
                     IPv6 local ranges), broadcast addresses, and loopback; the file was read, not the running relay, and \
                     coturn older than 4.5.2 let loopback through by default"
                ),
            )
            .in_part(),
        );
    } else {
        crate::verified::unless_credited(PEERS, &report.passed);
    }
    if quota_ok {
        report.passed.push(
            Verified::new(
                QUOTA,
                &["V17.1.2"],
                format!(
                    "the TURN relay's settings in {read} set `user-quota` and `total-quota` above 0; the file was read, \
                     not the running relay"
                ),
            )
            .in_part(),
        );
    } else {
        crate::verified::unless_credited(QUOTA, &report.passed);
    }
}

#[cfg(test)]
mod tests;
