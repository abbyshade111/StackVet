# Try running the test programs at the same time (cargo-nextest), measured first

**Status:** claimed by backlog-org-0270, 12 October 2026

Found on 12 October 2026 while timing the suite for `docs/paper/LOCAL-TEST-FAILURES.md`, and filed at the owner's
request. `cargo test` runs the test programs one after another. In CI's run on #1512 the test step took about 15½
minutes: `sv-check`'s unit tests about 4½ of them, and nine container test programs each waiting 20 to 80 seconds for an
app to start (`email_change_example` 62 s, `loopback_start` 62 s, `host_cookie` 54 s in CI). cargo-nextest runs every
test as its own process, all at once, so those waits would overlap.

**Measure first.** One experimental CI run on a branch, never on `main`, with cargo-nextest, compared with `cargo test`'s
test step on the same commit. Nothing is adopted from this item alone; the owner decides after the measurement.

**Before adopting it:**

- It is a new tool in CI, so it needs a decision record. `.github/workflows/rust.yml` deliberately avoids third-party
  steps, even a build cache, to keep the supply chain small; the record weighs the minutes against that.
- Tests that share a fixed scratch folder would get in each other's way when run at once. List them and give each its
  own folder first.
- Every credit the tests write to `SV_CREDIT_LOG` must still land when many processes write at once, or
  `tools/coverage.py --credits` would read a short file and pass.
- How it fits with backlog 0192 part 1 (the slow test-job split): it may make part of that split unnecessary, or the
  two may combine.

**Scope of this claim (12 October 2026):** the measurement only. A workflow on an experimental branch that is never
merged, running on that branch alone, with two jobs on the same commit: `cargo test --workspace` as `rust.yml` runs it,
and `cargo nextest run --workspace`, each with the container backend, and each step timed. cargo-nextest is built from
crates.io with `cargo install --locked` (no third-party action), and its install time is reported apart from the test
step. The tests that share a fixed scratch folder, and how `SV_CREDIT_LOG` behaves with many processes writing, are
listed in the write-up, not fixed. The numbers go into this item for the owner to decide on.
