# An unsealed `by = "owner"` entry is reported as the tool's word, but nothing tells the tool or the person that it was written by the tool

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: medium.

**What happened.** The tool wrote `by = "owner"` on four `[design]` answers and three `[checked-by-hand]` "done" entries
with a `how` it composed, against the pasted rule. sv counted them as the tool's word (right: unsealed), and `sv review`
then asked the owner "Type owner if this is your own answer", showing the tool's text as if it were theirs; the owner
sealed it.

**Fix.** `stackvet_check` names each unsealed `by = "owner"` line as "written by the AI coding tool, not yet yours";
`sv review`, when it meets one, says "this answer and its how were written by the tool; confirm only what you have
checked yourself" before asking. The prompt side is 0580 (the owner's choice A).

**Record.** ADR-026 Later (what the review shows before it asks).
