# fuzz/seed.sh prints macOS's padded counts, so its test fails on a Mac

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 13 October 2026 by session securevibe-trial-prep, running the whole `sv-cli` suite on the owner's Mac for
backlog 0394. `crates/sv-cli/tests/fuzz_seed_spaces.rs` (backlog 0320) fails there, and passes on CI, which runs
Linux. The test expects `fuzz/seed.sh` to print `seeded: 2 manifests, 1 lockfiles`, and the script builds each count
with `$(ls ... | wc -l)`. macOS's `wc` pads its count with spaces, so the Mac prints `seeded:        2 manifests,
       1 lockfiles and test reports, ...` and the test's `contains` check fails, though the seeding itself is right
(the test's other checks, on the files copied, pass).

To fix: make the script's counts the same on both, for example `$(ls ... | wc -l | tr -d ' ')`, so the line the
person reads is tidy on a Mac too, rather than loosening the test. Then the test passes on a Mac as it does on CI.
Small, and not urgent: CI is green, and a local run of the whole suite reports this one failure every time, which
hides a real failure among them.
