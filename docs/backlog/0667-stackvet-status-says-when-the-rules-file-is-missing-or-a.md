# stackvet_status says when the rules file is missing, or a tool will not see it, and how sv rules fixes it

**Status:** claimed by claude/0667-status-rules, 24 October 2026

Found 24 October 2026, at the owner's question whether the MCP server should direct AI tools to the rules when they
connect. It already does, in part: its instructions tell a connected tool to call `stackvet_guidance` before writing
code and to follow the rules it gives (`crates/sv-cli/src/mcp/mod.rs`, `INSTRUCTIONS`). But nothing in the MCP
server mentions `AGENTS.md` or `sv rules`. A tool working through StackVet is never told that the rules file is
missing, or that Claude Code will not see it, so the copy that matters when StackVet is *not* connected (its first
rule says to stop and tell the person) may never have been written.

**Order:** after 0665, whose `crates/sv-check/src/tool_files.rs` this reuses.

To do:

- `stackvet_status`, the tool for "is it set up, and what is missing", reads the app's folder and says, in `sv`'s own
  words:
  - that `AGENTS.md` has no section between `sv`'s markers, and that the person can run `sv rules` to write it;
  - that the section is there but older than the rules `sv` gives now, if that can be told without guessing (for
    example by comparing the rule ids it holds with those `stackvet_guidance` gives); otherwise nothing is said about
    age;
  - each note `tool_files::plan` gives (Claude Code with a `CLAUDE.md` that does not import `AGENTS.md`, Codex with an
    `AGENTS.override.md`, Aider, Cline), worded for a tool to pass on to the person.
- **Read-only.** The server gains no way to write `AGENTS.md`, `CLAUDE.md`, or any tool's file: ADR-066 keeps it to
  what `sv report` does, and a tool that rewrites instruction files when asked is exactly what injected text would
  try to reach. The write stays with `sv rules`, run by the person at a terminal.
- Nothing the app wrote is given as an instruction: file names only from the fixed set `tool_files` looks for,
  never a folder name or a file's text (ADR-066, point 5).
- A dated Later entry on ADR-066 (what `stackvet_status` reads), or a line saying why it stands; and a structured
  field for the result, in `crates/sv-cli/src/mcp/catalog.rs`, so a client reading only the structured answer sees it.
- Tests: an app with no `AGENTS.md`, one with the section, one with a `CLAUDE.md` that does not import it; the status
  answer says what is missing in each, and writes nothing (the folder's files are byte-for-byte the same after). Break
  each check on purpose and watch the test go red.
- **What it is worth:** nothing, as evidence. Whether the rules file is there changes no requirement's status.

**Claimed 24 October 2026.** The plan: one more line in `sv doctor`'s answers (`crate::doctor::answers`), which
`stackvet_status` already gives in its words and its structured answer, so the server's answer gains no new field;
and a dated Later entry on ADR-066.
