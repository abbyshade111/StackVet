# The gateway fence is IPv4-only: an IPv6 gateway on the fenced network is neither removed nor knocked on

**Status:** claimed by builder-full, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: the fence. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** A Docker daemon configured to give new networks IPv6 (`daemon.json` `default-network-opts` with `com.docker.network.enable_ipv6=true`, which Docker 27 and later support) gives the `--internal` network an IPv6 subnet whose gateway is the bridge, that is, the host or the Colima/Docker Desktop VM. `verify_gateway_closed` passes on the IPv4 side.

**Where.** `crates/sv-run/src/docker.rs:37-40` (`NO_GATEWAY` sets `gateway_mode_ipv4=isolated` or `inhibit_ipv4=true` only), `:1802` (the inspect template reads gateways and subnets of every family), `:2800-2818` (`gateway_targets` keeps only those that parse as `Ipv4Addr`), `:1836` (`ip -4 -o addr show`).

**Evidence.** `if let Ok(ip) = gateway.parse::<std::net::Ipv4Addr>()` and the failure text "Docker names no IPv4 gateway or subnet".

**How it bites.** The app reaches the host's SSH or any listener at `[<v6-gateway>]:22`: S2 of the deep review again, over IPv6. Not reproduced (no Docker in the review's sandbox); on default Docker Desktop and Colima IPv6 is off, so this depends on the owner's configuration.

**Fix.** Add `com.docker.network.bridge.gateway_mode_ipv6=isolated` beside the IPv4 option; parse IPv6 gateways and subnets in `gateway_targets` and knock on them (`nc -6`); refuse the run when the network reports an IPv6 subnet neither option isolated; a test on a daemon with IPv6 on, saying which branch it took without one.

**Record.** ADR-019 governs the fence: a Later entry (the fence holds over IPv6 too).
