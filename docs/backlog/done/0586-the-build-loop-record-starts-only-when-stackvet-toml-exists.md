# The build-loop record starts only when stackvet.toml exists, so the tool's first calls (status, guidance, spec) are not recorded

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: low.

**What was seen.** `.stackvet/build-loop.jsonl` in the Opus app begins at the first call after `stackvet.toml` was
written; the chat shows `stackvet_status`, `stackvet_guidance`, and `stackvet_spec` calls before it that the record
does not hold, so the record understates how the tool began.

**Fix.** Record every MCP call from the first, in the folder the server serves, with "no stackvet.toml yet" on the line
where that is so; ADR-076 says the record lives with the app, and the folder exists before the manifest does.

**Record.** ADR-076 Later (what the record holds).

**Done, 15 October 2026** (documentation). A call at the folder the server serves is written down before it holds a
`stackvet.toml`, its line marked `"no_manifest": true` (`mcp/mod.rs`, `write_line`; `build_loop::Line::no_manifest`);
a call at another folder with none still is not. ADR-076, Later. Tests: `mcp/build_loop_first_calls_tests.rs` (3);
with the old rule put back, the first-calls test fails.
