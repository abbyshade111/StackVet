# Each CI part rebuilds the workspace from scratch, about 125 s each, and the no-cache reason no longer distinguishes a cache (the owner's decision)

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: test and CI time. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** Each part pays the roughly 125-second workspace build again (0270's measurement), and the dependencies dominate it: `sv`'s own six crates are small. The comment's reason is "pinning another third-party action"; `actions/cache` is GitHub's own, the same publisher as `actions/checkout` and the `upload-artifact` and `download-artifact` pair ADR-106 just added, pinned the same way. What the reason really protects is the build's inputs: a restored cache holds compiled build scripts and proc-macros that then run in the build, so a cache entry is a new input. GitHub lets a pull request's run read `main`'s cache but never the reverse, which keeps the published image's build clean; a pull request could restore a poisoned entry from its own earlier run, which is the same trust as the branch itself.

**Where.** `.github/workflows/rust.yml:150-154` (the no-cache comment, present twice with two different build times); the three suite parts, the `lints` check build, and the `image` job's native build.

**Evidence.** The comment at `:150-154`; the 126 s build measured in 0270; clippy's check build at 59 s median.

**How it bites.** Estimate: 60 to 90 s per part; once on the critical path since the parts run beside each other. A cache entry is in the range of a gigabyte for a dependency-only cache, and Dependabot's `Cargo.lock` changes turn the key over often.

**Fix.** The owner decides whether a dependency-only cache keyed on `Cargo.lock` is worth one more input to the build. If yes: `actions/cache` pinned to a SHA, restricted to `~/.cargo/registry` and `target/debug/deps` for dependencies, measured on one trial run; a Later entry on ADR-106. If no: keep one copy of the comment with the measured number and the reason as it now stands (the fewest inputs in the job that holds the Docker Hub token).

**Record.** ADR-106 governs `rust.yml`: a Later entry either way, since the reason given changes.
