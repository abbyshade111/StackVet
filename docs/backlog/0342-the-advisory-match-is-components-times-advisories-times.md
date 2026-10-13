# The advisory match is components times advisories times affected ranges, with two string normalizations per PyPI comparison

**Status:** claimed by builder-full, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: sv run speed. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** An OSV PyPI export is tens of thousands of records: 300 components × 20,000 advisories × about 2 affected is about 12 million comparisons and 24 million allocations.

**Where.** `crates/sv-check/src/advisories.rs:604-623` (the three nested loops) and `:548-554` (`same_package`: `python_name(advisory) == python_name(ours)` allocates two normalized strings on every comparison); `read_database` at `:144-180` parses one file at a time.

**Evidence.** The loops and `same_package` as quoted.

**How it bites.** Estimate: several seconds per run with `--advisories` on a PyPI app, growing with the database the owner downloads.

**Fix.** Build once a map from (ecosystem, normalized name) to the advisories naming it, normalizing each advisory's name once and dropping withdrawn ones at index time; look each component up. Parse the advisory files on one thread per core. The existing `audit_against` tests hold the behavior (PEP 503 folding included); add one that `Jupyter_Server` still matches `jupyter-server` through the index.

**Record.** No record governs `advisories.rs` by name; the advisory data stays the owner's download (CLAUDE.md); say so.
