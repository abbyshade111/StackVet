# Files in a marked report folder are read whole, with no size cap, before write_report and resources/list decide anything

**Status:** done, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: MCP server. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** `refuse_someone_elses_folder` accepts any file whose name is one of `sv`'s, so a planted multi-gigabyte `report.json` or `.stackvet-report.holder` is read into memory on every `stackvet_write_report`; the lock file itself is capped at 64 KiB (`read_holder`), so that one is right. `resources/list` hashes up to 7 × 16 MiB in each of up to 100 marked folders on the request thread with no time limit, unlike a check.

**Where.** `crates/sv-cli/src/report_lock.rs:495-505` (`refuse_older_than` reads `report.json` whole), `report_lock.rs:415-421` (`read_holder_file`), and `crates/sv-cli/src/mcp/resources.rs:16-24` with `report_seal.rs:67-82` (`proven` on up to 100 folders).

**Evidence.** `std::fs::read_to_string(&path)` of `report.json` at `report_lock.rs:495-505` with no cap, where `read_resource` and `digests` already cap at `MAX_RESOURCE_BYTES`.

**How it bites.** A planted 4 GB `report.json` makes the server allocate it whole and parse it as JSON before refusing or writing anything.

**Fix.** Cap these reads at `MAX_RESOURCE_BYTES` (16 MiB) and treat a larger file as not one `sv` wrote; a test with a file one byte over the cap.

**Record.** ADR-041 governs the report folder's refusals: a Later line (a file over the cap is not `sv`'s) or an "unchanged, because" line.
