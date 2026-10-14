# Small run-speed tidiness: double UTF-8 validation, a lowercased copy of every file, and repeated stand-in readiness checks

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Each is small on its own.

**Where.** `crates/sv-scan/src/files.rs:233` and `:110` (two full UTF-8 validations per file); `crates/sv-scan/src/lib.rs:296` (`contents.to_lowercase()` for 11 signatures in `data/tech-signatures.json`); `crates/sv-run/src/docker.rs:1017-1024` (`http()` runs `provider_ready`/`model_ready`, up to 20 × 500 ms when down, on each of 7 calls from `script.rs`).

**Evidence.** The three locations.

**How it bites.** Milliseconds per file; only when the stand-in is down does the last one cost seconds.

**Fix.** Validate once; lowercase only when a pattern has uppercase, or search all patterns at once case-insensitively; cache one readiness answer per run.

**Record.** ADR-031 and ADR-019 govern: "unchanged, because" lines.
