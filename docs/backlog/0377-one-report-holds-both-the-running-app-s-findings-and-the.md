# One report holds both the running app's findings and the outside tools'

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the owner's review of the report on 12 October 2026, after an inventory of what a run gathers against what
the report shows (`docs/website/home-page.md` is the words; the inventory is the session's page "StackVet Report
Inventory"). The owner: use "the observability features and scan findings to provide the actual evidence for what was
found where possible", and the outside tools' findings "have been the most important time and time again".
Read on the second loop trial's build `haiku-a-5` (`~/sv-loop2/runs/haiku-a-5-out` and `-tools-out`).

Today a full check is two reports: the second loop trial checked each build with `sv report --run` and again with
`sv report --tools --keep-tool-output`, and neither report has both. On `haiku-a-5` the `--tools` report has a high
Bandit finding in the app's own code (`B613`, `app.py` line 84) that the `--run` report never mentions. To do: the
guide's full check, and the trials' protocol, run `sv report --run --tools` together, so one report holds both; check
that the two together work and are tested together; and say plainly, in the report and the guide, that the MCP server
never runs an outside tool (it does not, by design), so a person runs the full check at a terminal. Whether `--tools`
should run by default when a tool is installed is a decision of its own, asked before it is built.
