# stackvet_prompts with a path follows a symlinked stackvet-report folder outside the root

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: MCP server. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** `ln -s /somewhere/else app/stackvet-report` and `stackvet_prompts {path: "app"}` reads `/somewhere/else/report.json`. What reaches the tool is only requirement ids the prompt library itself names and fixed status words, and the error names the unresolved path, so nothing of the outside file's text leaks; it is still a read outside the root, and the one place a folder link is not refused.

**Where.** `crates/sv-scan/src/ecosystems.rs:1023-1030` (`default_report_dir_in` uses `exists()` and `is_dir()`, which follow links) and `crates/sv-cli/src/mcp/tools.rs:363-376`, which checks only that `report.json` itself is not a link.

**Evidence.** `if !new.exists() && old.is_dir() { return old; }` and, in `tools.rs`, `!meta.file_type().is_symlink()` on `report.json` only.

**How it bites.** A read outside the folder the server was started for, against the rule in `confine.rs`.

**Fix.** `refuse_link` on the folder too, as `claim_report_folder` does; a test with a linked folder.

**Record.** ADR-066 governs the server: an "unchanged, because" line.
