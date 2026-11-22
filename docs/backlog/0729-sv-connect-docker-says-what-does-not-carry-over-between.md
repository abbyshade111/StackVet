# sv connect --docker says what does not carry over between container sessions

**Status:** claimed by Claude Code session (AgentVet), 22 November 2026

Filed 22 November 2026, at the owner's word: recommendation 2 of the investigation for backlog 0719, design entry
0398 ("What sv in its container could gain from its records folder").

`sv` in its container is started with `--rm`, so anything it keeps outside the app's folder, other than what
`--keys` and `--notes` mount, is made in the container and lost with it. Two of those losses are said to nobody:

- **History.** It is switched on by `keep-history` beside the review key (`crates/sv-cli/src/history.rs`), which the
  container never sees, so a run there is not recorded even when the person turned history on.
- **Reports from an earlier session.** The MCP server seals each report it writes with a report key made inside the
  container (`crates/sv-cli/src/report_seal.rs`), so a later session cannot check that seal and does not offer the
  report as one `sv` wrote; the AI tool has to ask for a new one. Only `report_seal.rs`'s own notes say so.

- **To do:** add one or two plain sentences to what `sv connect --docker` prints after the settings block
  (`crates/sv-cli/src/connect.rs`, `where_it_goes`, beside the note on the image's tag): that history is not kept for
  runs in the container, and that a report the container wrote in an earlier session is not offered again, so the AI
  tool asks for a new one each session. Said, not changed: nothing more is mounted (design entry 0398).
- **Check:** a test reads both sentences in the output with `--docker`, and neither without it; the website's guide
  and the README, where they show this output, say the same.
- **Not here:** giving the container a report key of its own, which is the owner's choice (design entry 0398,
  recommendation 3) and would be a decision with its own record.
