# The test-user step took 129 s on one trial app with 7 s of it itemized; the timings cannot say where the time went

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the VS Code app. Severity: medium (a slow run nobody
can explain). Measured by the orchestrating session.

**What was seen.** Rerun of the VS Code app: "the questions asked as the test users" 150,390 ms; its 336 request entries
sum to 7,506 ms. Without the manifest's `browser` line the step still took 129,418 ms, so it is not the headless
browser; the manifest names no reset, email-code, or activation flow, so it is not the mail wait
(`crates/sv-run/src/docker.rs:679`). The Cursor app's same step took 9,453 ms. The VS Code app hashes with
`scrypt:32768:8:1` and answers 429 to bursts (`app.py:303`), which may be where the time goes, but nothing in the report
says so.

**Fix.** A timing entry per check (lockout, password rules, change-password, forged form, ...) beside the per-request
ones, so a slow check is told from a slow app; then find this wait. Related: 0365 (the mail wait) is held "until a real
report's timings show the cost"; this report shows a cost it cannot attribute.

**Record.** None.

**Done, 15 October 2026.** The wait was not findable here (no container backend in the building session, and the
VS Code app is not in the repository), but the likeliest cause was found in the code: `Patient` waits up to a minute
whenever the app answers 429 with a wait, up to 300 s in a run, and said only the requests the limiter still refused
after the wait. The app answers bursts with 429 (`app.py:303`), so two one-minute waits would make the trial's 129 s.
Now:
- **Every wait is said and timed.** The waits for the app's limit, the ones the limiter then let through included, are
  one of the run's steps ("waited 120 s in all for the app's limit on requests, 2 times, …: id (60 s), …") and one
  timing.
- **Every named check is timed** (the `asked!` wrapper), in `report.json`'s `timings` as `inside the suites, the check
  `name``, counted in no total.
- **The slowest-parts line names where a suite's time went:** after the five slowest parts, "Inside them, the
  longest: …", the three longest of the checks and the waiting that took a second or more.
Not done: the checks wrapped in `quiet!` (no name of their own) are not timed one by one; their time is the suite's
less the named checks'. Record: ADR-102, Later. Tests: `sv-check` `signed_in/timings_tests.rs` (3), `sv-report`
`timing_inside_tests.rs` (2), `sv-cli` `assemble/check_timings_tests.rs` (1) and `tests/report_check_timings.rs` (1,
with a container backend; without one it checks nothing is said to be timed). Each piece broken on purpose turned
its test red. 0365 (the mail wait) can now read a real report's timings when one shows the cost.
