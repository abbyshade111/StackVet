# The seventh weekly review of the decision records (week to 9 November 2026)

**Status:** done, 9 November 2026

The scheduled routine "Weekly decision-record review" (Monday 9 November 2026). Procedure: 0091 and `docs/adr/README.md`,
"When a record is written, and how it stays true". The sixth review is 0681.

**Claimed 9 November 2026 by session securevibe-weekly-adr-7** (the scheduled routine), in its own pull request, merged
before the review begins to write. Branch `claude/exciting-gates-9smc2c`.

**Review of 9 November 2026.** Every record, ADR-015 to ADR-156 (142), and the index, read against `main` at `90b6765` and the week's merges since the sixth review's `2660468`: three first-parent merges in
the seven days (#2398, the sixth review; #2396, a Dependabot bump of `github/codeql-action` v4.39.0 to v4.39.1, in
`.github/workflows/codeql.yml` and `action.yml`; and this claim, #2404). It was the quietest week so far: no pull request
changed code or a governed file. As last week, no helper agents were used; the records the week did not touch were
checked against the facts below, not re-read line by line. The clone was shallow, so the merge list above comes from
`git log --first-parent` since `76ef6a4`, which is complete for these days.

*One line per record.*
- **Did not match:** none.
- **Matched, no entry needed:** all of ADR-015 to ADR-156. The only governed-adjacent change is the CodeQL pin bump:
  ADR-051 mentions `.github/workflows/codeql.yml` (the analysis is advisory, blocks nothing) and the bump changes none of
  that, and ADR-128 already lets Dependabot move pins without a record. ADR-019's "Later, 2 November 2026" entry still
  matches the Dockerfile; Dependabot has five more bumps open (#2399 headless-shell 157.0.8103.15, #2400 busybox digest,
  #2401 tree-sitter-dart, #2402 toml_edit, #2403 zip), not merged, so nothing has moved yet. When #2399 merges, ADR-019's
  version line will lag by that bump, as it did last week; the next review closes it.
- **Governs lists:** no file gained a record this week that its list lacked. The long-standing gaps the fifth review listed
  are unchanged and not added to.

*Decisions with no record.* None: there were no decisions in the week's code. Still open from the fifth review and not
touched: the reasons for "Part 12 only" and "hold part 4" of 0625, the owner's turning off Cloudflare's Bot Fight Mode,
`[[left-out]]` tried and taken back, and the "as recommended" answers on 0550, 0602, 0589, 0590, 0515, 0603, and 0604.
They are the owner's and their reasons are not written down anywhere, so none is written here.

*Lag, merge of the change to the record.* No record was written or given an entry this week, so there is no new lag to
measure. The last figure stands: 0 days for last week's building pull requests, 2 days for ADR-019 (Dependabot merges of
31 October to the entry of 2 November).

*"Unchanged, because" lines.* Of the three merges, only this claim (#2404) has one, and it is correct (it adds a backlog
item and changes no governed file). The Dependabot pull request has none and needed none. Nothing passed over.

*Requirement ids.* Every id cited by ADR-015 to ADR-156 (V, C, and AC forms) exists in `data/frameworks` (660 ids known),
checked by a script this session. The weak fits the fifth review listed are unchanged and not re-read.

*Checks.* `cargo fmt --all --check`: clean. `python3 tools/adr_check.py --self-test`: ok. `cargo clippy --all-targets -- -D
warnings`: clean. `cargo test --workspace --no-fail-fast` (Linux cloud container, no Docker, no outside network): 347
`test result` lines, 4,591 passed, 3 failed, the same three as the fifth and sixth reviews and all this container:
`each_stored_7z_unpacks_to_exactly_what_it_says_in_py7zr` (no `py7zr` module) and the two `sv probe` tests
(`an_address_that_cannot_be_reached_exits_2_and_one_that_answers_exits_0`,
`an_address_that_answers_is_told_what_to_fix_what_held_and_what_could_not_be_settled`; no outside network). The records'
reference test (`decision_records`) is among the passes.

**For the owner to decide:** (1) The items still open from the fifth review (the Governs gaps, and the reasons above that
are not written down). (2) Whether a week with no governed change needs the full review at all, or a shorter one (this
one took the facts above and no more). (3) Two nightly failures are open: 0679 (Windows) and 0680 (the fuzz lock file).
