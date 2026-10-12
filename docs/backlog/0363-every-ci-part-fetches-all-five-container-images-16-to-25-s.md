# Every CI part fetches all five container images, 16 to 25 s per part, where each part starts only some

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: test and CI time. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Only `crates/sv-run/src/docker.rs` names the images, and which part's tests start which is not written down.

**Where.** `.github/workflows/rust.yml:145-146` (busybox, mailpit, node, python, headless-shell), fetched in every part.

**Evidence.** The step time, 16 to 25 s per part, from the measurements.

**How it bites.** 10 to 20 s per part, if someone measures which part uses which; a wrong list costs nothing but a slower pull inside the suite, as the comment says.

**Fix.** A per-part image list, derived once from the tests each part runs and held by a test to `docker.rs`'s names.

**Record.** ADR-106 governs `rust.yml`: an "unchanged, because" line.
