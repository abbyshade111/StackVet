# Proposal: an independent action log from a hook the person installs, chained, with its tip sealed in the report

**Status:** open

From the loop-test session's observability ideas, 14 October 2026, assessed by the orchestrating session and agreed by
the owner the same day: third of three. A proposal, held until the owner says go. It credits nothing.

**What was seen.** Every check sv has sees the build through something the AI tool controls: its calls, its files, its
start command, its own transcript. A hook the tool runs on every command and file write is the one view from outside
the tool's account of itself.

**What to build.** `sv connect` prints (never writes) a hook block for the tool, one tool at a time (Claude Code's hooks
first), that appends each command and file write to a log outside the app folder, each line chained to the one before.
sv reads the log with the person's consent, as the session-record proposal does, and reports what the hook saw against
what the report found. Two cautions built in: a tool with a shell can rewrite that log too, so the chain's tip is
sealed into the report (as the build-loop record's is) and a later report says when the chain no longer passes through
an earlier tip; and sv already notices hook settings being changed, so a switched-off hook is a line in the report.

**Record.** A new ADR, or a Later entry on the session-record proposal's: what sv asks to be installed, what it reads,
and that it writes nothing into the tool's settings.
