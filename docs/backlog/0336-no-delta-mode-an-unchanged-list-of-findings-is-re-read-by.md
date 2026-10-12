# No delta mode: an unchanged list of findings is re-read by the AI tool after every edit

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** A `stackvet_check` answer is about 19,000 characters (about 4,750 tokens) for the flask-booking example with the questions counted (`mcp/fold_tests.rs:136-138`), 38 to 50 KB in the pilot's builds (`parts.rs:1-22`). After a one-finding fix the tool re-reads all of it.

**Where.** `crates/sv-cli/src/mcp/tools.rs`, `catalog.rs`, `check_text.rs`, `parts.rs`: no `since` and no "unchanged" mode anywhere; yet the server already keeps the last check's fingerprints for the build-loop record (`mod.rs`, `write_line`, `last_fingerprints`).

**Evidence.** The absence of any `since` parameter, and the fingerprints already held per session.

**How it bites.** Estimate: most of a 19,000-character check (about 4,000 tokens) per check in a loop that checks after every edit; the pilot's tools checked tens of times per build.

**Fix.** By default, findings whose fingerprint was in this session's previous answer come as one line each (fingerprint, rule, file:line) under "N unchanged since the check at <time>", new ones in full, gone ones named; `all` gives everything as now. Must not lose: the counts and the not-examined list every time; "unchanged" must say it means the same fingerprint, not the same code; nothing left out of `all`.

**Record.** ADR-066 governs what the tool is told: a Later entry, since what an answer holds by default changes.

**The owner's word, 12 October 2026:** held until the loop test (0257) has finished, since this changes what the AI coding tool
is told, and the trial's comparisons should not move under it; the same reason as 0338.
