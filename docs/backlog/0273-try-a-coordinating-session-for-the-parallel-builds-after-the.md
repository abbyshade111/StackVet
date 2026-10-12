# Try a coordinating session for the parallel builds, after the next loop trials and the website mock-ups

**Status:** claimed by stackvet-orchestrator, 12 October 2026

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

**The owner's decisions, 12 October 2026 (claimed by session stackvet-orchestrator, the same day):**
- **When:** moved up. It is set up once the two other sessions are *on* the loop trial (0257) and the website work
  (0224), not after them, and starts on the owner's "go"; the owner tells this session, which does not poll.
- **Who:** this session holds the role, with no fourth session; the board in git lets another take it over. It also
  does the deep review of the documentation and the code that the owner asked for next; every finding goes on the
  board's queue. (The hold until the loop trials were done was lifted by the owner the same day: "the hand out of
  findings no longer depends on the test loop".) The role rule holds: the orchestrator never grades the builders'
  own work.
- **Part 3 (merges one at a time): left out.** Auto-merge stays as it is; the trial measures hand-outs and numbers.
- **Budget:** the trial workflow already puts its estimate to the owner for approval, so this item asks for none.
- **Three principles, the owner's words:** keep it simple, prioritize transparency, document and test thoroughly.
  So: no new software agent; the role is a written one (`docs/COORDINATION.md`) using the tools that exist; its whole
  memory is a board file in git that only the orchestrator writes, with a dated log line for every hand-out and why;
  one small tool, `tools/coordination.py`, with its own tests, that prints the board and counts the measures below
  from git. The record is a Later entry on ADR-061, proposed with this claim.

**Two arms, added at the owner's asking on 12 October 2026:** "one of the two sessions maintains the full context they
both currently have (CLAUDE.md, etc.) while the other session does not have access to any of that context", to test
"how much context matters once there's orchestration, and whether there's an appreciable cost difference from bloated
context files". The loop trials' `instructions` arm (docs/prompts/loop-protocol.md) is the model: the same work, with
and without the standing text.
- **The full-context arm** is a fresh session in the usual place: `CLAUDE.md` (3,100 words, read every turn), the
  auto-memory folder, and the `.claude/` rules and skills.
- **The no-context arm** is a fresh session in a separate clone of the repository (for example
  `~/stackvet-trial-noctx`): the memory folder is keyed on the project's path, so a new path starts empty; `CLAUDE.md`
  is replaced in that clone by a two-line stub that says only to take its work from the coordination board, and git
  is told to ignore that one local change (`git update-index --skip-worktree CLAUDE.md`), so no commit can carry it.
  The `.claude/settings.json` permissions are the same in both arms, so the difference is knowledge, not prompts.
- **Both fresh, and both Opus 5.5,** the owner's choice ("fresh session too"; "Both Opus 5.5, please"): a running
  session has already read the files into its conversation, and taking the files away does not take away what it
  knows, so the no-context arm has to be new; the full-context arm is new as well, so the only difference between
  them is the standing instructions, not one session's history or its model. Two fresh sessions also mean the trial
  no longer waits on the two running sessions finishing their work, as the owner noted the same day; the "when"
  above is overtaken by this, and the start is still the owner's "go".
- **What the no-context session still has:** the repository (`docs/ARCHITECTURE.md`, the backlog, the records, the
  tools' own help text), the CI checks that enforce many of the rules, and the board's hand-out. Every hand-out, to
  either arm, ends with the same five-line safety footer (no deleting, no force-push, no spending, no change to the
  repository's settings; ask the owner), which is a small amount of context by design.
- **What it measures,** on matched pairs of items the orchestrator alternates between the arms, written in the
  board's log with the arm beside each hand-out, and counted by `tools/coordination.py measure --arms`:
  1. Cost: the app's reading of each session's context size per turn, and tokens per item from the transcripts. A
     caveat to carry into the write-up: standing context is cached after the first turn, and a cached read costs
     about a tenth of a fresh one, so the money difference may be small even where the attention difference is large.
  2. The rules no check enforces, which only a session that read `CLAUDE.md` would follow: the `Assisted-by:` line
     on commits (ADR-092 reads it), American English, a claim before the build, auto-merge turned on, plain language
     in anything a person reads, one open build pull request at a time.
  3. The rules the checks do enforce: CI failures per pull request, and merges of `main` needed.
  4. Items finished, and questions to the owner per item.
- **The useful result** is line by line: a rule the no-context session follows anyway is one CI or the repository
  already carries, and a candidate to cut from `CLAUDE.md`; a rule only the full-context session follows is one the
  file earns its place with.

**How to judge it.** Compare the trial day with an ordinary day of the same week, counted from git: pull requests
that only record a claim, merges of `main` into branches, numbers taken twice, the same work built twice, and pull
requests refused after their checks passed. Keep the coordinator only if those fall and the items finished per day
do not; write down what happened either way, in this item and the record.

**Built 12 October 2026** (design entry 0381; ADR-061, Later, accepted): `docs/COORDINATION.md` (the role),
`docs/coordination/board.md` (the board, opened with the two arms' sessions named and 0263 queued), `tools/coordination.py`
(open, session, queue, assign, handout, finish, board, check, measure, setup-no-context, and a self-test),
`crates/sv-cli/tests/coordination.rs`, a line in `CLAUDE.md`, and a row in `docs/ARCHITECTURE.md`. The item stays
claimed until the trial day is run and written up here with `measure`'s counts.

**The owner's word, 12 October 2026, evening: both arms are cloud sessions** ("can we make the two fresh sessions
Cloud sessions so my computer doesn't get overloaded?"). What changes: the no-context arm is a second cloud environment
with the setup script `tools/coordination.py cloud-setup-script` prints (tested once in a session before the trial, since
the documentation does not say the script runs inside the clone); neither arm has this computer's memory, so they
differ on `CLAUDE.md` alone; hand-outs are pasted by the owner or sent with `claude -p ... --cloud <id>`; the cost
reading comes from the sessions' own usage display, not the desktop app. `docs/COORDINATION.md` says the same.
