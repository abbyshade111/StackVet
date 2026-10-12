# Print progress to the terminal while a scan runs, so the person can see what is happening

**Status:** done, 12 October 2026

Asked for by the owner on 11 October 2026: while `sv` runs, the terminal is quiet until the report is written, so
the person cannot tell whether it is working, stuck, or waiting on something. Look into printing progress as each
stage starts and finishes (the checks, the app's start, the probes, the report), in plain words, and say how long
the slow stages take. Look first at what the stages already name (`STAGES`, which the MCP server reports) and what
the terminal already prints, so the progress is added to them rather than in a second place. Settle, with the
owner, whether the progress goes to the terminal only or also to the report, and whether it is quiet with `--quiet`.

**Built 12 October 2026 (ADR-102),** at the owner's word as the session organizing the backlog passed it on the same
day: all three parts, in the terminal whatever tool runs `sv`. A stage of five seconds or more says how long it took
when it ends; the slowest parts are said at the end; a long `--slow` wait says how far it is at each step; and
`sv report --quiet` leaves out those lines, never the counts, warnings or errors. The report and the MCP server's
progress are unchanged. Breaks: `--quiet` ignored, the threshold moved, and the wait's minutes rounded down each fail a
test.
