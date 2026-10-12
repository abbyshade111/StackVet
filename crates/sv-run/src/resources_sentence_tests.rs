//! What the report says of a run's resource readings (ADR-093): the engine's numbers in its own units, the limits or
//! their absence, and that they decide nothing. The reading is the one `docker stats` printed on 11 October 2026.

use super::*;

fn taken(nano_cpus: u64, mem_bytes: u64) -> Taken {
    Taken {
        reading: parse_stats("0.00% | 404KiB / 64MiB | 736B / 126B | 0B / 0B").unwrap(),
        limits: Limits {
            nano_cpus,
            mem_bytes,
        },
    }
}

#[test]
fn a_reading_is_said_in_the_engines_units_with_its_limits() {
    let said = sentence(Some(&taken(500_000_000, 64 * 1024 * 1024)));
    for part in [
        "0% of one CPU",
        "404 KiB of memory (of 64 MiB it could use)",
        "received 736 B and sent 126 B",
        "read 0 B and written 0 B",
        "a limit of 0.5 CPU and a memory limit of 64 MiB",
        "they decide no requirement",
    ] {
        assert!(said.contains(part), "{part:?} is missing from: {said}");
    }
}

#[test]
fn a_container_started_without_limits_says_so() {
    let said = sentence(Some(&taken(0, 0)));
    assert!(said.contains("no CPU limit and no memory limit"), "{said}");
}

#[test]
fn larger_numbers_are_scaled_and_trimmed() {
    let mut t = taken(2_000_000_000, 0);
    t.reading.cpu_percent = 153.25;
    t.reading.net_out = 1_500_000;
    t.reading.disk_write = 2_000_000_000;
    let said = sentence(Some(&t));
    for part in [
        "153.25% of one CPU",
        "sent 1.5 MB",
        "written 2 GB",
        "a limit of 2 CPU",
    ] {
        assert!(said.contains(part), "{part:?} is missing from: {said}");
    }
}

#[test]
fn no_reading_is_said_rather_than_left_out() {
    let said = sentence(None);
    assert!(said.starts_with("Docker did not say"), "{said}");
    assert!(
        !said.contains('%'),
        "a reading that was not taken shows no number: {said}"
    );
}
