# README, ARCHITECTURE, and the MCP catalog count five report files where sv writes seven

**Status:** done, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: documentation. Severity: high. Verified against the code by the reviewing session before filing.

**What is wrong.** The code writes seven files every run: the five named plus `low-findings.md` (ADR-095) and `seen.json` (ADR-082): `crates/sv-cli/src/report_files.rs` (`REPORT_FILES: [ReportFile; 7]`, whose own docstring says "five then"), written whole by `crates/sv-cli/src/lib.rs:1821`.

**Where.** `README.md:129-133`; `docs/ARCHITECTURE.md:27` ("the five renderings") and `:58` ("the six files (the five reports, and `seen.json`"); `crates/sv-cli/src/mcp/catalog.rs:570` (the `stackvet_write_report` description an AI tool reads).

**Evidence.** `pub const REPORT_FILES: [ReportFile; 7]` against "the five renderings" and "the six files".

**How it bites.** A reader told five files does not know the low-severity findings are listed in full in a file of their own, and the AI tool is told the same.

**Fix.** Name all seven in README and ARCHITECTURE ("seven renderings", "the seven files") and in the catalog description; a test already holds ARCHITECTURE's paths (`architecture_map.rs`), so consider holding this count to `REPORT_FILES.len()` too.

**Record.** ADR-095 and ADR-082 govern the two files; ADR-066 the catalog: "unchanged, because" lines (wording only).

**The owner's word, 12 October 2026:** the description in `crates/sv-cli/src/mcp/catalog.rs` waits until the loop test (0257)
has finished, since it is text the AI coding tool reads; the documents (README, ARCHITECTURE) can be fixed now.
