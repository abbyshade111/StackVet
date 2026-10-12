# Try a coordinating session for the parallel builds, after the next loop trials and the website mock-ups

**Status:** open

Asked for by the owner on 12 October 2026, after a review of whether an orchestrator agent would help in building `sv`
or inside `sv`'s own workflows (session stackvet-orchestrator-agent-review). **Not before** the next trial of the
build loop (0257, the draft prompts' trial, and any loop trial run ahead of it) and the website mock-ups (0224); the
owner's word: "file it as a backlog item for AFTER the next build loop tests and website work". It also waits for
any roadmap phase above that is still open, as the items after the roadmap do.

**Why.** Many sessions build `sv` at once, and nothing hands out the work or the numbers: each session coordinates
with the others only through `main`. From the history, 6 to 12 October 2026: 602 pull requests merged into `main`,
187 of them (31%) only to record a claim, and 429 merges of `main` back into working branches. Nearly every rule in
`CLAUDE.md` about claims, numbers, and merging was written after two sessions collided: one item built twice
(20 September), two pairs of duplicate urgent fixes in one evening (9 October), ADR-094, 096, and 100 each taken twice,
and backlog 0248 taken twice. Backlog 0265 (`tools/record_numbers.py`, merged 12 October) removes the number
collisions with no agent at all; this item measures what is left after it.

**What to try.** One session that coordinates and builds nothing, for one working day:
1. It reads the roadmap and `python3 tools/backlog.py list --open` and gives each building session one item. Claims
   still go on `main` as the record (a claim counts only there), so nothing depends on the coordinator's memory,
   which is summarized like any session's.
2. It hands out record numbers with `tools/record_numbers.py next`, so two sessions never take the same one.
3. It sends pull requests in to merge one at a time. This is close to the merge queue the owner declined on 12
   October (0192, part 2), so ask the owner before this part, with the difference stated.
4. When `main` goes red, it gives the fix to exactly one session.

**What it may not do.** Decide anything that is the owner's: it brings the question to the owner. Review the work it
handed out: reviews stay with a separate session, so the one who assigned the work is not the one grading it, as
`sv` itself keeps the author apart from the grader. Nothing in this item puts an agent inside `sv`: the review found
that an agent there would need a network connection `sv` does not open, would make the same app's report differ from
run to run, and would rank its own word at the bottom tier anyway; the person's AI coding tool already leads the build
loop with `sv` as its tools. The trials stay fixed scripts (`docs/prompts/loop-protocol.md`), since a trial that
changes course after seeing results is what that protocol forbids.

**Before starting.** The owner's yes and a budget, since every extra session spends the owner's money. The record:
written as `Status: proposed` with the claim, as a Later entry on ADR-061 (how the work is shared out), and accepted
in the pull request that starts the trial day.

**How to judge it.** Compare the trial day with an ordinary day of the same week, counted from git: pull requests
that only record a claim, merges of `main` into branches, numbers taken twice, the same work built twice, and pull
requests refused after their checks passed. Keep the coordinator only if those fall and the items finished per day
do not; write down what happened either way, in this item and the record.
