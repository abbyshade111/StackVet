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
it)", and goes on: "If it says this folder is not inside the folder the server was started for, StackVet is serving
another app's folder: on a Mac or Linux, show me what `docker ps` lists, stop the old StackVet container only after I
say yes, and tell me to restart you." The item's "ask stackvet_status which folder it serves" waits on 0541, which makes
the status name its folder; until then `stackvet_status` prints only the path it was asked about. Asked about a path
outside its folder, though, the server already says so, in the words the prompt uses
(`crates/sv-cli/src/mcp/confine.rs`), so this works now and stays true after 0541. `docker ps` is for a Mac or Linux
only: on Windows the server is not a container. `crates/sv-cli/tests/setup_prompt_record.rs` holds both sentences;
with the page as it was, it went red.
