# ARCHITECTURE's counts are stale: 435 design entries, 91 records, and every crate's line count has moved

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** Today: 435 design entries, 17,762 lines; 91 records, ADR-015 to ADR-105. The crate counts: `sv-frameworks` 1,200 (now 1,413 without test files, 1,539 with), `sv-manifest` 4,100 (4,640 / 4,749), `sv-scan` 4,100 (4,267), `sv-check` 97,700 (93,094 / 115,996), `sv-run` 6,900 (8,700 / 9,669), `sv-report` 8,100 (10,257 / 11,539), `sv-cli` 21,300 (24,185 / 34,322); the page does not say how it counted.

**Where.** `docs/ARCHITECTURE.md:5` ("nearly 300 [design entries], some 13,000 lines"; "the 44 decisions") and the crate table at `:22-28`.

**Evidence.** `ls docs/design/*.md | wc -l`, `cat docs/design/*.md | wc -l`, `ls docs/adr/ADR-*.md | wc -l`, and `wc -l` per crate.

**How it bites.** The page says the code wins when they disagree, but it is the first thing a new reader opens, and every number on it is wrong.

**Fix.** Refresh the numbers and say what is counted ("lines of `src/*.rs`, test files left out"), or replace exact counts with rounded ones and the date; or have a small script print the table so a test can hold it.

**Record.** No record governs `ARCHITECTURE.md`; a test (`architecture_map.rs`) holds its paths; say so.
