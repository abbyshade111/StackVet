//! The fence's network: how it is made, the knock on its gateway, and what the knock's answers mean (ADR-019). Moved
//! out of `docker.rs` unchanged (backlog 0324).

use super::*;

/// The command that makes the fenced network with `options`: internal, and asking for no IPv6 at all.
/// `sv` needs none, and an IPv6 network gives the app a way to the host the IPv4 options do not
/// close (the bridge's link-local address, which no gateway list names; backlog 0410). A daemon whose
/// default gives new networks IPv6 may not honor the request; the gateway check then finds an IPv6
/// address on the network and stops the run.
pub fn network_create_args(options: &[&str], network: &str) -> Vec<String> {
    let mut args: Vec<String> = ["network", "create", "--internal", "--ipv6=false"]
        .map(str::to_owned)
        .to_vec();
    for option in options {
        args.extend(["-o".to_owned(), (*option).to_owned()]);
    }
    args.push(network.to_owned());
    args
}

/// The script the gateway check runs in a container on the fenced network: first the container's own
/// addresses, every family (`ip -o addr`, never `-4`: an IPv6 address must be seen), a line for each IPv6
/// one but loopback, and whether the list could be read at all; then the control, a knock on its own
/// loopback; then a knock on each target that is not its own address.
pub(super) fn knock_script(targets: &[String]) -> String {
    let mut script = String::from(
        "A=\"$(ip -o addr show 2>/dev/null)\"; \
         case \"$A\" in *\"inet 127.0.0.1/\"*) echo \"sv-addrs=1\";; *) echo \"sv-addrs=0\";; esac; \
         echo \"$A\" | while read -r _ _ family address _; do \
         if [ \"$family\" = inet6 ] && [ \"${address%%/*}\" != ::1 ]; then echo \"sv-inet6=$address\"; fi; done; ",
    );
    // `-vv`, and not `-z`: busybox's netcat (`nc_bloaty.c`, 1.36) reports a refused connection only
    // at the second level of verbosity ("if we're scanning at a one -v verbosity level, don't print
    // refusals"), and a refusal kept quiet would read as silence, which must never pass for a fence.
    // CI found the first two tries silent. With nothing to send, a connection that opens ends at once.
    script.push_str(&format!(
        "nc -vv -w 3 127.0.0.1 {GATEWAY_PORT} </dev/null 2>&1; echo \"sv-self=$?\""
    ));
    // An address that is the knocking container's own is not knocked on: with no gateway, Docker
    // gives the subnet's first address to the first container, which is this one, and its own
    // refusal would read as the host answering. CI found exactly that.
    for target in targets {
        script.push_str(&format!(
            "; case \" $A \" in *\"inet {target}/\"*|*\"inet6 {target}/\"*) echo \"sv-own={target}\";; \
             *) nc -vv -w 3 {target} {GATEWAY_PORT} </dev/null 2>&1; echo \"sv-gateway=$?\";; esac"
        ));
    }
    script
}

/// Why the fenced network is plain, its bridge keeping an address on this computer, or `None` when it
/// is not. It is plain when Docker took none of the ways of leaving out a gateway (`made_with` is
/// `None`), and also when, asked for one of them, it still names a gateway address for the network:
/// a daemon that accepts an option it does not know and ignores it builds a plain network all the
/// same, and the gateway it names tells (backlog 0414). `config` is the inspect output the gateway
/// check reads, `Gateway|Subnet` pairs.
pub(super) fn plain_reason(made_with: Option<&[&str]>, config: &str) -> Option<String> {
    if made_with.is_none() {
        return Some("Docker made it with `--internal` alone, taking none of the ways of leaving out a gateway".to_owned());
    }
    let named: Vec<&str> = config
        .split_whitespace()
        .filter_map(|pair| pair.split_once('|').map_or(Some(pair), |(g, _)| Some(g)))
        .filter(|g| g.parse::<std::net::IpAddr>().is_ok())
        .collect();
    (!named.is_empty()).then(|| {
        format!(
            "Docker names a gateway address for it ({}) though it was asked for none: it took the \
             option without honoring it",
            named.join(", ")
        )
    })
}

/// What the gateway check saw. See `verify_gateway_closed`.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum GatewayVerdict {
    Closed,
    Reachable,
    Unknown(String),
}

