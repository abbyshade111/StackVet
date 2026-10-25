# Nightly: Rust fails: the scope job was cancelled after its steps had finished

**Status:** done, 25 October 2026

Recorded by the nightly CI routine on 25 October 2026. The Rust workflow's push run on `main` for commit `d8e1d03`
(pull request #2302, "Claim 0665", a change to backlog items only) ended red on 24 October 2026:
<https://github.com/abbyshade111/StackVet/actions/runs/40114524464>.

- **What failed:** no test ran. The first job, "scope", ran all its steps by 17:05:27 UTC and printed
  `The whole suite runs: true`, and its log ends with the runner setting its output; yet GitHub marked the job
  cancelled at 17:12:33 UTC, seven minutes later. With "scope" cancelled, lints, the suite, and both image jobs were
  skipped, and the job "test" failed on its first step, saying `scope did not finish`.
- **What it looks like:** a failure of GitHub's runner, after the job's own work was done, rather than of anything in
  the repository. `main` does not cancel its own runs when a newer push comes (`cancel-in-progress` is off for
  `main` in `.github/workflows/rust.yml`), so this was not a newer run taking its place.
- **Was it there before?** Not on the runs read: every other Rust run on `main` in the 26 hours before this routine
  ran passed, including the next one, commit `cde19bf` (pull request #2303), four minutes later. The last pull request
  to merge before it was #2301 (`e5b1660`), whose run passed.
- **Is `main` red?** No: it went green again on the next push, so no merging session owes a fix within the hour.
- **What to do:** nothing in the code, unless it happens again. If it does, the run's "Re-run failed jobs" button is
  the check: a job that passes on a re-run with nothing changed was the runner's. This routine records; it fixes
  nothing.

**Checked 25 October 2026, and done with no build.** Not seen again: none of the 40 Rust runs on `main` before that
day's merge of #2352 failed or was cancelled, in this way or another. As the item says, there is nothing in the code
to change unless it comes back, and the run's "Re-run failed jobs" button is the check if it does. A new item is the
way to bring it back, should it.

