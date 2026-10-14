# The coordination board, for the next burst: a waiting line, insert at a position, and a close command

**Status:** done, 13 October 2026

From the coordination trial's write-up (backlog 0273, 13 October 2026). The owner chose the same day to keep the
coordinating session for bursts of independent findings, such as a review's items, with these three things added to
`tools/coordination.py` first:

- **A waiting line.** During the trial a builder waiting on a question to the owner looked the same as a stalled one:
   the board saw only git's silence, and two waits cost about twelve hours each. A building session should be able to
   say "waiting on the owner: <the question>" on its own line of the board (or a file beside it the tool reads), and
   `board` should show it, so the orchestrator can tell the owner at once whose action is pending.
- **Insert at a position.** A lane can only be added to at its end; putting the 0301 review's findings straight after
   0410 took two withdrawals and two re-assignments, which inflate the hand-out count. `assign ... --after <number>`.
- **A close command.** The trial's board was closed by hand: `session remove` for each session, and the queued items'
   marker line rewritten by a script. `close --why` should do both, log it, and leave the board readable as a record.

Each with a self-test that fails without it, broken on purpose and watched. ADR-061 governs the board: a Later line.

**Built, 13 October 2026** (session stackvet-deep-review): the three commands in `tools/coordination.py`, each in its
self-test and each broken on purpose and watched (the lane order, the branch scan, the markers at close, and the check
on waiting files each turned the test red at their own assertion). The waiting line is a file beside the board,
`docs/coordination/waiting/<number>.md`, written by `wait <number> --on "..."` and pushed on the builder's branch, so no
pull request is needed for the board to show it; `board` reads this checkout and every branch pushed in the last seven
days. `assign --after <number>` inserts into a lane. `close --by --why` removes the sessions, rewrites the markers
(`queue add --mark` writes them), empties the queue, and keeps the board; `open` keeps a closed board beside the new
one. The hand-out footer tells a builder to use `wait`. ADR-061, Later; `docs/COORDINATION.md` steps 1, 3, 7, and 8.
