# `sv preflight` does not say that the test command's program is not among the packages `install = true` installs

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity app. Severity: low (usability).

**What is wrong.** The manifest says `test = "pytest -q"`; pytest is only in `requirements-dev.txt` (`-r requirements.txt`
plus `pytest==8.3.3`); `crates/sv-run/src/install.rs:56` installs `requirements.txt` only. The run printed
`sh: 1: pytest: not found` (exit 127) and the tests counted as failed, in the tool's own report and the rerun. Honest,
but the person learns it only after a full run, and the AI coding tool read "tests failed" as the app's fault.

**Fix.** Preflight reads the first word of `test` and says when no installed package provides it (pytest, jest, vitest,
`go test` and the rest), with the line to add or the `requirements-dev.txt` to name; and the run's own line says "the
test runner is not installed; see preflight" rather than only the shell's words.

**Record.** None.
