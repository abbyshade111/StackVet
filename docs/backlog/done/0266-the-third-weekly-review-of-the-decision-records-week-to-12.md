# The third weekly review of the decision records (week to 12 October 2026)

**Status:** done, 12 October 2026

The scheduled routine "Weekly decision-record review" (Monday 12 October 2026, 8:45 Eastern). The earlier entry,
"A weekly review of the decision records" (0091), was marked done on 10 October with the routine continuing outside
the backlog, so this week's review is claimed here and listed under that entry's Reviews when finished. It is also the
first scheduled firing 0223 was waiting for. Procedure: 0091 and `docs/adr/README.md`.

**Claimed 12 October 2026 by session securevibe-weekly-adr-3** (the scheduled routine), in its own pull request (#1510),
merged before the review began to write. Branch `claude/exciting-gates-gr1vgj`.

**Review of 12 October 2026.** Every record, ADR-015 to ADR-099, and the index, read against `main` at `3e5b72b` and the
week's merges (778 first-parent merges since 5 October, 529 of them not claims), by six read-only helper agents (one per
group of records, one for decisions with no record, one for the "unchanged, because" lines). The session checked the
findings it acted on against the code. ADR-099 was proposed and unbuilt when read (it merged during the review) and is not
judged. The clone was shallow; it was deepened to 28 September to read the week's merges.

*One line per record.*
- **Match:** ADR-015, 016, 019, 022, 023, 024, 026, 027, 028, 029, 030, 032, 033, 036, 037, 038, 039, 045, 047, 048,
  049, 050, 052, 054, 055, 056, 057, 059, 060, 061, 062, 063, 065, 066, 068, 069, 070, 071, 073, 074, 075, 077, 078, 079,
  080, 081, 082, 085, 087, 088, 089, 090, 091, 092, 093, 094, 095, 096, 097. (Governs lists lengthened for 019, 040, 041, 043,
  051, 052, 057, 070, 076, 083, 085, 086, 094; see "Governs".)
- **Did not match, and given a dated "Later, 12 October 2026" entry:**
  - ADR-018: 35 rules, not 21. ADR-020: fourteen `unsafe` blocks in `sv`'s non-test code (nine in `sv-run`, five in `sv-cli`),
    not five or nine. ADR-021: 57 refusal rows, not 39. ADR-025: points at ADR-020's count. ADR-031, ADR-044: a function moved
    files. ADR-034: the seal covers seven files, not six. ADR-035: 30 signs, not 28.
  - ADR-040, ADR-041, ADR-043: tests moved, names changed with the rename, dependency versions moved. ADR-058: the docs
    pages are at `~/stackvet-docs`.
  - **Credit changed to "in part" with no entry on the record that described it as full** (#1410, 10 October, and #1450,
    11 October): ADR-042 (C7.1.1), ADR-046 (V5.2.3), ADR-053 (V8.2.2, all four refused), ADR-067 (V7.2.2). The change is
    the owner's, of ADR-090 and ADR-053's "Later"; the four records now say so.
  - ADR-051: the required check is in force (Status line out of date); #1013 and #1330 had no entry (below).
  - ADR-072 (any 2xx, not only 200), ADR-076 (what a line holds is more than "counts only", see ADR-084), ADR-083, ADR-084,
    ADR-086 (accepted; the index said "proposed" for all three; ADR-084's `size` is the map `sizes`), ADR-098 (grouped low
    findings carry no next step), ADR-057 (folder names after the rename).
  - **ADR-064: the planted gap of backlog 0223.** The record said the tool holds its answer 25 seconds; the code has always
    held 40. The 25 came from #1261 (9 October), a change to plant a gap for this review. Found by the helper reading ADR-053
    to ADR-066, confirmed by the session against `model-provider.mjs` and `ai.rs`, and given its entry. The plant is out.
- **Names.** Every record written before 8 October still names `securevibe.toml`, `securevibe_*`, `~/.config/securevibe`
  and the like. One note in the index ("Names in the records") says so once, instead of an entry in each.

*Governs.* Added to lists where a record's decision lives in a file it did not name (ADR-019, 040, 041, 043, 051, 052, 057,
067, 070, 076, 083, 085, 086, 094). **Not added**, following the owner's decision of 6 October that the large shared files
stay ungoverned: `signed_in/mod.rs`, `signed_in/rules.rs`, `ai.rs`, `assemble.rs`, `main.rs`, `lib.rs` in `sv-cli`,
`sv-scan/src/lib.rs`, `sv-report/src/lib.rs`, `config.rs`. Several records' Later entries put decisions in exactly those
(ADR-015, 021, 022, 023, 029, 037, 053, 082, 088, 091). Left for the owner: that decision again, now that 13 records name them.

