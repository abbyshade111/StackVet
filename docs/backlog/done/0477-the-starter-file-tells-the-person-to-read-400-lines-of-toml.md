# The starter file tells the person to read 400 lines of TOML before trusting the report, where the report already shows the answers

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-manifest/src/spec.rs:8` (`# Written by your AI coding tool; read it yourself before you trust
  the report.`); the readable form is the report's "What the app says about itself" table
  (`markdown.rs:699-711`: about, stackvet.toml, the code, what that means) and `held_to`'s level sentence
  (`bluf.rs:257-292`), which already says "check them, and confirm them with `sv review`".
- What is wrong: the one line a non-programmer might read in the file asks them to read the file. The two answers
  that set the level are confirmed in `sv review` (`confirm_scope`), and the rest are shown against the code in
  the report; the line should send them there.
- What to do: ``# Written by your AI coding tool. The report's "What the app says about itself" shows each answer
  here beside what the code shows, and `sv review` asks you to confirm the two that set the level.``
- How verified: read.
- Overlap: 0401 (the tool says what it changes; the trial showed it rewrote these lines unasked), 0403 (a
  requirement's reason for not applying). Neither changes this line.
- Needs a decision record?: no.
