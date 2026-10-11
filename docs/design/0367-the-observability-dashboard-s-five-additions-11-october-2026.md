# The observability dashboard's five additions (11 October 2026)

Backlog 0227 asks for an observability page. The owner was shown a mock-up on 11 October 2026 and chose five things
(ADR-086): extend `sv dashboard` rather than add a page; keep every run on this computer; show requirement changes since
the last run, with a drill-down to each requirement's full row; show the build loop's prompts and reports by name and
size only; and make the backlog overview a page of its own.

## What already exists, and what each addition adds

- **Every run kept, and the requirement changes.** `sv dashboard` already reads the history kept under
  `~/.local/share/stackvet/history/` and shows the changes between runs ("Over time"). The part to check is that it lists
  every run and that each changed requirement links to its full row. Built as its own part, with a test for the link.
- **The build loop, by name and size.** Not in the dashboard yet. Its records are written by `sv`'s build loop
  (ADR-084). This part adds a section listing each prompt and report file by name, its size, and the findings fixed,
  set aside, and new. No contents are shown.
- **The backlog overview, as its own page.** Read from each item's status line and its parts' status lines (backlog 0228),
  with who holds what. Read-only; linked from the dashboard.

## Not decided here

- The sv health section (crashes, data files that did not load, records that could not be read) is in 0227's first list
  but not among the owner's five answers. It stays open in 0227 until the owner decides.
- The mock-up's sample numbers are not real and are not built from anything.

The mock-up is the one sent on 11 October 2026 (`dashboard-mockup.html`). The decisions are ADR-086.
