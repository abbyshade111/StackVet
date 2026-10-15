# Prompt 4: ask which folder the server serves, and stop an older container before going on

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity trial. For the prompt library
(0370); the sv side is the `stackvet_status` item.

**What was seen.** The tool spent 16 minutes connected to a container that served an older app's folder; three stale
containers were running.

**What to change.** Prompt 4: "Ask stackvet_status which folder it serves. If it is not this one, show me `docker ps`,
stop the old container with my yes, and tell me to restart the tool."

**Done, 15 October 2026:** prompt 4 now has the tool call `stackvet_status` "with this folder's full path (`pwd` prints
it)", say "which folder it says it serves", and goes on: "If that is not this folder, or it says this folder is not
inside the folder the server was started for, a StackVet server started for an earlier app is still running: on a Mac
or Linux, show me what `docker ps` lists, stop the old StackVet container only after I say yes, and tell me to restart
you." The folder the status names comes from 0541, which landed while this was open and added the "which folder"
words; the server's answer to a path outside its folder (`crates/sv-cli/src/mcp/confine.rs`) catches the same case
for a server older than 0541. `docker ps` is for a Mac or Linux only: on Windows the server is not a container.
`crates/sv-cli/tests/setup_prompt_record.rs` holds both sentences; with the page as it was, it went red.
