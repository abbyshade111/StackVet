# Process: shorter CI, a merge queue decision, and a nightly routine on `main`

**Status:** partly done: part 1's shards: sv-check split across parallel jobs, which the owner wants (12 October 2026); part 1's other half, docs-only pull requests skipping the suite, is built narrower in backlog 0272 (backlog items only, ADR-104), since most documents are read by tests; part 2, the merge queue, was declined by the owner the same day

From the same write-up; each
claimable on its own, and the second is the owner's to decide.
1. **Shorter CI.** The test job is one job of about ten minutes and `sv-check` is most of it: a matrix that runs
   `sv-check` in two or three shards beside the other crates brings it to about five; docs-only pull requests skip
   the test and image jobs behind a path filter, with a job of the required name still reporting. ADR-051 governs
   `rust.yml`: a Later entry or an "unchanged, because" line.
   **Part status:** claimed by securevibe-trial-prep, 12 October 2026
2. **A merge queue** (a repository setting, the owner's). GitHub merges a pull request that was green against an
   older `main`, so the first test of the combination is `main`'s own; a queue tests each pull request on top of
   the ones ahead of it, at one more CI run each. The write-up's advice: try auto-merge alone first (on since
   8 October), and turn the queue on only if `main` goes red from an untested combination more than once a week.
   **Part status:** done, 12 October 2026
3. **A nightly routine on `main`.** The full workspace tests with a container backend, `tools/coverage.py
   --credits`, and the example apps re-scored, with the counts compared to the night before and a one-line note
   when anything changed. The pieces exist; the comparison does not. The weekly decision-record review's routine
   ("A weekly review of the decision records") is the model.
   **Part 3 claimed 8 October 2026 by session securevibe-review**, at the owner's word ("can you help me set up the
   nightly routine you proposed"), in branch `claude/securevibe-review-nightly`, in the form of a `schedule` trigger
   on `rust.yml` (ADR-051, Later) rather than a Claude routine, since CI already runs every piece with Docker; parts
   1 and 2 stay open.
   **Part 3 done the same day** (ADR-051, Later, 8 October 2026; design entry "A nightly run on main, and the example
   apps' verdicts held to a snapshot"): a `schedule` trigger on `rust.yml`, 03:17 UTC daily, running the `test` and
   `image` jobs on `main`; the `publish` job keeps its push-only condition. The comparison the write-up wanted is the
   snapshot test of the ideas item's part 2.
   **Part status:** done, 12 October 2026

**The owner's decision, 9 October 2026**, asked by session securevibe-e2 with a recommendation for each open choice: **part 2, the merge queue: hold off for now, as recommended.** Auto-merge alone stays; the queue is reconsidered only if `main` goes red from an untested combination of pull requests more than once a week. (On 9 October one such combination, #1246 with #1248, turned `main` red once; #1255 fixed it.)

**The owner's word, 12 October 2026:** part 1, the slow test job split for a shorter CI, is to be built ("definitely want the slow test job split"). Part 2, the merge queue, is declined ("we can do without the merge queue"), and part 2 is done on that word; the owner asked instead for protection against two pull requests taking the same ADR number, filed as an item of its own.

**12 October 2026, later:** part 1's second half, a docs-only pull request skipping the suite with a job of the required name still reporting, is built in backlog 0272 (ADR-104) for backlog items only. Other documents stay on the whole suite: `docs/adr`, the design entries, `CLAUDE.md`, and `data/` are read by tests across it. What part 1 still needs is the shards.

**The owner's decision, 12 October 2026, after backlog 0270's measurement of cargo-nextest:** nextest is not adopted ("I don't want to move forward with nextest, so if splitting into jobs can speed up or provide value, go ahead with that"). Part 1's split is claimed the same day, with this plan (a new decision record, proposed here and numbered in the pull request that builds it):

- **Three jobs side by side**, from the last full run's times (1,221 s of tests: `sv-cli`'s 877, the rest 345): every crate but `sv-cli`, with `sv-cli`'s own unit tests; and `sv-cli`'s test files dealt alternately between two more. Each builds again, about two minutes, so the longest should end near nine minutes where the one job takes about eighteen.
- **The required `test` check keeps its name**, as the job that waits for the others and fails when any did, formatting and lints among them, so nothing can merge on a part that did not run. The short path of ADR-104 stays in it.
- **No test file left out:** a small tool deals the files, and a test fails when a file is in neither job or both.
- **The credit log still whole:** each job's `SV_CREDIT_LOG` is passed to the final job, which runs `tools/coverage.py --credits` on all of them together, through GitHub's own upload-artifact and download-artifact actions, pinned to commits like every other step. That is a new dependency in CI, and the record weighs it.
- **Costs:** about three times the container image pulls per pull request (through the Google mirror, ADR-051's Later entry), and three builds' runner minutes, which cost nothing on a public repository.
