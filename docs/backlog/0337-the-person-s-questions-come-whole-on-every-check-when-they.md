# The person's questions come whole on every check when they fit, about 540 characters each, though they do not change between edits

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** The questions do not change between edits, and the tool is told to ask them one at a time anyway. A small app with 30 questions re-reads about 16,000 characters (about 4,000 tokens) per check.

**Where.** `crates/sv-cli/src/mcp/tools.rs:136-152` and `check_text.rs:131-143`: with no `section`, a check that fits the 40,000 budget with its questions carries all of them in full.

**Evidence.** `let counted = matches!(ask, Ask::First) && !fits(...)`: whole whenever they fit.

**How it bites.** Estimate: about 4,000 tokens per check on a small app.

**Fix.** Always default to the count plus how to ask (the existing `counted` branch); whole only on `section: "questions"` or `all`. Must not lose: the count and the how-to-ask line in every answer. ADR-066 says "a check that comes whole with them still does", so the record changes with it.

**Record.** ADR-066 governs: a Later entry.

**The owner's word, 12 October 2026:** held until the loop test (0257) has finished, since this changes what the AI coding tool
is told, and the trial's comparisons should not move under it; the same reason as 0338.

**Released, 13 October 2026, the owner's word** ("I'm fine releasing those items"): the loop test's second trial has its results (#1669), and the hold above is lifted, the catalog description included. 0338 stays held.
