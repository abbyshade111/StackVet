# When sign-in fails, the run shows only "(400)" and seen.json cuts the page before its error text

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity app. Severity: medium (the one
answer a person needs to fix the run is missing). Read by the reviewing subagent and the orchestrating session.

**What is wrong.** sv's sign-in to the Antigravity app answered 400, so 16 signed-in checks "stopped". The run step says
"signed in as A (400)" and nothing else; `seen.json` keeps 4,000 characters of the page, and the app's inline CSS runs
past that, so the error text (the only 400 branch is the CSRF check, `app/main.py:288-292`) is cut. Whether the fault
is the app's token or sv's token extraction cannot be told from the report.

**Fix.** For a failed sign-in, keep the whole page or pull the visible error text (`.error`, `.alert`, flash blocks,
`<title>`) into the run step, and say which form fields were sent (names only, never values). Then the person, or the
AI coding tool reading the report, can fix the manifest or the app.

**Record.** ADR-0NN: unchanged, because what seen.json keeps is a presentation matter; check the record that governs it.
