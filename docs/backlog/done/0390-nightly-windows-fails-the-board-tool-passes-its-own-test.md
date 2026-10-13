# Nightly: Windows fails: the_board_tool_passes_its_own_test

**Status:** done, 13 October 2026

Recorded by the nightly CI routine on 13 October 2026. The Windows workflow failed on 19 pushes to `main` between
18:28 UTC and 20:50 UTC on 12 October, each time on one test, `the_board_tool_passes_its_own_test`
(`crates/sv-cli/tests/coordination.rs`, line 34), with totals such as 3082 passed and 1 failed. Rust and CodeQL passed
on the same pushes. It was not there the night before.

- First seen in [run 38291192026](https://github.com/abbyshade111/StackVet/actions/runs/38291192026) on `d0496b2`, the merge of pull request #1548
  (`claude/0273-cloud`), which is the change that brought it.
- Last seen in [run 38306761299](https://github.com/abbyshade111/StackVet/actions/runs/38306761299) on `d70f7a7` (#1568).
- What the failure said: the board tool's own test printed its steps ("opened ...\docs\coordination\board.md",
  "session a added, arm full", "session b added, arm none", "queued 0001") and then failed; the annotation does not
  carry the last line, so the exact assertion was not read.

`main`'s Windows job has passed since #1569 (`2de0536`, 20:50 UTC), which took the board's test out of the
Windows build at the owner's word (`3b68f06`, ADR-061, Later). **What stays open:** the board tool is not tested on
Windows. If that is the intended end state, mark this done with that note; if not, the cause on Windows (most likely
a path written with `\` where the tool expects `/`, not checked) is still to find.

**Done, 13 October 2026:** the intended end state. The board is the sessions' own tool for the coordination trial, run on
Mac and Linux, not part of `sv`, and the owner asked on 12 October 2026 that its test leave the Windows build
(`crates/sv-cli/tests/coordination.rs`, `#![cfg(not(windows))]`, ADR-061 Later). The owner approved closing it with the
trial's cleanup on 13 October 2026. If the board is used again on Windows, the cause (most likely a path written with `\`)
is still to find.

