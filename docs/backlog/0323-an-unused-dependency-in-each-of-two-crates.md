# An unused dependency in each of two crates

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: code quality. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Each adds a link into the layering diagram that does not exist in code.

**Where.** `crates/sv-report/Cargo.toml` lists `sv-scan` (no `sv_scan` in `crates/sv-report/src` or `tests`); `crates/sv-run/Cargo.toml` lists `serde` (no `serde::`, `Serialize`, or `Deserialize` in its `src` or `tests`; `serde_json` is used).

**Evidence.** The two `Cargo.toml` lines and the empty greps.

**How it bites.** The architecture map's layering reads one edge that is not real.

**Fix.** Delete the two lines; `cargo build` proves it.

**Record.** ADR-020 governs the dependencies: an "unchanged, because" line (two removed, none added).
