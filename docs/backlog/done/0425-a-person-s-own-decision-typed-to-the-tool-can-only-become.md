# A person's own decision, typed to the tool, can only become theirs by hand-editing security-notes.md; `sv review` has no "this is mine"

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: medium. Verified against the code
by the reviewing session before filing.

**What is wrong.** The build prompt tells the tool "Record my answers with `stackvet_record_answer`, and only what I
actually answer as mine", and `sv` marks everything the tool records `Written by: AI coding tool` (rightly: it cannot
tell whose words they are). So every answer the person gave in chat arrives in the report as "stated by the AI coding
tool". To make it theirs, a person who has never opened an editor must find `security-notes.md`, find the section,
change one line without touching the rest (a second `Written by:` line or a wrong word makes the section "unreadable
and named", `notes.rs:40`), and only then run `sv review`. `sv review` is already the terminal the person is typing
in, holding the key and the passphrase, and it shows the section's text; it is the right place to ask "is this your
own decision, or the tool's reading that you agree with?", and today it can only ask the second: typing `owner` there
confirms the entry as the tool's word a person agreed with, "never as your own answer" (`review.rs:1263-1267`).

**Where.** `crates/sv-cli/src/review.rs:1256-1300` (`record_notes_confirmation`: a name or Enter), `:1211-1254`
(`record_own`: "If it is wrong, change it in the file first"); `docs/GETTING-STARTED.md:241-246`;
`crates/sv-report/src/interview.rs:117-120`; `crates/sv-cli/src/mcp/tools.rs:650-657`.

**Fix.** In `sv review`, for a section marked `Written by: AI coding tool` (and a `[design]` or `[checked-by-hand]`
entry with `by = "ai-tool"`), offer three answers: `mine` ("this is my own decision; the tool only wrote it down"),
which rewrites the `Written by:` line (or `by`) to the owner and seals it as the owner's own (the path
`Waiting::Notes` already takes); a name, which confirms it as today; Enter, which leaves it. The word must not be
`owner`, which today means the second thing. Then the guide, the interview text, and the tool result stop sending the
person into the file: "run `sv review`; it asks whether each answer is your own". Must not lose: a tool still cannot
do it (terminal only, ADR-026); the seal covers the rewritten line; a confirmation is still shown as "written by the AI
coding tool, confirmed", never as the owner's own (0115, the owner's decision of 27 September 2026); the person's
`mine` answer at their own terminal is exactly what editing the line and sealing does today, so the bar does not
move. The reviewer did not find a Later entry on ADR-022 or ADR-026 that declined this; check the design entries
before building.

**Record.** Yes: it changes how an answer reaches the *documented by the owner* and *attested by the owner* tiers. A
Later entry on ADR-026 (`sv review`'s questions), proposed in the claim; ADR-022's Governs list (`notes.rs`) kept true
or an unchanged line.

**The owner's decision, 13 October 2026** (asked by session stackvet-deep-review): yes, build it. The third answer's word must not be `owner`, which today confirms the tool's word; a confirmation stays shown as the tool's word confirmed by a person, never as the owner's own (the owner's rule of 27 September 2026, 0115). The record is a Later entry on ADR-026, proposed in the claim and accepted in the pull request that builds it.
