# The Docker Hub token is on the runner's disk while a branch's own tests and build scripts run

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** `docker login --password-stdin` stores the token base64-encoded in `/home/runner/.docker/config.json`, and `cargo test` runs the branch's test code, build scripts, and proc-macro crates as the same user. A same-repository branch has the secrets (a fork's does not).

**Where.** `.github/workflows/rust.yml:126-149` ("Sign in to Docker Hub") comes before `:151-167` ("Tests", `cargo test --workspace`), with the sign-out at `:175`; the same order in the `image` job (`:215-246`: sign in, then `docker build`, `cargo build` with build scripts, `tools/image_smoke.py`).

**Evidence.** The step order above; `rust.yml:97-98` already pulls the suite's images through the mirror before the sign-in.

**How it bites.** A branch adds a test, or a dependency with a build script, that reads that file and posts it to an outside address; the token then pushes to the owner's Docker Hub account.

**Fix.** `docker logout` right after the sign-in has pulled what the suite needs (add any image left off the list at `:97-98`), so no credential is on disk when `cargo test`, `cargo build`, or `image_smoke.py` runs; or sign in only in the `publish` job, which runs on `main` alone.

**Record.** ADR-051 governs `rust.yml`: a Later line (when the token is on disk).
