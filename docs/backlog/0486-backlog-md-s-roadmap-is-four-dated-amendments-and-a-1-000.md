# BACKLOG.md's roadmap is four dated amendments and a 1,000-word status snapshot, where a table from the status lines would do

**Status:** open

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
