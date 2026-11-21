# AgentVet part 1: the agent records move into a crate of their own, and av report reads them

**Status:** open

Asked for by the owner on 21 November 2026, with ADR-159 (proposed) and the design entry "AgentVet, a companion that
vets the agent: what it did, what it said, and where the two part (21 November 2026)". This is step 1 of that entry's
build order, and the first thing ADR-159 decides to build (decision 2). It waits for the owner to accept ADR-159, or to
say which of its decisions to change; the pull request that builds this makes the record accepted and fills in "As
built".

1. **The shared crate.** The build-loop record (`crates/sv-cli/src/build_loop.rs`, ADR-076), the hook record
   (`crates/sv-cli/src/hook_record.rs`, ADR-122), and the reader of what the tool did (`crates/sv-cli/src/tool_behavior.rs`,
   ADR-131) move into a crate of their own, word for word, with their tests. `sv-cli` re-exports them under the old
   paths, and `sv`'s report says what it said before: the tests that hold its section on the tool pass unchanged.
   **Part status:** open
2. **The `av` binary and `av report`.** A binary of its own, `av`, on the shared crate and `sv-core`, never on
   `sv-check`'s checks. `av report <folder>` reads the records for one session and writes the table of the white
   paper's Figure 19: findings silenced by rewriting, findings fixed for real, and what could not be looked for, each
   with the tier of its witness. It reads no transcript yet, so it says claims were not looked at. It opens no network
   connection, and a test holds that.
   **Part status:** open
3. **The records it governs.** ADR-076, ADR-122, and ADR-131 each get a Later entry saying where their code lives
   now, with their Governs lists kept true; `docs/ARCHITECTURE.md` gets the new crates and a line for `av` under "The
   fence".
   **Part status:** open
