# Try a coordinating session for the parallel builds, after the next loop trials and the website mock-ups

**Status:** done, 13 October 2026

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

**The cloud environment tested, 12 October 2026:** the owner's test session printed the stub, the `S` flag, and a clean
tree. It then read the committed `CLAUDE.md` from history and called the substitution possible tampering: so the
no-context arm's transcripts are checked for whether the session read the committed file on its own, and the stub says
the replacement is deliberate.

## The trial, written up (13 October 2026)

**What ran.** From 12 October, 20:46 UTC (the first builder pull request merged) to 13 October, 20:43 UTC (builder-full's
last item, 0339, merged: the end point the owner set), two fresh cloud sessions on Opus 5.5 built the findings of the
deep review (0277) and the efficiency review (0335), handed out by this session through the board. builder-full had
`CLAUDE.md`; builder-none had a two-line stub and the five-line footer. Hand-outs went by the board alone after each
session's first: a lane of up to six items, planned so no file was in both lanes at once, matched pairs split across
them. All counts below are from git (merge times in UTC) and the board's log unless marked otherwise.

**Items finished inside the window:** builder-full 39, builder-none 41 (two more, 0414 and 0355, merged after the end
point; 0365 was handed back unbuilt, by the owner's choice, since its own condition was not met). Nothing was built
twice, no claim collided, and no backlog, design, or record number was taken twice during the trial (the tree's 36
duplicate numbers all date from 8 to 11 October).

**The day against ordinary days** (`measure`, whole repository, every session, so the trial is one part of each):

| Day | Pull requests merged | Of them only a claim | Merges of `main` into a branch |
|---|---|---|---|
| 10 October | 71 | 22 (31%) | 8 |
| 11 October | 61 | 23 (38%) | 6 |
| 12 October (trial from evening; the review filings too) | 163 | 40 (25%) | 25 |
| 13 October (the trial's day) | 103 | 13 (13%) | 27 |

Claims as their own pull requests fell, because `assign` made each claim inside the board's own pull request (46 board
pull requests for 82 finished items). Merges of `main` into branches rose, and nearly all are builder-none's: it merged
`main` into 45 of its branches, against builder-full's 1, because the stub held by `skip-worktree` made a local merge the
only safe way for it to stay current (see the interventions). Pull requests refused after passing are on GitHub only
and were not counted.

**Per arm, from each finished pull request** (82 in all, including the two after the end point):

| | builder-full (full context) | builder-none (no context) |
|---|---|---|
| Items finished | 39 | 43 |
| Changed a decision record | 19 | 13 |
| Record expected by the item and written (a Later entry where the item said one was owed) | 17 of 18 | 12 of 14; the two others were met another way the item allowed |
| "ADR-NNN: unchanged, because" lines in descriptions | 129 | 110 |
| Description says what was not verified | 18 of 39 | 43 of 43 |
| Average description length | 2,708 characters | 3,665 characters |
| British spellings in descriptions | 0 | 2 ("behaviour") |
| Commits not naming the tool (merge commits aside) | 1 | 0 |
| Pull requests ending "Generated with Claude Code" | 39 of 39 | 43 of 43 |

**Time lost waiting, and why** (gaps between merges over two hours, from git):
- builder-full, 7.3 hours (before 0323) and 4.6 hours (before 0326): its cloud disk filled with its own build folder,
  and it asked the owner before deleting it, as `CLAUDE.md` and the footer's "delete nothing" both say; the owner was
  away the first time, and the second question sat inside a bullet of a report with no check-in set after it. About 12
  hours of the full arm's 24.
- builder-none, 7.7 hours (before 0279): it stopped to ask about 0263, which this session had assigned in error (the
  owner had set it aside in the item's text, below a status line that read "open"); then it waited about four hours more
  because this session held back the next lane for a second yes the owner had already given.

**Questions each arm put to the owner:** builder-full 2, both about deleting its own build folder. builder-none 8:
three were the owner's to decide and right to ask (0263, the refusal on old Docker in 0301, 0365 without evidence);
five came from what the stub lacked: whether to take its second item without a pasted go-ahead, merging its own green pull requests (its own permission check refused
without `CLAUDE.md`'s "Git is pre-approved" line), a `CLAUDE.md` change on `main` it could not merge past, conflicts on a
status line, and a real conflict it could settle itself.

**Interventions** (things the owner told one arm beyond the footer): builder-none, seven: auto-merge; merging its own
green pull requests; `gh pr update-branch`; a scratch worktree for status-line conflicts; settling conflicts itself;
keep checking the board; delete its own build folder. builder-full, one: delete its own build folder. The last was given
to both, in the same words, and is not a difference between them.

**What the line-by-line result says** (the item's own test: a rule the no-context arm follows anyway is a candidate to
cut from `CLAUDE.md`; a rule only the full arm follows is one the file earns):
- *Followed anyway, carried by CI, the tools, or the defaults:* decision records (the "Decision records" check; both
  arms equal), naming the tool in commits and the PR footer (the default attribution), claims and `done`
  (`tools/backlog.py` and the board), red-first tests and saying what was not verified (builder-none did both every
  time, more often than builder-full).
- *Only the full arm followed, so `CLAUDE.md` earns them:* auto-merge on every pull request; merging its own green work
  (this one is also the permission a session's own guard reads: not knowledge but authority); bringing `main` in and
  settling conflicts (`merge_main.py`); American spelling; taking the next board item without a pasted go-ahead.
- *A `CLAUDE.md` rule that cost the full arm:* "ask before deleting" cost about 12 hours, for a build folder in a
  throwaway cloud container.
- *Not measured:* tokens and money per arm. The cloud sessions' own usage displays were not read; the owner can read
  them there, and the write-up adds them if so. The caveat from the plan stands: cached context costs about a tenth
  of fresh, so the money gap may be small even where the attention gap is large.

**What the board did well:** no duplicate work, no claim or number collisions, a claim visible on `main` before work
began, lanes that let both sessions run for hours without a hand-off, and a record of every hand-out with its reason.
The two separate security reviews the owner asked for (0300 and 0301, read-only) each found real gaps, filed as 0410
and 0414 and built inside the trial.

**What it did badly** (this session's own faults first):
- A builder waiting on a question looked the same as a stalled one: the board saw only git's silence. A "waiting on the
  owner" line a builder can write is the first thing to add.
- It assigned 0263 from a status line without reading the whole item; `withdraw` was built that day (#1684), and items
  are read whole since.
- It held a released lane back for a second yes the owner did not know was needed: about four hours of builder-none's.
- Its board check, its Windows test, and six of the review's record lines were wrong at first; the builders and CI
  caught them.
- The no-shared-file rule made `crates/sv-run/src/docker.rs` and `.github/workflows/rust.yml` one queue each, so the
  last day could not be split evenly, and when one arm stalled the other ran short of work.
- It could not see sessions it did not run: an off-board change (0394) conflicted with a board item.
- A lane can only be added to at the end; reordering took two withdrawals.
- 46 board pull requests, each with its own CI run, for 82 items.

**Candidates for the owner** (none is made here):
1. In `CLAUDE.md`: a session's own build output in its own cloud container may be deleted without asking.
2. Keep the coordinating session for bursts of independent findings (a review's items, as here), with a "waiting"
   line, insert-at-position, and a `close` command added to the board; not for ordinary days.
3. 0322 and 0325 (they edit `CLAUDE.md`) were held for the trial and can go now.

**Cleanup, for the owner's yes** (nothing deleted): the 43 open items that were never on the board; the nine left on
the board's queue, whose "queued" line now says the board closed (0289, 0290, 0322, 0324, 0325, 0338, 0341, 0365, 0369;
0263 stays set aside); the seven Dependabot pull requests (#1627 to #1634); 0390's question (the board test stays off
Windows?); the no-context cloud environment; and this session's scratch files and worktree.

**The board is closed:** both sessions are off it, so the `CLAUDE.md` rule about taking work from it no longer applies,
and the remaining items go the normal way.

**The owner's choices, 13 October 2026** ("yes to 1 and 2, go ahead with the cleanup"):
1. `CLAUDE.md` now lets a cloud session delete its own build output in its own cloud container without asking, and
   nothing else (the "Keep the disk tidy" rule).
2. The coordinating session is kept for bursts of independent findings, such as a review's items, not for ordinary
   days; the three additions it needs first are backlog 0416 (a waiting line, insert at a position, a close command).
3. The cleanup goes ahead: 0390 closed as the intended end state; the open items never on the board checked against
   `main` for any overtaken by later work; the rest reported to the owner, nothing deleted without saying what.

