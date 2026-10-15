# `confirmed = { by = "ai-tool" }` and `[checked-by-hand] ... by = "ai-tool"` are accepted without a word at write time

**Status:** claimed by documentation, 14 October 2026

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
