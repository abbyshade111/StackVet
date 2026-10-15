# `sv preflight` does not say that the test command's program is not among the packages `install = true` installs

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity app. Severity: low (usability).

**What is wrong.** The manifest says `test = "pytest -q"`; pytest is only in `requirements-dev.txt` (`-r requirements.txt`
plus `pytest==8.3.3`); `crates/sv-run/src/install.rs:56` installs `requirements.txt` only. The run printed
`sh: 1: pytest: not found` (exit 127) and the tests counted as failed, in the tool's own report and the rerun. Honest,
but the person learns it only after a full run, and the AI coding tool read "tests failed" as the app's fault.

**Fix.** Preflight reads the first word of `test` and says when no installed package provides it (pytest, jest, vitest,
`go test` and the rest), with the line to add or the `requirements-dev.txt` to name; and the run's own line says "the
test runner is not installed; see preflight" rather than only the shell's words.

**Record.** None.

**Done, 15 October 2026.** `sv preflight` and `stackvet_preflight` now answer about the test runner
(`crates/sv-cli/src/preflight/test_runner.rs`, topic `test-runner`): with `install = true`, the program the test
command starts (its first program, the module after `python -m`, the package after `npx`, or the first program of the
`package.json` script `npm test` or `npm run X` runs) is looked up among what the install step installs:
requirements.txt for pytest, nose2, tox, and behave; package.json's dependencies and devDependencies for jest, vitest,
mocha, ava, jasmine, cypress, and Playwright. Missing, it is *look at this*, with the line quoted from another
`requirements*.txt` that names it (the trial's `requirements-dev.txt`) and what to add; present, *looks right*. Without
`install = true`, or for a runner it does not know (`go test`, `python run_tests.py`, unittest), nothing is said. And
when the tests end with exit 127, the reports and `sv run` say the tests could not start, most often because the test
runner is not installed, and point to `sv preflight` (`sv_report::test_output_intro`). Tests:
`preflight/test_runner_tests.rs` and `sv-report/tests/test_runner_not_found.rs`, broken on purpose three ways (the
check not wired into `of`, every runner taken as installed, the exit-127 words switched off), each turning red.

Not done: no run was made with Docker here; the trial's app was rebuilt as files in the tests.
