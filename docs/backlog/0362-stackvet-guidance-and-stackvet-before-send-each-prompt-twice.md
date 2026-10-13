# stackvet_guidance and stackvet_before send each prompt twice, in the text and in the structured result

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Wire cost only (the model reads one of the two), as `parts.rs:17-20` records for every answer: the text for clients that read text, the structured result for those that read it.

**Where.** `crates/sv-cli/src/mcp/tools.rs:336-340`, `catalog.rs:289-290`.

**Evidence.** The two copies.

**How it bites.** Twice the bytes on the wire per answer; no token cost.

**Fix.** Leave the prompt bodies out of the structured result and keep ids and titles there, if the structured-reading client can fetch the body by id; otherwise record that the duplication is deliberate.

**Record.** ADR-066 governs: an "unchanged, because" line.

**The owner's word, 12 October 2026:** held until the loop test (0257) has finished, since this changes what the AI coding tool
is told, and the trial's comparisons should not move under it; the same reason as 0338.

**Released, 13 October 2026, the owner's word** ("I'm fine releasing those items"): the loop test's second trial has its results (#1669), and the hold above is lifted, the catalog description included. 0338 stays held.
