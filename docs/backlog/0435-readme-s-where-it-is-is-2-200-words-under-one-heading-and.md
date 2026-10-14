# README's "Where it is" is 2,200 words under one heading, and the heading does not say what is under it

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

- Where: `README.md:22-168` ("## Where it is" ... the AI tool's own files, ADR-049); no `###` between.
- What is wrong: The section a programmer reads first is one block of fifteen paragraphs: the list of commands, the
  fifteen languages (lines 44-55), the exit codes (57-79, about 330 words in one paragraph), the container backend,
  the run record, `install = true`, the network rule and Semgrep (94-106), the anonymous questions, the app's own
  tests (115-132), the eight report files (134-145), the nine statuses (147-150), the tests worth writing, the threat
  model, and the AI tool's files. Nothing in it is about "where it is". A reader looking for the exit codes, or for what
  the report folder holds, has no heading to jump to.
- What to do: Give it sub-headings that name each topic ("What it reads", "What a run writes", "Exit codes", "Network"),
  turn three of the paragraphs into tables (the exit codes: command × code → meaning; the report files: file → what
  it holds → who opens it, which 0413 already plans; the languages: read / read as pages / present but unread), and
  move the 330-word exit-code paragraph to a reference page (F4), leaving two sentences and a link. Saves roughly
  600 of the 2,227 words in the README and makes the rest findable. Must not lose: the rule that a `stackvet.toml`
  folder exclusion never hides a vulnerability (76-79), and the "no code rule claims anything while an unread file is
  present" rule (46-48), both of which are the honest-evidence rule in its sharpest form.
- How verified: read; word counts by `awk` over the headings.
- Overlap: 0279 (done) fixed the file count in this section; 0325 (open) has one Oxford comma at line 57. Neither
  restructures. None found for the structure.
- Needs a decision record?: no; wording and layout only.
