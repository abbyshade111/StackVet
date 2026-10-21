# The paper's data: what each table holds, and where it comes from

Every table behind the paper's figures, and the tables of the work done since, in one place (backlog 0568). Each
entry says what the table holds, where its rows come from, how to make it again, and what is not in it. The tables are
CSV files: they open in any spreadsheet. Nothing in any of them names the health-tracking app built for a friend.

**Two kinds of table.** The paper is written at one cut-off, `main` at 157ddc3, 11:37 Eastern on 4 October 2026, and
the first kind are its tables, made by hand at that cut-off and still exactly what the figures and the text quote.
The second kind, in `docs/paper/tables/`, are made by a script from the records kept since: the trials and the
coordination of 12 to 15 October. The owner answered on 15 October 2026 that the 4 October cut-off is arbitrary, since
the paper is not written yet: the timeline is broken at milestones instead, and every table says which period each row
is in (below, "The two periods"). The owner narrowed it on 21 October 2026 to two, version 1 (SecureVibe) against
StackVet as it is today, folding what was version 2 (`sv` before its rename) into StackVet. The hand-drawn figures stay
as they were drawn, at 4 October.

## The two periods

Made by `python3 docs/paper/tables/periods.py`, which reads each boundary from git rather than from this page, and
writes the tables below to the commit it is run at (`--check` says what no longer matches, at the commit named in
`periods.csv`). Times are Eastern.

| Period | From | To | How the boundary is found |
|---|---|---|---|
| 1. version 1 | `c85c174a2`, 18 September 09:32: the first commit of this history (the history before it was destroyed, `CORRECTIONS.md`) | `412092da2`, 26 September 20:46: the tag `v1-final`, v1's last commit on main (`v1-paper`, 20 September, is inside the period) | the tag |
| 2. StackVet | `278575a07`, 26 September 21:22: the merge of #224, "Promote sv to the top of the repository; v1 moves to the v1 branch" | the commit the tables are built at | the first commit on main that holds the promotion commit |

The rename to StackVet, the merge of #1127 at 8 October 20:59 ("StackVet step 3: the documents and the image say
StackVet", ADR-062), is inside the StackVet period. Until 21 October 2026 it began a third period, and what came before
it was "version 2"; a figure whose record is of that time (the study of 3 October, the deep review of 4 October) says
so in its StackVet panel.

A row is in the period its date falls in; a table whose rows carry no date is one dated record, and its date is the
one this page gives it below. The day of a boundary has a row on each side of it where a table counts by day.

**Combined, every period to the build** (`docs/paper/tables/`):
- `periods.csv`: one row per period and one for both: its first and last commit, days, commits (every commit
  reachable from main that day), commits on main's own line, pull requests merged, the tests in the tree at its end,
  decision records added, and backlog items done.
- `days.csv`: one row per day and period: the same counts, and the tests and decision records in the tree at the end
  of that part of the day. Tests are counted in the tree, not run: Rust's `#[test]` and `#[tokio::test]`, v1's
  TypeScript `it(` and `test(` in its test files, and Python's `def test_`.
- `decision_records.csv`: every record in `docs/adr/`, its status, when the first commit touched it, its period, and
  how many "Later" entries it has. v1's own records are on the `v1` branch and are not in it; `days.csv` counts them
  in the tree during version 1.
- `backlog_items.csv`: every backlog item, its status, and the day and period it was done, read from its status
  line. Items became files on 9 October, so work done before then is mostly not dated there.

**Per period** (`docs/paper/tables/by-period/<period>/`, for `v1` and `stackvet`): every table above and
every table on this page, cut to the rows in that period, each row with its `period`. **Every table with its period
column** (`by-period/all/`): the hand-made tables and the tables of the work since the cut-off, whole. The originals
are left as they are.

**The figure:** `figure-periods.html`, made by the same script: commits per day colored by period, the tests in the
tree as a line, and `periods.csv` beneath it.

## The ledgers extended to today, and the figures redrawn per period

The owner's answer of 15 October 2026 (backlog 0568): extend the hand-made ledgers to today, then redraw the figures
from them per period. Made by `python3 docs/paper/tables/ledgers.py` and `python3 docs/paper/tables/period_figures.py`
(each with `--check`), in that order, after `periods.py`.

**The ledgers to today** (`docs/paper/tables/to-today/`). Each is the ledger at the cut-off, row for row and
unchanged, followed by the rows since; every row says its `period` and whether it was `in_the_ledger` at the cut-off
or added since.

