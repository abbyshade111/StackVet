# Fence tags cost 141 characters per finding where 6 hex digits would do

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The retry loop (`named_from`) already guarantees the name is absent from the draft, so uniqueness does not rest on the name's length. 10 findings cost 1,410 characters of tags; 80 cost 11,280 (about 2,800 tokens).

**Where.** `crates/sv-report/src/fence.rs:46-50` makes a 12-hex (48-bit) name; a finding wraps three pieces (`check_text.rs:261-271`).

**Evidence.** The tag length and the retry loop.

**How it bites.** About 2,900 characters on an 80-finding answer.

**Fix.** 6 hex digits, saving 12 characters per wrap (36 per finding). Must not lose: the name absent from the draft, the header first, `sv`'s own words outside every tag; the fence tests hold these.

**Record.** ADR-066 governs the fence rule: an "unchanged, because" line.

**The owner's word, 12 October 2026:** held until the loop test (0257) has finished, since this changes what the AI coding tool
is told, and the trial's comparisons should not move under it; the same reason as 0338.
