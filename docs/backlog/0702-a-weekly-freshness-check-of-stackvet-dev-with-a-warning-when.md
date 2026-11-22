# A weekly freshness check of stackvet.dev, with a warning when a report shown on it is old

**Status:** claimed by Claude Code session (packaging and ConfigVet), 21 November 2026

Filed 21 November 2026, at the owner's word, from the whole-site review (backlog 0696). Two reports shown on the site
were weeks old and a version behind (the self-assessment of 26 October and the example of 14 October, both by
`sv` 0.1.0), and stale sentences sat on the site for weeks because the site is only built when someone changes it.

- **What to build:** (1) `tools/website.py` prints a warning, never a failure, when a report the site shows is more
  than 30 days old, or was made by an `sv` older than `Cargo.toml`'s version; and (2) a scheduled weekly GitHub
  workflow that builds the site with `--self-test` and runs `crates/sv-cli/tests/website.rs`, opening or updating one
  issue when anything fails or warns. It deploys nothing.
- **Needs the owner first:** a new workflow is a change to what runs in the repository, so it needs a decision record
  (ADR-132, Later, or a new one) and the owner's word on the schedule. Actions minutes are free for a public
  repository; check that before building.
- **Done when:** a run against a branch where the example report's date is moved back 31 days warns, and a broken
  page makes the scheduled run open an issue.

**The owner's word, 22 November 2026:** yes, weekly on Mondays, beside the other weekly checks (the tap's install, long files, and the rulesets comparison). Actions minutes are free for a public repository.
