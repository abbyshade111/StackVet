# Let CodeQL skip a pull request of backlog files only, if main's code-scanning rule allows it

**Status:** open

Filed on 12 October 2026 at the owner's request, after backlog 0272's measurement. With the short path of ADR-104, a
pull request of backlog files only merged in 5 minutes 41 seconds (#1534). Its `test` check took 2 minutes 22 seconds.
CodeQL's Rust analysis took 5 minutes 22 seconds, and it ran on each of that day's pull requests for 6 to 6½ minutes,
whatever they changed. Backlog items are Markdown, which CodeQL does not analyze, so the Rust analysis of such a pull
request reads the same code `main` already has.

**Look into first, before changing anything:**

- Whether `main`'s ruleset ("Main - ADR Check") accepts a pull request that CodeQL skipped. It requires code-scanning
  results as well as the two checks. How GitHub treats a pull request with no new analysis, when its workflow is
  skipped by a path filter or by a step inside the job, decides which way, if any, is open. Read GitHub's own
  documentation for the code-scanning merge rule, and try it on a branch, not on `main`.
- How `.github/workflows/codeql.yml` is set up today (three languages, build mode "none", its `paths-ignore`), and
  whether the same fail-safe pattern as ADR-104 fits. `tools/ci_scope.py` decides, and a missing answer runs
  everything.
- What is lost. Every push to `main`, and the weekly run, still analyze everything, so a fault in the code is still
  found there. The analysis of a backlog file would have found nothing, because CodeQL does not read Markdown.

**Not decided:** whether to do it. The ruleset is a repository setting, and any change to it is the owner's. If the
only way needs the ruleset changed, this item says so and stops. A change to the workflow alone needs a decision
record, or a "Later" entry on ADR-104.
