//! The fence holds over IPv6 too (backlog 0300). A daemon set to give new networks IPv6 gives the internal
//! network an IPv6 gateway, which is the host (or Docker's virtual machine) just as the IPv4 one is. Until
//! 0300 the gateway check read IPv4 alone and passed such a network. Each branch says which it took: a
//! daemon that cannot make an IPv6 network, or whose IPv6 gateway does not answer, checks less. Since
//! backlog 0410 the runner asks for no IPv6 at all, and a network that has it is refused.

use std::process::Command;
use sv_run::Backend;
use sv_run::docker::DockerBackend;

fn docker(args: &[&str]) -> (bool, String) {
    let out = Command::new("docker")
        .args(args)
        .output()
        .expect("docker runs");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), text)
}

/// The network's IPv6 gateway, or the first address of its IPv6 subnet where none is named.
fn ipv6_gateway(network: &str) -> Option<String> {
    let (ok, config) = docker(&[
        "network",
        "inspect",
        "-f",
        "{{range .IPAM.Config}}{{.Gateway}}|{{.Subnet}} {{end}}",
        network,
    ]);
    if !ok {
        return None;
    }
    for pair in config.split_whitespace() {
        let (gateway, subnet) = pair.split_once('|').unwrap_or((pair, ""));
        if let Ok(ip) = gateway.parse::<std::net::Ipv6Addr>() {
            return Some(ip.to_string());
        }
        if let Some((base, bits)) = subnet.split_once('/')
            && let (Ok(base), Ok(bits)) = (base.parse::<std::net::Ipv6Addr>(), bits.parse::<u32>())
            && bits < 127
        {
            let mask = u128::MAX.checked_shl(128 - bits).unwrap_or(0);
            return Some(std::net::Ipv6Addr::from((u128::from(base) & mask) + 1).to_string());
        }
    }
    None
}

#[test]
fn an_ipv6_gateway_left_open_stops_the_run() {
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; the IPv6 gateway cannot be exercised");
        return;
    }
    // IPv4 closed the way the runner closes it, IPv6 left as the daemon gives it: the case 0300 is about.
    let net = "sv-fence-ipv6-open-net";
    let _ = docker(&["network", "rm", net]);
    let (made, said) = docker(&[
        "network",
        "create",
        "--internal",
        "--ipv6",
        "-o",
        "com.docker.network.bridge.gateway_mode_ipv4=isolated",
        net,
    ]);
    if !made {
        println!(
            "this daemon cannot make an internal IPv6 network with an isolated IPv4 gateway: {said}"
        );
        return;
    }
    let Some(gateway) = ipv6_gateway(net) else {
        let _ = docker(&["network", "rm", net]);
        println!("the IPv6 network has no IPv6 subnet; nothing to knock on");
        return;
    };
    // What a container on the network finds at the IPv6 gateway, asked directly: the ground truth the
    // runner's own check is held to.
    let (_, knock) = docker(&[
        "run",
        "--rm",
        "--network",
        net,
        sv_run::docker::PROBE_IMAGE,
        "sh",
        "-c",
        &format!("nc -vv -w 3 {gateway} 9 </dev/null 2>&1; echo \"exit=$?\""),
    ]);
    let verdict = backend.verify_gateway_closed(
        net,
        Some(&["com.docker.network.bridge.gateway_mode_ipv4=isolated"]),
    );
    let _ = docker(&["network", "rm", net]);
    let lower = knock.to_lowercase();
    if lower.contains("refused") || lower.contains("exit=0") {
        println!(
            "the IPv6 gateway {gateway} answered a container on the network; the check must stop the run"
        );
        let refused = verdict.expect_err("an answering IPv6 gateway was not found reachable");
        assert!(
            format!("{refused:?}").contains("reached its gateway"),
            "it stopped for the wrong reason: {refused:?}"
        );
    } else {
        println!(
            "the IPv6 gateway {gateway} did not answer here ({}); the check must still stop the run, \
             since the network has IPv6 (backlog 0410)",
            knock.trim()
        );
        let refused = verdict.expect_err("a network with IPv6 was not refused");
        assert!(
            format!("{refused:?}").contains("has IPv6"),
            "it stopped for the wrong reason: {refused:?}"
        );
    }
}

#[test]
fn a_network_with_ipv6_is_refused_even_with_both_gateways_isolated() {
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; the IPv6 gateway cannot be exercised");
        return;
    }
    let net = "sv-fence-ipv6-closed-net";
    let _ = docker(&["network", "rm", net]);
    let (made, said) = docker(&[
        "network",
        "create",
        "--internal",
        "--ipv6",
        "-o",
        "com.docker.network.bridge.gateway_mode_ipv4=isolated",
        "-o",
        "com.docker.network.bridge.gateway_mode_ipv6=isolated",
        net,
    ]);
    if !made {
        println!(
            "this daemon refuses an isolated IPv6 gateway, so the runner falls back to IPv4 alone: {said}"
        );
        return;
    }
    let verdict = backend.verify_gateway_closed(
        net,
        Some(&[
            "com.docker.network.bridge.gateway_mode_ipv4=isolated",
            "com.docker.network.bridge.gateway_mode_ipv6=isolated",
        ]),
    );
    let _ = docker(&["network", "rm", net]);
    // `sv` asks for no IPv6, and over IPv6 the bridge's link-local address, which no gateway list
    // names, may be the host: a network that has it is refused whatever its gateways (backlog 0410).
    let refused = verdict.expect_err("a network with IPv6 was not refused");
    assert!(
        format!("{refused:?}").contains("has IPv6"),
        "it stopped for the wrong reason: {refused:?}"
    );
}

#[test]
fn the_network_the_runner_makes_is_judged_by_the_branch_it_took() {
    // Backlog 0414: through the runner's own way of making the network, fallback included, saying which
    // branch it took. One made with a way of leaving out the gateway must pass where nothing answers;
    // one made plain must never pass.
    let backend = DockerBackend::new();
    if backend.available().is_err() {
        println!("no container backend here; the runner's network cannot be exercised");
        return;
    }
    let net = "sv-fence-runner-net";
    let _ = docker(&["network", "rm", net]);
    let made = backend
        .make_fenced_network(net)
        .expect("the runner could make its network");
    let verdict = backend.verify_gateway_closed(net, made);
    let _ = docker(&["network", "rm", net]);
    match made {
        Some(options) => {
            println!("the runner made its network with {options:?}");
            assert!(
                verdict.is_ok(),
                "the runner's network, made with {options:?}: {verdict:?}"
            );
        }
        None => {
            println!("the runner fell back to --internal alone");
            assert!(verdict.is_err(), "a plain network passed");
        }
    }
}
