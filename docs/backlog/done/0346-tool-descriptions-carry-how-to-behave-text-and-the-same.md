# Tool descriptions carry how-to-behave text and the same paragraphs several times, and the instructions restate the catalog

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** The tool list is 18,083 characters (about 4,500 tokens) without output schemas, 31,185 (about 7,800) with them, read on every turn; the instructions about 2,970 tokens. Together roughly 7,500 tokens per turn before any call (about 10,700 if the client forwards output schemas).

**Where.** `crates/sv-cli/src/mcp/catalog.rs`: `stackvet_record_answer` (`:583`) 1,615 characters, mostly the `Written by` and `sv review` rules; `stackvet_check` (`:559`) 1,480; `stackvet_before` (`:517`) 1,180; `stackvet_plan` (`:506`) 1,125; `stackvet_status` (`:473`) 978 restating `sv doctor`'s list. The 230-character "too long to take in whole ... comes in parts" paragraph is in plan, check, and the instructions. The eleven-feature list is in `stackvet_before`'s description, its `enum`, and the instructions. "Reads files only", "never starts the app", "credits nothing" appear four, three, and four times. The instructions (`mod.rs:126-176`): 36 sentences, 16 naming a tool and largely repeating its description.

**Evidence.** The measured sizes, from the catalog rebuilt as `tools()` assembles it.

**How it bites.** Estimate: 5,000 to 6,000 characters (about 1,400 tokens) per turn from the descriptions, and more from the instructions.

**Fix.** One or two sentences per description (what it gives, when to call it), the rules moved into the answer text or the refusal; the instructions as the flow only (which tool, in what order: ADR-066 rule 8), the descriptions as the what. Must not lose: "never says a requirement passed" and the no-safe-words test (`flow_text_tests.rs`), the readOnly and destructive annotations, and the test that holds the instructions, the guide prompt, and the spec in step.

**Record.** ADR-066 governs: a Later entry.

**The owner's word, 12 October 2026:** held until the loop test (0257) has finished, since this changes what the AI coding tool
is told, and the trial's comparisons should not move under it; the same reason as 0338.
