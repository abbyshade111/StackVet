# Nightly: Rust fails: suite (cli-2) ran past its 30-minute limit and was cancelled

**Status:** open

Recorded by the nightly CI routine on 25 October 2026. The Rust workflow's push run on `main` for commit `489f3f3`
(pull request #2308, "Claim 0666", a change to backlog items only) ended red on 24 October 2026:
<https://github.com/abbyshade111/StackVet/actions/runs/40117379390>.

- **What failed:** the job "suite (cli-2)" started its "Tests" step at 17:43:53 UTC and was still in it when GitHub
  cancelled the job at 18:18:13 UTC, past the suite's `timeout-minutes: 30` in `.github/workflows/rust.yml`. The other
  three parts of the suite (rest, check, cli-1) finished in 3 to 5 minutes on the same commit, and lints and both
  image jobs passed. The job "test" then failed on its first step, saying `the suite: cancelled`.
- **What the log says:** nothing more could be read. GitHub returns no log for the cancelled job (HTTP 404), so which
  test was still running when the limit came is not known. The part `cli-2` runs half of `sv-cli`'s test files, dealt
  by `tools/ci_shards.py args 2 2`.
- **Was it there before?** Not on the runs read: every other Rust run on `main` in the 26 hours before this routine
  ran passed, including the next one, commit `9ba9009` (pull request #2309), three minutes later, and last night's
  scheduled run on `2d63c16`. No backlog item named a hang in `cli-2` before this one. Pull request #2308 changed only
  backlog items, so it did not bring a hang in the code; the last pull request to merge before it was #2307 (`30d9218`),
  whose run passed.
- **Is `main` red?** No: it went green again on the next push, so no merging session owes a fix within the hour.
- **What to do:** if it happens again, the step's log will say which test did not finish, as long as the job ends
  before the limit cuts it off. One way to make sure a hang names itself is a per-test time limit inside the step
  (for instance, `timeout` around `cargo test` set a few minutes under the job's limit, so the log is uploaded), which
  is a change to the workflow and wants its own decision. A test that waits on Docker or on a child process with no
  limit of its own is the first place to look. This routine records; it fixes nothing.
