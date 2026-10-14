# Parse the OSV advisory files on one thread per core, keeping the findings' order

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Left from backlog 0342, which built the index that made the matching itself fast (26 s to 0.13 s at 300 components and
20,000 records, debug build). The review's estimate for this part was not measured.

**What is wrong.** `read_database` (`crates/sv-check/src/advisories.rs`) reads and parses the advisory files one at a
time. OSV's PyPI export is tens of thousands of files, so the read may now be most of what `--advisories` costs.

**Fix.** Measure first: time `read_database` on a real OSV export beside `audit_against`. If the read dominates, parse
on one thread per core (`std::thread::scope` over chunks of the file list), then put the records back in the order the
walk found them, since that order is the order the findings are listed in; `unread` stays sorted as now.

**Record.** No record governs `advisories.rs`; the advisory data stays the owner's download (ADR-027).
