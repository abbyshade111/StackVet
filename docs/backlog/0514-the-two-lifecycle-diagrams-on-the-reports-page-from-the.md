# The two lifecycle diagrams on the reports page, from the owner's drawings of 13 October

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Split from backlog 0413 (14 October 2026), which built the rest of the reports page (`docs/website/reports-page.md`).
0413 asked for two diagrams on the page, drawn by the owner on 13 October 2026 in the website palette and kept for
now outside the repository, on the owner's USB drive (`whitepaper_claude/diagrams`: an SVG and a PNG of each, and
`make_diagrams.py`, which draws them). A cloud session cannot reach that drive, so the diagrams wait on the owner's
files rather than being drawn a second time:

- **The software development lifecycle:** OWASP's six phases as a ring (Planning, Analysis, Design,
  Implementation, Testing & Integration, Maintenance), with "SDLC" in the middle.
- **Where Secure by Design, ASVS, and AISVS touch the lifecycle:** each framework against the six phases, each cell
  tagged **Main** or **In part** (the owner's two words), with the sections it rests on.

To build, once the files are in the repository (the owner copies `make_diagrams.py` and its output into
`docs/website/diagrams/source/` and `docs/website/diagrams/`, or says a session may draw them again from 0413's
description):
- Check each cell of the second diagram against the frameworks' own text in `data/frameworks` before the page is
  settled, since a citation is a claim.
- `docs/brand/README.md` gains the two tags and what each means, so later diagrams use the same words.
- The reports page's "Not yet on this page" loses its line for them.

- How verified: not built; the files are not in the repository.
- Needs a decision record?: no.
