//! Tests of `docker.rs`: the fence's gateway check: which addresses are knocked on, and what each answer, or none,
//! means. Moved out of `docker.rs` unchanged (backlog 0324).

use super::*;

const REFUSED_SELF: &str =
    "sv-addrs=1\nnc: can't connect to remote host (127.0.0.1): Connection refused\nsv-self=1\n";

/// The verdict, told as many addresses as the output reports on.
fn verdict(out: &str, plain: bool) -> GatewayVerdict {
    let targets = out
        .lines()
        .filter(|l| l.starts_with("sv-gateway=") || l.starts_with("sv-own="))
        .count();
    let reason = plain_reason(None, "");
    gateway_verdict(out, reason.as_deref().filter(|_| plain), targets, &|| None)
}

#[test]
fn a_network_is_plain_by_what_docker_built_not_only_by_what_was_asked() {
    // Backlog 0414: a daemon that takes `gateway_mode_ipv4=isolated` without knowing it builds a
    // plain network, and names its gateway; that is what tells.
    let isolated: &[&str] = &["com.docker.network.bridge.gateway_mode_ipv4=isolated"];
    assert!(
        plain_reason(None, "|172.18.0.0/16 ").is_some_and(|r| r.contains("`--internal` alone"))
    );
    assert_eq!(plain_reason(Some(isolated), "|172.18.0.0/16 "), None);
    let named = plain_reason(Some(isolated), "172.18.0.1|172.18.0.0/16 ");
    assert!(
        named
            .as_deref()
            .is_some_and(|r| r.contains("172.18.0.1") && r.contains("without honoring it")),
        "{named:?}"
    );
    // A gateway of either family.
    assert!(plain_reason(Some(isolated), "|172.18.0.0/16 fd00::1|fd00::/64 ").is_some());
    assert_eq!(plain_reason(Some(isolated), ""), None);
}

#[test]
fn a_plain_network_whose_addresses_are_all_the_containers_own_does_not_pass() {
    let out = format!("{REFUSED_SELF}sv-own=172.18.0.1\n");
    assert_eq!(verdict(&out, false), GatewayVerdict::Closed);
    assert!(
        matches!(verdict(&out, true), GatewayVerdict::Unknown(why) if why.contains("`--internal` alone"))
    );
}

#[test]
fn a_gateway_that_answers_in_any_way_is_reachable() {
    for gateway in [
        "nc: can't connect to remote host (172.20.0.1): Connection refused\nsv-gateway=1\n",
        "sv-gateway=0\n",
    ] {
        assert_eq!(
            verdict(&format!("{REFUSED_SELF}{gateway}"), false),
            GatewayVerdict::Reachable,
            "{gateway}"
        );
    }
}

#[test]
fn a_gateway_that_cannot_be_reached_is_closed() {
    for gateway in [
        "nc: timed out\nsv-gateway=1\n",
        "nc: can't connect to remote host (172.20.0.1): No route to host\nsv-gateway=1\n",
        "nc: can't connect to remote host (172.20.0.1): Network is unreachable\nsv-gateway=1\n",
        "nc: can't connect to remote host (172.20.0.1): Connection timed out\nsv-gateway=1\n",
    ] {
        assert_eq!(
            verdict(&format!("{REFUSED_SELF}{gateway}"), false),
            GatewayVerdict::Closed,
            "{gateway}"
        );
    }
}

#[test]
fn on_a_plain_internal_network_an_unanswered_knock_is_not_a_fence() {
    // Backlog 0301: with `--internal` alone the bridge keeps its address on the host, and a
    // firewall that drops all but SSH times the knock on port 9 out. That silence passed.
    for gateway in [
        "nc: timed out\nsv-gateway=1\n",
        "nc: can't connect to remote host (172.20.0.1): No route to host\nsv-gateway=1\n",
    ] {
        let verdict = verdict(&format!("{REFUSED_SELF}{gateway}"), true);
        assert!(
            matches!(&verdict, GatewayVerdict::Unknown(why) if why.contains("--internal` alone")),
            "{gateway}: {verdict:?}"
        );
    }
    // An answer is still the host answering, and the control still has to hold.
    assert_eq!(
        verdict(
            &format!(
                "{REFUSED_SELF}nc: 172.20.0.1 (172.20.0.1:9): Connection refused\nsv-gateway=1\n"
            ),
            true
        ),
        GatewayVerdict::Reachable
    );
    assert!(matches!(
        verdict("sv-self=0\nnc: timed out\nsv-gateway=1\n", true),
        GatewayVerdict::Unknown(_)
    ));
}

