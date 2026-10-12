# The build-loop record's hash chain does not catch lines cut from the end, and the report's wording says removal is caught

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: MCP server. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Each line's hash covers the previous line's chain and its own text, so deleting the last N lines leaves a chain that checks out. The report says a break means "a line was changed, removed, or added after it", and when the chain holds, that a rewrite of the whole record would keep a valid chain (honest, since the chain is keyless). Truncation at the tail is the one case neither sentence covers, and it is the cheapest edit: drop the checks that showed many findings and keep the early ones.

**Where.** `crates/sv-cli/src/build_loop.rs:330-403` (the chain); the wording at `crates/sv-report/src/lib.rs:1995-2005`.

**Evidence.** `lib.rs:1997`: "The record's chain of hashes breaks at its line {at}: a line was changed, removed, or added after it"; `:2003`: "Each of the record's {} chained lines checks out against the one before it".

**How it bites.** An AI coding tool trims the record's last lines; the report says every line checks out.

**Fix.** Say that lines cut from the end are not detected, or carry the last chain head into the sealed `report.json` so the next report can show the record only grew; a test that truncates a record and reads what the report says.

**Record.** ADR-076 governs the record and what the report says of it: a Later line.
