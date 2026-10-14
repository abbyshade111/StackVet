# `sv review` reads report.json whole, through a link or a pipe, and repeats its count without the seal

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-cli` and the MCP server. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-cli/src/review.rs:29-31` (`findings_without_entries`:
  `std::fs::read_to_string(folder.join("report.json"))`), said at `:38` ("The last report in {} lists {n} findings
  that count.").
- What is wrong: the new line `sv review` prints when nothing is waiting reads `stackvet-report/report.json` with
  `read_to_string`: no `symlink_metadata` check, no size cap, and a named pipe there is waited on for ever, on the
  person's own terminal (Ctrl-C ends it, so it is a nuisance, not the server hang of 0282). The same crate just
  taught `prompts_for_report` the other way (`crates/sv-cli/src/lib.rs:1247-1270`: an ordinary file only, read with
  `take(MAX_RESOURCE_BYTES + 1)`). And the count is read from whatever `report.json` is there, sealed or not: the AI
  coding tool can write that file, and `sv explain` refuses to repeat an unsealed one (backlog 0226, part 1, item
  9), but `sv review` now says "The last report lists no finding that counts" on its word.
- What to do: read it as `prompts_for_report` does (ordinary file, capped), and either check
  `report_seal::proven(&folder)` first and say "no sealed report was read" otherwise, or word the line as "the file
  at … lists", so it is not taken for sv's own count. Must not lose: ADR-098's line itself, which the Cursor build
  showed was needed.
- How verified: read.
- Overlap: 0282 and 0309 (done) are the MCP server's reads; 0253/ADR-098 built this line. None names this read.
- Needs a decision record?: no; ADR-098 governs `review.rs`, so the pull request owes it a line (unchanged: the words
  stay, the read is made safe).
