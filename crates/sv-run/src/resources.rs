//! What the container engine says about a running app's resource use (backlog 0243, the research of 11 October
//! 2026, and the owner's approval to build it).
//!
//! `docker stats --no-stream` gives, for one container, its CPU as a percentage, its memory as used and limit, and
//! the network and disk traffic since it started (the last two are running totals, so a rate needs two readings).
//! `docker inspect` gives the CPU and memory limits it was started with. This module reads those words into
//! numbers. It starts nothing and asks the engine nothing: the run asks, and passes the words here.

/// One reading of a running container: what the engine said at one moment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reading {
    /// The CPU the container used at that moment, as a percentage of one CPU.
    pub cpu_percent: f64,
    /// Memory in use, and the limit it runs under, in bytes.
    pub mem_used: u64,
    pub mem_limit: u64,
    /// Bytes received and sent over the network since the container started.
    pub net_in: u64,
    pub net_out: u64,
    /// Bytes read from and written to disk since the container started.
    pub disk_read: u64,
    pub disk_write: u64,
}

/// The limits a container was started with. Zero means no limit was set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// CPU, in billionths of one CPU (500000000 is half a CPU).
    pub nano_cpus: u64,
    /// Memory, in bytes.
    pub mem_bytes: u64,
}

/// What a run took of its app's container: one reading, and the limits it started with (backlog 0243).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Taken {
    pub reading: Reading,
    pub limits: Limits,
}

/// The `--format` `docker stats` is asked for, in the order `parse_stats` reads it.
pub const STATS_FORMAT: &str = "{{.CPUPerc}} | {{.MemUsage}} | {{.NetIO}} | {{.BlockIO}}";

/// The `--format` `docker inspect` is asked for, in the order `parse_limits` reads it.
pub const LIMITS_FORMAT: &str = "nanocpus={{.HostConfig.NanoCpus}} mem={{.HostConfig.Memory}}";

/// Reads one line of `docker stats --no-stream --format "{{.CPUPerc}} | {{.MemUsage}} | {{.NetIO}} | {{.BlockIO}}"`,
/// such as `0.00% | 404KiB / 64MiB | 736B / 126B | 0B / 0B`. `None` for any line that is not that: the run then says it
/// could not read the engine, rather than guessing.
pub fn parse_stats(line: &str) -> Option<Reading> {
    let parts: Vec<&str> = line.split('|').map(str::trim).collect();
    let [cpu, memory, net, disk] = parts.as_slice() else {
        return None;
    };
    let cpu_percent = cpu.strip_suffix('%')?.parse::<f64>().ok()?;
    if !cpu_percent.is_finite() || cpu_percent < 0.0 {
        return None;
    }
    let (mem_used, mem_limit) = pair(memory)?;
    let (net_in, net_out) = pair(net)?;
    let (disk_read, disk_write) = pair(disk)?;
    Some(Reading {
        cpu_percent,
        mem_used,
        mem_limit,
        net_in,
        net_out,
        disk_read,
        disk_write,
    })
}

/// Reads `nanocpus=N mem=M`, as the run asks `docker inspect` to print the two limits. `None` for anything else.
pub fn parse_limits(text: &str) -> Option<Limits> {
    let mut nano_cpus = None;
    let mut mem_bytes = None;
    for field in text.split_whitespace() {
        let (key, value) = field.split_once('=')?;
        let value: u64 = value.parse().ok()?;
        match key {
            "nanocpus" => nano_cpus = Some(value),
            "mem" => mem_bytes = Some(value),
            _ => return None,
        }
    }
    Some(Limits {
        nano_cpus: nano_cpus?,
        mem_bytes: mem_bytes?,
    })
}

/// Two sizes joined by " / ", as `docker stats` writes its use-against-limit and in-against-out columns.
fn pair(text: &str) -> Option<(u64, u64)> {
    let (first, second) = text.split_once(" / ")?;
    Some((size(first.trim())?, size(second.trim())?))
}

/// A size as `docker stats` writes it: a number and a unit. Memory uses binary units (`KiB`, `MiB`, `GiB`); network
/// and disk traffic use decimal ones (`kB`, `MB`, `GB`), as the engine does. A bare `B` is bytes.
fn size(text: &str) -> Option<u64> {
    let split = text
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(text.len());
    let (number, unit) = text.split_at(split);
    let number: f64 = number.parse().ok()?;
    let factor: f64 = match unit {
        "B" => 1.0,
        "kB" | "KB" => 1e3,
        "KiB" => 1024.0,
        "MB" => 1e6,
        "MiB" => 1024.0 * 1024.0,
        "GB" => 1e9,
        "GiB" => 1024.0 * 1024.0 * 1024.0,
        _ => return None,
    };
    if !number.is_finite() || number < 0.0 {
        return None;
    }
    Some((number * factor).round() as u64)
}

#[cfg(test)]
#[path = "resources_tests.rs"]
mod tests;
