# resources/list reads and parses report.json a second time, whole, for each sealed folder, after the seal checked it

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-cli` and the MCP server. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-cli/src/mcp/resources.rs:25` (`let when = written_when(&folder)` inside the loop over report
  folders, after `proven`), `:221-223` (`std::fs::read(folder.join("report.json"))`, no cap, follows a link).
- What is wrong: `proven` has just read `report.json` with a 16 MiB cap and refused a link or a pipe
  (`report_seal.rs:80-96`); `written_when` then opens the same path again with no cap and no type check, on every
  `resources/list`, for every sealed folder below the root (up to the listing limit). In the ordinary case it is a
  second read and parse of up to 16 MiB per folder per listing; between the two reads the file can be replaced (a
  pipe planted there would hold the server's thread, as 0282 described). The window is small and needs a writer in
  the app's folder racing the server, so this is tidiness with a thin edge, not a hole I can show.
- What to do: have `check`/`digests` hand back the bytes of `report.json` (they are in memory) and read
  `run_record.started_unix_ms` from those, or read with `take(MAX_RESOURCE_BYTES)` after `symlink_metadata` as the
  other readers do. Must not lose: the date in the description (0286).
- How verified: read.
- Overlap: 0309 (done) capped the other reads; 0286 (done) added this one. Neither names it.
- Needs a decision record?: no.
