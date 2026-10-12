# The AI coding tool's own settings are quoted into the report without redaction

**Status:** claimed by builder-full, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: secrets. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** `quote` only blanks control characters and cuts at 160 characters; nothing downstream runs `redact_text` on a note's text. The text lands in `report.html`, `compliance.md`, `report.json`, and is handed to the AI tool as an MCP resource.

**Where.** `crates/sv-check/src/ai_tool.rs:59-70` (`quote`), used at `:172` (hook `command`), `:178` (hook `url`), `:217` (apiKeyHelper and the other helper commands), `:229-243` (`ANTHROPIC_BASE_URL`, `*_BASE_URL`, `HTTPS_PROXY`), `:288-313` (MCP server `command` + `args`, and `url`); `crates/sv-cli/src/assemble.rs:2661` passes `ai_tool::read(listing)` straight into the report.

**Evidence.** `fn quote(text: &str) -> String { … .map(|c| if c.is_control() { ' ' } else { c }) … }` and no `redact` anywhere in `ai_tool.rs`.

**How it bites.** `.mcp.json` holds `"url": "https://mcp.example.com/sse?api_key=…"` or `"args": ["--token", "…"]`, or a hook command is `curl -H "Authorization: Bearer …"`, or a proxy URL carries `user:pass@`: the report prints the key in full, against the rule that `sv` never prints a key. The bundle's backstop scan (S8) catches only vendor-shaped keys, so a random token passes into the zip too.

**Fix.** Give `ai_tool::read` the `SecretRules` and pass every quoted text through `secrets::redact_text` (one line in `quote`), with a test that plants a key built from pieces at run time in each quoted field and asserts the report shows the first four characters and the length only.

**Record.** ADR-092 governs what is read of the AI tool's files: a Later line (what is quoted is redacted first).
