# stackvet_prompts with no argument sends all 36 prompts, twice

**Status:** claimed by builder-none, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** 28,259 characters of prompt bodies in the text and the same again in `structuredContent` (both under 40,000 each, about 60,000 on the wire). The instructions say "get the design-time prompt for it from stackvet_prompts", so a tool that does not guess a requirement id pulls them all.

**Where.** `crates/sv-cli/src/mcp/tools.rs:352-440`.

**Evidence.** The measured body sizes.

**How it bites.** About 7,000 tokens read per call that could be 36 lines.

**Fix.** Default to a list (id, title, status, requirements), full text by id. Estimate.

**Record.** ADR-035 governs the prompt library and ADR-066 the tool: an "unchanged, because" line if only the default changes.

**The owner's word, 12 October 2026:** held until the loop test (0257) has finished, since this changes what the AI coding tool
is told, and the trial's comparisons should not move under it; the same reason as 0338.

**Released, 13 October 2026, the owner's word** ("I'm fine releasing those items"): the loop test's second trial has its results (#1669), and the hold above is lifted, the catalog description included. 0338 stays held.
