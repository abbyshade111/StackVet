# Prompt 6: a sandboxed shell, a kept copy of sv, and a commit that must match the pulled source

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Antigravity (needed `BypassSandbox` for brew,
Docker, `sv report`, and `open`), Cursor (found a kept copy behind a broken link and hand-built in the owner's
checkout; the first install copied a stale binary). For the prompt library (0370).

**What to change.** Prompt 6: "If a command fails with 'operation not permitted', tell me and run it outside your
sandbox. If you find an older or kept copy of StackVet, tell me and install with Homebrew unless I say otherwise.
After installing, `sv --version` must name the commit you expect; if not, stop and tell me."
