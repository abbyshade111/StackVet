# A page for each framework on stackvet.dev, built from the data (20 October 2026)

**Status:** done, 20 October 2026

The owner asked, on 20 October 2026, for a page on stackvet.dev for each framework, built from the facts first put
into the requirement spreadsheets that day. No backlog item planned one: the site's build (0376) covers the home page,
the credits, and two more pages. The owner chose to begin with a generator and a local preview (ADR-129, proposed
with this claim), leaving where the site is hosted for later.

1. **A generator in `tools/`,** writing one page each for ASVS 5.0, AISVS 1.0 with its Appendix C, and the Secure by
   Design checklist 0.5.0, from `data/` and `tools/coverage.py`'s classification, into `--out` (by default
   `exports/site/`). Every requirement under its family, with a filter by level, an anchor of its own, the standard's
   words and StackVet's, the coverage, each check and what it needs to run, what a person is asked and how, and when
   it applies. Direction C's palette and type; one column at phone width; it fetches nothing.
   **Part status:** done, 20 October 2026
2. **A test** that runs the generator's self-test, holds the pages' counts to `docs/COVERAGE.md`'s, and fails when a
   requirement in `data/frameworks/` has no entry on its page.
   **Part status:** done, 20 October 2026
3. **ADR-129 accepted,** with its Governs list naming the generator and the test.
   **Part status:** done, 20 October 2026

Not here: hosting, publishing, and a page per requirement (the anchors serve links for now); each is the owner's to
ask for.

**Built, 20 October 2026.** `tools/framework_pages.py` writes `index.html`, `asvs.html` (345 requirements),
`aisvs.html` (259, AISVS and its Appendix C), and `secure-by-design.html` (36) into `exports/site/`, from `data/` and
`tools/coverage.py`'s classification, in direction C, with a level filter and an anchor for each requirement.
`crates/sv-cli/tests/framework_pages.rs` holds every requirement to its page, each page's counts to
`docs/COVERAGE.md`, and the pages to fetching nothing. Seen in a browser at desktop and phone width: the filter shows
exactly a level's requirements, nothing scrolls sideways at 375 px, and its buttons are 44 px high. ADR-129 is
accepted.
