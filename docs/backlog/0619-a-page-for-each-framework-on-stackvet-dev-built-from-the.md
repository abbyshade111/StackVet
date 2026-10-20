# A page for each framework on stackvet.dev, built from the data (20 October 2026)

**Status:** claimed by claude/framework-pages, 20 October 2026

The owner asked, on 20 October 2026, for a page on stackvet.dev for each framework, built from the facts first put
into the requirement spreadsheets that day. No backlog item planned one: the site's build (0376) covers the home page,
the credits, and two more pages. The owner chose to begin with a generator and a local preview (ADR-129, proposed
with this claim), leaving where the site is hosted for later.

1. **A generator in `tools/`,** writing one page each for ASVS 5.0, AISVS 1.0 with its Appendix C, and the Secure by
   Design checklist 0.5.0, from `data/` and `tools/coverage.py`'s classification, into `--out` (by default
   `exports/site/`). Every requirement under its family, with a filter by level, an anchor of its own, the standard's
   words and StackVet's, the coverage, each check and what it needs to run, what a person is asked and how, and when
   it applies. Direction C's palette and type; one column at phone width; it fetches nothing.
   **Part status:** open
2. **A test** that runs the generator's self-test, holds the pages' counts to `docs/COVERAGE.md`'s, and fails when a
   requirement in `data/frameworks/` has no entry on its page.
   **Part status:** open
3. **ADR-129 accepted,** with its Governs list naming the generator and the test.
   **Part status:** open

Not here: hosting, publishing, and a page per requirement (the anchors serve links for now); each is the owner's to
ask for.
