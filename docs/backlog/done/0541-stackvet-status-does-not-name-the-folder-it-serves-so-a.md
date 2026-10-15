# `stackvet_status` does not name the folder it serves, so a container for an older app passes as connected

**Status:** done, 15 October 2026

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

**Done, 15 October 2026.** `stackvet_status` now opens, before any answer, with the folder the server was started for
and the app its stackvet.toml names ("This StackVet server serves <folder>, where stackvet.toml names the app <name>"),
both fenced as the app's own text, then what to do when that is not the app being built: tell the person, and with
their yes stop the older server (`docker ps`, `docker stop`) or restart the tool. The structured answer gains `root`
and `appName`, declared in the tool's output schema. `sv connect` says, after where the settings go, that a server
started for an earlier app may still be running when a tool keeps one MCP setting for the whole account
(`connect::OLDER`), and setup prompt 4 has the AI tool say which folder `stackvet_status` serves and stop if it is not
this one. Tests: `mcp/status_serves_tests.rs`, broken on purpose three ways (the line moved after the answers, the
app's name left unfenced, `sv connect`'s sentence dropped for one tool), each turning red; the run also caught the
output schema and the path helper, both fixed.

Not done: `sv mcp` refusing a folder whose stackvet.toml names another app. A tool does not say which app it means, so
the server has nothing to compare against; naming the folder first lets the tool and the person see the mismatch.
Not tried in Antigravity; the trial (backlog 0502) will show whether the line is enough.
