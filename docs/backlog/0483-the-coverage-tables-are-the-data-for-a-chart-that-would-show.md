# The coverage tables are the data for a chart that would show in one glance what forty rows say, and the website will need it

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `docs/COVERAGE.md:34-41` (the summary: three frameworks × can settle / can be credited / supporting /
  nothing), `:43-50` (ASVS by level × kind of check), `:65-85` (ASVS by chapter, 17 rows), `:316-` (AISVS by chapter),
  `:392-` (Secure by Design by domain); `docs/website/home-page.md:108-112` (four numbers from the same file, read at
  build time).
- What is wrong: The by-chapter table is the honest picture of where `sv` can speak and where it cannot (V17 WebRTC
  0 of 12; V10 OAuth 8 of 36; V3 Web Frontend 23 of 31), and it is the thing a visitor most wants to see before
  trusting the tool. Forty rows of five integers do not show it; a bar per chapter would.
- What to do: Have `tools/coverage.py` also write `docs/coverage-by-chapter.svg` (and `-by-level.svg`): one stacked
  horizontal bar per chapter, segments *can be credited* / *can only be found failing* / *supporting only* /
  *nothing*, each segment labeled with its count (never color alone), in the brand palette, with the "nothing" segment
  drawn in the page-background gray so the picture is honest about how much is unreached (DASHBOARD:23-25 states this
  rule for the report; it applies here). Generated with the tables, so `coverage_doc.rs` holds it current and it cannot
  drift. Link it from `COVERAGE.md`'s head, the README's one coverage sentence (`:25-28`), and the website's section
  5. Two more charts worth the same treatment, later: the tier counts of one example app's report (from
  `report.json`'s `counts`, as DASHBOARD's proposal A describes: "mostly one color, and it is the color of nobody has
  looked"), for the reports page; and the loop trial's findings by group, which `docs/prompts/loop-scale/` already
  draws as a PNG with `vuln_chart.py`. Not proposed here: a chart inside `report.html` (that changes what a report
  shows, and DASHBOARD's ADR-057 covers it).
- How verified: read.
- Overlap: 0376 (open, the website) reads the four numbers from COVERAGE.md; `docs/DASHBOARD.md` proposes charts in
  the report and the dashboard (ADR-057), not in the documents. None for a generated chart in `docs/`.
- Needs a decision record?: no for the SVG in `docs/` (a generated document, no change to what `sv` does). The
  "Decision records" check will ask about `tools/coverage.py` if a record governs it; say "unchanged, because" on it.
