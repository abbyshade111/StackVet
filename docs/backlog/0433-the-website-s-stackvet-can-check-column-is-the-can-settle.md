# The website's "StackVet can check" column is the "can settle" number, which counts 40 requirements it can only ever fail

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: coverage accuracy. Severity: medium. Verified against the code by the reviewing session before filing. Sibling of the in-part census item from the requirements review: once that gate runs, the "can be credited" count itself moves, so choose the column after it.

- Where: `docs/website/home-page.md:108-111` (the table: "ASVS 5.0, levels 1 and 2 | 253 | 153", "AISVS 1.0 | 191 |
  46", column head "StackVet can check"); `:130` ("The counts are `docs/COVERAGE.md`'s 'can settle' on 12 October
  2026 ... read from that file when the site is built"); `docs/COVERAGE.md:41-45` (the level table) and `:47` ("52 of
  the 174 ... can only ever be marked *needs attention*").
- What is wrong: 153 is 58 + 95, the level 1 and 2 "Can settle" counts. Of those, 14 + 26 = 40 can only ever be
  found failing: a check can show the control missing and finding nothing credits nothing. The number a reader of
  "can check" expects, the one that can ever come back *checked*, is 44 + 69 = 113 of 253. For AISVS the column says 46
  where 27 can be credited, and it counts all three AISVS levels where the ASVS row counts two. The rule that makes
  153 honest (that "settle" includes "fail only") is in `COVERAGE.md`, not on the page; the page's only gloss is in a
  build note the visitor never sees. The site is to read the number mechanically at build time (0376), so the label
  and the number it reads are decided once and then repeat. Item 0224 chose the level-1-and-2 basis because "174 of
  345 read as worrying"; it did not discuss finding-only, and 0006.5 built the "can be credited" column after
  COVERAGE.md was the source of the paper's figure, leaving the figure as is.
- What to do: either read "Can be credited" (113 of 253; 27 of 191) under a head such as "StackVet can confirm or
  fail", or keep 153 under a head that says what it is ("StackVet has a check for") with one sentence beside the table:
  "for 40 of these the check can only show the control missing; a clean run says nothing". Count AISVS on the same
  level basis as ASVS, or say that it is not. Keep the note at `:130` true to whichever is chosen, so the build reads
  the right column.
- How verified: read (`coverage.py`'s `credited()` at `tools/coverage.py:1043-1049` and the level table), and ran the
  script's own functions to recount: L1 44, L2 69, L3 9 can be credited.
- Overlap: 0224 (done; chose the basis, not the column), 0006.5 (done; built the column in COVERAGE.md, noted the
  paper's `figure-security.html` still quotes "can settle" at an older commit, 163 of 345), 0376 (open; builds the
  site from these words), 0372 (open; plain-language requirements, not the counts). New here: the column label, and
  that the AISVS basis differs from the ASVS one.
- Needs a decision record?: no; it is the owner's choice of which generated number the public page shows, recorded
  on 0376's item (or 0224's) as the choice is made.
