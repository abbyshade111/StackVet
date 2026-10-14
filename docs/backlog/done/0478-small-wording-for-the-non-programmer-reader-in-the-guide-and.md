# Small wording for the non-programmer reader in the guide and the glossary, and the glossary's "Report" entry that 0413 misses

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: low. Verified against the code by the reviewing session before filing. The documentation review found the same glossary entry (its F14): the entry names three of the eight files and calls `compliance.md` the one to read; 0413 corrects this everywhere but there. Worth lines in the glossary too, since the guide uses them unexplained: "seal", "needs attention", "checked in part", "the fence".

- Where: `docs/GETTING-STARTED.md:349-356` (the CI exit codes, in the non-programmer's guide; the README has them
  at 57-79); 635-639 ("Known problems while this is new": two fixed on 27 September, "Nothing from that build is
  known to be open"); `docs/GLOSSARY.md:107-110` (Report: names three of the eight files and calls
  `compliance.md` the one "to read"); `docs/GETTING-STARTED.md:328-332` and README 138-139 ("the same in
  Markdown"; 0413 has this); `docs/GETTING-STARTED.md:104-108` (the VS Code note still says the tool "asked every
  question from `stackvet_questions`", a name retired on 9 October; true as history, confusing beside "Did it
  connect?" which names `stackvet_status` and `stackvet_check`).
- What is wrong: each is a sentence written for someone else or for an earlier day, met by this reader on their
  path. None misleads about evidence.
- What to do: move the exit-code paragraph to a one-line pointer at the README; drop or date the "Known problems"
  section; the glossary's Report entry points at 0413's table once it exists and names `report.html` first; the
  VS Code note says "(then called `stackvet_questions`, now `stackvet_check`'s section `questions`)".
- How verified: read.
- Overlap: 0325 (open: seven small corrections from 0277; these are not among them), 0413 (claimed: the file
  table). Could be added to 0325 rather than filed.
- Needs a decision record?: no.

**Done 14 October 2026 (session documentation).** The guide's exit-code paragraph is now one sentence pointing at the
table in `docs/ARCHITECTURE.md` (0479 made that the one table). Its "Known problems while this is new" section, true
of a build in September and listing nothing open, is gone. The glossary's Report entry names `report.html` first,
`compliance.md` and `security.md` as the same in plain text, and points at the README's table of the eight files.
The glossary gains "Needs attention", "Checked in part", "The fence", and "Seal". Already done before this: the VS
Code note's `stackvet_questions` line (it gives the new name), and "the same in Markdown", which 0413 holds.
`crates/sv-cli/tests/glossary_report_words.rs` holds each.
