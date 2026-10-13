# The crash sweeps' time grows with the square of the request count, and is not measured since 0271

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: test and CI time. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** They run every request-crash scenario once per request, so each new probe makes them slower; `sv-check`'s lib tests fell from 267 to 279 s to 149 s in CI with 0271, and the sweeps' remaining share is not measured.

**Where.** `crates/sv-check/src/signed_in/mod.rs:4513` and `:4626` (the two sweeps), sharing the thread-per-core work list at `:3922` since 0271.

**Evidence.** The two loops and 0270's before-and-after times.

**How it bites.** A slow creep nobody sees until a part crosses the ceiling.

**Fix.** Time them once (`cargo test -p sv-check --lib crash -- --test-threads=1` in the Terminal panel, outside the sandbox) and write the number into 0271's item; propose nothing until the number is known.

**Record.** No record governs these tests by name; say so.
