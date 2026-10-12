# Outside tools run one after another, their start-ups never overlapping

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Semgrep, Opengrep, Bandit, gosec, Brakeman, and CodeQL start one at a time. Semgrep and CodeQL are multi-core themselves, so running all at once helps only with the smaller tools' start-up; the saving is their sum, not a multiple.

**Where.** `crates/sv-check/src/adapters.rs:1636-1650`: `for adapter in adapters.for_languages(languages) { … run_one_recorded(...) }`.

**Evidence.** The loop.

**How it bites.** Seconds per small tool; rate it medium only if the tool timings in a real report show several short tools.

**Fix.** Run the single-threaded tools (Bandit, Brakeman, gosec) beside one heavy one; keep the output order deterministic by collecting then sorting by adapter id. `tests/tools_interrupt.rs` and `tests/adapters.rs` hold behavior.

**Record.** ADR-032 governs the adapters: an "unchanged, because" line (the same tools, the same cleared environment).
