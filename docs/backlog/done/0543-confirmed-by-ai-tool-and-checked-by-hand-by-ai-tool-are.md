# `confirmed = { by = "ai-tool" }` and `[checked-by-hand] ... by = "ai-tool"` are accepted without a word at write time

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Antigravity (12 `confirmed` blocks), VS Code
(V2.2.1 `done`). Severity: low (sv counts them right; the tool is not told).

**What is wrong.** Antigravity invented `confirmed = { by = "ai-tool", how = "..." }` on 12 `[design]` entries;
`crates/sv-cli/src/assemble.rs:2402` records them "in a way that does not count" and the question is asked again. VS
Code wrote `[checked-by-hand] "V2.2.1" = { result = "done", by = "ai-tool" }`, shown as `hand.stated-by-ai`
(`crates/sv-check/src/hand.rs:199`). Both right; neither `stackvet_check` nor `sv report` tells the tool at write time
that the line is worth nothing, so it carries on.

**Fix.** One warning line in `stackvet_check` naming each such line ("a confirmation by the AI coding tool counts for
nothing; only sv review records a person's confirmation"); consider refusing `confirmed` with `by = "ai-tool"`.

**Record.** ADR-026 Later if anything is refused; none for a warning.

**Done, 15 October 2026.** The report's "What was not examined", and so the start of every `stackvet_check`, has one
entry naming each line in stackvet.toml that only the AI coding tool vouches for (`crates/sv-cli/src/assemble/self_confirmed.rs`):
a `[design]` or `[checked-by-hand]` entry with `confirmed = { by = "ai-tool" }`, and a `[checked-by-hand]` entry with
`result = "done"` and `by = "ai-tool"` that no person confirmed. A confirmation by the tool "counts for nothing"; a
check by hand it writes down "counts only as the tool's word, never as a check, even when it passes on what the person
saw", because setup prompt 6 asks the tool to write one down when the person describes a check, so it is never called
worthless. Each sentence is said only when its kind of line is there, and both end on `sv review` as the one way to
make the line the person's. The tool's own confirmations leave the older entry "N confirmations of what your AI coding
tool said", so nothing is listed twice; a problem the tool reports, a check not yet made, and a check a person
confirmed are not named. Not done: refusing such lines at write time (no ADR-026 entry, as nothing is refused).
Tests: `assemble/self_confirmed_tests.rs` (3), `mcp/self_confirmed_tests.rs` (2), and `tests/confirmations.rs` updated.
