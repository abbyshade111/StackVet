# Nightly: Rust fails: publish step, GHCR answers unknown blob

**Status:** on hold: the GHCR "unknown blob" answer coming back; one failure is not worth a change

Recorded by the nightly CI routine on 21 October 2026. No test failed: the Rust workflow's push run on `main` for
commit `920ff99` (pull request #2158, "Claim backlog 0618 and 0619, with ADR-129 proposed", a backlog claim with no
code in it) went red in its last job, `publish`, at the step "Publish them, as one image for both platforms":
<https://github.com/abbyshade111/StackVet/actions/runs/39449540115> (20 October 2026, 15:44 UTC).

The `test`, `image`, and `image-arm64` jobs had passed, both image artifacts downloaded with the digests they were
uploaded with, and `docker login ghcr.io` succeeded. The first `docker push` of the amd64 image sent all seven layers
("Pushed") and then GitHub's container registry answered:

    unknown blob
    ##[error]Process completed with exit code 1.

So nothing was published or signed for `920ff99`; `latest` was left to the next commit's run.

- **Was it there the night before?** No. The scheduled Rust run of 20 October (08:29 UTC, `83b5492`) and every Rust
  run on `main` before this one in the last 26 hours passed, and so did every one after it: the next push, #2145 at
  15:54 UTC (`3fc8db5`), published, and the scheduled run of 21 October (`1eb7c5e`) passed. `main` was red for about
  ten minutes and is green now.
- **Which pull request merged last before it went red:** #2158 itself (`920ff99`); the one before it, #2157
  (`c35bd4d`), passed. #2158 changed only the backlog and a proposed decision record, so the change did not cause
  this: it reads as the registry refusing a layer it had just been sent, a failure outside the repository. One
  occurrence; the routine has not re-run it.
- **What to decide:** whether one such refusal is worth anything beyond this note. If it comes back, the publish
  step could retry a `docker push` that fails with "unknown blob" once, which is a change to `.github/workflows/rust.yml`
  and its record (see the `publish` job's comment for the decisions it carries). This routine records; it fixes nothing.
