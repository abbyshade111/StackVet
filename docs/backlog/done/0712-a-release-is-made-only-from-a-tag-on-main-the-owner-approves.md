# A release is made only from a tag on main that the owner approves

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A2). Title and text
kept to the fix until it is built, as the review asked.

- **To build:** `.github/workflows/release.yml` checks that the tag's commit is on `main` before it builds or signs
  anything, and stops naming the commit when it is not. `.github/rulesets.json` records a tag ruleset on
  `refs/tags/v*`, so `tools/rulesets_check.py` holds it. ADR-108 or the release's own record gains a Later entry.
- **The owner's:** that tag ruleset on GitHub, which only the owner can pass, and the owner as required reviewer on
  the `release` Environment.

**Done, 21 November 2026:** the release job's first step stops before signing unless `main` holds the tagged commit
(`crates/sv-cli/tests/release_from_main.rs`); `.github/rulesets.json` records the "Release tags" ruleset; ADR-108 and
ADR-158, Later; 0422's done note corrected. **The owner's, made on 21 November 2026:** the "Release tags" ruleset on
`refs/tags/v*` (creating, moving, or deleting one only for a repository administrator), compared clean by
`tools/rulesets_check.py --gh` the same day; and the owner as required reviewer on the `release` Environment, read back
with its `v*` tag rule kept.
