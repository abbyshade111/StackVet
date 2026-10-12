# The rest part is the critical path and the only undealt one: a fourth part of sv-check alone would cut the ceiling

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: test and CI time. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** After the split (one measured run): `rest` 431 s, `cli-1` 392 s, `cli-2` 358 s. `rest` holds every crate but `sv-cli` plus `sv-cli`'s lib tests, including `sv-check`'s 1,518 unit tests (149 s in CI after 0271) and its 40 test files; its content is a crate list, not a dealt set. The fixed cost of a part is about 2.5 to 3 minutes (toolchain, Docker restart, 16 to 25 s of image pulls, py7zr on `rest`, the 125 s build). `cli-1` and `cli-2` came out within 34 s because the twelve container-starting files happened to deal evenly by name; a new file can unbalance that, so the deal is lucky, not designed.

**Where.** `.github/workflows/rust.yml` (the `suite` matrix) and `tools/ci_shards.py`.

**Evidence.** The job times from the one post-split run; the deal listed by `tools/ci_shards.py`.

**How it bites.** Estimate: a fourth part of `sv-check` alone brings `rest` to roughly 280 s and the ceiling to `cli-1`'s 390 s, 40 to 100 s off; beyond about four parts nothing more is gained without a cached build.

**Fix.** A fourth part for `sv-check`, or deal `sv-check`'s test files together with `sv-cli`'s; `ci_shards.rs` must still hold every file to exactly one part; consider dealing by a recorded weight (the container files) rather than by name.

**Record.** ADR-106 governs the split: a Later line.
