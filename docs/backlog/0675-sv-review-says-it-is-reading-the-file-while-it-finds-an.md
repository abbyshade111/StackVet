# sv review says it is reading the file while it finds an entry's line, so nobody types ahead into a silent prompt

**Status:** open

**Asked for by the owner, 26 October 2026:** "we should probably add a note or something so the user knows to wait
until the prompt comes up", after recording a `[[finding-review]]` entry for `data/reach.json` twice that day.

**What happened.** After showing an entry's rule and file, `sv review` finds the line with the entry's fingerprint
(`sv_check::review::lines_with_fingerprint`, then `todays_form`, from `record_finding` in `crates/sv-cli/src/review.rs`)
before it prints the line, the reason, and the question "Type your name, or `owner`...". On `data/reach.json` (a large
generated file) with a debug build, that took about two minutes each time, with nothing on the screen. Both times the
owner asked why there was no prompt, and typed `owner` ahead of it; the terminal kept the word, and it was read as the
answer when the question came. That worked, but only by luck: had anything else been typed, it would have been read as
the answer too.

**What it needs.**

- **A line before the wait**, such as "Reading data/reach.json to find the line this entry is about...", and, when it
  is slow, a line saying it is still at it, so a person knows to wait for the question.
- **Input typed ahead is not taken as the answer.** What was typed before the question appeared is set aside (read and
  discarded, or the question asked again when it is not one of the answers it takes), so a word meant for something
  else cannot record a decision. Recording a decision is signed with the owner's key, so it should rest on an answer to
  the question shown.
- **Why it is slow**, measured: the time spent in `lines_with_fingerprint` and `todays_form` on a file of
  `data/reach.json`'s size, in a debug and a release build, and the fix if it reads the file more than once or does
  work per line that grows with the file.

**Done when.** A test drives `sv review` on an entry in a large file and sees the line before the wait; a test types
ahead of the question and sees it is not taken as the answer; and the wait is measured, and short, or said.
