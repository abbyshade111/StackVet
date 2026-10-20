# Answers given in the chat reach sv review as the owner's, and are recorded together (20 October 2026)

**Status:** done, 20 October 2026

The owner asked, on 20 October 2026, how the questions only a person can answer can be served during the build and
their answers taken in. Most of it is built: `stackvet_check`'s "questions" section hands the AI coding tool every
design question, written decision, and check by hand, with the lines of code each rests on, and the tool interviews
the person one question at a time (backlog 0123). The weak step is the handoff. An answer the person gives in the chat
to a design question or a check by hand is written as the tool's, and setup prompt 5 forbids the tool to write it any
other way, so it can only become the person's by a hand edit of `stackvet.toml`; and `sv review` then asks about each
such answer one at a time. In the trials, owners handed the questions back to the tool after one to six of them
(backlog 0550). Elicitation stays declined (ADR-028, Later, 6 October 2026), and nothing about what counts changes:
an answer is the owner's only once `sv review` records it, at a terminal, with a seal.

The owner's choices, 20 October 2026, asked in a session:

1. **The tool writes `by = "owner"` for an answer the person gave it in the chat, and only then.** This settles a
   contradiction in the instructions: `crates/sv-report/src/interview.rs` (what `stackvet_check` hands the tool) and
   the starter `stackvet.toml` (`crates/sv-manifest/src/spec.rs`) already say so, and setup prompt 5
   (`docs/prompts/setup.md`, line 314) says "Never write `by = \"owner\"`". Prompt 5 is corrected to match, and so is
   the module text at the top of `interview.rs`, which still describes `Written by: owner` for a notes section the
   tool wrote down (the tool now writes every section as its own, and the person makes it theirs with `mine`). It
   stays the tool's word until `sv review` records it, which is the path the owner kept on 14 October 2026 (backlog
   0511, option c); no `mine` is added for a `[design]` or `[checked-by-hand]` entry. Record: a Later entry on ADR-022.
   **Part status:** done, 20 October 2026
2. **`sv review` shows every answer given as the owner's, then records them at one `owner`.** The `[design]` answers
   and `[checked-by-hand]` results with `by = "owner"`, and the notes and design-decisions sections marked
   `Written by: owner`, are listed together, numbered, each with its requirement, the answer, and the file it points
   at, under the warning `record_own` gives now (the tool can write `by = "owner"` too). One prompt then takes
   `owner` to record all of them, numbers to leave some out (the list is shown again without them), or Enter to go
   through them one at a time as now. Each is sealed over its own fields exactly as before, and saved only if it then
   counts. Findings, confirmations, left-out folders, and sections to confirm are still asked one at a time. Record: a
   Later entry on ADR-026.
   **Part status:** done, 20 October 2026
3. **Two counts that are out of date.** The starter `stackvet.toml` says there are "at most sixteen" design questions
   (`spec.rs`, line 397); there are 18. `crates/sv-check/src/hand.rs` says "Twenty ASVS requirements" are checked by
   hand; `human-checks.json` holds 64 (37 ASVS, 25 AISVS, and 2 of its Appendix C).
   **Part status:** done, 20 October 2026

**Built, 20 October 2026.** Prompt 5 and the guide's copy say to write `by = "owner"` only for an answer the person
gave the tool themselves, and `by = "owner"` for a check by hand they made and told it of; the interview's rule no
longer says "or confirmed", and its module text describes `mine` (ADR-022, Later). `sv review` lists two or more
answers given as the owner's together and records them at one `owner`, leaves out the ones the person numbers, or
goes one at a time at Enter (`crates/sv-cli/src/review/together.rs`, ADR-026, Later), each sealed over its own
fields through the same `own_what` and `apply_own` the one-at-a-time path uses. The counts are gone from the starter
`stackvet.toml`, `design.rs`, `hand.rs`, and the README rather than corrected, so they cannot go out of date again.
