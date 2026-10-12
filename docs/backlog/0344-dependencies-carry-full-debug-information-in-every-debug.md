# Dependencies carry full debug information in every debug build and in each of about 81 grammar-laden test binaries

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: build time. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** Each of the about 81 test binaries that link `sv-check` (34 of `sv-cli`'s 133 integration files, 34 of `sv-check`'s 40, all of `sv-run`'s and `sv-report`'s) carries the debug information for 35 MB of grammar tables and 84 crates, and the linker copies it each time. The gain of `opt-level = 2` was measured (126 s to 32 s on 46 rule tests, #1013); its cost, and the debug-info cost, were not.

**Where.** `Cargo.toml` `[profile.dev.package."*"]` sets `opt-level = 2` and nothing else, so every dependency is compiled with full DWARF and the C grammars with `-g`.

**Evidence.** The profile block; the counts from `Cargo.lock` (91 packages, 84 from crates.io) and the test files (199 binaries, about 81 linking the grammars).

**How it bites.** Estimate: 10 to 30 percent of the 126-second build-the-tests step, and a much smaller `target/` folder (the owner's disk is nearly full; `CLAUDE.md` says each target folder is 1 to 7 GB).

**Fix.** Add `debug = false` under `[profile.dev.package."*"]` (or `debug = "line-tables-only"` on `[profile.dev]`); the workspace crates keep full debug information, so a failing test's backtrace still names `sv`'s own files and lines; only stepping into a dependency in a debugger is lost. Measure before and after, as design entry 0053 did for the release profile, and record both the opt-level cost and this.

**Record.** ADR-020 governs the dependencies and the profiles: a Later line with the measurements.
