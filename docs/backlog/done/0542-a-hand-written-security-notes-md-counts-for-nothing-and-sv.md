# A hand-written security-notes.md counts for nothing, and sv says so only in the counts

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor trial. Severity: medium (the owner's
real decisions were written down and counted as nothing, with no line saying why).

**What is wrong.** The Cursor tool wrote `security-notes.md` by hand under its own headings with the owner's real
choices (session length, guessing limit, rate, who may do what) and never called `stackvet_record_answer`;
`report.json` `counts.documented = 0`, `stated = 0`. Nothing in `stackvet_check` or the report says "N sections of
security-notes.md are not under a question id and none of them count."

**Fix.** When `security-notes.md` exists and holds text outside the question headings, `stackvet_check` and the
report say so in one line, with the count and the first heading, and point at `stackvet_record_answer` and
`sv notes`. The prompt side is a separate item.

**Record.** None.

**Done, 15 October 2026.** The notes reader now names the headings of somebody's own that come before any of the
file's questions (`notes::Answers::outside`, carried to `notes::Evidence::outside`); `sv`'s own title line is not one
of them, and a heading after a question stays in the list the report already had. When there are any, the report's
"What was not examined", and so `stackvet_check`'s, has one entry: how many headings of your own come before any of
the questions in security-notes.md, the first named, that nothing under them counts, and that the decisions count once
the AI coding tool records each with `stackvet_record_answer`, or `sv notes` adds the questions and each answer moves
under its own, then `sv review` makes them the owner's. Tests: `sv-check` `notes/outside_tests.rs` and `sv-cli`
`tests/notes_outside_questions.rs` (the trial's file, built in the test), broken on purpose three ways (the headings
not collected, the title line counted, the report's entry switched off), each turning red.

Not done: the prompt side (have the tool call `stackvet_record_answer`) is a separate item, as this one says.
