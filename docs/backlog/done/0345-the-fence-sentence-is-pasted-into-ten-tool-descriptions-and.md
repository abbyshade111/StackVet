# The fence sentence is pasted into ten tool descriptions, and stated again in the instructions and at the top of every fenced answer

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** 3,250 characters (about 810 tokens) per turn in the tool list for a sentence said twice more.

**Where.** `crates/sv-cli/src/mcp/catalog.rs:11-15` appends `fence::ABOUT` (325 characters) to every tool with a `path`; `mod.rs:160-163` states the rule in the instructions; the 376-character header tops every answer that fences anything.

**Evidence.** `tool["description"] = json!(format!("{description} {}", sv_report::fence::ABOUT))` for each tool with `path`.

**How it bites.** About 810 tokens per turn.

**Fix.** Say it once in the instructions and drop `ABOUT` from the descriptions, or cut it to a twelve-word pointer. Must not lose: the per-answer header (ADR-066 rule 5, "the result says first"); deep review R9 asked for the description text, so this wants an ADR-066 line.

**Record.** ADR-066 governs: a Later line.

**The owner's word, 12 October 2026:** held until the loop test (0257) has finished, since this changes what the AI coding tool
is told, and the trial's comparisons should not move under it; the same reason as 0338.

**Released, 13 October 2026, the owner's word** ("I'm fine releasing those items"): the loop test's second trial has its results (#1669), and the hold above is lifted, the catalog description included. 0338 stays held.
