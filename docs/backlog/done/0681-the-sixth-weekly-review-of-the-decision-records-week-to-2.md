# The sixth weekly review of the decision records (week to 2 November 2026)

**Status:** done, 2 November 2026

The scheduled routine "Weekly decision-record review" (Monday 2 November 2026). Procedure: 0091 and `docs/adr/README.md`,
"When a record is written, and how it stays true". The fifth review is 0673.

**Claimed 2 November 2026 by session securevibe-weekly-adr-6** (the scheduled routine), in its own pull request, merged
before the review begins to write. Branch `claude/exciting-gates-y0f8of`.

**Review of 2 November 2026.** Every record, ADR-015 to ADR-156 (142), and the index, read against `main` at `76ef6a4` and
the week's merges since the fifth review's `f79af37`: 25 first-parent merges (4 Dependabot, 4 claims, 2 nightly-CI
reports, 15 others). This was a quiet week: the pull requests that changed code (#2374 `sv review` speed and the
typed-ahead drop; #2379 and #2382, a fetched project's report in the reader's words, `by = "sv-default"`; #2386 and
#2387, the NIST crosswalk and the OSCAL export) each wrote their "Later" entries or record in the same pull request.
This session read those four in full and the rest by title and file list; it did **not** use helper agents this week,
so records the week's merges did not touch were checked only against the facts below, not re-read line by line.
Checked against the code: the Dockerfile pins, `py7zr==1.1.4`, `node:24-alpine`, the index (ADR-155 and ADR-156 are
listed), the files each week's change touched against every **Governs:** list (50 files; the ungoverned ones are tests
and `sv-report/src/fetched*`, which ADR-154's Later entries cover in prose), and `sv-cli`'s new `boon` dev-dependency
(ADR-156 records it).

*One line per record.*
- **Did not match, and given a dated "Later, 2 November 2026" entry:** ADR-019 (headless-shell 157, Mailpit v1.32, moved
  by Dependabot on 31 October without touching the record, as ADR-128 allows).
- **Matched, with their own "Later" entry from this week's pull requests:** ADR-020 (sixteen `unsafe` blocks, #2374),
  024 (audience, tls, internet, level 2 named as defaults, #2379, #2382), 026 (typed-ahead drop), 081 (`oscal` input),
  132 (the self-assessment made again), 154 (the reader's words, two entries), 155 and 156 (new, accepted).
- **Matched:** all others, ADR-015 to 018, 021 to 023, 025, 027 to 080, 082 to 088, 090 to 131, 133 to 153. ADR-089's
  pin is still `1.1.4`. The count of `unsafe` blocks in ADR-020 (sixteen) was taken from the pull request's own entry
  and not recounted.
- **Governs lists:** no file gained a record this week that its list lacked, except what is above; the long-standing
  gaps the fifth review listed for the owner are unchanged and not added to.

*Decisions with no record.* None found that is costly to undo. The week's decisions were recorded as made: the
findings-only rule (ADR-155, the owner's word on 0674), the export (ADR-156, proposed 26 October, accepted 27 October),
the `sv-default` marking and the typed-ahead drop (Later entries). Still open from the fifth review and not touched
this week: the reasons for "Part 12 only" and "hold part 4" of 0625, the owner's turning off Cloudflare's Bot Fight
Mode, `[[left-out]]` tried and taken back, and the "as recommended" answers on 0550, 0602, 0589, 0590, 0515, 0603, and
0604. They are the owner's and their reasons are not written down anywhere, so none is written here.

*Lag, merge of the change to the record.* 0 days for every record written or given an entry by a building pull request
this week: ADR-155 (#2377, same pull request as its decision), ADR-156 (proposed in #2383 and #2384 before the build,
accepted in #2386 and #2387, 0 days), and the Later entries of ADR-020, 024, 026, 081, 132, and 154 (0 days each). The
one lag this review had to close is ADR-019: the Dependabot merges of 31 October (#2391, #2392) to this record, 2 days.
Compared with the fifth review (1 to 6 days, median about 3) it is shorter, because the week held no change to a
governed file under an "unchanged" line that was wrong.

*"Unchanged, because" lines.* Counted in the descriptions of the pull requests read in full (#2374, #2379, #2382,
#2386): 5, 21, 24, and 1 lines; the other merges' descriptions were not fetched, so no count for the whole week is
given. Reasons read: all fifty in those four hold as written; each names the file and what it only does. One thing to
note, not a passed-over decision: #2379 and #2382 name ADR-015, 045, 046, 087, 097, 099, 117, 149, and 151 because
`sv-manifest/src/lib.rs` is on their Governs lists and `AppSection` and `Capabilities` changed shape; the answers can
now be missing, and ADR-024 says so.

*Requirement ids.* Every id cited by ADR-015 to ADR-156 (V, C, and AC forms) exists in `data/frameworks`, checked by a
script this session. ADR-155 and ADR-156 cite none by id. The weak fits the fifth review listed are unchanged and not
re-read.

*Checks.* `cargo fmt --all --check`: clean. `python3 tools/adr_check.py --self-test`: ok. Clippy and the workspace
tests: see the end of this item.

**For the owner to decide:** (1) The items still open from the fifth review (the Governs gaps, whether a count quoted
as current is an entry, and the reasons above that are not written down). (2) Whether the weekly routine should use
helper agents again, as the fifth review did; this one did not, and says so above. (3) Two nightly failures are open:
0679 (Windows) and 0680 (the fuzz lock file).

*Test run, 2 November 2026 (Linux cloud container, no Docker, no outside network):* `cargo clippy --all-targets -- -D warnings`:
clean. `cargo test --workspace --no-fail-fast`: 347 `test result` lines, 4,591 passed, 3 failed, the same three as the
fifth review and all this container: `each_stored_7z_unpacks_to_exactly_what_it_says_in_py7zr` (no `py7zr` module) and the
two `sv probe` tests (`an_address_that_cannot_be_reached_exits_2_and_one_that_answers_exits_0`,
`an_address_that_answers_is_told_what_to_fix_what_held_and_what_could_not_be_settled`; no outside network). The records'
reference test (`decision_records`) passed in that run.
