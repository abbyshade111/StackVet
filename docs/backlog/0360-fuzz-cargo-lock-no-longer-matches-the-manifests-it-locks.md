# fuzz/Cargo.lock no longer matches the manifests it locks

**Status:** claimed by builder-none, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the efficiency review of 12 October 2026 (backlog 0335), area: build time. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** `cargo +nightly fuzz run` rewrites the lock on first use, so the fuzz build is not the reproducible one ADR-077 describes.

**Where.** `fuzz/Cargo.lock` (109 packages): 24 shared crates differ in version from `Cargo.lock`, and some cannot satisfy the current manifests (`ed25519-dalek 2.2.0`, `aes 0.8.4`, `ctr 0.9.2`, `bcrypt-pbkdf 0.10.0`, where `crates/sv-check/Cargo.toml` asks for 3.0, 0.9, 0.10, 0.11).

**Evidence.** The version comparison between the two lock files.

**How it bites.** The fuzz run is built from whatever resolves that day.

**Fix.** Regenerate `fuzz/Cargo.lock` (`cargo update -w` in `fuzz/` on a machine with the registry) and commit it; a test could compare the shared crates' versions between the two locks.

**Record.** ADR-077 governs the fuzz targets: an "unchanged, because" line.
