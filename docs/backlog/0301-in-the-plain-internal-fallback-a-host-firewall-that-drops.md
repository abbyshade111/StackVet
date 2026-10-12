# In the plain --internal fallback, a host firewall that drops rather than refuses makes the gateway knock pass while other ports stay reachable

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: the fence. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** `let closed = code.is_some() && (lower.contains("timed out") || ...)`. Only the fallback branch is exposed; with either isolation option the bridge has no address at all.

**Where.** `crates/sv-run/src/docker.rs:43` (`GATEWAY_PORT = "9"`), `:559-562` (the plain `--internal` fallback keeps a gateway address), `:2880-2884` (a knock that timed out reads as `Closed`).

**Evidence.** The three locations above.

**How it bites.** An older Docker on a Linux host (the fallback branch) with ufw or nftables set to drop unsolicited input from the docker bridge except port 22: the knock on port 9 times out, the verdict is Closed, and the app reaches the host's SSH.

**Fix.** Treat "made plain internal" as not fenced and refuse the run (the message already says which way the network was made), or knock with something the host's stack must answer at layer 2 (`ping -c1 -W1` of the gateway) rather than one TCP port.

**Record.** ADR-019 governs the fence: a Later entry (what the fallback branch is allowed to conclude).
