# Proposal: fetch the address an uploaded or pasted page is served at, and credit V3.2.1 only when it is sandboxed or on another origin

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502). A proposal: hold until the pointer's proposals are
settled. Read by the reviewing subagent.

**What was seen.** The Antigravity app serves member HTML same-origin with `'unsafe-inline'` scripts, reachable on its
own at `/games/{id}/frame` (`app/main.py:116-127, 503-524`): a stored script there is account takeover. The Cursor app
serves the same kind of page with `Content-Security-Policy: sandbox allow-scripts` inside a frame without
`allow-same-origin` (`familygames/app.py:25-30, 400-406`). Nothing static tells the two apart, and V3.2.1 for Antigravity
is "stopped".

**What to build.** When the manifest declares `upload.serves-at` or `rich-text = true`, fetch the served address
directly as a signed-in user and read its headers: credit V3.2.1 (read its text first) only when the answer carries a
`sandbox` directive or comes from a different origin; otherwise report it with the header seen.

**Record.** A new ADR or a Later entry: what counts as evidence for V3.2.1 changes.
