# The native comparison build is the one release build without --locked

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: build time. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** With a consistent committed lock it changes nothing today; a stale lock would be rewritten silently on the runner and the native binary compared against the image would be built from different crate versions.

**Where.** `.github/workflows/rust.yml:358`: `cargo build --release -p sv-cli`; the Dockerfile (`:34`) and `windows.yml:42` both pass `--locked`.

**Evidence.** The three build lines.

**How it bites.** A comparison between two binaries built from different dependency versions, unnoticed.

**Fix.** Add `--locked`. Loses nothing.

**Record.** ADR-106 governs `rust.yml`: an "unchanged, because" line.
