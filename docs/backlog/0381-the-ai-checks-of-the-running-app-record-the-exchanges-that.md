# The AI checks of the running app record the exchanges that show what they found

**Status:** partly done: the words sv sent are not kept, only the app's answer: recording.rs keeps no request body, since it can carry the test accounts' passwords and cookies; the prompt sv wrote itself, which holds no secret, could be kept from sv's side instead (a change to what ADR-082's records hold, so the owner's word first)

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

**Built, 20 October 2026: every AI finding names its questions.** The eight left by #1650 now go through `on`, each
naming the questions whose answers showed it: the burst past the owner's limit its first and last messages
(`ai-plain-100` and the last), a saved injection `ai-recall-38`, an agent's endless loop its loop question
(`ai-fetchloop-26` or `ai-mcploop-33`, passed into `judge_tool_loop`), an ignored kill switch the message sent to the
switched copy (`ai-kill-switch`), a flagged injection passed on the injection and the plain message after it
(`ai-inject-4`, `ai-plain-5`), an incomplete record of a model call the plain message that made it (`ai-plain-1`), and
the test MCP server's two results their own questions (`ai-mcpbad-7`, `ai-mcpinject-8`). Held by
`crates/sv-check/src/ai/questions_named_tests.rs` (seven tests, from the scenarios that already made each finding);
with `on` made to drop its ids, all seven fail.
