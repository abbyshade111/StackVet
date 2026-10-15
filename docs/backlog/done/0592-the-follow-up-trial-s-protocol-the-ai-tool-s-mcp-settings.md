# The follow-up trial's protocol: the AI tool's MCP settings and the earlier app are part of the blank slate, and stackvet_status must name the new folder before prompt 5

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Cursor + Flash run of 15 October 2026 (backlog 0577). For the owner's trial protocol and prompt 4. Severity:
high for the trial (the run was lost to it).

**What happened.** Cursor's user-level `~/.cursor/mcp.json` from the 14 October trial still pointed sv at that trial's
app folder. `stackvet_status`'s first line, which now names the folder it serves (0541), told the tool so, and the tool
opened that folder, read the earlier app, and copied it into the new one as its own work. Nothing in the prompts told
it not to, and the owner, who was not watching the tool's reading, could not have known.

**What to change.** In the trial protocol (0577 and any later trial): before each run, remove or rewrite the tool's
user-level MCP entry, or make the tool write a project-level one for the new folder; and prompt 4 gains one sentence:
"If stackvet_status names a folder other than this one, stop and tell me; do not read or use that folder." The setup
page's prompt 4 carries it (the wording lane).

**Record.** None.

**Done, 15 October 2026.** Setup prompt 4 now says, where `stackvet_status` names another folder: "Then stop and tell me,
and do not read or use the folder it names: it holds an earlier app, not this one", before it has the old container
stopped with a yes; its stamp was written again. The trial write-up (`docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`)
gains "The blank slate, for any later trial": a new folder, and the tool's own account-wide MCP settings removed or
rewritten, or replaced by one for the new folder, and `stackvet_status` naming the new folder before prompt 5. Held by
`crates/sv-cli/tests/setup_prompt_four_folder.rs`; `setup_prompt_record.rs` holds the rest of prompt 4's sentence, split
in two by the new one. Not tried in an AI coding tool.
