# `.cursor/mcp.json` is listed as "not read" though `sv connect cursor` writes it and it has the shape `.mcp.json` has

**Status:** done, 15 October 2026. Cursor's MCP page (<https://cursor.com/docs/mcp>) could still not be reached from
the cloud session, so the owner copied it, and its parts `sv` relies on are kept in
`docs/research/CURSOR-MCP-2026-10-15.md` (choice B). `.cursor/mcp.json` is read as `.mcp.json` is, named as Cursor's;
no note says Cursor starts its servers without asking, as the page says it asks first. The other `.cursor/` files are
still named as not read. ADR-049, Later, 15 October 2026; held by `crates/sv-check/src/ai_tool_cursor_tests.rs`.

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor app. Severity: low.

**What is wrong.** The Cursor trial's report lists `.cursor/mcp.json` under files not read, with "Cursor's own
documentation of its files was not available to read their format from" (`crates/sv-check/src/ai_tool.rs:96-100`). The
file has the `mcpServers` shape sv reads from `.mcp.json` (the VS Code trial's `.mcp.json` was read and given an AST02
note), and `sv connect cursor` prints it. Cursor documents it on its MCP page.

**Fix.** Read `.cursor/mcp.json` the way `.mcp.json` is read, citing Cursor's documentation page; keep the honest line
for the other `.cursor/` files.

**Record.** ADR-0NN: unchanged, because the file's reading is the same as `.mcp.json`'s; check `ai_tool.rs`'s record.

**The owner's answer, 15 October 2026 (asked through the board's waiting line):** A: the owner adds cursor.com and docs.cursor.com under Allowed domains in the cloud environment's Network access; wording then reads Cursor's MCP page and builds from it, as ADR-049 requires. Until the owner has made the change, the item waits; B (the page saved into docs/research/) is the fallback if the setting cannot be made.
