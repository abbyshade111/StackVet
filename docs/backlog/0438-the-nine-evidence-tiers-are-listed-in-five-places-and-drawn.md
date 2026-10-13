# The nine evidence tiers are listed in five places and drawn in none, and no list says who can grant each

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

- Where: `README.md:147-150` (one sentence naming all nine), and the tiers explained one at a time across
  `README.md:123-126, 184-186, 221-226, 257-260`; `docs/GETTING-STARTED.md:334-347` (a list of nine with a line each);
  `docs/ARCHITECTURE.md:69-87` (a table, the best of the five); `docs/website/home-page.md:85-94` (five example
  lines); `docs/paper/HOW-SV-WORKS.md:48-53`.
- What is wrong: The tiers are the one idea a reader must hold to read any report, and "who produces this tier" is
  the question that explains why `sv review` exists, why the AI tool's word ranks last, and why "checked" is not a
  pass. That mapping is in nobody's list: the README's `sv review` section says it in 859 words of prose, and the
  guide's section 5 says it again in 789.
- What to do: One layered diagram, nine rows stacked strongest at the top, each row carrying its word (never color
  alone, per `docs/brand/README.md:61`) and, in a second column, **who grants it**: a check of `sv`'s own (needs
  attention, checked, checked in part); the app's own tests, run inside the container (tested by the app's own
  tests); the owner at their own terminal through `sv review` (documented, checked by hand, attested); the AI coding
  tool, or an unrecorded owner (stated by the AI coding tool); nobody (not verified). A line across the diagram under
  "checked in part": "nothing below this line ever becomes checked, settles a threat, or leaves the list of tests to
  write" (ARCHITECTURE:87). Beside it, outside the ladder, the two boxes that are not tiers: *not assessed* and *does
  not apply*. Home: the reports page (0413), linked from the README, the guide, and the glossary; the README's
  sentence at 147-150 and the guide's list at 334-347 become the diagram and three sentences.
- How verified: read.
- Overlap: 0413 (claimed) plans the file table and two lifecycle diagrams, not the tier ladder; 0372 (open) is the
  plain-language line per requirement in the report itself. Build on 0413: a third diagram for its page.
- Needs a decision record?: no; ADR-022, ADR-050, ADR-053 define the tiers and are unchanged.
