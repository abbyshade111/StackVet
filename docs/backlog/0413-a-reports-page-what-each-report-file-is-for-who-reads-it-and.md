# A reports page: what each report file is for, who reads it, and an example of each

**Status:** claimed by documentation, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's request of 13 October 2026. Looking at the my-app report, the owner found that `security.md` and
`compliance.md` "often look very similar if not the same". They are not the same: `security.md` is the findings in
full (where, the code, how sure, why it matters, what to do, how to set one aside) and what was not examined, for
whoever fixes the code; `compliance.md` is every requirement that applies, by chapter, with what is known of each,
the threats, what only the owner can check, and the tests worth writing, for whoever needs the whole picture against
the standards. On a small app they look alike because they open the same way: the same first lines, the same
paragraph on whether `sv` was used during the build, and the same "What was not examined", word for word. The owner:
"we should make note in GitHub and on the website what the difference is and the intended audience. Maybe a reports
page on the website that says the intended audience and shows an example of each type of report."

The docs say otherwise today: the README calls `compliance.md` and `security.md` "the same in Markdown", and the
getting-started guide says they "say the same in plain text for your AI tool". Both are wrong.

To build, after 0412 (the PDF), so the page can show the PDF too:
- **README and `docs/GETTING-STARTED.md`:** a short table of the report folder's files, each with what it holds and who
  it is for (a person reading, the person or AI tool fixing the code, an editor or GitHub, `sv` itself on the next
  run), in place of "the same". The report folder's own `report.html` says the same in its list of files.
- **A reports page for the website,** `docs/website/reports-page.md`, in the home page's style (words first, settled
  by the owner, nothing live): each file, its audience, and when to open it, with a link to an example of each.
- **Example reports** from one of the example apps in `examples/` (one with findings and several statuses), made with
  the released `sv` and kept with the page, with the command that made them and the date, so they can be made again.
  Never the health-tracking app. The home page's section 4 links the page, and its invented example lines can point
  at the real examples.
- **A test** that the README's table names every file `sv report` writes, so a new file cannot be left out (break:
  drop a row and see it fail).
- **Two diagrams on the page,** added at the owner's request the same day. Both were drawn on 13 October in the
  website palette (`docs/brand/README.md`) and are kept for now outside the repository, on the owner's USB drive
  (`whitepaper_claude/diagrams`: an SVG and a PNG of each, and `make_diagrams.py`, which draws them):
  - **The software development lifecycle:** OWASP's six phases as a ring (Planning, Analysis, Design,
    Implementation, Testing & Integration, Maintenance), after the diagram on OWASP's Integration Standards page,
    with "SDLC" in the middle.
  - **Where Secure by Design, ASVS, and AISVS touch the lifecycle:** each framework against the six phases, each
    cell tagged **Main** (a phase the framework is chiefly for) or **In part** (one it reaches into for part of its
    work), with the sections it rests on. The owner chose those two words. The cells were placed from the
    frameworks' own text in `data/frameworks`; check each again against it before the page is settled, since a
    citation is a claim.

  They go in `docs/website/diagrams/`, with the script under `docs/website/diagrams/source/` as the logo's is kept
  (`docs/brand/source/`), and `docs/brand/README.md` gains the two tags and what each means, so later diagrams
  use the same words.

**Claim released, 14 October 2026** (the owner's word): session securevibe-trial-prep was archived after the coordination trial with no branch pushed for this item, so the claim was stale. Open again, queued on the board.
