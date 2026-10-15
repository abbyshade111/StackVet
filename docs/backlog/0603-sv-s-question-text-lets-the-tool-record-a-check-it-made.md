# sv's question text lets the tool record a check it made itself by reading the code, which prompt 5 forbids

**Status:** claimed by wording, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found while building backlog 0580, 15 October 2026. A decision for the owner. Severity: medium (the project still tells
the tool two different things, on the half of 0580's question its answer did not name).

**What was seen.** `crates/sv-report/src/interview.rs:148`, the text `stackvet_check` gives the tool for its checks by
hand, says: "A check you made yourself, reading the code, is by = \"ai-tool\"; to propose that the person make it too,
add confirmed = { by = \"ai-tool\", how = ... }". Setup prompt 5 (and the guide's step 4) says: "Never mark a check by
hand as done yourself" and "Write a `[checked-by-hand]` entry only for a check I made and described to you, with
`by = \"ai-tool\"` and my words in `how`" (backlog 0547). 0580 matched the prompt to sv on `confirmed` proposals, as the
owner chose; this half was left as it was, since the answer named only that sentence.

**The choice.** (A, recommended) Match the prompt to sv: the tool may write a check it made by reading the code, with
`by = "ai-tool"`, which the report shows as the tool's word (`hand.stated-by-ai`), and propose with `confirmed` that the
person make it too. (B) Match sv to the prompt: drop the sentence from `interview.rs`, so only checks the person made are
written down. (C) Leave both.

**Record.** ADR-026 Later, either way.

**The owner's answer, 15 October 2026:** (A): match the prompt to sv. The tool may record a check it made by reading the code with by = "ai-tool" (shown as the tool's word) and propose with confirmed that the person make it too; prompt 5 and the guide's step 4 say so. ADR-026 Later.
