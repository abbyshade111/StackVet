# The AI checks of the running app record the exchanges that show what they found

**Status:** partly done: eight AI findings still name no question: the two that rest on the app's log, a burst of messages, the kill switch's second copy, a stored injection, an agent's loop, and the test MCP server's two tool results (C10.4.1, C10.4.2); and the words sv sent are not kept, only the app's answer (no request body is kept, by design)

From the owner's review of the report on 12 October 2026, after an inventory of what a run gathers against what
the report shows (`docs/website/home-page.md` is the words; the inventory is the session's page "StackVet Report
Inventory"). The owner: use "the observability features and scan findings to provide the actual evidence for what was
found where possible", and the outside tools' findings "have been the most important time and time again".
Read on the second loop trial's build `haiku-a-5` (`~/sv-loop2/runs/haiku-a-5-out` and `-tools-out`).

On `haiku-a-5`, `probe.ai-hidden-content-passed`, `probe.ai-injection-other-languages`,
`probe.ai-input-charset-unrestricted`, and `probe.ai-output-unbounded` carry no evidence: no exchange id, so even with
"What StackVet saw" (backlog 0378) they would show nothing. To build: each records the prompt it sent and the reply
that showed the problem, in `seen.json`, and names them in `evidence`, redacted as every exchange is.

**Checked against `main`, 20 October 2026** (the backlog clean-up): built: each question recorded as `ai-<kind>-<n>`, and findings naming theirs for the four rules above (bbc25959, #1650; ADR-082, Later, 12 October 2026). The two MCP findings were missing from the status line; they are asked through numbered questions, so they could name one.
