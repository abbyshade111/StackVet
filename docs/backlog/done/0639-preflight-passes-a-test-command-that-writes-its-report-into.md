# Preflight passes a test command that writes its report into the read-only app folder, and the report calls the crash failing tests

**Status:** done, 23 October 2026

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

**Claimed, 22 October 2026,** at the owner's word ("Please continue with 0638 and the other items filed today and outside
the roadmap"), with 0638, each to be built in a pull request of its own.

**Done, 23 October 2026.** Two parts, as the item asked:

- **The preflight says it first** (`test-report-path`, in `crates/sv-cli/src/preflight/test_runner.rs`): when the test
  command names where it writes a report (`--junitxml`, `--junitfile`, `--outputFile`, `--test-reporter-destination`,
  `--json-report-file`, mocha's `--reporter-options output=`, or a `>` redirect) and the place is not under
  `/sv-reports`, it is "worth a look", with the reason (the app's folder is read-only while it runs) and the form to
  use instead; under `/sv-reports` it "looks right"; a command that names no report says nothing.
- **The report says what happened:** a test run whose output says `Read-only file system` (or `EROFS`) and that did
  not exit 0 is `could-not-write`, not `failed`, and the report says the tests stopped on writing into the app's
  read-only folder, so whether they pass is not known from this run, and how to write the report under `/sv-reports`.
  A test run that fails for itself is still said as failing.

`sv init`'s comment, `test-report = ""  # … e.g. "junit.xml"`, was read again and is right: a relative `test-report`
is read from `/sv-reports`. Its fault was the test command writing beside the code, which the preflight now catches.
Tests: `crates/sv-cli/src/preflight/test_report_path_tests.rs` and `crates/sv-report/src/read_only_tests.rs`.
