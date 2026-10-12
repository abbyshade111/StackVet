# Quadratic grouping over findings in settle and in the report build

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** O(n²) in findings with a `Location` string compare per step.

**Where.** `crates/sv-check/src/finding.rs:503-504` and `:599-600` (`groups.iter().position(...)` per finding); `crates/sv-report/src/dashboard.rs:296`; `crates/sv-report/src/lib.rs:2378-2470` (`build`: per applicable requirement, one scan of all findings and several of `checks` and `by_word`).

**Evidence.** The `position` calls.

**How it bites.** Estimate: 2,000 findings (a large app with `--tools`) is 4 million compares, 50 to 200 ms; 10,000 would be seconds.

**Fix.** Key groups by `(file, line)` in a `HashMap`; one index from requirement id to findings and credits built before the build loop. `tests/one_per_line.rs` and `tests/finding_lines.rs` hold the merging.

**Record.** ADR-023 governs settling: an "unchanged, because" line (the same merges, indexed).
