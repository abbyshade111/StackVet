# `.cursor/mcp.json` is listed as "not read" though `sv connect cursor` writes it and it has the shape `.mcp.json` has

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor app. Severity: low.

**What is wrong.** The Cursor trial's report lists `.cursor/mcp.json` under files not read, with "Cursor's own
documentation of its files was not available to read their format from" (`crates/sv-check/src/ai_tool.rs:96-100`). The
file has the `mcpServers` shape sv reads from `.mcp.json` (the VS Code trial's `.mcp.json` was read and given an AST02
note), and `sv connect cursor` prints it. Cursor documents it on its MCP page.

**Fix.** Read `.cursor/mcp.json` the way `.mcp.json` is read, citing Cursor's documentation page; keep the honest line
for the other `.cursor/` files.

**Record.** ADR-0NN: unchanged, because the file's reading is the same as `.mcp.json`'s; check `ai_tool.rs`'s record.
