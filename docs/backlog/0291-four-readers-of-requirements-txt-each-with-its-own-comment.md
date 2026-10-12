# Four readers of requirements.txt, each with its own comment and option rules, and no test holding them to one another

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: code quality. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** All four handle `\` continuations, but they already differ: `python_wants` ends a comment at `" #"` or `"\t#"`, the other three at `" #"` only; `fully_hash_pinned` treats any `-r` line as not pinned while `from_requirements` follows it.

**Where.** `crates/sv-scan/src/ecosystems.rs:523` (`fully_hash_pinned`), `crates/sv-check/src/sbom/lockfiles.rs:617` (`from_requirements`), `crates/sv-check/src/manifest_lock.rs:257` (`python_wants`), `crates/sv-run/src/install.rs:391` (`unpinned`).

**Evidence.** The four functions at the lines above.

**How it bites.** The same file can be judged a hash-pinned lockfile by one reader and read as a different package list by another, and a fix to one reader (a new pip option) is made in one place out of four.

**Fix.** One `requirements` module in `sv-scan` that yields logical lines (comments and continuations resolved) for the three check-side readers to consume; a test that feeds one fixture through all four and compares the names.

**Record.** ADR-052 governs the install step's reading of dependency files: a Later line if `install.rs`'s reading moves; otherwise "unchanged, because".
