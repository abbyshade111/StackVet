# The fourth weekly review of the decision records (week to 19 October 2026)

**Status:** claimed by securevibe-weekly-adr-4, 19 October 2026

The scheduled routine "Weekly decision-record review" (Monday 19 October 2026). Procedure: 0091 and `docs/adr/README.md`,
"When a record is written, and how it stays true". The earlier reviews are listed under 0091, "Reviews"; the third is 0266.
This review is listed there when finished.

**Claimed 19 October 2026 by session securevibe-weekly-adr-4** (the scheduled routine), in its own pull request, merged
before the review begins to write. Branch `claude/exciting-gates-4q1q2z`.

**Review of 19 October 2026.** Every record, ADR-015 to ADR-127, and the index, read against `main` at `296432d` and the
week's merges (629 first-parent merges since 12 October, 596 of them not claims), by five read-only helper agents (four
groups of records, one for decisions with no record). The session checked the counts it acted on against the code
(`data/ast-rules.json` 40 rules; fifteen non-test `unsafe` blocks; `REPORT_FILES` 14; `SIGN_INS_IN_A_RUN` 62; the CI matrix
`[rest, check, cli-1, cli-2]`; the helper images in `crates/sv-run/images/Dockerfile`; `unreadPackages` empty). ADR-100 to
ADR-127 are all accepted; none was proposed and unbuilt.

*One line per record.*
- **Match:** ADR-015, 016, 017, 024, 025 (apart from ADR-020's count), 027, 028, 029, 030, 031, 032, 033, 035, 036, 038,
  039, 040, 042, 043, 044, 045, 047, 048, 049, 050, 052, 054, 055, 056, 057, 058, 059, 061, 062, 064, 065, 067 to 081, 083,
  086 to 094, 096 to 100, 102 to 105, 107 to 113, 115, 117 to 127. Their Governs lists miss some files that carry later
  entries (listed by the agents: `signed_in/mod.rs`, `rules.rs`, `ai.rs`, `assemble.rs`, `sv-report/src/lib.rs` are kept
  ungoverned on purpose, ADR-103; the rest below).
- **Did not match, and given a dated "Later, 19 October 2026" entry:** ADR-018 (40 rules, not 35; the `unreadPackages`
  paragraph untrue since 9 October; tools now run by default), ADR-019 (helper images moved), ADR-020 (fifteen `unsafe`
  blocks), ADR-021 (62 refusal rows, not 57), ADR-022 (`POINTERS_SAY` sentence, three locators for V8.1.1 and V8.3.1,
  `policy_word.rs`), ADR-023 (a fourth `Outranked` variant), ADR-034 (fourteen files sealed), ADR-037 (a finding retitled),
  ADR-041 (a manifest unreadable at the end of a run is said), ADR-051 (four suite parts; `publish` no longer signs in to
  Docker Hub), ADR-060 (record numbers are free across branches), ADR-063 (the site palette and typefaces), ADR-066
  (`stackvet.sarif`; the shorter fence tag), ADR-082 (fourteen files), ADR-084 (the record's place), ADR-085 (a function
  moved files), ADR-095 (file name, count), ADR-101 (Windows drive path for `--keys`; `--hook`, `--folder`; a seventh
  prompt), ADR-106 (four parts, not three), ADR-114 (62 sign-ins, not 60), ADR-116 (the PDF is the last of fourteen).
  Not given an entry, cosmetic: ADR-074 (`has_timestamp` is `pub(crate)`), ADR-068 (0.0.0.0 also exempt), ADR-100 (the
  Status says "three days"; it was two, and a closed pipe now writes no crash file), ADR-103 (quote marks in the index row),
  ADR-109 (`--tools` still accepted), ADR-120 (9 words of headroom).
- **Governs** gained a file on ADR-018, 020, 021, 022, 023, 034, 066, 082, 085, 114. Not added for lack of a clear owner
  or because of ADR-103's choice, left for the owner: `human.rs`-like files for ADR-102, 108, 110, 111, 117, 121, 123 (their
  Later entries live in `signed_in/mod.rs`, `sv-report/src/lib.rs`, `sarif.rs`, `doctor.rs::WINDOWS_RUN_UNTRIED`,
  `capabilities_word.rs`, `pages.rs`, `browser.rs`, which could be named with `path::NAME`); ADR-026 (`report_again.rs`,
  `review/`), ADR-029 (`baseline.rs`, `crash.rs`), ADR-035 (`unset_keys.rs`, `preflight/`), ADR-043 (two test files),
  ADR-046, ADR-052, ADR-053, ADR-066 (files not named). The agents found these gaps; the session did not decide them.