#[test]
fn the_network_asks_for_no_ipv6_and_for_no_gateway_of_either_family_first() {
    // Backlog 0410: nothing went red when the IPv6 option left the list, since the Docker tests
    // make their own networks.
    assert_eq!(
        NO_GATEWAY[0],
        [
            "com.docker.network.bridge.gateway_mode_ipv4=isolated",
            "com.docker.network.bridge.gateway_mode_ipv6=isolated",
        ]
    );
    for options in NO_GATEWAY.iter().copied().chain([&[][..]]) {
        let args = network_create_args(options, "n");
        assert_eq!(
            args[..4],
            ["network", "create", "--internal", "--ipv6=false"],
            "{args:?}"
        );
        assert_eq!(args.last().map(String::as_str), Some("n"));
    }
}

#[test]
fn the_knock_lists_every_address_family_and_each_ipv6_address() {
    // Backlog 0410: `ip -4` would hide an IPv6 address on the network, and its own IPv6 address
    // would be knocked on as the host's.
    let script = knock_script(&["172.18.0.1".to_owned(), "fd00::1".to_owned()]);
    assert!(script.contains("ip -o addr show"), "{script}");
    assert!(
        !script.contains("ip -4") && !script.contains("-o -4"),
        "{script}"
    );
    assert!(
        script.contains("sv-inet6=") && script.contains("sv-addrs="),
        "{script}"
    );
    assert!(script.contains("*\"inet6 fd00::1/\"*"), "{script}");
    assert_eq!(script.matches("sv-gateway=").count(), 2, "{script}");
}

#[test]
fn any_ipv6_address_on_the_network_stops_the_run() {
    // Link-local included: the bridge's fe80:: address is on no gateway list, and may be the host.
    for address in ["fe80::42:acff:fe12:2/64", "fd00:9::2/64"] {
        let out = format!(
            "sv-addrs=1\nsv-inet6={address}\nnc: can't connect to remote host (127.0.0.1): Connection refused\nsv-self=1\nnc: timed out\nsv-gateway=1\n"
        );
        let got = verdict(&out, false);
        assert!(
            matches!(&got, GatewayVerdict::Unknown(why) if why.contains(address) && why.contains("default-network-opts")),
            "{got:?}"
        );
    }
    // A gateway that answers is still said as reached.
    let out = "sv-addrs=1\nsv-inet6=fd00:9::2/64\nnc: can't connect to remote host (127.0.0.1): Connection refused\nsv-self=1\nnc: 172.18.0.1 (172.18.0.1:9): Connection refused\nsv-gateway=1\n";
    assert_eq!(verdict(out, false), GatewayVerdict::Reachable);
}

#[test]
fn a_container_whose_addresses_cannot_be_listed_does_not_pass() {
    let out = "sv-addrs=0\nnc: can't connect to remote host (127.0.0.1): Connection refused\nsv-self=1\nnc: timed out\nsv-gateway=1\n";
    assert!(matches!(verdict(out, false), GatewayVerdict::Unknown(_)));
    let out = "nc: can't connect to remote host (127.0.0.1): Connection refused\nsv-self=1\nnc: timed out\nsv-gateway=1\n";
    assert!(matches!(verdict(out, false), GatewayVerdict::Unknown(_)));
}

