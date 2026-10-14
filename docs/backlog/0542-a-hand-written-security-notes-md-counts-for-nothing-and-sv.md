# A hand-written security-notes.md counts for nothing, and sv says so only in the counts

**Status:** open

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
