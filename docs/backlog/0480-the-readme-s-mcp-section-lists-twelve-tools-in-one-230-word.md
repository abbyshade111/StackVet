# The README's MCP section lists twelve tools in one 230-word sentence, where a table and one loop diagram would do

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `README.md:446-464` (the twelve tools, one sentence from "It offers twelve tools:" to "if you want one)");
  `:482-505` ("A plan before any code", "Before each feature", "Before `--run`": three paragraphs, 320 words);
  `:507-510` (the time limit); `:429-480` around them, 1,170 words in the section.
- What is wrong: Twelve tool names with their terminal twins and caveats, in one sentence with nested parentheses
  three deep (`stackvet_prompts` runs to four lines). A reader cannot see at a glance which tools write into the
  folder (two: `stackvet_write_report`, `stackvet_record_answer`), which credit nothing (plan, brief, preflight,
  guidance, prompts, spec), and which answer in pages.
- What to do: A table: tool | what it gives | writes anything? | credits anything? | the same at a terminal. And one
  diagram of the MCP loop: the person's prompt → the AI tool → `stackvet_spec` (writes `stackvet.toml`) → code →
  `stackvet_check` after each feature (what was not examined first, then findings) → fix → ... → `questions` to the
  person → `stackvet_record_answer` (the tool's word) → `stackvet_write_report`; with the two things the server
  never does drawn as crossed arrows (start the app; run the outside tools), since that is the limit a reader most
  needs. The time-limit and paging paragraphs become two rows of a "Limits" table (50 s; 40,000 characters; 1 MiB).
  Saves about 300 words. Must not lose: "a prompt given where the tool starts did better than the same prompt
  fetched mid-build" (451-453), which is a measured result.
- How verified: read.
- Overlap: 0338 (open, the owner's) is about what the server itself sends, not the README; 0346 (done) the tool
  descriptions. None for the README's catalog.
- Needs a decision record?: no; ADR-066 (the catalog) is unchanged: the README describes it.
