# `stackvet_status` does not name the folder it serves, so a container for an older app passes as connected

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity trial. Severity: medium (16
minutes lost, and a report could have been written for the wrong folder).

**What is wrong.** Antigravity keeps one account-wide MCP entry, so each new app overwrites it while the old container
keeps running; `stackvet_status` answered "ready" for `.` while the container's `--root` was an older app's folder;
the tool found out only when a path "is not a folder inside /Users/abigail/my-app" (the trial's step 66) and three
stale containers were up 20 and 25 hours.

**Fix.** The first line of `stackvet_status` names the absolute root folder and the app name from its stackvet.toml;
`sv connect` and prompt 3 say that an older container for another folder must be stopped (`docker ps`, `docker stop`)
or the tool restarted. Consider `sv mcp` refusing a root whose stackvet.toml names a different app than the one the
tool asks about, when the tool passes a folder.

**Record.** None for the status line; a Later entry on the MCP record if `sv mcp` refuses anything.
