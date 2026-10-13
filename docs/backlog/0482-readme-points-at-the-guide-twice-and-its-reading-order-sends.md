# README points at the guide twice, and its "Reading order" sends every reader to the 240,000-word design record

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `README.md:12-14` and `:375-380` (the section "Building an app from scratch with `sv` alongside" is only
  the same pointer again, 56 words); `:625-628` ("## Reading order": the design record, 438 entries, "explains the two
  changes from v1").
- What is wrong: The second pointer is the whole of a section. The reading order names one place, `docs/design/`,
  which no reader reads end to end (`docs/DESIGN.md:3` says itself "this is the dated record, not the map"), and
  names neither `docs/ARCHITECTURE.md` (the map, which `CLAUDE.md` and `docs/DESIGN.md` both say to read first) nor
  the glossary nor the reports page.
- What to do: Delete `:375-380`. Make "Reading order" a four-row table by reader: not a programmer → the guide, the
  glossary, the reports page; a programmer using `sv` → this page, the exit-code table, the MCP table; a contributor →
  `docs/ARCHITECTURE.md`, `CONTRIBUTING.md`, `docs/adr/README.md`, `docs/BACKLOG.md`; a researcher → `docs/paper/`,
  `docs/design/`. That table is also the index proposed in F11.
- How verified: read.
- Overlap: none found.
- Needs a decision record?: no.
