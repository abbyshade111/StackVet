# Try running the test programs at the same time (cargo-nextest), measured first

**Status:** done, 12 October 2026

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

**Measured on 12 October 2026** (session backlog-org-0270): one run of `.github/workflows/nextest-trial.yml` on the
branch `claude/0270-nextest-trial`, never merged, at `main` as of that afternoon (before backlog 0271's faster crash
sweeps merged). Two jobs on the same commit, each on an ordinary GitHub runner with the container backend
(`SV_REQUIRE_BACKEND=1`):

| Step | `cargo test --workspace` | `cargo nextest run --workspace` |
|---|---|---|
| Install the tool | none | 219 s (`cargo install cargo-nextest --locked`) |
| Build the tests | 126 s | 127 s |
| Run the tests | 975 s | 626 s |
| Result | every test passed | 3,188 passed, 1 skipped, none failed |
| `SV_CREDIT_LOG`: lines, distinct credits | 242,440, 257 | 242,431, 257 |

- **Time.** The test step was 349 s (36 percent) shorter. Built from crates.io, the install took 219 s, so the job was
  about 130 s shorter in all. A download of nextest's own release would take seconds, but that is a new download from
  outside crates.io in the job that holds the owner's Docker Hub token: the trade-off the decision record would weigh.
- **Shared scratch folders.** No test failed with every test run at once. One run cannot rule out a collision that
  happens only sometimes; the list asked for above was not made.
- **`SV_CREDIT_LOG`.** `tools/coverage.py --credits` passed in both jobs, with the same 257 distinct credits, but the
  nextest log had 9 lines fewer. Whether lines were lost to processes writing at once, or a few tests credit a varying
  number of times, this one run cannot say; the logs were not kept. Adopting nextest would want both logs compared
  credit by credit first.
- **Not measured.** With 0271's faster crash sweeps (CI's `sv-check` unit tests 267 to 279 s down to 149 s), `cargo test`
  should be about two minutes shorter, which narrows the difference; nextest runs those two tests as processes of their
  own, so it gains less from 0271.

**Nothing adopted.** Whether to adopt cargo-nextest, and how it is installed, is the owner's decision, asked on
12 October 2026. The trial branch stays until the owner says it may be deleted.
