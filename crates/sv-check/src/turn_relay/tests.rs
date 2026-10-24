//! Backlog 0625, part 3, ADR-146: a coturn relay's own settings, from `turnserver.conf` or a compose service's flags,
//! refusing every reserved range and setting its quotas is credited in part; one that leaves a range open, or sets
//! no quota, is a finding that says what; and no relay settings at all say nothing.

use super::*;
use crate::test_support::scratch;

/// Every range a relay must refuse, as `denied-peer-ip` lines, with the two flags.
const SAFE: &str = "listening-port=3478\n\
    denied-peer-ip=0.0.0.0-0.255.255.255\n\
    denied-peer-ip=10.0.0.0-10.255.255.255\n\
    denied-peer-ip=100.64.0.0-100.127.255.255\n\
    denied-peer-ip=169.254.0.0-169.254.255.255\n\
    denied-peer-ip=172.16.0.0-172.31.255.255\n\
    denied-peer-ip=192.168.0.0-192.168.255.255\n\
    denied-peer-ip=fc00::-fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff\n\
    denied-peer-ip=fe80::-febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff\n\
    no-multicast-peers\n\
    user-quota=12\n\
    total-quota=1200\n";

fn report_of(files: &[(&str, &str)]) -> ConfigReport {
    let dir = scratch("");
    for (name, text) in files {
        let path = dir.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    let listing = Listing::of(&dir);
    let mut report = ConfigReport::default();
    check(&listing, &mut report);
    std::fs::remove_dir_all(&dir).ok();
    report
}

fn credited(report: &ConfigReport) -> Vec<(&str, bool)> {
    report
        .passed
        .iter()
        .map(|v| (v.check_id.as_str(), v.in_part))
        .collect()
}

fn found(report: &ConfigReport) -> Vec<&str> {
    report.findings.iter().map(|f| f.rule_id.as_str()).collect()
}

#[test]
fn a_relay_that_refuses_every_reserved_range_and_sets_its_quotas_is_credited_in_part() {
    let report = report_of(&[("turn/turnserver.conf", SAFE)]);
    assert!(report.findings.is_empty(), "{:#?}", report.findings);
    assert_eq!(credited(&report), [(PEERS, true), (QUOTA, true)]);
    assert!(
        report.passed[0].scope.contains("turn/turnserver.conf"),
        "{}",
        report.passed[0].scope
    );
    assert!(report.passed[0].scope.contains("not the running relay"));
}

#[test]
fn each_range_left_open_is_named_and_withholds_the_credit() {
    // The setup: the safe file is credited, so what follows is each piece taken out.
    assert_eq!(credited(&report_of(&[("turnserver.conf", SAFE)])).len(), 2);
    for (taken, named) in [
        ("denied-peer-ip=10.0.0.0-10.255.255.255\n", "10.0.0.0/8"),
        (
            "denied-peer-ip=172.16.0.0-172.31.255.255\n",
            "172.16.0.0/12",
        ),
        (
            "denied-peer-ip=fc00::-fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff\n",
            "fc00::/7",
        ),
        ("no-multicast-peers\n", "no-multicast-peers"),
    ] {
        let report = report_of(&[("turnserver.conf", &SAFE.replace(taken, ""))]);
        assert_eq!(found(&report), [PEERS], "{named}");
        assert!(
            report.findings[0].description.contains(named),
            "{}",
            report.findings[0].description
        );
        assert_eq!(credited(&report), [(QUOTA, true)], "{named}");
    }
    // A range split into two lines that meet is still covered; one that leaves a gap is not.
    let split = SAFE.replace(
        "denied-peer-ip=10.0.0.0-10.255.255.255\n",
        "denied-peer-ip=10.0.0.0-10.127.255.255\ndenied-peer-ip=10.128.0.0-10.255.255.255\n",
    );
    assert!(found(&report_of(&[("turnserver.conf", &split)])).is_empty());
    let gap = SAFE.replace(
        "denied-peer-ip=10.0.0.0-10.255.255.255\n",
        "denied-peer-ip=10.0.0.0-10.127.255.255\ndenied-peer-ip=10.128.0.1-10.255.255.255\n",
    );
    assert_eq!(found(&report_of(&[("turnserver.conf", &gap)])), [PEERS]);
}

#[test]
fn an_allowed_address_inside_a_range_or_loopback_let_through_is_named() {
    for (added, named) in [
        ("allowed-peer-ip=10.0.0.5\n", "let back in"),
        ("allow-loopback-peers\n", "allow-loopback-peers"),
    ] {
        let report = report_of(&[("turnserver.conf", &format!("{SAFE}{added}"))]);
        assert_eq!(found(&report), [PEERS], "{added}");
        assert!(
            report.findings[0].description.contains(named),
            "{}",
            report.findings[0].description
        );
    }
    // An allowed address outside every reserved range is the relay's own business.
    let outside = format!("{SAFE}allowed-peer-ip=203.0.113.7\n");
    assert!(found(&report_of(&[("turnserver.conf", &outside)])).is_empty());
}

#[test]
fn no_quota_or_a_zero_quota_is_a_finding_against_v17_1_2() {
    for quotas in ["", "user-quota=12\n", "user-quota=0\ntotal-quota=0\n"] {
        let text = SAFE
            .replace("user-quota=12\n", "")
            .replace("total-quota=1200\n", "")
            + quotas;
        let report = report_of(&[("turnserver.conf", &text)]);
        assert_eq!(found(&report), [QUOTA], "{quotas:?}");
        assert_eq!(report.findings[0].requirement_ids, ["V17.1.2"]);
        assert_eq!(credited(&report), [(PEERS, true)], "{quotas:?}");
    }
}

#[test]
fn a_coturn_service_in_a_compose_file_is_read_from_its_flags() {
    let flags: Vec<String> = SAFE
        .lines()
        .filter(|l| !l.starts_with("listening-port"))
        .map(|l| format!("--{l}"))
        .collect();
    let listed = flags
        .iter()
        .map(|f| format!("      - \"{f}\"\n"))
        .collect::<String>();
    let compose =
        format!("services:\n  turn:\n    image: coturn/coturn:4.6\n    command:\n{listed}");
    let report = report_of(&[("docker-compose.yml", &compose)]);
    assert!(report.findings.is_empty(), "{:#?}", report.findings);
    assert_eq!(credited(&report), [(PEERS, true), (QUOTA, true)]);
    // As one string, with a flag left out.
    let one = format!(
        "services:\n  turn:\n    image: coturn/coturn\n    command: {}\n",
        flags
            .iter()
            .filter(|f| !f.contains("no-multicast"))
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    );
    let report = report_of(&[("compose.yaml", &one)]);
    assert_eq!(found(&report), [PEERS]);
    assert_eq!(report.findings[0].location.line, 4);
    // A service that is not coturn is not read as a relay.
    let other = "services:\n  web:\n    image: nginx\n    command: --no-multicast-peers\n";
    assert!(
        report_of(&[("docker-compose.yml", other)])
            .passed
            .is_empty()
    );
}

#[test]
fn an_app_with_no_relay_settings_is_told_nothing_and_credited_nothing() {
    let report = report_of(&[("app.py", "print('hello')\n"), ("README.md", "# An app\n")]);
    assert!(report.findings.is_empty() && report.passed.is_empty());
}