*Decisions with no record, and what was done.* Written: **ADR-100** (the opt-in `SV_LOG` file and the crash file, #1399,
10 October) and **ADR-101** (`sv connect` and the setup prompt, #1264, 9 October), with the owner's reasons as the design
entries give them; the owner's own reasons were "yes". A "Later" on ADR-051 for #1013 and #1330 (the second is CI only;
the owner's reasons beyond the workflow's comment are not written down). **Not written, for the owner:** #1291 (a
credential the app printed is kept out of the report's crash line; `docs/design/0343-*`): a default that changes what a
report contains, but it may be a bug fix; the helper could not tell. The sample of 25 of 141 merges that touched code and
added no record was read by title and design entry; the other ~115 were not read line by line.

*Lag, merge of the change to the record.* This week's new records and entries, from the Status lines and merge dates:
ADR-038 to 041: 2 days (the second review's), ADR-046: 3, ADR-066: up to 11 (it records decisions from 27 September to
9 October), ADR-096, 097: 1 to 2, ADR-098: 1, ADR-028 "Later": 5, ADR-053 "Later" group A: 1. Every other new record and
"Later" between 5 and 12 October: 0 days (most were proposed with the claim). Written by this review: ADR-100: 2 days,
ADR-101: 3, ADR-051 for #1013: 4 and for #1330: 3, ADR-042/046/053/067 (#1410, #1450): 1 to 2, ADR-058 (#1127): 4,
ADR-034 (11 October): 1, ADR-043 dependencies: 3, ADR-064 (planted 9 October): 3. The gaps are all in the 9 to 10 October
burst of observability and non-technical-user work (0217, 0226, 0237), where reasons went to design entries instead.

*"Unchanged, because" lines.* A search of merged pull requests from 5 October (by day, so it includes a few from the
morning of the 5th) found 425; 419 carry the exact line, 1,642 lines in all (the second review counted 73 lines in 50
pull requests, which this is not comparable with: the check was new then). Most by record: ADR-017 (104), ADR-018 (95),
ADR-046 (93), ADR-042 (91), ADR-049 (80), ADR-061 (76), ADR-055 (75), ADR-045 (74), because every new record's index row
touches the index, which each of these governs. The heaviest are 35, 21, 19 and 18 lines in one pull request. For each
new record the pull request repeats one sentence across 6 to 10 records ("this adds one index row for ADR-098"), plausible
when only the index row changes, and not checked against the diff. About 150 distinct reasons were read; the doubtful
ones were #1013 (ADR-051; a workflow change argued as "unchanged"; now a "Later"), #1013 (ADR-020, a build profile; fine),
#1347 (ADR-019 and ADR-052; `docker.rs` gained about 55 lines to read the stand-ins' records, recorded in ADR-082; fine),
and #1287, #1371, #1193, #1079, #1134, which were not read in the diff. **For the owner:** the index is governed by 8 or more
records, so every record's row drags eight to twelve boilerplate lines; consider taking the index out of those Governs lists.

*Requirement ids.* Every id cited by ADR-015 to ADR-098 exists in `data/frameworks` and fits what is said; none is cited
as met where it should not be.

*Checks.* `cargo fmt --all --check`: clean. `cargo clippy --all-targets -- -D warnings`: clean. `python3 tools/adr_check.py
--self-test`: ok. `cargo test --workspace` (see the end of this item for the final count).

*Also done at the owner's asking the same day:* a sweep of the documentation for "SecureVibe" that should read "StackVet".
Changed: `docs/DESIGN.md` (its opening and one sentence), `docs/GAP-ANALYSIS.md` and `docs/SEMGREP-FALSE-ALARMS.md` (named
for StackVet, with a note that the names inside are as they were before the rename). Left, because they explain the rename,
are v1, or are another's title: `README.md` (v1, the rename note, the old key folder), `CONTRIBUTING.md`, `SECURITY.md`,
`CLAUDE.md` (v1), `action.yml` (reads the old `securevibe.toml`), `tools/docs_page.py` (old marker), the Cloud Security
Alliance's "Secure Vibe Coding Guide", session names (securevibe-e9), and the dated history in `docs/adr`, `docs/design`,
`docs/backlog`, `docs/paper`. Not swept: the code's own strings (user-visible text in `crates/`); about 40 files name
`securevibe`, mostly the old names `sv` still reads until 1 January 2027, and which are intended is a separate pass.

**The owner's answers, 12 October 2026** (gathered here on the same day, since they were recorded only in the items
they became):
- *The large shared files left ungoverned:* answered by a new form of Governs pattern, a named part of a file
  (backlog 0268, ADR-103), done.
- *The index in thirteen records' Governs lists:* taken out (backlog 0267), done.
- *#1291, a credential the app printed kept out of the report's crash line, written without a record:* no answer is
  recorded in any item, record, or design entry, as read on 12 October 2026. Asked again the same day, and answered:
  a bug fix, needing no record ("yes, bug fix, record it and close 0266"). It enforces a rule `sv` already keeps
  (CLAUDE.md: never print, log, echo, or commit a key, in a report or anywhere), and decides nothing new.
