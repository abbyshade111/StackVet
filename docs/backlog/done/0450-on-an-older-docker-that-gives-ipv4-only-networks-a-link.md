# On an older Docker that gives IPv4-only networks a link-local IPv6 address, the fence refuses every run with daemon.json advice that cannot apply

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-run`, `sv-report`, `sv-scan`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-run/src/docker.rs:3085-3100`: any `sv-inet6=` line but loopback returns `GatewayVerdict::Unknown("the fenced network has IPv6 (...), though sv asked Docker for none: the daemon gives new networks IPv6 by default. Turn that off (remove `com.docker.network.enable_ipv6` from `default-network-opts` in Docker's daemon.json, and restart Docker), then run again")`; `knock_script` at `:2920-2930` lists every `inet6` address, `fe80::` included (by design, 0410).
- What is wrong: Before Docker set `net.ipv6.conf.<iface>.disable_ipv6=1` on container interfaces of IPv4-only
  networks (I believe Docker 26.0, March 2024; it may be 27.0), the kernel gave every container a link-local
  `fe80::` address on `eth0` even on a network made with `--ipv6=false`. On such a daemon the knocking container
  always reports an `inet6 fe80::...` line, every run is refused, and the refusal tells the person to remove
  `com.docker.network.enable_ipv6` from `default-network-opts`, which is not set and is not the cause. The refusal is
  the right outcome (0410's rule is that any IPv6 on the network stops the run), but the advice cannot be followed.
- What to do: When the only IPv6 addresses seen are link-local (`fe80::/10`) and Docker's server version is below the
  one that disables IPv6 on IPv4-only networks (read with `docker version --format '{{.Server.Version}}'`, which the
  code already calls elsewhere for `NCPU`), say that the Docker is too old for the fence to hold over IPv6 and name
  the version to move to, instead of the daemon.json advice. Keep the refusal itself.
- How verified: read; the Docker version boundary is from memory, not checked (no Docker reachable from the sandbox).
- Overlap: 0410 (done) chose to refuse on any IPv6 address and wrote the daemon.json advice; it considered Docker 27
  (IPv6-by-default without gateway modes), not a daemon that assigns link-local addresses on IPv4-only networks.
  0300 and 0414 do not cover it.
- Needs a decision record?: no (the refusal stands; only the words change). ADR-019 would get an "unchanged, because" line.
