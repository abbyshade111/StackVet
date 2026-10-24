# The backlog reviewed: every item done says so and is in done/, and every item on hold is in a folder of its own

**Status:** done, 24 October 2026

Filed 23 October 2026 at the owner's word: "review what is left in the backlog - for anything done, make sure the
status says so and move to the done folder, for anything on hold, move to a new folder called "hold" or something like
that so I can tell what is truly left to fix".

On 23 October 2026 the backlog had 12 items open, 5 claimed, and 3 partly done, beside 626 done. Some open ones wait on
something outside `sv` (Podman's networking, 0520; a nightly's upstream fault, 0627) or on the owner, and read as work
left to do.

To do: read each item that is not done against `main`. One whose work has landed gets its done note and moves to
`done/`. One that waits on something named (an upstream fix, the owner's word, another item) moves to
`docs/backlog/hold/`, with a status line that says what it waits on. `tools/backlog.py` learns the new status and folder
(`list --hold`, `--check`, `tidy`, and the board), and the folder layout is ADR-061's, so this is a Later entry on it,
with the owner's word. What is left beside `done/` and `hold/` is then what is truly left to fix.

**Order,** at the owner's word of 23 October 2026: after 0649.

**Built, 24 October 2026.** `tools/backlog.py` has a fifth status, `on hold: <what it waits on>`, a `hold` command
that moves an item under `docs/backlog/hold/`, a claim that brings one back, `list --hold`, and an "On hold" list on
the board; `tools/docs_page.py`, `tools/record_numbers.py`, and `tools/ci_scope.py` read the folder (ADR-061 and
ADR-104, Later, 24 October 2026). Every done item already said so and was in `done/`: `--check` found none out of
place. At the owner's word of 24 October, 0324 is done and the weekly check that follows it is 0662; held are 0365,
0375, 0520, 0627, 0120, and 0217, each with what it waits on. What is left to fix, beside `done/` and `hold/`: 9 open,
2 claimed (0642, and this), and 0653 partly done. 0217's parts 1 and 2 still carry claims by `securevibe-e2` from
before; they are kept as written, since the parts' history is the owner's to settle when the item is taken up.
