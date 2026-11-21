# AgentVet, a companion that vets the agent: what it did, what it said, and where the two part (21 November 2026)

A first design for AgentVet, a companion to `sv` and `cv` that reviews the agent rather than what it made: what it
did, what it said it did, and where the two part. Written at the owner's asking on 21 November 2026; nothing here is
built. The decision it rests on is ADR-159 (proposed). The owner's answers so far:

- **Its first reader is the owner:** a solo builder who is a security engineer.
- **Report first,** working toward stopping an action in certain circumstances.
- **Any agent,** not only one that writes code.
- **The ConfigVet model:** this workspace, `sv`'s engine, a binary of its own, `av`.

`sv` vets the app, `cv` vets the deployment, and `av` vets the agent.

## Why now

The white paper built on this repository ends on two lessons. The second is that seeing what an agent did, against
what it told the person, is the foundation for trusting what it built: the paper's Figure 19 lists, for six runs,
the claims the record contradicted, the findings silenced by rewriting, and the ones fixed for real. That table was
made by hand, by reading transcripts against files and `sv`'s records. AgentVet's first job is to make it for any
session, without a person reading transcripts.

## What `sv` already keeps, so AgentVet starts from it

| What | Where | What it holds |
|---|---|---|
| The build-loop record (ADR-076) | `crates/sv-cli/src/build_loop.rs` | One line for each call the tool made to `sv`'s MCP server: the time, the tool's name, and counts. Never an argument or a finding's text. |
| The hook record (ADR-122) | `crates/sv-cli/src/hook_record.rs` | One line for each command the tool ran and each file it wrote, kept by a hook the person installed, outside the app's folder. Never a file's contents, a command's output, the person's prompts, or the tool's words. |
| What the tool did that is worth a look (ADR-131) | `crates/sv-cli/src/tool_behavior.rs` | Read for a report from git, the hook record when the person consented, and the last report `sv` can show it wrote. It reads what changed, so a tool that games a check cannot make its own line vanish. |
| AI-made commits straight onto the default branch (ADR-092) | `crates/sv-check/src/git.rs` | Read from commit trailers. |

These move into a shared crate, word for word, and both `sv` and `av` read them there.

## The problem every part of this has to answer: how independent is the witness?

The recorder runs as the same person the agent runs as. The agent can rewrite a file the recorder wrote, remove the
hook, or start the recorder with settings of its own choosing. The security review of 21 November 2026 found each of
these in `sv`. So AgentVet ranks its evidence by who could have altered it, the way `sv` ranks whose word a credit
rests on:

```text
  strongest                                         who could have altered it
 ┌─────────────────────────────────────────────┐
 │ recorded outside the agent's reach          │  nobody the agent can become: a sandbox's network
 │                                             │  and process log, a recorder under another account
 │ recorded by the harness                     │  the agent, if it can edit the hook, the settings,
 │                                             │  or the record's file
 │ derived from git and the checkers' records  │  the agent, by rewriting history or a report
 │ the agent's own word                        │  the agent, by saying something else
 └─────────────────────────────────────────────┘
  weakest
```

Every line of a report says which of these it rests on. Two sources that disagree about one session are a finding
in themselves, and a gap in a record is reported as a gap.

The first build makes rewriting show rather than making it impossible: the chain joining a record's lines takes a
key, and its latest link is signed at intervals with the key `sv review` uses. A recorder under another account,
which the agent cannot reach at all, is the stronger step and comes later.

## What it collects

"Any agent" decides the order: the collectors that need nothing from a particular coding tool come first.

