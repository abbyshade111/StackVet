# BACKLOG.md's roadmap is four dated amendments and a 1,000-word status snapshot, where a table from the status lines would do

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `docs/BACKLOG.md:31-115` (the roadmap: phases 0 to 5, then "After the roadmap", "The owner's order of
  9 October", "Added 12 October", "Process"); `:117-166` ("Status checked against the code, 11 October 2026", about
  1,000 words saying phase 1's three items are closed and phase 4's parts are in various states).
- What is wrong: The instruction at `:36-37` ("takes the first unclaimed sub-item below, in this order") sends a session
  through phases 1 to 3 whose items are, by the file's own later section, closed; then through three paragraphs that
  reorder them. `tools/backlog.py` reads every item's status line already, so the file is asking a reader to do by
  hand what the tool does.
- What to do: The roadmap as a table (phase | item number and title | status, read by `backlog.py roadmap` from
  the status lines | the owner's date), regenerated like COVERAGE.md and held by `backlog_board.rs`; the dated
  amendments folded into the order with their dates kept in a short "Changes to the order" list; the 11 October
  snapshot deleted once the table shows the same. Saves about 1,500 of 2,679 words. Must not lose: the rule that a
  claim goes on the item, never here; the dates of the owner's orders.
- How verified: read.
- Overlap: 0299 (done) fixed one clause of this section; 0416 (open) is the board. None for the roadmap's form. ADR-061
  governs the file.
- Needs a decision record?: no new decision; ADR-061 gets an "unchanged, because" line (the layout rule stands;
  the roadmap's form changes).

**What was built (14 October 2026, session documentation).** The roadmap in `docs/BACKLOG.md` is a table between
`<!-- roadmap:start -->` and `<!-- roadmap:end -->`, one row per item in the old order (order, phase, item, status,
asked for). `python3 tools/backlog.py roadmap` compares each row's status with the item's own status line, and
`roadmap --write` rewrites a stale row; `claim` and `done` refresh it when they change an item named there, and
`--check` fails while a row is stale. The four dated amendments became a short "Changes to the order" list with their
dates. The rule that a claim and a done note go on the item, never here, stays in the section's opening. Held by
`crates/sv-cli/tests/backlog_roadmap.rs` and by `backlog_board.rs` through `--check`; with one status cell set to
`open` by hand, a test in each went red. The file went from 2,678 to about 2,340 words, short of the item's 1,500-word
saving, because the table keeps one row per item. Not deleted: the 11 October status snapshot moved word for word to
the design entry `docs/design/0385-the-roadmap-s-status-checked-against-the-code-11-october.md`, since deleting it
was not mine to do; the phases' own longer wording is in this file's history.
