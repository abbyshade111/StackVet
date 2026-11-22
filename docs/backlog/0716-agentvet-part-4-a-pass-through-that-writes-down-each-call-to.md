# AgentVet part 4: a pass-through that writes down each call to a tool and its answer, and a chain the owner checkpoints

**Status:** claimed by Claude Code session (AgentVet), 21 November 2026

Asked for by the owner on 21 November 2026 ("go ahead with the pass-through and keyed chain after that"). Step 3 of
the build order in the design entry "AgentVet, a companion that vets the agent: what it did, what it said, and where
the two part (21 November 2026)", and ADR-159's decisions 10 and 12. Each part lands on its own, and ADR-159's "As
built" grows with each. Two things here are decisions and not only building, and are asked of the owner before the
part that needs them: what the pass-through's record keeps of a call, and which key a checkpoint is signed with and
when.

1. **The pass-through.** `av proxy -- <the tool server's command>` sits between an agent and an MCP server that
   speaks over standard input and output: it starts the server, relays every line each way unchanged, and writes down
   each call to a tool and its answer. A line holds the time by `av`'s clock, the tool's name, whether the answer was
   an error, a SHA-256 of the arguments, and their size; what more it keeps of the arguments, cut short and with keys
   cut, is the owner's to decide. Never an answer's text. It adds no connection of its own and changes nothing it
   relays; if it cannot write the record it says so on standard error and keeps relaying, so a full disk does not stop
   the agent's work, and the gap is in the record's own count. This is the first time `av` starts a program and the
   first time it writes a file: ADR-159's decision 10 and the architecture page's fence say so when it is built, and
   the test of `av`'s own source changes to say what it may start.
   **Part status:** done, 21 November 2026
2. **The report reads it.** `av report` reads the pass-through's record, with how far it can be relied on: which tools
   were called and how often, which answers were errors, and, under "Trying again after a no", a call answered with a
   refusal and made again. The task file gains `[may] call`, the tools the agent may call, and a call outside it is
   listed. What still cannot be held is said: `[may] reach`, unless a tool's own arguments name a host, and `ask-first`,
   unless the server's answer says the person was asked.
   **Part status:** open
3. **The chain takes a key, and the owner checkpoints it.** Each record's chain is made with a key kept beside `sv`'s
   own (readable only by the person), and `av checkpoint`, run by the owner at their own terminal, signs each record's
   latest link with the key `sv review` uses. The report then says which lines stand under a checkpoint the owner
   made and which rest on the chain alone. Said plainly wherever it applies: a tool that runs as the owner can read
   the chain's key, so the key stops a rewrite by something that cannot, and a checkpoint is what shows a rewrite of
   lines the owner already signed for. Waits on the owner's answers about the key.
   **Part status:** partly done: the chain's own key, so a rewrite by something that cannot read the owner's files shows; the checkpoints are built
