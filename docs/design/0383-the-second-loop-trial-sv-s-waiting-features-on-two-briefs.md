# The second loop trial: sv's waiting features on two briefs, and an outside check (12 October 2026)


The loop trials so far asked whether a prompt prevents a mistake; the drafts trial of the same day showed that the 5.5
models already avoid what seven drafts warn about. This trial asks a different question: do `sv`'s own features, built
in the last week and run only against the scripted app, work on apps an AI coding tool really built (ADR-109,
`docs/prompts/loop-2/protocol.md`).

**Two briefs, both with `sv` attached.** The club brief, corrected: it named the settings file by its old name, and a
drafts-trial build wrote both and was refused. And a garden-plots site with sign-up, a reset by email, and four account
changes, in a form offering all four and one offering only the email change, so the account-change checks of ADR-105
and the `not-offered` list meet real apps.

**An outside check the builder never sees.** Read against the code, 0006 part 21 was mostly built already: `sv report
--tools` runs Semgrep and Bandit, and keeps their own reports, but the trials never asked for it. The MCP server never
runs an outside tool, so a second check with the outside tools is a view the builder could not have tuned the app to.
Tried on one drafts-trial build before the protocol: Bandit 5 findings and Semgrep 1, in 23 seconds.

**A script standing in for the owner, labeled.** `sv review` runs only in a terminal someone is typing in; the script
gives it one and confirms only the `not-offered` list, with a signing key made for the trial in a settings folder of
its own. Tried on a drafts-trial build that had listed `not-offered` itself, which five of the twenty did.

**The observability features in use.** `SV_LOG`, the trial's own history (two runs per build, compared with
`sv compare`), the build-loop record against the transcript's count of calls, what `sv` kept of each running app, and
any crash file.

**Found while writing the measures:** `loop_measures.py` counted `sv`'s tools only under their names before the rename,
so every new loop-arm build would have shown no checks. And the AI-commits note is a finding of `info` whether it
found commits or none, so the scorer reads its count from its title rather than from its presence.
