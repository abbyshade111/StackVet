# The report broken into short pages by what a person does with each, so things stop getting lost in one long file

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's ask, 15 October 2026, after reading the Opus run's report (backlog 0577): the report is so long that
things get lost; break it out. Severity: high (usability; the report is the product). A sibling of 0589 (the
pre-deployment checklist), which would be one of the pages.

**What exists.** On a small level-1 app the report is three long files: compliance.md 789 lines and 22,000 words in
16 sections, security.md 801 lines, report.html 905 lines, plus low-findings.md. The sections run from "The short
version" and "5 things to fix" through the requirement tables, threats, what only the person can check, before going
live, tests worth writing, findings about requirements not assessed against, checks that found nothing, controls above
the level, what the app says about itself, and an appendix of every requirement's text. Everything is there, and a
person who is not a programmer cannot find the one thing they need.

**What to build.** One file per purpose, each short enough to read whole, with report.html as the front page that
leads to them and says in one line what each is for:
- *Fix now*: the findings that need attention, worst first, each with the line and the fix (today's "things to fix").
- *Before going live*: the checklist (0589).
- *Your decisions*: the questions only the person can answer, with the pointer's lines beside each.
- *What was verified, and how*: the requirement tables, the threats, and the evidence, for the person who wants proof
  (today's compliance.md, kept as the long reference and named as such).
- *What was not examined*: the gaps, the checks that could not run, the settings loosened for the run, in one place.
- *Information*: test-code hits, findings about requirements the app is not held to, checks that found nothing, the
  AI-process section, and the appendix of requirement texts.
Each page carries the same header (the app, the sv commit, the date, the seal) and a one-line "this page credits
nothing on its own" where that is true. The counts and the honesty lines move with their sections, never dropped.
report.json is unchanged, so the MCP tools and the dashboard keep reading what they read.

**Questions for the owner.** Whether security.md (the version for the person who shares the report with somebody who
knows security) stays one file or splits the same way; and which page `sv report` prints the path of at the end
(recommended: the front page, then "Fix now" when it has anything).

**Record.** ADR-0NN Later on the record that names the report's files (the one `docs/ARCHITECTURE.md`'s reports page
cites), since what sv writes into the person's folder changes; 0589's record covers the checklist page.