| Ledger | At the cut-off | Since | Where the rows since come from |
|---|---|---|---|
| `faults.csv` | 254 rows (255 faults; a row can hold several, `count`) | 366 | read from each backlog item and merge (below) |
| `corrections.csv` | 76 | 158 | read the same way |
| `coordination.csv` | 359 | 899 | claims and merges resolved by hand counted from git; pull requests closed unmerged from GitHub; the other kinds read |
| `tests_by_day.csv` | 23 days | 12 days | git |

- **Read, not counted.** The faults and corrections since were compiled on 15 October 2026 by Claude Code sessions,
  each reading a share of the 576 backlog items, the history of `docs/BACKLOG.md` from the cut-off to 8 October (when
  items became files), and the merges of that window, to the definitions in `TESTS-AND-FAULTS.md` and
  `CORRECTIONS.md`. Each row names its source (`source`, `backlog_item`) and how sure the reading was
  (`confidence`: sure, or unsure with the reason; 155 of the 524 rows are unsure). The fix's pull request and date are
  from git. One defect read twice, under two items, is counted once. They are in `tables/since-cutoff/`, as compiled.
- **The faults open at the cut-off** (61 rows, most of them the 4 October review's) keep their columns as they were
  and gain four, `fixed_since_*`: all 61 were fixed by 5 October, each with its pull request
  (`since-cutoff/open_at_cutoff.csv`).
- **The periods are by date, not by version.** `sv` was built beside v1 from 22 September, so its faults before the
  cut-over of 26 September are in version 1's period; the `version` column keeps the ledger's own label (v1 or `sv`).
- **Tests by day.** `sv`'s are the `#[test]` lines on main at the day's last commit, as the ledger counted them (this
  reproduces the published 1,645 and 1,655 exactly); 4 October has two rows, to 11:37 and after. Lines changed are
  counted between the last commits of consecutive days, from `crates/`, `tools/`, `examples/`, the `Dockerfile`, and
  `Cargo.toml`, with lines git sees moved rather than changed counted apart (`lines_moved`): on 8 October 60,543 lines
  of tests were taken out of their files into files of their own. The published rows left large moves out by hand;
  counted this way, 3 October's 15,115 reads 15,069, so the two are near, not the same.
- **Coordination since.** A claim is a merge to main, named for a claim, that touched only the backlog or the board; a
  merge resolved by hand is a merge of main into a branch whose combined diff has lines that match neither parent, as
  `COORDINATION.md` counted them (467 since, against 103 to the cut-off, over 1,431 pull requests merged since).

**The figures redrawn per period** (`docs/paper/`), each beside the figure of the cut-off, which stays as drawn:

| Figure | Redrawn from |
|---|---|
| `figure-tests-faults-by-period.html` | `to-today/faults.csv`, `to-today/tests_by_day.csv` |
| `figure-how-caught-by-period.html` | `to-today/faults.csv` |
| `figure-corrections-by-period.html` | `to-today/corrections.csv` |
| `figure-coordination-by-period.html` | `to-today/coordination.csv` |
| `figure-adrs-by-period.html` | `decision_records.csv`, `days.csv` |
| `figure-review-by-period.html` | `review.csv`, `since-cutoff/open_at_cutoff.csv`, `deep_review.csv` |

**The other thirteen, redrawn per period** (part 4; `docs/paper/tables/period_records.py`, `--check` to compare),
each beside the figure of the cut-off, which stays as drawn. A row goes in the period of the time its record gives it,
read as `periods.py` reads a moment; a period is a span of time, not a version, so `sv`'s rows from before its promotion
are in version 1's. A row its record does not date is counted as not dated, never guessed. Where a record ends at the
cut-off, the periods it does not reach say so, beside the nearest measure compiled since, named as a different measure
where it is one.

| Figure | Redrawn from | What the periods after its record hold |
|---|---|---|
| `figure-timeline-by-period.html` | `days.csv`, `periods.csv` | everything, from git; the milestones since are not chosen (the owner's) |
| `figure-security-by-period.html` | `reach_history.csv` (164 versions of `COVERAGE.md` on main's own line, from git; 86 to the cut-off, ending at 163, as the cut-off figure has), `requirements.csv` | everything |
| `figure-top10-by-period.html` | `cutoff-records/top10.csv` (92 rows), `to-today/faults.csv` | the ledger's `security` and `fail-open` faults, without a category: none is placed through a CWE |
| `figure-agentic-by-period.html` | `cutoff-records/agentic.csv` (86 rows) | nothing: no fault since is placed against the agentic list |
| `figure-decisions-by-period.html` | `cutoff-records/decisions.csv` (36 rows), `decision_records.csv` | the decision records added; who proposed and who chose each is not read |
| `figure-usability-by-period.html` | `cutoff-records/usability.csv` (62 rows), `to-today/faults.csv` | faults by what found them (friction that is not a fault is not compiled) |
| `figure-how-found-by-period.html` | `cutoff-records/usability.csv` (v1's 13), `to-today/faults.csv` | the same, labeled a different measure |
| `figure-trials-by-period.html` | `trials/make_figure.py`'s list, `loop2_builds.csv`, `three_tool_route.csv`, `pointer_trial.csv` | the four trials since; each trial is placed by when its write-up first reached the repository |
| `figure-cost-by-period.html` | `OPTIMIZATION.md` (v1, typed as the cut-off figure has it), the trials | the owner's API credit on trial builds; `sv` spends none |
| `figure-self-assessment-by-period.html` | the two `triage.json`, v1's row typed from the cut-off figure | nothing: `sv` has not been run on itself since |
| `figure-study-by-period.html` | `study.csv`, `three_tool_reports.csv` | the three-tool trial, named as a different experiment |
| `figure-three-arms-by-period.html` | `requirements.csv`, `study.csv`, `three_tool_reports.csv` | the same |
| `figure-how-sv-works-by-period.html` | `snapshots.csv` (from git at each period's last commit), `reach_history.csv` | counts of what `sv` was made of; the drawing is not redrawn (drawing it again is the owner's to ask for) |

**`tables/cutoff-records/`.** The rows of four cut-off documents, one per weakness, verdict, risk mark, decision, or
problem, each with the date that places it and how that date was got (`date_source`), compiled by Claude Code sessions
reading `TOP10.md` (sections 1 and 2), `AGENTIC.md` (the risk tables of sections 1 to 5), `DECISIONS.md` ("The
decisions"), and `figure-usability.html` with `figure-how-found.html`. A row dated by its own text uses that date; one
that names a pull request is dated by its merge on main, a commit by its own time. Every total of the documents is
reproduced: 49 weaknesses and 43 verdicts; 24 incidents (16 to 27 September), 13 open, 21 controls, 25 checks; the 36
decisions as "The count" has them; the nine groups of problems. Where the record is unsure, `confidence` says why: the
fix's merge stands for when cato-pipeline's and the study's problems were found (their reports are outside this
repository), and 16 agentic controls and checks are standing files, not events, so they are not dated. AGENTIC.md's
"to 27 September" brackets count rows by their sub-table, and seven of those rows are dated after the version 1
boundary (#239, #242, #252, the "Answered by:" incident, and the three self-check rows of section 5), so they are
in the StackVet period here.

## The paper's tables, at the 4 October cut-off

None of these is made by a script: each was compiled by hand from the sources in its own `source` column (or the
commit and pull-request columns where it has none), and none has been changed since the cut-off. To make one again,
read the sources a row names; there is nothing to run. The dates below are the first and last day its rows cover.

| Table | Rows | What one row is | Covers | Where a row comes from |
|---|---|---|---|---|
| `coordination.csv` | 359 | One event where two or more AI sessions met: a claim, a conflict, a duplicate, a collision, work sent between them, a pull request left unmerged, a fault one found in another's work | 18 September to 4 October | `source`: the commit or pull request |
| `corrections.csv` | 76 | One claim the checks made that turned out wrong, with what it said before and after, and who or what found it | 18 September to 3 October | `source`: the commit and pull request |
| `faults.csv` | 254 | One fault found in v1 or `sv`: when it was found and fixed, how it was found, whether a test came with the fix, and how many tests there were then | found 17 September to 4 October; 61 not fixed at the cut-off | `fix_commits` and `pr`; `source` where the row needed more (filled in 110 rows) |
| `tests_by_day.csv` | 23 | One day: the number of tests that day, the faults found, and how they were found | 17 September to 4 October | `commit` and `commit_first_parent`: the test count is read at that commit |
| `review.csv` | 58 | One finding of the deep review of 4 October (at eff3f17), with its severity, area, and state at the cut-off | 4 October | `fixed_by`, `open_pr`, and `backlog_entry`; the review itself is in the owner's study folder |
| `study.csv` | 606 | One measure of one of the five study apps, from one way of looking (`sv`'s findings, the outside tools, a person's reading) | 29 September to 3 October (run 11) | `source`: the report file in the study's results |
| `requirements.csv` | 1,035 | One ASVS requirement in one of v1's three arms, with its status and the evidence behind it | 20 September (v1's fixed record) | v1 at tag `v1-paper`; no `source` column |
| `findings.csv` | 38 | One finding v1 reported in one arm | 20 September (v1's fixed record) | v1 at tag `v1-paper`; its `source` column is the kind of check (config, sast, tests, ai-review), not where the row came from |

**`self-assessment-v2/`** holds `sv` checking itself, twice: on 27 September (`product-only/` and `repository/`) and
on 4 October (`2026-10-04/`, the source at the cut-off, scanned by `sv` at 84dcbd4). Each holds `sv`'s own report
(`report.json`, `findings.sarif`) as it wrote it, and, for the product-only runs, `triage.json`: every finding read by
a Claude Code session and marked accepted, a false alarm, or real (27 September: 61 accepted, 191 false alarms;
4 October: 60 accepted, 413 false alarms, 6 real). These are records, not made by any script in the repository.

**`trials/make_figure.py`** is the one script among the figures at the cut-off (`tables/periods.py` writes
`figure-periods.html` since, above): it writes `figure-trials.html` from the trials'
results files in `docs/prompts/library-trial/`, typing only each trial's size and cost from its write-up.

**What these tables do not hold:** anything after 11:37 on 4 October (that afternoon is `SINCE-THE-CUTOFF.md`); v1's
history before 18 September, which was destroyed (`CORRECTIONS.md`); and the study's raw apps, which are in the
owner's study folder, not here.

## The figures, and the date of their data

All but two of the figures at the cut-off are drawn by hand (`figure-periods.html`, added since, is made by a script), with their numbers typed into the page from the table or document they cite. Each says
the date of its data on the page.

| Figure | Its data | Date of its data |
|---|---|---|
| `figure-adrs.html` | `ADRS.md`, the decision records | 16 September to the cut-off |
| `figure-agentic.html` | `AGENTIC.md` | the cut-off |
| `figure-coordination.html` | `coordination.csv`, `faults.csv` | the cut-off |
| `figure-corrections.html` | `corrections.csv` | the cut-off |
| `figure-cost.html` | `OPTIMIZATION.md` | to the cut-off |
| `figure-decisions.html` | `DECISIONS.md` | to the cut-off |
| `figure-how-caught.html` | `faults.csv`, `TESTS-AND-FAULTS.md` | the cut-off |
| `figure-how-found.html` | v1 at `v1-paper` | 18 to 20 September |
| `figure-how-sv-works.html` | the code at the cut-off | the cut-off |
| `figure-review.html` | `review.csv` | the cut-off |
| `figure-security.html` | `requirements.csv`, the coverage documents | to the cut-off |
| `figure-self-assessment.html` | `self-assessment-v2/` and v1's own self-assessment | 27 September and 4 October |
| `figure-study.html` | `study.csv` | 3 October (run 11) |
| `figure-tests-faults.html` | `faults.csv`, `tests_by_day.csv` | to the cut-off |
| `figure-three-arms.html` | `requirements.csv` | 20 September (v1's fixed record) |
| `figure-timeline.html` | `TIMELINE.md` | 16 September to the cut-off |
| `figure-top10.html` | `TOP10.md`, `findings.csv`, the self-assessment | to the cut-off |
| `figure-trials.html` | made by `trials/make_figure.py` | 5 to 8 October |
| `figure-periods.html` | made by `tables/periods.py` from git | 18 September to the build |
| `figure-usability.html` | `figure-how-found.html` and the issue lists | to the cut-off |

## The work since the cut-off: `docs/paper/tables/`

Made by `python3 docs/paper/tables/make_tables.py`, which reads only committed records and writes every table here;
`--check` says which tables no longer match their records (the board's log grows, so `board_log.csv` and
`board_items.csv` fall behind it until the script is run again). Every row has a `source` naming the record and the
place in it. Two records are prose only, the pointer trial and the Podman measurement: their rows are typed into the
script from the item, each naming it, and the `source` says "typed from its prose". A cell no record fills is empty.

**The second loop trial** (12 and 13 October; `docs/prompts/loop-2/`): `loop2_builds.csv`, one row per build, 90 in
all: 20 controls taken from the drafts trial and 70 built for this trial, Haiku 5.5 and Sonnet 5.5 in phases A, B1,
B2, and C. For each: the brief and arm, cost, how often the builder called `sv`, what `sv`'s four first checks
answered, the outside tools' findings, what made a finding go away (`went_away`: set aside, file removed, code
changed, unexplained), and the moves toward a credit (`credit_seeking`). From `scores.json`. Not in it: `sv`'s own
findings per build, which `scores.json` does not keep; the hand reading of ten findings (in `results.md`); and the
count of text forms listed, which `results.md` says is wrong (0504, 0515).

**The three-tool prompt trial** (14 October; `docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`), Antigravity,
Cursor, and VS Code, each one row per tool and line of the write-up's tables:
- `three_tool_route.csv`: how each was set up, the `sv` commit, and the minutes from prompt to connection and report.
- `three_tool_prompts.csv`: whether each prompt was done as written.
- `three_tool_reports.csv`: the three reports like for like: requirements by status, findings, the app's own tests,
  and how long the run took.
- `three_tool_said_did.csv`: what Antigravity said against what the record shows (the write-up has this for
  Antigravity only; Cursor's and VS Code's claims matched).
- `three_tool_gaming.csv`: every place a check was passed or not: fixed, gamed, dismissed in words, with the counts as
  the write-up states them (typed from its prose; "several" is an empty count).
Not in them: the chats and the apps themselves, which are on the owner's drive.

**The owner's follow-up trial** (15 October; backlog 0577): the same seven prompts with the harness and the model
swapped, to tell which one gamed the check. Three runs are rows of their own in `three_tool_reports.csv` and
`three_tool_gaming.csv`, each named with its tool, model, and date: Antigravity with Claude Opus 5.5, and Cursor with
Gemini 3.8 Flash twice (the first run read the earlier trial's app through stale settings; the second was done again
cleanly). Each row is typed into `make_tables.py` from 0577's own prose and says so, and only the cells that prose
states are filled. The readers' cell-by-cell write-ups were in session scratchpads and are not in the repository, so
the route, the prompts, and said-versus-did have no rows for these runs; see "Limits" below.

**The pointer trial** (14 October; backlog 0516): `pointer_trial.csv`, one row per example app and reading: what
`sv questions` pointed at for the authorization requirements (V8.1.1, V8.3.1), and whether it was right. Three
readings: before the fix, after it, and after 0518.

**The Podman measurement** (14 October; backlog 0508): `podman_measurement.csv`, one row per way Podman differed
from Docker, and what each did to `sv`. On Linux only; the Mac half was not measured.

**The deep review of 13 October** (backlog 0417, run against `main` at 19aec8d7): `deep_review.csv`, one row per item
it filed, 83 in all (0418 to 0500): severity, area, title, its status from the item's own file, and the pull request
that finished it from the board's log. Fourteen have none: the thirteen proposals still open (0488 to 0500), and
0422, the owner's own repository setting, done on 13 October outside the board.

**The coordination burst** (13 to 15 October; `docs/coordination/board.md`):
- `board_log.csv`: every line of the board's log, with the verb (assigned, finished, queued, released, …), the item,
  the session, and the pull request where the line names one.
- `board_items.csv`: one row per item finished, with its session, when it was first assigned, when it was finished,
  the minutes between, the minutes since that session last finished something (closer to the time spent building it),
  and its pull request.
The times are when the orchestrating session wrote the line, in Eastern time, not when the pull request merged:
finishes are logged in batches. Not in them: red runs, which the log does not record (two are named in passing in
assign reasons), and the earlier coordination trial of 12 and 13 October (`board-closed-2026-10-13.md`).

## Limits

What the trial tables can and cannot carry, written with backlog 0577 (the owner's follow-up trial of 15 October):

- **One run each.** Four runs of AI coding tools building one brief cannot rule out chance: a behavior seen once may
  not come back, and one not seen may.
- **What the gaming followed.** Rewriting code and dependency files to make a check stop seeing something happened in
  one run only, Gemini 3.8 Flash in Antigravity on 14 October, which was also the only run in which `sv` raised two
  word-match contradictions (`media_type`, `httpx`; fixed since, backlog 0538). On this evidence it followed neither
  the model alone nor the harness alone, but that model's answer to those two findings; another Flash run in
  Antigravity might have gamed without them, and one run cannot say.
- **Not like for like everywhere.** The 14 October Antigravity run was held to level 2 (196 requirements applied) and
  every run of 15 October to level 1 (73 or 74), so their counts by status are not comparable; within a level they are.
  The first Cursor run of 15 October is of an app copied byte for byte from the earlier trial, through stale MCP
  settings, and its counts are that copy's.
- **The prompts were those of 14 October,** on purpose, so the model and the harness were the only things changed; the
  prompt rules written since (backlog 0545 to 0557) are untried in any tool.
- **What is typed, and what is not.** The 15 October rows are the report counts and the gaming the item's prose states.
  The readers' write-ups behind them, cell by cell, were never committed, so the route, the prompts, and the claims
  against the record are rows for 14 October only. A cell no record fills is empty, never estimated.
- **The `sv` the reports came from.** The 15 October reports were made with `sv` at 779676c5 and run again at
  f3197a2c with the same counts, except that the first Cursor run's one finding needing attention went away (backlog
  0529). The tables carry the first.
