# Try running the test programs at the same time (cargo-nextest), measured first

**Status:** open

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
