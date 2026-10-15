# Proposal: a pre-deployment checklist as a report of its own, beside compliance.md and security.md

**Status:** open

The owner's ask, 15 October 2026, after running prompt 7 in the follow-up model trial (backlog 0577): consider a
pre-deployment checklist as a separate report. A proposal; the owner shapes it before it is built.

**What exists.** The report already has a "Before going live" section (`crates/sv-report/src/markdown.rs`, from
`report.before_going_live`: the commands and the things only the live site can answer), the "your decision" questions,
the gaps that say what a check could not reach, and `sv probe` for the live address. They are spread across
compliance.md, security.md, and report.html, in the order the requirements come, not in the order a person deploys.

**What to build.** One more file in `stackvet-report/`, `before-going-live.md` (and a tab in report.html), that reads as a
checklist in the order of a deployment, each line one thing a person does or confirms, with the requirement it serves
in brackets and a box to tick:
- what to set at the host before the first deploy (TLS, the headers the fenced run could not see, the secret key and
  the settings the report saw loosened for the run (0564), the security contact address, the lockfile);
- what to run after the first deploy (`sv probe <address>` with its lines, the sign-in and password flows the fenced
  run tried, and the ones it could not);
- the decisions still unanswered that decide whether a requirement applies, each with the question;
- the findings left as "needs attention" and the accepted risks, so the person signs off on them knowingly;
- what the report could not verify at all, so nobody reads the checklist as a pass.
Ticks are recorded through `sv review` (so a tick is the person's word, sealed, and the next report shows what was
ticked and when), never by editing the file. The checklist credits nothing: it is a list of what to do, drawn from
what the report found, and it says so at the top. Plain words, since the person deploying is not a programmer.

**Questions for the owner before building.** Whether a tick should count as "checked by hand" for the requirement it
serves (through `sv review`'s existing hand checks) or only as a note; and whether the checklist belongs in the
report folder or is printed by a command (`sv before-live`) so it is never stale.

**Record.** A new ADR: what the checklist is, that it credits nothing by itself, and what a tick records.
