# The name server's builder carries its own copy of the hardening, against ADR-019's record, and the chokepoint test does not cover it

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: the fence. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** ADR-019's entry of 8 October says "The five arguments now go on in `prepared` ... and no builder carries a copy". The test never passes `name_server::start_args` through `hardened_by_prepared`, which would fail on it. Today the five flags are sent twice, which Docker accepts, so the container is hardened; the record is wrong and the guard for the name server is accidental.

**Where.** `crates/sv-run/src/name_server.rs:45` (`crate::docker::hardened(args...)`) and `docker.rs:1912-1914` (`self.docker(&args)` runs `prepared`, which hardens again); the test `every_container_started_is_hardened_in_one_place_and_a_plain_exec_is_not` at `docker.rs:3357`.

**Evidence.** The `hardened()` call and the test's list of builders.

**How it bites.** A later change to `prepared` leaves the name server hardened by its own stale copy, and nothing says so.

**Fix.** Delete the `hardened()` call in `start_args` and add `hardened_by_prepared(&start_args("n", "d"))` to the chokepoint test; break it first to see the test go red.

**Record.** ADR-019 governs: an "unchanged, because" line (the code is brought to the record).
