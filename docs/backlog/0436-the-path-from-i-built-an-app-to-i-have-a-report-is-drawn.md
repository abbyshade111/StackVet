# The path from "I built an app" to "I have a report" is drawn nowhere a person reads, though the paper's figure already draws it

**Status:** claimed by documentation, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

- Where: `README.md:8-14` and `docs/GETTING-STARTED.md:8-21` (prose only); `docs/paper/figure-how-sv-works.html`
  (13.5 KB, two columns and arrows, ten command boxes) is linked from no file outside `docs/paper/`
  (`grep -rn figure-how-sv-works README.md docs/*.md` finds nothing).
- What is wrong: Both first pages explain in words that the person builds in their own AI tool, the tool writes
  `stackvet.toml`, `sv` reads the code and (optionally) runs the app, and writes a report. A reader who is not a
  programmer has to hold five stages and three actors (the person, the AI tool, `sv`) in their head from prose. The
  one drawing of exactly this, made for the paper, is a day old relative to the cut-off it names (4 October) and not
  reused.
- What to do: One diagram near the top of both the README and the guide, a left-to-right flow with three swim lanes:
  **the person** (describes the app; answers the questions; runs `sv review` and `sv report --run` at a terminal),
  **the AI coding tool** (writes `stackvet.toml`, writes the code, calls `stackvet_check` after each feature, asks the
  questions, writes `security-notes.md` as its own word), and **`sv`** (works out what applies; reads the code; starts
  the app behind the fence; writes the report folder). Arrows cross lanes at the four hand-offs (spec, check, questions,
  report). Redraw it from `figure-how-sv-works.html` in the website palette (`docs/brand/README.md`) as an SVG under
  `docs/website/diagrams/` where 0413 puts its two, with the script beside it. The words the figure saves are small
  (perhaps 150 each in README and the guide); what it buys is that the rest of each page reads as detail on a picture
  the reader already has.
- How verified: read; the figure's file inspected for its structure (ten `class="cmd"` boxes, two columns, two arrows).
- Overlap: 0413 (claimed) plans two diagrams for the reports page, the OWASP lifecycle ring and the frameworks against
  it, not the run's flow; 0373 (open) is a video walk-through of a setup, not a diagram; 0376 (website) would show
  whatever exists. This is a third diagram, to be filed beside 0413's two so they share one folder and one palette.
- Needs a decision record?: no.
