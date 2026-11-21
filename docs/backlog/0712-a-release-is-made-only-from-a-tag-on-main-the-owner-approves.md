# A release is made only from a tag on main that the owner approves

**Status:** partly done: the owner's two settings: the Release tags ruleset and the reviewer on the release Environment, commands given on 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A2). Title and text
kept to the fix until it is built, as the review asked.

- **To build:** `.github/workflows/release.yml` checks that the tag's commit is on `main` before it builds or signs
  anything, and stops naming the commit when it is not. `.github/rulesets.json` records a tag ruleset on
  `refs/tags/v*`, so `tools/rulesets_check.py` holds it. ADR-108 or the release's own record gains a Later entry.
- **The owner's:** that tag ruleset on GitHub, which only the owner can pass, and the owner as required reviewer on
  the `release` Environment.
