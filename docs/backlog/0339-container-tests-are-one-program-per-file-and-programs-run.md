# Container tests are one program per file and programs run one after another, so about half the suite's time is apps coming up in sequence

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: test and CI time. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** Within a file the tests run on threads at once; across files nothing overlaps, because `cargo test` runs the programs in sequence. Each container file takes 20 to 78 s on the Mac (`docs/paper/LOCAL-TEST-FAILURES.md`: `email_change_example` 78 s, `seen_record` 75 s, `loopback_start` 66 s, `host_cookie` 62 s, `report_lock` 26 s, `killed_run` 20 s) and 20 to 80 s in CI (0270), most of it waiting for the app to come up.

**Where.** `crates/sv-cli/tests/` (133 files, each a separately linked test program; `sv-check` 40 more). The files that start an app in a container: `email_change_example.rs`, `host_cookie.rs`, `seen_record.rs`, `loopback_start.rs`, `run_progress.rs`, `report_suite_timings.rs`, `report_lock.rs`, `killed_run.rs`, `interrupt.rs`, `review_container.rs`, `tools_interrupt.rs`, `history_unfinished.rs`, `connect.rs`.

**Evidence.** Measured from the last six runs on `main`: before the three-part split the Tests step's median was 832 s, of which about 125 s is compiling (0270's "Build the tests" 126 s) and about 700 s running; 0270 names nine container programs at 20 to 80 s each, roughly 400 to 500 s of that. After the split (one run): parts of 431, 392, and 358 s, the required check green at about 7.5 minutes.

**How it bites.** Estimate: 3 to 5 minutes of run time across the two `sv-cli` parts, so about 1.5 to 2.5 minutes off the critical path. The owner declined cargo-nextest (0270), which would have overlapped the programs.

**Fix.** Put the container tests of one kind into one file (the six that run `sv report --run` on a copied example app, for a start), so the harness runs them on threads inside one process; six 60-second waits become one. Must not lose: each test still starts its own app copy (they change `app.py` differently); `SV_REQUIRE_BACKEND=1` still fails when no backend answers; `tools/ci_shards.py` still deals every file to exactly one part (fewer files, same rule). Measure before and after on a branch, as 0270 did.

**Record.** ADR-106 governs the dealing of test files: an "unchanged, because" line (fewer files, same rule), or a Later line if the deal's balance changes.
