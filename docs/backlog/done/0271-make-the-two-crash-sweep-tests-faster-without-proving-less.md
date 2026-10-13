# Make the two crash-sweep tests faster without proving less, measured first

**Status:** done, 12 October 2026

Found on 12 October 2026 while timing the suite for `docs/paper/LOCAL-TEST-FAILURES.md`, and filed at the owner's
request. `a_crash_never_turns_a_finding_into_a_pass` and `a_crash_on_a_correct_app_raises_no_finding`
(`crates/sv-check/src/signed_in/mod.rs`) each ran for over 60 seconds on the owner's Mac. They are among the slowest of
`sv-check`'s unit tests, a program that took 269 s in CI. Each runs every scenario once, then once more for every request
with that request crashed. Each crashed run sends every request again, so the time grows with the square of the number
of requests.

**Measure first.** Time each test on its own (`cargo test -p sv-check --lib crash_tests -- --test-threads=1`), and see
where the time goes, before changing either.

**Ideas to measure:**

- Reuse the uncrashed run instead of making it again.
- Crash only one of two requests that are exactly alike (same method, path, and body), and only where crashing either
  is the same experiment.
- Stop each sweep starting one thread per core while `cargo test` already runs tests on every core.

**The condition.** These tests guard "evidence is honest": a crash never turns a finding into a pass, for every rule in
`RESTS_ON_A_REFUSAL`. A faster version proves the same thing, or it is not a fix. Break it on purpose before and after:
a sweep that misses a credit turned by a crash must still fail.

**Measured again, 13 October 2026 (backlog 0364).** On a 4-core Linux container, not the owner's Mac. Each was run on
its own with `cargo test -p sv-check --lib <name> -- --exact --test-threads=1`, twice, after `main` had taken 0271, so
each sweep still used its own thread per core:

- `a_crash_never_turns_a_finding_into_a_pass`: 164.3 s, then 165.1 s.
- `a_crash_on_a_correct_app_raises_no_finding`: 26.4 s, then 28.3 s.
- `cargo test -p sv-check --lib crash -- --test-threads=1`, all 14 tests with "crash" in their names: 195.7 s, of
  which the two sweeps are about 191 s.

So the first sweep is about six times the second and is nearly all of the crash tests' time. Nothing is proposed here;
these numbers are the baseline a later change is measured against.
