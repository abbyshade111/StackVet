# The test-user step took 129 s on one trial app with 7 s of it itemized; the timings cannot say where the time went

**Status:** open

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
