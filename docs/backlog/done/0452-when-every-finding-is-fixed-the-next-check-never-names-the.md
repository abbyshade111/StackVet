# When every finding is fixed, the next check never names the ones that went

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-cli` and the MCP server. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-cli/src/mcp/check_text.rs:273-287` (the "No findings" lead gains "GONE since your check at …"
  and a line per gone finding); `crates/sv-cli/src/mcp/tools.rs:176-180` (the delta answer is sent only when
  `text.len() < full.len()`).
- What is wrong: with no findings left, the delta text is the plain "No findings" text plus the GONE lines, so it is
  always longer than the full text and the condition at `tools.rs:176` drops it: the answer is the ordinary one,
  with no `gone` list and no `since`. The branch at `check_text.rs:277-287` cannot be reached through the tool, and
  `since_tests.rs` has no test in which everything is gone (its three tests: a second check unchanged, a moved file,
  a page asked for). The catalog's output schema promises `gone` "those gone since" (`catalog.rs`, the `gone`
  field). Nothing is hidden that is still there; what is lost is the confirmation that the fix took, which is the
  one case 0336 named ("call it again to see the fix took").
- What to do: compare without the GONE lines, or always send the delta when the only addition is `gone`, and a test
  that fixes every weak hash and asserts the gone list and the words. Must not lose: the rule that a delta shorter
  than one finding is not worth its explanation.
- How verified: read.
- Overlap: 0336 (done) is the feature; its done note does not mention the all-gone case.
- Needs a decision record?: no.
