# Would a records folder in sv's container be useful to anything sv does besides the note? An investigation

**Status:** open

Asked for by the owner on 22 November 2026 ("file a backlog item one of the other sessions can pick up to investigate
if the records file in the container would be useful to any other sv functions"). Open for any session. An
investigation: its result is a write-up with a recommendation for the owner, not a build.

**Where this comes from.** When `sv` runs through Docker, as `sv connect <tool> --docker` sets it up, the container is
given the app's folder, and with `--keys` the list of trusted keys, read-only. It is given nothing of the folder
where `sv` keeps its own records on the person's computer (`$XDG_STATE_HOME/stackvet`, or `~/.local/state/stackvet`).
On 22 November 2026 `sv`'s MCP server began to note, in that folder, where the build-loop record has got to (ADR-159,
"As built, part 8"; ADR-076, Later). Through Docker it could not, so `sv connect --docker` is gaining an option that
mounts the one folder of those notes, and nothing else, into the container (backlog 0716). The owner's question is
whether anything else `sv` does would gain from reaching its records folder from inside the container.

**What to look at.** For each, say what `sv` in a container does today, what it would do with the folder, and whether
that is worth what it costs.

1. **Everything `sv` writes or reads outside the app's folder.** Find it in the code, not from memory: the hook
   record and `sv hook-record` (ADR-122), which the hook runs with the `sv` installed on the computer and tells a
   person with only Docker "is not offered for them yet"; the report key and the seal that lets the MCP server tell
   `sv`'s report from another's (ADR-034); the review key and signing key (`sv review` is run at a terminal, ADR-026,
   ADR-043); anything under `sv`'s state or config folders that a run of `sv report`, `sv check`, or `sv mcp` in a
   container cannot reach. `docs/ARCHITECTURE.md` ("What `sv` writes into somebody's folder") and
   `crates/sv-cli/src/connect.rs` are where to start.
2. **What a person with only Docker cannot do today because of it.** The hook record is the known one. List any
   other, with the sentence `sv` says about it now.
3. **What the container could then read or write on the person's computer.** Today it is the app's folder and, read
   only, one list of keys. Each folder added is something an app's own files, read by `sv` in that container, are
   one step nearer to. Say for each candidate whether it would be mounted to read or to write, what it holds, and
   what a fault in `sv` or a hostile app could do with it.
4. **Whether one records folder or one folder for each use.** The notes are mounted alone, on purpose, so the
   container does not see the hook record, the pass-through's record, or the owner's checkpoints. Say whether that
   should stay the rule.
5. **Windows,** where the app's folder has another path inside the container (ADR-153), so a record named by the
   folder's path is filed under a name the computer's own `sv` or `av` would not look for.

**What to hand back.** A design entry (`python3 tools/design_entry.py new`) with what was found for each, a
recommendation, and the choices that are the owner's. Anything worth building becomes an item of its own. Changing
what the container is given is a decision with a record (ADR-101, ADR-153, and the fence in `docs/ARCHITECTURE.md`).
