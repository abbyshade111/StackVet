# The three repositories take merge commits only

**Status:** claimed by Claude Code session (packaging and ConfigVet), 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A6, its second point).
CLAUDE.md says a pull request merges with a merge commit, never squash or rebase, and `main`'s history is cited by
commit hash. The "Main - ADR Check" ruleset still allows all three merge methods, and the three repositories' own
settings offer all three; the bucket's and the tap's rulesets already allow merge commits only.

- **The owner's, in the repository settings:** squash and rebase merging turned off on StackVet, the Scoop bucket, and
  the Homebrew tap, and the "Main - ADR Check" ruleset's `allowed_merge_methods` set to `merge` alone. The build's
  pull request gives the exact `gh` commands.
- **To build:** `.github/rulesets.json` says the same; `tools/rulesets_check.py` also compares each repository's
  three merge settings, listed as not read when GitHub does not show them, never as matching; ADR-051 and ADR-108 gain
  Later entries. Nothing in the repository merges with squash or rebase (looked for on 21 November 2026).
