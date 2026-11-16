# The eighth weekly review of the decision records (week to 16 November 2026)

**Status:** done, 16 November 2026

The scheduled routine "Weekly decision-record review" (Monday 16 November 2026). Procedure: 0091 and `docs/adr/README.md`,
"When a record is written, and how it stays true". The seventh review is 0682.

**Claimed 16 November 2026 by session securevibe-weekly-adr-8** (the scheduled routine), in its own pull request, merged
before the review begins to write. Branch `claude/exciting-gates-l73r06`.

**Review of 16 November 2026.** Every record, ADR-015 to ADR-156 (142), and the index (142 rows, one per file), read
against `main` at `626dcd6` and the week's merges since the seventh review's `ba58c8c`: five first-parent merges in the
seven days (#2405, the seventh review's own account; #2406, #2411, #2412, and #2413, the nightly CI routine's notes on
the new backlog item 0683; and this claim, #2414). Every one changed a backlog file and nothing else: `git diff --stat
ba58c8c..626dcd6` is one file, item 0683. No pull request changed code, `data/`, a workflow, a Dockerfile, or any other
governed file, and no record changed hands. As in the sixth and seventh reviews, no helper agents were used; the records
the week did not touch were checked against the facts below (the Governs and reference test, the id check, the pins),
not re-read line by line. The clone was fetched to depth 200, which holds every merge of these days.

*One line per record.*
- **Did not match:** none.
- **Matched, no entry needed:** all of ADR-015 to ADR-156. ADR-019's "Later, 2 November 2026" entry still names the
  pins `crates/sv-run/images/Dockerfile` carries (`headless-shell:157.0.8103.6`, `mailpit:v1.32`, `busybox:1.38`,
  `node:24-alpine`); Dependabot's bumps of the helper images (#2407, headless-shell 158.0.8127.2; #2400, the busybox
  digest) and of the root `Dockerfile`'s `debian` digest (#2409, governed by ADR-062 and ADR-081, which name no digest)
  are open and not merged, so nothing has moved. ADR-149 is still `proposed`, which is right: backlog 0625's fourth
  part is on hold at the owner's word and nothing of it is on `main`. ADR-051 (the Rust workflow is advisory on its
  lints only where it says so) is not touched by the week's one new fact, below.
- **Governs lists:** no file gained a record this week that its list lacked. The long-standing gaps the fifth review
  listed are unchanged and not added to.

*Decisions with no record.* None: there were no decisions in the week's code. The one new fact of the week is not a
decision anyone made here: on 13 November the CI runner's stable Rust moved from 1.99.1 to 1.100.0, whose Clippy flags a
loop at `crates/sv-report/src/pdf/layout.rs:617` (`needless_range_loop`), so the Rust workflow's lints job has been red
on `main` four nights running (item 0683, open and unclaimed when this review began; the nightly routine recorded it
each day and fixed nothing). Rewriting that loop changes no decision: ADR-116 governs the PDF files for what the PDF
says, not how a loop is written. Still open from the fifth review and not touched: the reasons for "Part 12 only" and
"hold part 4" of 0625, the owner's turning off Cloudflare's Bot Fight Mode, `[[left-out]]` tried and taken back, and the
"as recommended" answers on 0550, 0602, 0589, 0590, 0515, 0603, and 0604. They are the owner's and their reasons are not
written down anywhere, so none is written here.

*Lag, merge of the change to the record.* No record was written or given an entry this week (`git log ba58c8c..626dcd6
-- docs/adr` is empty), so there is no new lag to measure. The last figure stands: 0 days for the building pull requests
of the week to 2 November, 2 days for ADR-019 (Dependabot merges of 31 October to the entry of 2 November).

*"Unchanged, because" lines.* Of the five merges, one has a line: #2405, the seventh review's account, which wrote
`ADR-0NN: unchanged, because this only writes the review into backlog item 0682; no governed file changes.` The reason is
true, but `0NN` is the template's placeholder, not a record's number; the check accepted it because the pull request
touched no governed file, so the line was never read against a record. Harmless this time, and worth not repeating: a
pull request that touches no governed file should say "No decision", as this review's two do. The four nightly pull
requests have none and needed none. Nothing passed over.

*Requirement ids.* Every id cited by ADR-015 to ADR-156 (343 distinct citations in the V, C, and AC forms) exists in
`data/frameworks` (662 ids known, ASVS 5.0.0, AISVS 1.0 and its appendix C, and the Secure by Design checklist),
checked by a script this session. The weak fits the fifth review listed are unchanged and not re-read.

*Checks.* `cargo fmt --all --check`: clean. `python3 tools/adr_check.py --self-test`: ok. `cargo clippy --all-targets --
-D warnings`: **red**, the first review where it is, and for the reason above alone: this container's Rust is also
1.100.0, and Clippy stops at `sv-report` with the one `needless_range_loop` error of item 0683; nothing else is flagged
before it stops, and whether other crates hold more is not known until it is fixed. `cargo test --workspace --no-fail-fast` (Linux cloud container, no Docker, no outside network): 347 `test result` lines, 4,590 passed, 4 failed, 1 ignored. Three are the same container-only failures as the fifth to seventh reviews: `each_stored_7z_unpacks_to_exactly_what_it_says_in_py7zr` (no `py7zr` module) and the two `sv probe` tests (`an_address_that_cannot_be_reached_exits_2_and_one_that_answers_exits_0`, `an_address_that_answers_is_told_what_to_fix_what_held_and_what_could_not_be_settled`; no outside network). The fourth, `the_listing_names_the_commit_sv_was_built_from`, was this session's own doing: it merged `main` (the claim's merge, `7fff4e5`) into its branch while the suite, built at `e69094b`, was still running, and the test compares the commit `sv` was built from with the checkout's head. Run again at `7fff4e5` it passes; it is not a fault in `main`. The records'
reference test (`decision_records`) is among the passes.

**For the owner to decide:** (1) The items still open from the fifth review (the Governs gaps, and the reasons above that
are not written down). (2) Whether a week with no governed change needs the full review at all, or a shorter one (this
one, like the seventh, took the facts above and no more). (3) Dependabot has nine pull requests open and unmerged, the
oldest from 30 October (#2393, #2395); while they wait, ADR-019's pin lines stay true, and when they merge the next
review closes the lag as before. (4) Item 0683, the red lints job: this session claims and fixes it next, in its own
pull request, as the lowest-numbered open item and the one check every code pull request now fails; the owner decides
nothing there unless the fix is unwanted.
