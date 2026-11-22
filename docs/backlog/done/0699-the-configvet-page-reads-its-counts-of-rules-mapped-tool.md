# The ConfigVet page reads its counts of rules, mapped tool rules, and defaults from the data files

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, from the whole-site review (backlog 0696). The ConfigVet page types its
counts by hand: "four rules of its own", "23 mapped so far", and "six defaults". They were right on 21 November, and
nothing ties them to `crates/cv-rules`, `data/cv-adapters.json`, or `data/config-defaults.json`.

- **What to build:** as `with_counts` does for the home page's coverage numbers, `tools/website.py` puts the counts
  read from those sources in place of the ones written in `docs/website/configvet-page.md`, and stops if the words
  they sit in have changed. The rule count comes from the rule ids `cv` reports, not from a hand-kept list.
- **Done when:** adding one rule map entry to `data/cv-adapters.json` changes the built page's "23", and changing the
  sentence around a number stops the build with a message naming it.
- **Order,** after 0696.

**Done, 22 November 2026:** `tools/website.py` reads the three counts from their sources; one more mapped rule turned 23 into 24, and rewording the sentence stopped the build. ADR-132, Later, 22 November 2026.
