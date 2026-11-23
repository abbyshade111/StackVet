# The ninth weekly review of the decision records (week to 23 November 2026)

**Status:** done, 23 November 2026

The scheduled routine "Weekly decision-record review" (Monday 23 November 2026). Procedure: 0091 and `docs/adr/README.md`,
"When a record is written, and how it stays true". The eighth review is 0684.

**Claimed 23 November 2026 by session stackvet-weekly-adr-9** (the scheduled routine), in its own pull request, merged
before the review begins to write. Branch `claude/exciting-gates-vg732f`.

**Review of 23 November 2026.** Every record, ADR-015 to ADR-159 (145), and the index, read against `main` at
`029be97` and the week's merges since the eighth review's account (`1c2df03`, #2415): 143 first-parent merges, #2416
to #2557, 80 of which changed more than the backlog. It was the busiest week since the reviews began: ConfigVet
(`cv`, ADR-157) and AgentVet (`av`, ADR-159) were each proposed, accepted, and built through several parts; sessions
began opening pull requests as a GitHub App (ADR-158); three releases were made (v0.2.0, v0.3.0, v0.3.1); and a
security review's findings were fixed. Three records are new and 26 older ones gained entries during the week.

*How it was read.* Six read-only helper agents, as the second review used: one for the pull requests' descriptions,
one each for ADR-157 and ADR-159 with the records their pull requests touched, one for the process, release, and site
records, and two for the other records whose governed files changed. Each read its records whole against the code
and the week's diffs. This session then checked in the code every finding written below before writing it. The 68
records that the week changed, or whose governed files it changed, were read this way. The other 77 were not re-read
line by line: nothing they govern changed, and they were checked by the facts (the reference test, the id check).

*One line per record.*
- **Did not match, and given a dated entry today** ("Later, 23 November 2026 (the ninth weekly review)"):
  - ADR-019: the helper images' pins moved on 20 November (Dependabot, #2400, #2418, #2419, #2420): headless-shell
    159.0.8144.0, mailpit v1.33, and new digests for busybox and Node. The decision holds.
  - ADR-020: the count of `unsafe` blocks by crate (four moved from `sv-cli` to `av-record` with #2506; still sixteen).
  - ADR-021: the new V10.5.3 credit (#2457) can rest on a crashed answer, which this record says never happens.
    Read from the code, not run. Backlog 0744.
  - ADR-043: `av checkpoint` (#2506) adds to the list of trusted keys, which the record said only `sv review` does,
    and prints nothing when it does; the passphrase-hiding code moved crates; and `av checkpoint` does not wipe the
    typed passphrase as `sv review` does. Backlog 0743 and 0747.
  - ADR-051: "moves the `latest` tag only when its commit is still the newest on `main`" was overtaken by #2526
    (ADR-080 has the rule); and the ruleset's code-scanning rule, in `.github/rulesets.json` since #2446, was never
    described.
  - ADR-101: `--notes` is no longer refused on Windows (#2544).
  - ADR-108: v0.2.0, and the packages' move to it, were not named.
  - ADR-122: `av checkpoint` reads the hook record too, to find where its chain ends.
  - ADR-132: the site went live and the old repository-wide Cloudflare keys were deleted (21 November), which the
    record was waiting to say; and the self-assessment was remade by `sv` 0.3.0 (#2534) with no entry.
  - ADR-157: the Status line; "`cv check` ... reads files only" (it starts three outside tools since #2456); `cv-cli`'s
    dependencies; five small choices in the code with no words; and two parts that disagree about V13.3.1.
    Backlog 0745 and 0746.
  - ADR-159: the trusted-keys write; "Not on Windows yet"; what the hook record keeps for an asking (narrowed twice
    by #2525 and #2543); "the four" dependencies (five); an exit code; and `av`'s build script running `git`.
- **Matched, and given an entry or a Governs line for another reason:** ADR-127 (the code that makes the second
  start joins Governs), ADR-149 (still proposed; the owner kept it on hold on 20 November, with reasons, in backlog
  0625), ADR-032, ADR-153, ADR-158 (Governs only).
- **Matched, no entry needed:** every other record. Of those read whole or against the week's diffs: ADR-016, 017,
  018, 022, 023, 026, 034, 040, 050, 052, 053, 054, 056, 059, 061, 062, 066, 076, 077, 080, 081, 082, 084, 085, 089,
  090, 091, 092, 093, 094, 096, 098, 099, 102, 104, 106, 111, 113, 115, 116, 128, 129, 131, 136, 138, 139, 141, 146,
  148, 151, 154, 155. The week's own entries on ADR-026, ADR-032, ADR-052 (the three guards of #2470), ADR-080,
  ADR-127 (V10.5.3), and ADR-132 (Cache-Control, as it ended: pages only) are true of the code.
- **Governs lists, added to today:** ADR-020 and ADR-043 (`crates/av-record/src/plain.rs`; ADR-043 also
  `crates/av-cli/src/checkpoint.rs`), ADR-032 (`git_changes.rs`), ADR-051 (`.github/CODEOWNERS`, `codeql.yml`),
  ADR-122 (the asking tests), ADR-127 (`renamed_args`, `start_again`), ADR-132 (`site-weekly.yml`, its test, the
  AgentVet page), ADR-153 (`crates/sv-frameworks/src/paths.rs` and its test), ADR-157 (`data/cv-adapters.json`,
  which holds the outside tools' offline flags and was governed by nothing), ADR-158 (`decision-records.yml`), and
  ADR-159 (the asking tests, the Windows notes test, `build.rs`). The long-standing gaps the fifth review listed are
  unchanged.
- **The fence page** (`docs/ARCHITECTURE.md`, "The fence") had two sentences behind the code, corrected in this pull
  request: `cv check` starts outside tools, and `av checkpoint` adds to the trusted-keys list.

*Decisions with no record.* The building pull requests carried their records; these are what slipped past.
- **`av checkpoint` names the owner's key as trusted for the app, silently** (#2506). A write into the person's
  settings folder that changes whose answers a later report counts. No record, and no reason beyond a code comment.
  Written as a fact on ADR-159 and ADR-043 today; what to do about it is the owner's (backlog 0743).
- **ConfigVet credits V13.3.1 in part for a GitHub setting** (#2509), against its own part 3. A default that changes
  what a report concludes. Not changed here (backlog 0745).
- **The owner's picks for backlog 0625's held parts** (#2434, "Go with your picks"): parts 9, 10, and 11 to be built
  as listed, part 4 kept on hold. Their reasons are written, in the backlog item; one is on ADR-157 and, from today,
  one on ADR-149. Nothing is built from the rest yet, and the item promises a record with each build.
- **v0.2.0 and the packages' move to it** (#2436, #2440): the owner's asking; reasons in backlog 0688; on ADR-108
  from today.
- **The self-assessment remade, and its 13 set-asides signed again** (#2534): the owner's word; reasons in backlog
  0723; on ADR-132 from today.
- **The owner had two witness branches deleted** (#2450): in backlog 0691's note only. Small, and left there.
- **The ruleset's code-scanning rule, the ruleset that protects v1's tags, and "dismiss stale reviews":** set in
  GitHub at some earlier time, and first written anywhere when #2446 copied the settings into a file. Their reasons
  are not written down anywhere, so none is written here.
- No new download from outside the workspace: the lock file gained only the workspace's own seven new crates. No
  workflow gained a permission, a secret, or a download that a record does not name.
- Still open from the fifth review, the owner's, with reasons not written anywhere: turning off Cloudflare's Bot
  Fight Mode, `[[left-out]]` tried and taken back, and the "as recommended" answers on 0550, 0602, 0589, 0590, 0515,
  0603, and 0604.

*Lag, merge of the change to the record.*
- **The week's own records: 0 days.** ADR-157, ADR-158, and ADR-159 were each proposed before any code and accepted
  in the pull request that built their first part, the same day. Every one of the week's entries on the 26 older
  records was in the pull request that made the change, or (for #2552, the fixes that missed #2551) 5 hours after.
- **This review's entries: 1 to 3 days.** ADR-019, 3 days (Dependabot, 20 November). ADR-108 (v0.2.0) and ADR-149,
  3 days. ADR-020, ADR-043, ADR-122, and ADR-159's trusted-keys line (#2506, 21 November), ADR-021 (#2457), ADR-157's
  fence sentence (#2456), and ADR-132's deleted keys, 2 days. ADR-051 (#2526), ADR-101 and ADR-159's Windows line
  (#2544), ADR-159's asking lines (#2543), and ADR-132's self-assessment (#2534), 1 day.

*"Unchanged, because" lines.* 48 of the week's 143 pull requests used at least one, 160 lines in all; none used the
`0NN` placeholder the eighth review warned of. No session's pull request lacked a line it owed; the 7 that owed one
without it were Dependabot's pin bumps, which ADR-128 lets through. About 55 lines were read against the record and
the diff. Most are true and say something. Those that passed a change over:
- #2526 on ADR-051 ("only publish's handling of latest changes, which ADR-080 records"): true, and the diff deleted
  the very test ADR-051's sentence describes. The one firm miss.
- #2544 on ADR-159 and ADR-101, and #2543 on ADR-159: each reason is true of the decision and leaves a sentence of
  the record false.
- #2506 on ADR-043 ("makes no key"): true, and silent on the trusted-keys list and the passphrase.
- #2534 on ADR-132: a remake that the last time had an entry of its own.
About 70 of the 160 lines are there only because one shared file is in several records' lists (`Cargo.toml` for a
version number, the architecture page for a new row, the index for a new row, `rust.yml` for a comment), with the
same words pasted across pull requests. They are true and say nothing, and the firm miss sat among four of them.

*Requirement ids.* Every id cited by ADR-015 to ADR-159 exists in `data/frameworks`: 262 distinct ids in 401
citations, against the 825 ids the four files hold, checked by a script this session (it counts the checklist's ids
too, so the figures are not the eighth review's). Fourteen ids were newly cited in the week; V10.5.3 was read against
its text and fits. The four that fit poorly are ConfigVet's and are backlog 0745.

*Could not be checked from here.* The three Environments' reviewers and rules, the repository's secrets, the App's
permissions, the tap's and the bucket's rulesets, and what stackvet.dev serves: the records' statements on these
rest on the owner's dated readings. Every "with the guard removed, the test goes red" claim in the week's entries
was read, not replayed.

*Checks.* `cargo fmt --all --check`: clean. `cargo clippy --all-targets -- -D warnings` (Rust 1.100.0): clean, the
lints of 0683 having been fixed on 16 November. `python3 tools/adr_check.py --self-test`: ok. `cargo test --workspace
--no-fail-fast` at `029be97` (Linux cloud container, no Docker, no outside network): 388 `test result` lines, 4,870
passed, 2 failed, 1 ignored. The two are the `sv probe` tests that need an outside address
(`an_address_that_cannot_be_reached_exits_2_and_one_that_answers_exits_0`,
`an_address_that_answers_is_told_what_to_fix_what_held_and_what_could_not_be_settled`), the same container-only
failures as the fifth to eighth reviews. Tests that start an app took the honest-absence path, there being no Docker.
After this review's edits, the 23 test files that read the records, the backlog, the architecture page, the code
owners, or the rulesets were run again (`decision_records` among them): 134 passed, none failed.
`python3 tools/adr_check.py` on this pull request: nothing owed, with its one line for ADR-154.

**For the owner to decide:**
1. **Backlog 0743:** should `av checkpoint` say when it names your key as trusted for an app, as `sv review` does, or
   leave that to `sv review`? Today it does it without a word.
2. **Backlog 0745:** whether ConfigVet should credit V13.3.1 (a secrets vault) for GitHub's secret scanning being on,
   and three looser citations.
3. **Backlog 0744:** whether a crashed answer may count as a refusal for the V10.5.3 credit.
4. **The ruleset's code-scanning rule, the v1 tags' ruleset, and "dismiss stale reviews"** have no reasons written
   anywhere. If you remember why each was set, a sentence each on ADR-051 would close it.
5. **The "unchanged, because" lines.** About 70 of the week's 160 were boilerplate forced by one shared file sitting
   in several records' lists, and the week's one firm miss hid among them. Narrowing those lists to the part of the
   file each record decides (`path::NAME`, ADR-103) would make a line mean something again. Not done here; it changes
   what the required check asks, which is yours to choose.
6. **Still open from the fifth review:** the Governs gaps it listed, and the reasons that are not written down (Bot
   Fight Mode, `[[left-out]]`, and the "as recommended" answers).
