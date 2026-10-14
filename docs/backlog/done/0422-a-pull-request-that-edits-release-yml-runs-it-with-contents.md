# A pull request that edits release.yml runs it with contents, id-token, and attestations write before any merge (the owner's setting)

**Status:** done, 13 October 2026

From the deep review of 13 October 2026 (backlog 0417), area: CI. Severity: medium. Verified against the file by the
reviewing session before filing; whether this repository's settings grant a same-repository pull request run write
permissions was not checked (GitHub's documented default does).

**What is wrong.** `release.yml` runs on a push of a `v*` tag and on a pull request that changes the workflow file
itself (so the release path is tried before it is merged, ADR-108). The job asks for `contents: write`,
`id-token: write`, and `attestations: write`, which GitHub grants to a same-repository pull request run, and the
`if: github.event_name == 'push'` gates on the signing and the release live in the same file the branch controls.
A branch with push rights that rewrites the two `if:` lines and drops `--verify-tag` gets a run that builds the
branch's `sv.exe`, signs it keylessly under this repository's name, and creates a GitHub Release without any merge.
ADR-108 says the Scoop bucket's `checkver` and `autoupdate` read this repository's latest release, so what such a run
publishes is what a Windows install downloads next. A fork's pull request gets a read-only token, so this needs push
rights, like 0289; the trial-on-pull-request design is what makes the write permissions reachable before a merge.
The workflow landed at 16:11 on 12 October, after the deep review's `main`, so 0277's reviewer could not see it.

**Where.** `.github/workflows/release.yml:20-21` (`pull_request: paths:`), `:37-39` (the permissions), `:97`, `:103`
(the `if: push` gates), `:109-110` (`gh release create "$TAG" ... --verify-tag`).

**Fix.** The owner's setting, as 0289 asks for the image's `publish` job: put the release job behind a GitHub
Environment whose deployment rule allows only tags `v*` (a pull request run cannot enter it), recorded as a Later
line on ADR-108 when made. In the file, keep the trial as a job of its own with `contents: read` and no `id-token`
or `attestations`, and the signing and release in a second job that needs it and runs only on a push; a branch can
still edit that, which is why the Environment is the real fix. Must not lose: the trial on a pull request that
changes the file, which needs no write permission at all.

**Record.** ADR-108 governs `release.yml`: a Later line with the Environment. Held for the owner's decision, like
0289 and 0290.

**The owner's decision, 13 October 2026** (asked by session stackvet-deep-review after the review): yes. The owner makes two GitHub Environments in the repository's settings, `release` allowing only tags `v*` and `publish` allowing only the branch `main`; then a pull request adds `environment: release` to this workflow's job and `environment: publish` to `rust.yml`'s `publish` job, with the Later lines on ADR-108 and ADR-051 that say the setting exists and what it holds. Until that pull request lands, the hold stands.

**Done, 13 October 2026** (session stackvet-deep-review): the owner made the `release` Environment (tags `v*`, read back through the API: one rule, no other protection); `release.yml` is two jobs, the trial with `contents: read` and the signing and release inside the Environment, on a tag only; ADR-108, Later.
