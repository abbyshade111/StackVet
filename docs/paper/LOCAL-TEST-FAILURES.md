# Known failures: the tests that failed only on the owner's Mac

This file is after the cut-off (`SINCE-THE-CUTOFF.md`). It records the state of `main` on 12 October 2026.

Before 12 October, a full `cargo test --workspace` of `sv` on the owner's Mac never came out clean, while CI's runs
did. Sessions learned to expect certain failures and to explain them away as "the environment". On 12 October each
expected failure was traced to its cause and fixed. The suite now passes in full on the Mac.

**Before:** `main` at `ad0a4a69`. 206 test programs; 3,154 tests passed and 9 failed. This run went outside the
Bash sandbox, after py7zr had been installed, so the two failures fixed earlier the same day do not appear in it.

**After:** `main` at `41e8f044` plus the test fix (backlog 0269). 209 test programs; 3,177 tests passed and 0 failed.

## The failures, their causes, and their fixes

| Test (file) | Added | Failing on the Mac since | Cause | Fix |
|---|---|---|---|---|
| `each_stored_7z_unpacks_to_exactly_what_it_says_in_py7zr` and `a_7z_whose_data_is_altered_is_refused_by_py7zr` (`sv-check`, `signed_in/archives.rs`) | 11 Oct (`d2a6ee25`, backlog 0242) | 11 Oct, from the day they were added | They use py7zr, a Python library, to check `sv`'s 7z archives from outside. CI installs py7zr; the Mac did not have it. | The owner installed py7zr 1.1.3 (CI's pinned version) on 12 Oct. Both pass. |
| `what_a_person_records_counts_and_the_report_says_where_its_seal_was_checked` (`sv-cli`, `tests/review_terminal.rs`) | 4 Oct (`990531b9`) | 4 Oct, whenever run inside the Bash sandbox | The test types into `sv review` through a pseudo-terminal, and the sandbox refuses to make one. | No code change. Run from the Terminal panel, outside the sandbox, it passes (8 s). |
| 8 tests that start an app in a container: `email_change_example.rs` (2 tests), `host_cookie.rs`, `killed_run.rs`, `loopback_start.rs` (2), `report_lock.rs`, `seen_record.rs` | 28 Sep to 11 Oct | From the day each was added, in every run from a build folder outside the home folder | Each test makes its app inside the build folder. On a Mac, Colima (the container backend) shares only the home folder, so the container saw an empty folder and the app never started. `sv` said so in its own report: "inside the container it was empty". Builds were in `/tmp` until 9 Oct, then on the Samsung USB drive; neither was shared. | Colima's settings (`~/.colima/default/colima.yaml`) now share the home folder, `/private/tmp/claude-502`, and the build folder on the USB drive. A start-up step also makes the `/tmp` folder appear at its `/tmp` path inside Colima's VM. All 8 pass. |
| `a_whole_session_over_stdio` (`sv-cli`, `tests/mcp_stdio.rs`) | 25 Sep; the line that failed, 27 Sep (`fc8227b9`) | 9 Oct, when builds moved to the USB drive, whose name holds a space | A bug in the test. `sv` correctly puts quotes around its own path when the path has a space (`'/Volumes/Samsung USB/.../sv' report …`), so the command still works when pasted into a terminal. The test expected the path without quotes. CI's paths have no spaces, so CI never saw the difference. | Backlog 0269: the test now expects the path quoted the way a shell reads it. A new test copies `sv` into a folder named `it's a folder`, so the quoting is checked in CI too. With the quoting switched off on purpose, five tests fail (the two above and three that already tested the quoting function). |

## Time

The table adds up the time each test program reported. The programs run one after another, so the totals are
comparable. They leave out compile time.

| | Before | After |
|---|---|---|
| Test programs | 206 | 209 |
| Tests passed / failed | 3,154 / 9 | 3,177 / 0 |
| Total test time | 913 s (15.2 min) | 1,043 s (17.4 min) |
| Wall clock for the test phase | not measured on its own | 1,293 s (21.6 min) |

The suite is slower now, because the fixed tests really run an app instead of stopping at once:

| Test file | Before (failed) | After (passed) |
|---|---|---|
| `email_change_example.rs` | 10 s | 78 s |
| `host_cookie.rs` | 8 s | 62 s |
| `seen_record.rs` | 8 s | 75 s |
| `report_lock.rs` | 65 s | 26 s |
| `killed_run.rs` | 149 s | 20 s |
| `loopback_start.rs` | 65 s | 66 s |
| `mcp_stdio.rs` | 0.8 s | 0.9 s |

Two of these files got faster: `killed_run.rs` and `report_lock.rs`. When their app could not start, they sat out a
waiting period before giving up, so failing cost more time than passing.

**Caveat on the totals:** part of the difference is the Mac's own load, not the fixes. The two runs were on different
commits, with 23 more tests in the second. Other work was also running: the load average was 12 at the start of the
second run. The `sv-check` unit tests went from 138 s to 209 s with the same 1,506 tests, which points to load rather than to any change.
The per-file figures for the fixed tests are the reliable ones; read the totals as about ±15%.

## What it shows

- **Known failures were treated as normal.** All nine failures had a cause that could be fixed. For the eight
  container tests, `sv`'s own report named the cause in plain words every time, but because the failure was "known",
  nobody read that message.
- **One "environment" failure was really a test bug.** The stdio test was right in CI and wrong anywhere a path held
  a space. A user who installs `sv` under a folder like that would meet it.
- **A failure that stops early can be slower.** It doesn't follow that a failing test costs less time than a passing one.

## Related

- **A separate failure the same day was real, and CI caught it.** On PR #1512 (backlog 0247), the new `--quiet` option
  changed the list of options `sv report` names, and a test from 10 October expected the old list. The building
  session's own run before opening the PR should have caught it, and did not. The fix went into the same PR.
- **How the record was made.** Backlog 0269 holds the stdio fix. The timings come from `cargo test`'s own
  "finished in" line for each test program, read from the two runs' logs. The two failures fixed earlier the same
  day come from the same day's notes.