/// The addresses to knock on for a network: every gateway Docker names, and the first address of every
/// subnet, where a gateway would be, IPv4 and IPv6 alike. Until backlog 0300 only IPv4 was read, so an
/// IPv6 gateway, the host just as the IPv4 one is, was never knocked on. Read from `Gateway|Subnet`
/// pairs, space-separated.
pub(super) fn gateway_targets(config: &str) -> Vec<String> {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let mut out: Vec<String> = Vec::new();
    for pair in config.split_whitespace() {
        let (gateway, subnet) = pair.split_once('|').unwrap_or((pair, ""));
        if let Ok(ip) = gateway.parse::<IpAddr>() {
            out.push(ip.to_string());
        }
        let Some((base, bits)) = subnet.split_once('/') else {
            continue;
        };
        let Ok(bits) = bits.parse::<u32>() else {
            continue;
        };
        match base.parse::<IpAddr>() {
            Ok(IpAddr::V4(base)) if bits < 31 => {
                let mask = u32::MAX.checked_shl(32 - bits).unwrap_or(0);
                out.push(Ipv4Addr::from((u32::from(base) & mask) + 1).to_string());
            }
            Ok(IpAddr::V6(base)) if bits < 127 => {
                let mask = u128::MAX.checked_shl(128 - bits).unwrap_or(0);
                out.push(Ipv6Addr::from((u128::from(base) & mask) + 1).to_string());
            }
            _ => {}
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Reads the knocks: the container's own loopback (the control, which must be refused) and then each
/// gateway address. A connection or a refusal from any of them is the host answering; a timeout, no
/// route, or an unreachable network from every one is the fence holding, on a network made without a
/// gateway address. On a `plain` one, whose bridge keeps its address on the host, the same silence
/// may be a firewall dropping one port and not others, so it is not taken for a fence (backlog
/// 0301). Anything else is not taken for either.
pub(super) fn gateway_verdict(
    out: &str,
    plain: Option<&str>,
    targets: usize,
    docker_version: &dyn Fn() -> Option<String>,
) -> GatewayVerdict {
    // Each knock's output, and its exit status, in order.
    let mut sections: Vec<(String, String, Option<i32>)> = Vec::new();
    let mut text = String::new();
    for line in out.lines() {
        if let Some((mark, code)) = line.split_once('=').filter(|(m, _)| m.starts_with("sv-")) {
            sections.push((
                mark.to_owned(),
                std::mem::take(&mut text),
                code.trim().parse().ok(),
            ));
        } else {
            text.push_str(line);
            text.push('\n');
        }
    }
    let Some((_, own, Some(own_code))) = sections.iter().find(|(m, _, _)| m == "sv-self").cloned()
    else {
        // What Docker said, for the one line that says why: on 9 October 2026 this ran three times
        // in CI with no reason given, while Docker Hub was failing, and nobody could say whether
        // the image or the fence was at fault.
        // Docker ends a refused `run` with "Run 'docker run --help' for more information", under
        // the line that says why; that hint is passed over.
        let said: String = out
            .lines()
            .filter(|l| !(l.trim_start().starts_with("Run 'docker") && l.contains("--help")))
            .collect::<Vec<_>>()
            .join("\n");
        return GatewayVerdict::Unknown(format!(
            "the check did not run; Docker said: {}",
            last_line(&said)
        ));
    };
    if own_code == 0 || !own.to_lowercase().contains("refused") {
        return GatewayVerdict::Unknown(format!(
            "the control, a knock on the container's own loopback, did not read as refused: {}",
            own.trim()
        ));
    }
    // The container's own addresses could be listed: without them neither its own address nor an
    // IPv6 one can be told, so a missing `ip` stops the run rather than passing.
    if !sections
        .iter()
        .any(|(m, _, code)| m == "sv-addrs" && *code == Some(1))
    {
        return GatewayVerdict::Unknown(
            "the knocking container's own addresses could not be listed".to_owned(),
        );
    }
    let knocks: Vec<&(String, String, Option<i32>)> = sections
        .iter()
        .filter(|(m, _, _)| m == "sv-gateway")
        .collect();
    // One report for every address knocked on, a knock or its own: output cut short must not
    // read a knock that never reported as the fence holding (backlog 0410).
    let own = sections.iter().filter(|(m, _, _)| m == "sv-own").count();
    if knocks.len() + own != targets || targets == 0 {
        return GatewayVerdict::Unknown(format!(
            "{} of the {targets} addresses knocked on reported back",
            knocks.len() + own
        ));
    }
    let mut unknown = None;
    for (_, said, code) in &knocks {
        let lower = said.to_lowercase();
        if *code == Some(0) || lower.contains("refused") {
            return GatewayVerdict::Reachable;
        }
    }
    // Any IPv6 address on the network but loopback: `sv` asked for none, and over IPv6 the bridge's
    // link-local address, which no gateway list names, may be the host (backlog 0410).
    let inet6: Vec<&str> = out
        .lines()
        .filter_map(|l| l.strip_prefix("sv-inet6="))
        .map(str::trim)
        .collect();
    if !inet6.is_empty() {
        let addresses = inet6.join(", ");
        // Link-local alone, on a Docker before 26.0.2: that Docker leaves IPv6 on in containers of
        // networks made without it (CVE-2024-32473), so the daemon.json advice cannot apply (0450).
        let old = inet6
            .iter()
            .all(|a| link_local(a))
            .then(docker_version)
            .flatten()
            .filter(|v| version_below(v, (26, 0, 2)));
        return GatewayVerdict::Unknown(match old {
            Some(version) => format!(
                "the fenced network has IPv6 ({addresses}), though sv asked Docker for none. They are \
                 link-local addresses only, and this Docker is {version}: Docker 26.0.2 and later \
                 switch IPv6 off in containers of networks made without it, as the fix for \
                 CVE-2024-32473. Update Docker to 26.0.2 or later, then run again"
            ),
            None => format!(
                "the fenced network has IPv6 ({addresses}), though sv asked Docker for none: the \
                 daemon gives new networks IPv6 by default. Turn that off (remove \
                 `com.docker.network.enable_ipv6` from `default-network-opts` in Docker's \
                 daemon.json, and restart Docker), then run again"
            ),
        });
    }
    // A plain network never passes on silence: its bridge has an address on this computer, and a knock
    // there that goes unanswered may be a firewall dropping one port while others stay open (0301).
    // Nor when every address was the knocking container's own (backlog 0414).
    let plain_refusal = |reason: &str| {
        GatewayVerdict::Unknown(format!(
            "the network has its gateway address on this computer ({reason}), and a knock there that \
             goes unanswered may be a firewall dropping one port while others stay open; a Docker \
             that makes the network with no gateway address (`gateway_mode_ipv4=isolated`, Docker 28 \
             and later, or `inhibit_ipv4` before it) is one this check can trust"
        ))
    };
    if knocks.is_empty() {
        // Every address was the knocking container's own: nothing of the host's is on the network.
        return match plain {
            Some(reason) => plain_refusal(reason),
            None => GatewayVerdict::Closed,
        };
    }
    for (_, said, code) in knocks {
        let lower = said.to_lowercase();
        let closed = code.is_some()
            && (lower.contains("timed out")
                || lower.contains("timeout")
                || lower.contains("no route")
                || lower.contains("unreachable"));
        if !closed && unknown.is_none() {
            unknown = Some(format!("nc said: {}", said.trim()));
        }
    }
    match unknown {
        Some(why) => GatewayVerdict::Unknown(why),
        None => match plain {
            Some(reason) => plain_refusal(reason),
            None => GatewayVerdict::Closed,
        },
    }
}

/// Whether an address as `ip` lists it (`fe80::1/64`) is IPv6 link-local, `fe80::/10`.
pub(super) fn link_local(address: &str) -> bool {
    let address = address.split('/').next().unwrap_or(address);
    address
        .split(':')
        .next()
        .and_then(|first| u16::from_str_radix(first, 16).ok())
        .is_some_and(|first| first & 0xffc0 == 0xfe80)
}

/// Whether a Docker server version (`26.0.1`, `24.0.7-ce`) is below `than`; `false` for one that does
/// not read as a version, so an unreadable one never changes the advice.
pub(super) fn version_below(version: &str, than: (u64, u64, u64)) -> bool {
    let mut parts = version
        .trim()
        .split(['.', '-', '+'])
        .map(|p| p.parse::<u64>().ok());
    match (
        parts.next().flatten(),
        parts.next().flatten(),
        parts.next().flatten(),
    ) {
        (Some(major), Some(minor), patch) => (major, minor, patch.unwrap_or(0)) < than,
        _ => false,
    }
}

impl DockerBackend {
    /// Confirms with the daemon that the network really is internal. Fail secure: anything other
    /// than a clear "true" stops the run.
    pub fn verify_fenced(&self, network: &str) -> Result<(), CannotRun> {
        match self.docker(&["network", "inspect", "-f", "{{.Internal}}", network]) {
            Ok((0, out)) if out.trim() == "true" => Ok(()),
            Ok((0, out)) => Err(CannotRun::BackendFailed {
                detail: format!(
                    "the network `{network}` is not internal (Docker reports Internal={}), so the \
                     app would have been able to reach the internet while it ran",
                    out.trim()
                ),
            }),
            Ok((_, out)) => Err(CannotRun::BackendFailed {
                detail: format!("could not confirm the fence: {}", first_line(&out)),
            }),
            Err(e) => Err(CannotRun::BackendFailed {
                detail: format!("could not confirm the fence: {e}"),
            }),
        }
    }

    /// Makes the fenced network: internal, with no IPv6, and without a gateway address by the first of
    /// `NO_GATEWAY`'s ways the daemon takes. Returns that way, or `None` when it took none and the
    /// network was made with `--internal` alone, which the gateway check then never passes.
    pub fn make_fenced_network(
        &self,
        network: &str,
    ) -> Result<Option<&'static [&'static str]>, CannotRun> {
        for options in NO_GATEWAY {
            let args = network_create_args(options, network);
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            if self.docker(&args).is_ok_and(|(code, _)| code == 0) {
                return Ok(Some(*options));
            }
        }
        let args = network_create_args(&[], network);
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        match self.docker(&args) {
            Ok((0, _)) => Ok(None),
            Ok((_, out)) => Err(CannotRun::BackendFailed {
                detail: first_line(&out),
            }),
            Err(e) => Err(CannotRun::BackendFailed { detail: e }),
        }
    }

    /// Refuses the run when a container on the fenced network can reach the bridge's gateway, the
    /// host (or the virtual machine Docker runs in). Knocked on from a throwaway container: an answer
    /// of any kind, a connection or a refusal, is the host's stack answering. The control is the same
    /// knock on the container's own loopback, which must read as refused, so an `nc` that cannot tell
    /// one from the other stops the run rather than passing for a fence.
    ///
    /// A plain network, one whose bridge keeps its address on the host, never passes on silence: a host
    /// firewall that drops what it does not allow, rather than refusing it, times the knock out while
    /// other ports, SSH among them, stay open to the app (backlog 0301). Whether it is plain is read
    /// from what Docker built as well as from how it was asked (`plain_reason`, backlog 0414).
    pub fn verify_gateway_closed(
        &self,
        network: &str,
        made_with: Option<&[&str]>,
    ) -> Result<(), CannotRun> {
        let fail = |detail: String| Err(CannotRun::BackendFailed { detail });
        let config = match self.docker(&[
            "network",
            "inspect",
            "-f",
            "{{range .IPAM.Config}}{{.Gateway}}|{{.Subnet}} {{end}}",
            network,
        ]) {
            Ok((0, out)) => out,
            Ok((_, out)) => {
                return fail(format!(
                    "could not read the fence's gateway: {}",
                    first_line(&out)
                ));
            }
            Err(e) => return fail(format!("could not read the fence's gateway: {e}")),
        };
        // A network made without a gateway address may list none at all, so the address a gateway would
        // have, the subnet's first, is knocked on as well as any Docker names.
        let targets = gateway_targets(&config);
        if targets.is_empty() {
            return fail(format!(
                "Docker names no gateway or subnet for the network `{network}` ({}), so whether the \
                 app could reach this computer through it cannot be checked",
                config.trim()
            ));
        }
        let gateway = targets.join(", ");
        let plain = plain_reason(made_with, &config);
        let script = knock_script(&targets);
        let out = match self.docker(&[
            "run",
            "--rm",
            "--network",
            network,
            PROBE_IMAGE,
            "sh",
            "-c",
            &script,
        ]) {
            Ok((_, out)) => out,
            Err(e) => return fail(format!("could not check the fence's gateway: {e}")),
        };
        // Asked only when the network shows link-local IPv6 alone, which an older Docker gives every
        // container (backlog 0450).
        let version = || match self.docker(&["version", "--format", "{{.Server.Version}}"]) {
            Ok((0, out)) => Some(out.trim().to_owned()),
            _ => None,
        };
        match gateway_verdict(&out, plain.as_deref(), targets.len(), &version) {
            GatewayVerdict::Closed => Ok(()),
            GatewayVerdict::Reachable => fail(format!(
                "a container on the fenced network reached its gateway, {gateway}, which is this \
                 computer (or the virtual machine Docker runs in): the app could have reached anything \
                 listening there, so it was not started"
            )),
            GatewayVerdict::Unknown(why) => fail(format!(
                "could not tell whether the fence's gateway, {gateway}, is reachable ({why}), so the \
                 app was not started"
            )),
        }
    }
}
