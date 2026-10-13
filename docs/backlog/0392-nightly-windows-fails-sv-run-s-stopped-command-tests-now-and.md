# Nightly: Windows fails: sv-run's stopped-command tests, now and then

**Status:** open

Recorded by the nightly CI routine on 13 October 2026. Two tests in `crates/sv-run/src/lib.rs` that stop a command
which runs too long failed on Windows now and then, on pushes to `main` where everything else passed or only an
unrelated test failed. Neither failed the night before. The two are recorded together because they share the code
under test (`run_bounded`) and failed together twice.

- `tests::a_stopped_command_leaves_nothing_it_started_running` failed three times:
  [run 38300378853](https://github.com/abbyshade111/StackVet/actions/runs/38300378853) on `0dca24d` (#1562), [run 38317430694](https://github.com/abbyshade111/StackVet/actions/runs/38317430694) on `9ddcfcd` (#1582),
  where it said `assertion failed: !pid.is_empty()` (line 1558), and [run 38323220814](https://github.com/abbyshade111/StackVet/actions/runs/38323220814) on
  `90fb86f` (#1590), where reading the pid file gave `Os { code: 2, kind: NotFound }` (line 1553).
- `tests::a_command_that_takes_too_long_is_stopped_and_says_so` failed twice, in the first and third of those runs
  (#1562 and #1590), at line 1473 (as the file stood then) with `Bounded { code: -1, text: "", stopped: true, cut: false }`: it was stopped, as
  it should be, but had printed nothing, so its `echo begun` had not run yet.
- Each passed on the next push, with no change to these tests between.

A likely reading, not checked: both give the shell 400 or 500 milliseconds, and on a busy Windows runner the shell
(Git for Windows' `sh`) had not yet started its first command when it was stopped, so there was no "begun" and no
pid file. The fix would make each test wait for its command to have begun (or give it longer) before the clock that
stops it, without weakening what is asserted after.
