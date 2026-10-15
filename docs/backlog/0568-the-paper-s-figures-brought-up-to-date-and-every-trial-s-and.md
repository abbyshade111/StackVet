# The paper's figures brought up to date, and every trial's and measurement's data compiled into tables the owner can analyze

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's ask, 14 October 2026 (evening): once the assigned and ready work is done, after the long-file split,
have one session (documentation) update all the figures for the paper and compile the data from the trials and the
other work into tables, so the owner can analyze the data themselves. For documentation's lane; the paper's folder is
`docs/paper/`.

**What to build.**
- Refresh every data file the figures read (`docs/paper/faults.csv`, `corrections.csv`, `coordination.csv`,
  `tests_by_day.csv`, `study.csv`, `review.csv`, `requirements.csv`, `findings.csv`, and the JSON the self-assessment,
  cost, decisions, and usability figures read) from their sources up to the day of the build, with the generating
  script named in each file's `source` column or header, so a row can be traced. Where a file was made by hand, say so
  and give the date range it covers.
- Re-render every `docs/paper/figure-*.html` from the refreshed data; each figure says the date of its data.
- New tables, one CSV per dataset with a `source` column, for work that has no table yet: the second loop trial
  (builds, arms, scores, findings per build, what dodged a check), the three-tool prompt trial
  (`docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`: route, times, prompt adherence, said-versus-did, the like-for-like
  counts, the gaming cases), the pointer trial (0516: pointers before and after, per app), the Podman measurement
  (0508), the deep review (0417: items by area, severity, and status with their closing pull requests), and the
  coordination burst of 13 to 15 October (the board log: items per session, minutes per item, pull requests, red
  runs). Numbers only from the records; a cell that cannot be filled from a record is left empty with a note, never
  estimated.
- `docs/paper/DATA.md`: an index saying what each table holds, where its rows come from, how to make it again, and
  what is not in it. Plain language: the owner reads the tables, not the scripts.
- Nothing in these tables names the health-tracking app built for a friend by its real name; the anonymity rule in
  CLAUDE.md holds in every cell.

**Record.** `ADR-0NN: unchanged, because ...` for the paper's own records if any govern `docs/paper/`; otherwise
none. The figures are the paper's and change no check.

**Part 1 built, 15 October 2026** (documentation). The survey first: every figure but `figure-trials.html` is drawn
by hand with its numbers typed in, and every table in `docs/paper/` was compiled by hand, all at the paper's cut-off
(`main` at 157ddc3, 11:37 Eastern on 4 October), which the paper's text quotes. Refreshing them "to the day of the
build" moves that cut-off, so it is asked of the owner (`docs/coordination/waiting/0568.md`): move the cut-off, or
keep the figures at 4 October and add the work since beside them. Built meanwhile, as either answer needs it:
- `docs/paper/DATA.md`: every table's contents, rows, dates, sources, how to make it again, and what it leaves out;
  every figure's data and its date.
- `docs/paper/tables/make_tables.py` and the eleven tables it writes from the records since the cut-off: the second
  loop trial, the three-tool trial (five tables), the pointer trial, the Podman measurement, the deep review of
  13 October, and the board's log (every line, and one row per item finished).
- `figure-three-arms.html`, the one figure that gave no date, now names its data and date.
Still to do, after the owner's answer: the figures' refresh, or a "since the cut-off" figure beside them.

**The owner's answer, 15 October 2026 (asked through the board's waiting line):** neither (a) nor (b): the paper is not written yet, so the 4 October cut-off is arbitrary. Break the timeline at three milestones and build every table and figure per period, then combined tables for everything up to now: (1) version 1, up to the cut-over when v1 was tagged in GitHub (the tags v1-paper and v1-final; the promotion of sv to main on 26 September 2026); (2) version 2, from that cut-over to the renaming to StackVet; (3) StackVet, from the renaming to the day of the build. Take each boundary's date and commit from git (the tags, the promotion merge, the commit that renamed the project), name them in DATA.md, and say in every table which period a row is in. The paper's prose is the owner's to write against these.
