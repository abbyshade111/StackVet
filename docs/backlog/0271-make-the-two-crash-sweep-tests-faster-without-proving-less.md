# Make the two crash-sweep tests faster without proving less, measured first

**Status:** open

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
