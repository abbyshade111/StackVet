# The person's last step, `sv review`, is the only step without a prompt that walks them to it

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `docs/prompts/setup.md:38-45` (the table ends at prompt 6) and 286-290 (`sv review` in prose only);
  `crates/sv-cli/src/review.rs:29-57` (what `sv review` says when nothing is waiting).
- What is wrong: prompts 1 to 6 have the tool do or explain each step; the step that turns the tool's word into
  the owner's has no prompt, and it is the one the person is most likely to skip, because nothing tells them it
  is owed (F1) and the only description is the guide's longest paragraph. The tool cannot run `sv review`
  (terminal only, by design), but it can do everything around it: read `stackvet.toml` and `security-notes.md`,
  list what is waiting in plain words (each answer, each proposed false alarm), say what each answer of
  `sv review`'s means, print the one command, wait, and then run the check again so the person sees the statuses
  move.
- What to do: a prompt 7, "Record my answers as mine", in the setup page's table and marked untried like the rest:
  ``Read stackvet.toml and security-notes.md and tell me, one at a time and in plain words, each answer or finding
  that is still only your word or your proposal, and what I should look at before I say it is mine. Then tell me
  the one command to type in my terminal (`sv review .`, or `stackvet review .` in PowerShell), say that you
  cannot run it for me, and wait. When I say I am done, run stackvet_check again and tell me what changed.``
  Must not lose: the prompt never has the tool write `by = "owner"` or a `Written by: owner` line.
- How verified: read.
- Overlap: 0398 (done: prompt 6), 0217 part 5 (the report prompt), 0401 (what changed). New.
- Needs a decision record?: no.