#[test]
fn a_knock_that_never_reported_is_not_the_fence_holding() {
    // Output cut short after the first of two knocks (backlog 0410).
    let out = format!("{REFUSED_SELF}nc: timed out\nsv-gateway=1\n");
    assert_eq!(
        gateway_verdict(&out, None, 1, &|| None),
        GatewayVerdict::Closed
    );
    let cut = gateway_verdict(&out, None, 2, &|| None);
    assert!(
        matches!(&cut, GatewayVerdict::Unknown(why) if why.contains("1 of the 2")),
        "{cut:?}"
    );
}

#[test]
fn the_subnets_first_address_is_knocked_on_when_no_gateway_is_named() {
    // What GitHub's Docker said of a network made without a gateway address: no gateway listed.
    assert_eq!(gateway_targets("|172.18.0.0/16 "), vec!["172.18.0.1"]);
    assert_eq!(
        gateway_targets("172.20.0.1|172.20.0.0/16 "),
        vec!["172.20.0.1"]
    );
    assert!(gateway_targets("").is_empty());
}

#[test]
fn an_ipv6_gateway_is_knocked_on_as_well() {
    // Backlog 0300: a daemon that gives new networks IPv6 gives the internal one an IPv6 gateway,
    // the host just as the IPv4 one is. Until then these were dropped, and the fence passed.
    assert_eq!(
        gateway_targets("10.9.0.254|10.9.0.0/24 |fd00::/64 "),
        vec!["10.9.0.1", "10.9.0.254", "fd00::1"]
    );
    assert_eq!(gateway_targets("|fd00:9::/64 "), vec!["fd00:9::1"]);
    assert_eq!(
        gateway_targets("172.18.0.1|172.18.0.0/16 fd5e:1::1|fd5e:1::/64 "),
        vec!["172.18.0.1", "fd5e:1::1"]
    );
    // The host part masked off, as for IPv4, and a subnet too small to have a gateway left alone.
    assert_eq!(gateway_targets("|fd00:9::7/64 "), vec!["fd00:9::1"]);
    assert!(gateway_targets("|fd00::5/127 ").is_empty());
}

#[test]
fn one_answering_address_of_several_is_reachable() {
    let out = format!(
        "{REFUSED_SELF}nc: timed out\nsv-gateway=1\nnc: can't connect to remote host (10.9.0.254): Connection refused\nsv-gateway=1\n"
    );
    assert_eq!(verdict(&out, false), GatewayVerdict::Reachable);
    let out = format!("{REFUSED_SELF}nc: timed out\nsv-gateway=1\nnc: timed out\nsv-gateway=1\n");
    assert_eq!(verdict(&out, false), GatewayVerdict::Closed);
}

#[test]
fn the_knocking_containers_own_address_is_not_the_host() {
    // CI, 4 October 2026: with no gateway, the subnet's first address was the knocking container's
    // own, and its refusal read as the host answering.
    assert_eq!(
        verdict(&format!("{REFUSED_SELF}sv-own=172.18.0.1\n"), false),
        GatewayVerdict::Closed
    );
    // Its own address skipped, another that answers still stops the run.
    let out = format!(
        "{REFUSED_SELF}sv-own=172.18.0.1\nnc: 172.18.0.254 (172.18.0.254:9): Connection refused\nsv-gateway=1\n"
    );
    assert_eq!(verdict(&out, false), GatewayVerdict::Reachable);
}

#[test]
fn a_check_that_cannot_tell_is_never_taken_for_a_fence() {
    let unknown = |out: &str| matches!(verdict(out, false), GatewayVerdict::Unknown(_));
    // The control did not read as refused: an nc that says nothing would pass any gateway.
    assert!(unknown("sv-self=1\nnc: timed out\nsv-gateway=1\n"));
    assert!(unknown("sv-self=0\nnc: timed out\nsv-gateway=1\n"));
    // Nothing ran, or the second knock never reported.
    assert!(unknown(""));
    assert!(unknown(REFUSED_SELF));
    // Something nobody has seen.
    assert!(unknown(&format!(
        "{REFUSED_SELF}sh: nc: not found\nsv-gateway=127\n"
    )));
}
