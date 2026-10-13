# The run note goes into compliance.md unescaped, and the health path in it is not validated

**Status:** done, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: secrets. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** R13's fix (`inert`) covered app names, paths, and values the app sent back, and missed this line.

**Where.** `crates/sv-report/src/markdown.rs:247-249` writes `report.run_note` raw (`report.html` escapes it at `html.rs:312`); the note holds `plan.health_path` (`crates/sv-cli/src/assemble.rs:1082-1092`), taken from `stackvet.toml` with no check beyond non-empty (`crates/sv-run/src/lib.rs:604`).

**Evidence.** `markdown.rs:247-249`: the note is written without `inert`.

**How it bites.** The AI tool writes `health = "/health?x=[click](https://tracker.example/)"`; the app answers 200, the run proceeds, and `compliance.md` carries a live link or image beacon wherever it is rendered (GitHub, a wiki).

**Fix.** `inert(note)` at `markdown.rs:248`; restrict `health` to `/` plus RFC 3986 unreserved characters and `?=&`; a test with a bracketed health path.

**Record.** No record governs `markdown.rs` by name; say so.
