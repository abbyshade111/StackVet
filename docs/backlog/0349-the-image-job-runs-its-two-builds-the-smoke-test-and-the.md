# The image job runs its two builds, the smoke test, and the Action twice on every pull request, backlog-only ones included

**Status:** claimed by builder-full, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: test and CI time. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** Median 240 s per run (208 to 259): the Docker build 108 s, then the native release build 106 s, the smoke test, and the Action run twice. Its result is not a required check (`main` requires `test`, Decision records, and code scanning), so on a pull request it is informational; on a backlog-only claim it runs in full while the suite takes its two-minute short path.

**Where.** `.github/workflows/rust.yml:294` (`image`): no `needs: scope` and no `if:`.

**Evidence.** The job header with no condition; the measured step times.

**How it bites.** No critical-path time (it runs beside the suite), but a runner job and four minutes of Docker Hub pulls per pull request, including every claim. Moving it to pushes to `main` and the nightly would show a Dockerfile or `image_smoke.py` break on `main` within four minutes of the merge, under the red-`main` rule; the pull request that breaks the image would not see it before merging.

**Fix.** `needs: scope` and `if: github.event_name != 'pull_request' || needs.scope.outputs.full != 'false'` (at least skip the short path), or `if: github.event_name != 'pull_request'`; `publish` keeps `needs: [test, image]` and runs on push only, so it is unaffected.

**Record.** ADR-051 governs: a Later entry (when the image is built).