*Decisions with no record, and what was done.* Entries on existing records (not new ones: each follows an existing rule or
a record's subject): ADR-101 (backlog 0547, 0550, 0602: what the AI tool may write about hand checks; the owner chose B
on 14 October), ADR-106 (0340: no dependency cache; the owner's decision of 12 October, reasons on the item), ADR-053 (six
credits cut to in part or withdrawn, #1561, #1591, #1607, #1622, #1637, #1657), ADR-018 (0471, reach per language),
ADR-066 (0361, the shorter fence tag). **Not written, for the owner:** the owner's own reasons for 0550 and 0602 are not
written beyond "as the owner chose" in the commits and the items; the answers "as recommended" on 0589, 0590, 0515, 0603,
and 0604 (#1973, #2064) were recorded in item text only and their builds were not checked; #1812 (0433, the website
column reading "Can be credited", 112 of 253) changed only `coverage.py` and the page. About 75 of the week's 307
code-touching merges were read by commit message and file list and none in full diff; the rest by title.

*Lag, merge of the change to the record.* Records and entries written or amended since 12 October by the build itself:
0 days for ADR-102 to 113, 115 to 118, 120 to 123, 126, 127 (written with the pull request that builds them), 1 for
ADR-124, 125, 2 for ADR-100, 3 for ADR-101 (written 12 October for 9 October). Written by this review: ADR-085, 3 days;
ADR-018 (five rules), 021, 023, 034, 037, 114: 3 to 4; ADR-020, 066, 082, 084, 095, 101 (drive path): 4 to 5; ADR-022, 019,
051, 106, 053 group: 5 to 7; ADR-041, 060: 7. So the lag is 3 to 7 days, median about 5, all on the changes that
landed with no record entry (the 13 to 16 October burst of CI, report-file, and sign-in-test work). Most of it is a count or
a name that moved inside a pull request that touched a governed file and said "unchanged".

*"Unchanged, because" lines.* 517 of the 668 pull requests merged since 12 October (search by day, so it includes the
12th's morning) carry the phrase, against 419 of 425 last week. Lines were not counted this time. Eight were read in the
text (#2138, #2132, #2130, #2124, #2116, #2115, #2113, #2108): each reason holds as written, though #2115 and #2116
repeat "carried here by the stack" for records the stack's earlier pull request was supposed to have checked, and #2113
said "ADR-022: Later" next to an "ADR-054: unchanged" that rests on a one-line diff. The ones that were passed over are the
decisions listed above, which carried a line or none and whose records were stale: ADR-051 (parts), ADR-034 (files),
ADR-114 (62) are cases of a count that changed in a pull request that argued "unchanged" for another reason. Nothing found
that was a decision hidden behind such a line beyond those.

*Requirement ids.* Every id cited by ADR-015 to ADR-127 exists in `data/frameworks` and fits what is said (the agents read
about 80 requirement texts: for example V6.6.1, V12.1.3, V7.2.2, V13.3.1, V1.3.1, V1.3.5, V7.5.1, V7.5.2, V10.3.1). The
weakest fits are ADR-056 and ADR-055, which say so themselves, and ADR-122's "secrets kept out of the code" against V13.3.1,
a loose reading. None is cited as met where it should not be. ADR-112 cites none, correctly.

*Checks.* `cargo fmt --all --check`: clean. `cargo clippy --all-targets -- -D warnings`: clean. `python3 tools/adr_check.py
--self-test`: ok. `python3 tools/adr_check.py --base origin/main --head HEAD`: nothing owed. `cargo test --workspace`:
see the end of this item.

**For the owner to decide:** (1) the Governs gaps above, and whether to name the file parts with `path::NAME` in one pass.
(2) The reasons for 0550 and 0602. (3) The board is reopened since 13 October (a burst of the deep review, with no
**Closed** line) while CLAUDE.md and `docs/COORDINATION.md` say it is closed. (4) Whether to ask each pull request to
list only the records it really touches, since 517 of 668 carry the line.

*Test run, 19 October 2026 (Linux cloud container, no Docker):* `cargo test --workspace --no-fail-fast`: 317 `test result`
lines, 4,196 passed, 3 failed. Two are this container: `each_stored_7z_unpacks_to_exactly_what_it_says_in_py7zr` (the
`py7zr` module was not installed for the `python3` the test starts) and `an_address_that_cannot_be_reached_exits_2_and_one_that_answers_exits_0`
(no outside network). The third, `every_ordinary_word_of_the_security_and_compliance_reports_is_in_the_pdf`
(`crates/sv-cli/tests/report_pdf.rs:89`), reports `flask-booking: not in the PDF: ["lt"]` and may be a real fault on
`main` (an escaped `<` read as the word "lt"); this review changed no code, so CI on this pull request shows whether it
fails there. Not investigated here.
