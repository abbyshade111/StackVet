# The readiness poll sleeps a fixed 2 s between tries, so an app up in one second is seen at about four

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** `output_with_input` already backs off 1 to 50 ms (`crates/sv-run/src/lib.rs:1010-1024`); the readiness poll does not.

**Where.** `crates/sv-run/src/docker.rs:2118-2146` (`wait_until_ready`): per loop one `docker inspect`, one exec `wget`, then `sleep 2 s`, up to `READY_TIMEOUT_SECONDS = 60`; the same step at `liveness` (`:2096`) and the switched-off start (`:1053`).

**Evidence.** The fixed sleep.

**How it bites.** Up to about 3 s per container start, several starts per run.

**Fix.** Start at 250 ms and double to 2 s; the docker tests at `docker.rs:3800+` run against a real busybox server, so assert `after_seconds` for an already-up server is under 1.

**Record.** ADR-021 governs readiness and liveness: an "unchanged, because" line (the same checks, sooner).
