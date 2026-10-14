//! What the fence says when the network it made shows IPv6 (backlog 0450). The run is refused either way (0410); on a
//! Docker before 26.0.2, which leaves IPv6 on in containers of networks made without it (CVE-2024-32473), link-local
//! addresses alone are that Docker's doing, and the daemon.json advice cannot be followed.

use super::*;

/// The knocking container's report: its addresses listed, the control refused, one knock timed out, and `inet6`.
fn out(inet6: &[&str]) -> String {
    let lines: String = inet6.iter().map(|a| format!("sv-inet6={a}\n")).collect();
    format!(
        "sv-addrs=1\n{lines}nc: can't connect to remote host (127.0.0.1): Connection refused\nsv-self=1\nnc: timed out\nsv-gateway=1\n"
    )
}

fn why(out: &str, version: Option<&str>) -> String {
    let version = version.map(str::to_owned);
    match gateway_verdict(out, None, 1, &|| version.clone()) {
        GatewayVerdict::Unknown(why) => why,
        other => panic!("the run was not refused: {other:?}"),
    }
}

#[test]
fn link_local_alone_on_a_docker_before_26_0_2_names_the_version_to_move_to() {
    let link_local = out(&["fe80::42:acff:fe12:2/64"]);
    for version in ["26.0.1", "26.0.0", "24.0.7", "20.10.21", "25.0.5-ce"] {
        let said = why(&link_local, Some(version));
        assert!(said.contains(version), "{version}: {said}");
        assert!(said.contains("26.0.2"), "{version}: {said}");
        assert!(said.contains("CVE-2024-32473"), "{version}: {said}");
        assert!(said.contains("fe80::42:acff:fe12:2/64"), "{said}");
        assert!(
            !said.contains("daemon.json"),
            "advice that cannot apply: {said}"
        );
    }
}

#[test]
fn the_daemon_json_advice_stays_where_it_can_apply() {
    // A newer Docker, a version that cannot be read, or an address that is not link-local: the daemon gives the network
    // IPv6, and daemon.json is where that is turned off.
    for (inet6, version) in [
        (&["fe80::1/64"][..], Some("26.0.2")),
        (&["fe80::1/64"][..], Some("27.3.1")),
        (&["fe80::1/64"][..], None),
        (&["fe80::1/64"][..], Some("dev")),
        (&["fd00:9::2/64"][..], Some("24.0.7")),
        (&["fe80::1/64", "fd00:9::2/64"][..], Some("24.0.7")),
    ] {
        let said = why(&out(inet6), version);
        assert!(
            said.contains("default-network-opts"),
            "{inet6:?} on {version:?}: {said}"
        );
        assert!(!said.contains("CVE-2024-32473"), "{said}");
    }
}

#[test]
fn the_version_is_asked_only_when_link_local_alone_is_seen() {
    let asked = std::cell::Cell::new(0);
    let version = || {
        asked.set(asked.get() + 1);
        Some("24.0.7".to_owned())
    };
    gateway_verdict(&out(&["fd00:9::2/64"]), None, 1, &version);
    gateway_verdict(&out(&[]), None, 1, &version);
    assert_eq!(asked.get(), 0, "asked with no link-local address alone");
    gateway_verdict(&out(&["fe80::1/64"]), None, 1, &version);
    assert_eq!(asked.get(), 1);
}

#[test]
fn link_local_is_fe80_slash_10_and_nothing_else() {
    for address in [
        "fe80::1/64",
        "FE80::1",
        "febf::1/64",
        "fe80::42:acff:fe12:2",
    ] {
        assert!(link_local(address), "{address}");
    }
    for address in [
        "fec0::1/64",
        "fd00::1/64",
        "::1/128",
        "2001:db8::1/64",
        "fe8",
        "",
    ] {
        assert!(!link_local(address), "{address}");
    }
}
