# CONTRIBUTING.md states the no-network rule without its two exceptions, and three of its instructions are overtaken by the 8 October layout

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** "`sv` opens no network connection of its own" is stated without `sv probe` (ADR-027) and the install step (ADR-052), which every other document names, so a contributor reading only this file would think `sv probe` breaks the rule. The claim instruction says to edit the entry; a claim is the status line set by `tools/backlog.py claim` (ADR-061). "Read the part of `docs/DESIGN.md` for whatever you touch": the entries are in `docs/design/` since ADR-060. "`docs/COVERAGE.md` is generated": so are `docs/REQUIREMENTS.md` and `data/reach.json`, and the test fails on any of the three.

**Where.** `CONTRIBUTING.md:50-51` (the rule), `:12-14` (claims), `:17` (`docs/DESIGN.md`), `:34-36` (generated files); `SECURITY.md:30` has the smaller gap.

**Evidence.** The lines above against `docs/BACKLOG.md:15-17`, `docs/DESIGN.md:21-25`, and `tools/coverage.py`'s docstring.

**How it bites.** A new contributor follows the file and makes a claim nobody sees, reads a file with no entries, and misses two generated files the test will fail on.

**Fix.** Add the two exceptions with their records; name the claim tool, the design folder, and all three generated files.

**Record.** ADR-027, ADR-052, ADR-060, ADR-061 are unchanged; the file is corrected to them: "unchanged, because" lines.
