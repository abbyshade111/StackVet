# Proposal: read the AI tool's own session record, with the person's consent, and list where what it said differs from what the files show

**Status:** claimed by wording, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the loop-test session's observability ideas, 14 October 2026, assessed by the orchestrating session and agreed by
the owner the same day: second of three. A proposal, held until the owner says go. It credits nothing.

**What was seen.** The three-tool trial (`docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`) needed a reviewer to read
each tool's transcript against the files to find "said the tests pass" with no test run, "nothing was silenced" after
two rewrites, and "the signed-in probing succeeded" when sign-in answered 400. A person who is not a programmer cannot
make that table, and it is the one that tells them whether to trust the tool's summary.

**What to build.** An opt-in read of the AI tool's local session record, under four conditions:
- Only formats sv can cite documentation for, with fixtures: Claude Code's transcript first (sv already reads its
  settings); the three trial formats (Antigravity, Cursor's database, VS Code's event stream) as fixtures when their
  formats are documented, and an honest "not read" line otherwise.
- The report shows only the mismatch, never the transcript: "the tool said the tests passed at 18:24; no test command
  ran", with times. The record holds the person's own words and their other projects; none of it is quoted.
- The record is the tool's file and can be rewritten, so the section says it is the tool's record read for
  contradictions, not evidence; it sits apart from the counts, as the agentic-tool files do, and credits nothing.
- Consent per run (a flag or a question), never a setting that stays on, since the record is outside the app folder.
Claims found in the transcript ("rate limiting added", "XSS fixed") land in the same section, worded as 0538 asks (what
the code shows, never a word to make go away). Commit messages and SECURITY.md are not read for claims: they are the
tool's text too.

**Record.** A new ADR: sv reading a file outside the app folder, what it shows, and what it never quotes.

**The owner's go, 14 October 2026:** build it whenever the board has room; in the order 0564, 0565, 0566, after the probe sweeps.

**The orchestrator's answer to wording, 15 October 2026 (a board-order question, not the owner's):** B, start now. The order 0564, 0565, 0566 was the owner's preference for building, not a dependency: 0565 reads a record outside the app folder and touches no probe file, and 0564 is honesty's. Write the decision record first, as proposed, then build. Clear the waiting line on your branch when you take it.
