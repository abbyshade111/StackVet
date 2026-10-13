# stackvet_spec is 47,187 characters, sent whole every time and never paged, with no test on its size

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: MCP token cost. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** 18 percent over the project's own 40,000 budget and 2,813 characters under the client's 50,000 save-to-file line, past which the model is shown a 2 KB preview of a file instead of the text. One more shown prompt of about 2,900 characters pushes it over.

**Where.** `crates/sv-cli/src/mcp/catalog.rs:624-637`: the starter manifest (33,485) plus the spec instructions (6,039) plus the ten shown prompts (7,662).

**Evidence.** The sizes, summed; no test holds the spec's size (grep found none).

**How it bites.** About 11,800 tokens per spec call, and one more prompt breaks the tool's answer for the client.

**Fix.** Page it through `parts` like plan and check (first page: the manifest's `[app]` and the filling instructions; the prompts a section), and a test on its size. Must not lose: the spec, instructions, and guide agreement test.

**Record.** ADR-066 governs: a Later line.

**The owner's word, 12 October 2026:** held until the loop test (0257) has finished, since this changes what the AI coding tool
is told, and the trial's comparisons should not move under it; the same reason as 0338.

**Released, 13 October 2026, the owner's word** ("I'm fine releasing those items"): the loop test's second trial has its results (#1669), and the hold above is lifted, the catalog description included. 0338 stays held.
