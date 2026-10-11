# Nightly: Windows fails: probe_exit (compile error, then a failed test), 10 to 11 October 2026

**Status:** done, 11 October 2026
**Status:** partly done: The nightly failure is fixed: the Windows job has passed on every push since 11 October 2026 (#1414, #1416). Still open: the whole sv probe command is tested on Unix only. A Windows check that runs the real command end to end needs sv to accept a stand-in for curl on Windows, which changes what sv runs, so it waits for the owner's decision (ADR-029).

Recorded by the nightly CI routine on 11 October 2026. The Windows workflow failed on 23 of the 57 pushes to `main`
between 19:03 UTC on 10 October and 02:27 UTC on 11 October, all in one test file, `crates/sv-cli/tests/probe_exit.rs`,
which came in with pull request #1387 (backlog 0235, `sv probe` exits 2 when it could not reach the address). Rust and
CodeQL passed on every one of those pushes. It was not there the night before: the nightly Windows run of 10 October
(on `8d9a15e`, before #1387) passed.

It failed in two ways, one after the other:

- **First, the test would not compile on Windows** (from #1387, `dbc0525`, to #1407): `probe_exit.rs` used
   `std::os::unix::fs::PermissionsExt` and `Permissions::from_mode`, which exist only on Unix. First seen in
   [run 38078244655](https://github.com/abbyshade111/StackVet/actions/runs/38078244655) on `dbc0525`.
   #1407 (`c6f3843`) kept that code to Unix, and the Windows runs on #1407 and #1408 passed.
- **Then, the test ran and failed on Windows** (from #1409, `d9329f6`, to #1413, `21d09f5`):
   `an_address_that_cannot_be_reached_exits_2_and_one_that_answers_exits_0` panicked at `probe_exit.rs:79` with "the
   setup did not reach the unreached branch": `sv probe` reached example.com rather than the test's stand-in `curl`,
   because on Windows the command runs the system's own `curl` whatever the PATH says. The Windows totals were 2911
   passed and 1 failed. Last seen in
   [run 38105190479](https://github.com/abbyshade111/StackVet/actions/runs/38105190479) on `21d09f5`. #1412 tried
   setting the test process's PATH as well, and it still failed.

`main`'s Windows job has been green since #1414 (`a0ebfe6`, 02:36 UTC on 11 October), which keeps `probe_exit.rs` to
Unix (`#![cfg(unix)]`), and #1416 (`76f09be`) moved the decision `sv probe` makes into `probe_verdict_tests.rs`, which
runs on every platform. The last Windows push run read, on `ae498be` (#1422), passed.

**What stays open:** the whole `sv probe` command, from the address to the exit code, is tested on Unix only. A
Windows stand-in for `curl` that `sv probe` really finds would close that, or this item can be marked done with a note
that the platform-wide verdict test is enough.

**Built 11 October 2026:** the Windows stand-in landed in PR #1433 (merged); the probe test runs the whole command on every platform, and the Windows job passed on the branch before merge.
