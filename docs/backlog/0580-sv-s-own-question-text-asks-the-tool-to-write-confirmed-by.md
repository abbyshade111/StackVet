# sv's own question text asks the tool to write `confirmed = { by = "ai-tool" }` proposals and `[checked-by-hand]` entries, which prompt 5 now forbids

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. A decision for the owner. Severity: high (the project tells
the tool two opposite things).

**What was seen.** `crates/sv-report/src/interview.rs:46` and `:148` (the text `stackvet_check` gives the tool for its
questions, since 4 October) say: write your proposed check beside your answer as `confirmed = { by = "ai-tool", how =
"what to look at, and what it should show" }`, and for a check you made yourself a `[checked-by-hand]` entry with
`by = "ai-tool"`. Both Antigravity runs did exactly that (the first trial's reading called the blocks "invented"; they
were not). Prompt 5, since 0547 (#1936), says "Do not write `confirmed = ...` or any `[checked-by-hand]` line". In the
Opus run the mechanism worked as designed: `sv review` showed each proposal to the owner, who confirmed it, and the
report counts them as the owner's (attested 3, by hand 2, documented 3).

**The choice.** (A, recommended) Keep sv's mechanism: the tool proposes how to check, the person confirms through
`sv review`, and nothing counts until they do. Change prompt 5 back to match: "write your proposed check as
`confirmed = { by = "ai-tool", how = ... }`; never `by = "owner"`; it counts only when I confirm it in sv review", and
drop the warning 0543 asked for. (B) Forbid proposals: remove them from `interview.rs` and keep prompt 5 as it is;
the person then writes every `how` themselves in sv review. (C) Keep both texts as they are (not recommended).

**Record.** ADR-026 (what `sv review` records) Later, either way; the prompt-library test holds the sentence.