1. **A pass-through in front of the agent's tools.** `av proxy -- <the tool server's command>` sits between an agent
   and an MCP server, relays every call and answer unchanged, and writes each down. It works for any agent that uses
   MCP, and it is the place a refusal can later be made. It adds no connection: it relays what was already sent.
2. **Harness hooks,** as `sv connect claude --hook` installs today, for each tool whose hook input is documented.
3. **Telemetry the agent's runtime emits,** for agents built on a framework rather than run in a coding tool, read
   from a file or a local socket the owner names. The OpenTelemetry conventions for generative AI are the likely
   format; their current state is to be read before anything is built on them.
4. **A transcript, with the owner's consent for that session.** The only source of what the agent said. `av` keeps
   the claims it found and where each was, with keys cut, not the transcript.
5. **Effects:** git's history and changes, a listing of the folder before and after, and the checkers' own records
   (`sv`'s findings and reports, `cv`'s, a test run's report).

## The record

One format for every source. An event holds: which session, which agent and which helper agent under it, the time by
`av`'s clock, the kind (a tool call, a tool's answer, a command, a file written, a claim, a permission asked or
refused, text that entered the agent's context from outside), a summary cut short with keys cut, a SHA-256 of the
full arguments so two sources can be matched without keeping them, the source, and the tier above.

## The task envelope

A file the owner writes, `agentvet.toml`, saying what the task is and what it may touch:

- the goal, in a sentence;
- the folders it may write, the commands it may run, the hosts it may reach, and the tools it may call;
- the files that guard oversight (the agent's own instruction and permission files, hooks, CI workflows, the
  envelope itself), which it may not change;
- the actions that must ask first (deleting, pushing, sending, spending).

Drift is what falls outside it. A line left out is a question in the report, never a pass, as with `stackvet.toml`.
An envelope the agent wrote is the agent's word until the owner confirms it.

## What it looks for

The first detectors are rules over the record; no model reads anything.

| Group | What it catches | Evidence it needs | A related risk in OWASP's Agentic Top 10 |
|---|---|---|---|
| A claim against the record | "The tests pass" with no test run; a count that does not match; "pushed" with no push; "I did not change X" with X changed | A transcript, and the record | ASI09, Human-Agent Trust Exploitation |
| Gaming a check | A finding gone with no fix; a suppression comment added; an assertion narrowed or a test skipped; a checker's settings or thresholds edited; a value changed only so the check stops seeing it | Git, and the checkers' records | ASI10, Rogue Agents |
| Outside the envelope | A file, command, host, or tool the owner did not allow; a new dependency | The record, and the envelope | ASI02, Tool Misuse & Exploitation; ASI03, Identity & Privilege Abuse |
| Tampering with oversight | A hook removed; a broken chain; the agent's instruction or permission files, or a CI workflow, edited | The record, and git | ASI10, Rogue Agents |
| Speaking as the owner | `by = "owner"` or a seal line written by the tool; a commit made in the owner's name | Git, and the record | ASI09, Human-Agent Trust Exploitation |
| Trying again after a no | The same action reached another way after it was refused | The record of permissions | ASI01, Agent Goal Hijack |

Each claim gets one of three verdicts: **contradicted by the record**, **not supported by the record**, or **could
not be checked**. Only the first is a contradiction.

Later, each behind its own entry:

- **Instructions that arrived from outside and were then followed:** text in a tool's answer or a fetched page that
  reads as an instruction, followed by an action that matches it (ASI01; ASI06, Memory & Context Poisoning).
- **Trends across sessions:** for one agent or model, the rate of contradicted claims, envelope breaches, and
  silenced findings over time. This is where slow drift shows, and it needs many sessions before it says anything.
- **A model's reading** of the spans a rule flagged, fenced, labeled as a model's reading, and never the source of
  a "contradicted" verdict.

A citation is a claim here too. The Agentic Top 10 is a list of risks, so a detector names a related risk and is not
credited against it. AISVS's chapters C9 (Orchestration & Agentic Security), C10 (Model Context Protocol Security),
and C12 (Monitoring, Logging & Anomaly Detection), and its appendix families AC.5 (Explainability & Traceability of
Code Suggestions), AC.8 (Autonomous Agent Change Control Constraints), and AC.10 (Generation Audit Trail Completeness
and Validation), are where requirements a detector could speak to would be. None is cited until its text has been
read against what the detector does.

## What a report is

For one session: the table of the paper's Figure 19 (claims contradicted, by severity; findings silenced; findings
fixed for real; an example of each), a timeline of events with each one's tier, what fell outside the envelope, what
could not be checked and why, and which sources were read and which were missing. The same as `report.json` and
SARIF, sealed as `sv`'s reports are. It never says the agent is trustworthy.

## AgentVet's own safety

- **Everything it reads is hostile text.** A transcript, a tool's answer, a file name, or a commit message can be
  written by the agent or by whatever the agent read. Each is shown on one line or cleaned for the terminal, and
  fenced before a model reads it. `sv review` learned this on 21 November (ADR-026, Later).
- **It holds more sensitive text than `sv` does.** Keys are cut on the way in, as `Secret` does; a transcript is read
  and not kept.
- **It opens no network connection of its own.**
- **A hostile-session corpus** is a standing test: for each detector, a recorded session that should trip it and one
  that should not. The paper's trials (fifty builds of 12 to 13 October and six runs of 13 to 15 October 2026) are
  the first fixtures, with the health-tracking app left out or made anonymous as everywhere else.

## Toward stopping an action

Not in the first builds, and behind a decision of its own (ADR-159, decision 11). The shape it would take: the
pass-through and the hook can each refuse or pause a call, so a short fixed list is set to "ask the owner": a
protected file, a host outside the envelope, a destructive command, writing as the owner. Open questions for that
decision: what happens when `av` itself fails (let the action through, or hold it), how the owner answers when away
from the terminal, and how an attempt made after a refusal is recorded.

## What it costs to run

The rules cost nothing per session. A model's reading, when it comes, is the only part that spends money: it would
read only flagged spans, with a small model and the fixed part of its prompt cached, and would be off unless asked
for.

## Proposed order of the build

1. **The records move** into a shared crate, word for word, with `av report` reading them and writing the Figure 19
   table for a session (backlog 0705).
2. **The envelope and the six groups of detectors,** each with a fixture from the trials that trips it and one that
   does not.
3. **The pass-through and the keyed, checkpointed chain.**
4. **Telemetry intake,** for agents that are not coding tools.
5. **"Ask the owner"** on the short list, after its own decision.

## Questions for the owner

1. **Transcripts:** read only with consent for each session (proposed), or on by default since the owner is the
   first reader?
2. **The witness in the first build:** the keyed, checkpointed chain alone (proposed), or the recorder under another
   account from the start?
3. **The envelope's name and place:** `agentvet.toml` beside `stackvet.toml`, or a section of `stackvet.toml`?
