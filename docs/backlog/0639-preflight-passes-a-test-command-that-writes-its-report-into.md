# Preflight passes a test command that writes its report into the read-only app folder, and the report calls the crash failing tests

**Status:** open

Found in the said-versus-did reading of the owner's Cursor build of 22 October 2026 (backlog 0373; the app and its records are on the owner's drive), checked against the record before filing.

The app's `stackvet.toml` has `test = "pytest -q --tb=short --junitxml=junit.xml"`. Inside `sv run`'s container the
app's folder is mounted read-only, so pytest stopped with `OSError: [Errno 30] Read-only file system:
'/app/junit.xml'` (the report's `report.json`, `test_output`). The report said "Its own tests failed; the report
shows the last lines they printed", and not-examined.md said only that no `/sv-reports/junit.xml` was written. The
tests themselves pass: 27 of 27 on a copy of the final code. StackVet's guide says to write the report under
`/sv-reports`, but `sv init`'s template comment gives `e.g. "junit.xml"`, and preflight (`stackvet_preflight`,
`test_runner.rs`) does not check where the command writes its report.

To do: preflight says so when the test command names a report path outside `/sv-reports` (as `--junitxml=`, `-o`,
`--reporter-options`, and the like); `sv init`'s comment gives the `/sv-reports` path; and when the test run ends on
a read-only file system error, the report says the tests could not write their report, not that they failed. Tests
for each.
