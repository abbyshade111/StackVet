# Incremental compilation is on in CI, where every build is fresh

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: build time. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Cargo's dev profile defaults to `incremental = true`, which in a from-scratch build only adds work: the workspace crates (`sv-check` is large) are split into many codegen units and an incremental cache is written that no later run reads.

**Where.** No `CARGO_INCREMENTAL`, `RUSTFLAGS`, or `.cargo/config.toml` anywhere (`.github/workflows/*.yml`; no `.cargo/` folder).

**Evidence.** The absence of the setting in every workflow.

**How it bites.** Estimate: 5 to 15 percent of the workspace crates' compile per job.

**Fix.** `env: CARGO_INCREMENTAL: "0"` at the top of `rust.yml` and `windows.yml`; local builds are unaffected. Measure once.

**Record.** ADR-106 governs `rust.yml`: an "unchanged, because" line (no artifact changes).
