# `sv doctor` inside a tool's sandboxed shell says there is no container backend when the socket is merely unreachable

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity trial. Severity: low.

**What is wrong.** Antigravity's step 463 (sandboxed shell): doctor said Docker was not reachable as if absent; step
465 (outside the sandbox): fine. A person reads "install Docker" and Docker is installed.

**Fix.** When the socket exists but cannot be opened, say "Docker is installed but this shell cannot reach it
(permission denied on <socket>); run sv doctor outside the tool's sandbox or in your own terminal"; prompt 6 says the
same.

**Record.** None.
