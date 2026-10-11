//! Reading what the container engine says about a running app (backlog 0243). The lines below are what
//! `docker stats` and `docker inspect` printed on 11 October 2026, for a busybox container with half a CPU and 64 MiB.

use super::*;

#[test]
fn a_stats_line_the_engine_printed_is_read_whole() {
    let reading = parse_stats("0.00% | 404KiB / 64MiB | 736B / 126B | 0B / 0B").unwrap();
    assert_eq!(reading.cpu_percent, 0.0);
    assert_eq!(reading.mem_used, 404 * 1024);
    assert_eq!(reading.mem_limit, 64 * 1024 * 1024);
    assert_eq!((reading.net_in, reading.net_out), (736, 126));
    assert_eq!((reading.disk_read, reading.disk_write), (0, 0));
}

#[test]
fn cpu_is_a_percentage_of_one_cpu_and_can_exceed_a_hundred() {
    let reading = parse_stats("153.25% | 1MiB / 64MiB | 1kB / 2kB | 4MB / 5MB").unwrap();
    assert_eq!(reading.cpu_percent, 153.25);
    assert_eq!((reading.net_in, reading.net_out), (1_000, 2_000));
    assert_eq!(
        (reading.disk_read, reading.disk_write),
        (4_000_000, 5_000_000)
    );
}

#[test]
fn memory_uses_binary_units_and_traffic_uses_decimal_ones() {
    let memory = parse_stats("0% | 1GiB / 2GiB | 1GB / 1GB | 1B / 1B").unwrap();
    assert_eq!(memory.mem_used, 1024 * 1024 * 1024);
    assert_eq!(memory.mem_limit, 2 * 1024 * 1024 * 1024);
    assert_eq!(memory.net_in, 1_000_000_000);
    assert_eq!(memory.disk_read, 1);
}

#[test]
fn a_line_that_is_not_the_format_is_refused_not_guessed() {
    for line in [
        "",
        "0.00% | 404KiB / 64MiB | 736B / 126B",
        "n/a | 404KiB / 64MiB | 736B / 126B | 0B / 0B",
        "0.00% | 404 / 64MiB | 736B / 126B | 0B / 0B",
        "0.00% | 404XB / 64MiB | 736B / 126B | 0B / 0B",
        "-1.00% | 404KiB / 64MiB | 736B / 126B | 0B / 0B",
        "0.00% | 404KiB | 736B / 126B | 0B / 0B",
    ] {
        assert_eq!(parse_stats(line), None, "{line:?} must be refused");
    }
}

#[test]
fn the_limits_the_container_started_with_are_read() {
    assert_eq!(
        parse_limits("nanocpus=500000000 mem=67108864"),
        Some(Limits {
            nano_cpus: 500_000_000,
            mem_bytes: 67_108_864,
        })
    );
}

#[test]
fn zero_limits_mean_none_were_set_and_are_said_as_zero() {
    assert_eq!(
        parse_limits("nanocpus=0 mem=0"),
        Some(Limits {
            nano_cpus: 0,
            mem_bytes: 0,
        })
    );
}

#[test]
fn limits_missing_a_field_or_with_another_are_refused() {
    assert_eq!(parse_limits("nanocpus=500000000"), None);
    assert_eq!(parse_limits("mem=67108864"), None);
    assert_eq!(parse_limits("nanocpus=500000000 mem=67108864 disk=1"), None);
    assert_eq!(parse_limits("nanocpus=half mem=67108864"), None);
}
