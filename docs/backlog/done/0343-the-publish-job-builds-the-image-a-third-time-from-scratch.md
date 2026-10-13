# The publish job builds the image a third time, from scratch, on the critical path, and attests a build nobody drove

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: build time. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** It is a fresh build of the same commit, and Rust release builds are usually but not provably byte-identical. Per push to `main` the pipeline compiles `sv` seven times: lints (check mode plus all the C grammars), the three suite parts, the image's Docker build, the image's native build, and publish's Docker build; the last is serial after `test` and `image`, so it adds its whole duration to every push to `main`.

**Where.** `.github/workflows/rust.yml:487` (`publish`'s `docker build`) repeats `:355` (`image`'s) on a different runner from the same commit; the comment near `:401-414` says what is published is what the smoke test drove.

**Evidence.** The two `docker build` lines, and the job order. Measured: the publish job's Docker build has a median of 96 s over six runs on `main`, and the whole publish job 123 to 142 s; the from-scratch estimate of 3 to 6 minutes was high (a from-scratch release build of 91 crates and 178 MB of grammar C inside `rust:1-slim`).

**How it bites.** Every merge to `main` waits about two minutes before the image is published, and the attested image is not the one the smoke test ran.

**Fix.** In `image`, `docker save stackvet/sv | gzip` to an artifact (estimate 60 to 120 MB); `publish` downloads it, `docker load`s, retags, pushes, and attests that digest. Keeps `--locked` (in the Dockerfile), the pinned base digests, the publish-only-on-`main` condition, and makes the attestation a stronger statement: the digest attested is the tested image's.

**Record.** ADR-051 and ADR-080 govern: a Later line on ADR-080 (what is attested is the tested image), since this is a decision.
