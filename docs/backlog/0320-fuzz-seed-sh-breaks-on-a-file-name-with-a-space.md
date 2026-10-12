# fuzz/seed.sh breaks on a file name with a space

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** No such file exists under `examples/`, `crates/`, or `data/` today; a fixture named with a space would seed two broken paths and stop the step (`set -eu`).

**Where.** `fuzz/seed.sh:22-28`: `for f in $(files -name stackvet.toml)` word-splits the find output.

**Evidence.** The unquoted command substitution in a `for`.

**How it bites.** The fuzz seeding step fails on the first fixture with a space in its name.

**Fix.** `files … -print0 | while IFS= read -r -d '' f`.

**Record.** ADR-077 governs the fuzz targets: an "unchanged, because" line.
