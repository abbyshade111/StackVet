# The backlog reviewed: every item done says so and is in done/, and every item on hold is in a folder of its own

**Status:** claimed by claude/0650, 24 October 2026

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
