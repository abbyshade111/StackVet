# config.versions-pinned and the SBOM complete credit give V15.1.2 whole for a lockfile

**Status:** claimed by builder-none, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** V15.1.2 (L2, automatable): "Verify that an inventory catalog, such as software bill of materials (SBOM), is maintained of all third-party libraries in use, including verifying that components come from pre-defined, trusted, and continually maintained repositories." Pinned versions and a complete list are the inventory half; the repository half is checked nowhere except npm under `install = true` (0188 item 2), and "continually maintained" nowhere.

**Where.** `crates/sv-check/src/config.rs:818` (`Outcome::Passed(&["V15.1.2"])`; `record()` at `:50-57` marks only `config.gitignore-covers-env` in part); `crates/sv-check/src/sbom/cyclonedx.rs:129` (`INVENTORY_REQUIREMENT`).

**Evidence.** The two whole credits at the lines above.

**How it bites.** V15.1.2 reads *checked* on a lockfile alone.

**Fix.** `.in_part()` on both credits, or cite V15.1.2 only from the SBOM and say what it does not show; regenerate the coverage files.

**Record.** ADR-053 governs in-part credit: an "unchanged, because" line.
