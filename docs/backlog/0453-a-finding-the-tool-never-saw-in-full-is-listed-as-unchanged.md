# A finding the tool never saw in full is listed as "unchanged since your check" after a check that came in parts

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-cli` and the MCP server. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-cli/src/mcp/tools.rs:211-212` (`shown_all()` after every answer that is not `Ask::Part`,
  recording all of `report.findings`); `crates/sv-cli/src/parts.rs:248-263` (a first answer over the budget holds
  only the pages that fit); `check_text.rs` lead: "{} unchanged since then, one line each".
- What is wrong: a first answer too long to come whole shows page 1 of the findings and lists the rest; the server
  then records every finding as seen. The next check with nothing changed can fit the budget as one line per finding
  and is sent as the delta, so a finding that was never given in full (page 2 onward, which the tool may not have
  asked for) is from then on a fingerprint, rule, and file:line, under "unchanged since your check", until the tool
  asks for `all` or `findings`. The lead's words say the lines are findings "you" were given. The counts and the
  not-examined list are intact; the fix text of those findings is what the tool never reads unless it asks.
- What to do: record as seen only the findings whose text was in the answer (the `units` `respond` chose), or say
  in the lead how many of the unchanged were only listed before; and a test with a check that comes in parts (the
  `club_app` helper makes one) followed by a plain check. Must not lose: pages asked for staying the same in every
  answer (`a_page_asked_for_is_never_cut_from_the_delta`).
- How verified: read.
- Overlap: 0336 (done); its done note does not cover the paged first answer.
- Needs a decision record?: no.
