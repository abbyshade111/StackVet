# Let CodeQL skip a pull request of backlog files only, if main's code-scanning rule allows it

**Status:** done, 12 October 2026

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

**Looked into on 12 October 2026 (session backlog-org-0275). The answer: not without the ruleset changed, so this item
stops here, as it says.**

- **The rule.** "Main - ADR Check" requires the checks `Decision records` and `test`, and a code-scanning rule: CodeQL,
  blocking at errors and warnings, or security alerts high or higher. GitHub's documentation of the rule
  ("Available rules for rulesets", "Require code scanning results") names two cases that block a merge, a required
  tool's analysis still in progress and a required tool not configured for the repository, and says nothing of a pull
  request whose analysis was skipped. So it was tried.
- **The trial.** Pull request #1543, never merged: `main` with CodeQL's analysis job switched off (`if: false`). Its
  `test` and `Decision records` both passed (`test` after one re-run of a failure unrelated to CodeQL), no CodeQL
  result check appeared, and GitHub's merge state stayed `BLOCKED` while the change itself was `MERGEABLE`. Every
  other rule was met, so the code-scanning rule is what held it. Closed unmerged.
- **What would work, and is the owner's:** a ruleset change. the code-scanning rule's own parameters, as the API gives them,
  are the tool and its two thresholds, with nothing about which files a pull request changes, so the change would be
  to drop the code-scanning rule from "Main - ADR Check" and rely on CodeQL's run on every push to `main` and every week, which would still find a fault, a merge later.
- **Not done, on purpose:** uploading an empty CodeQL result for a backlog-only pull request would satisfy the rule, but
  it would say an analysis ran that did not; that is not honest evidence, so it is not offered.
- **Seen on the way:** the trial's first `test` run failed on
  `docker::container_record_tests::a_teardown_names_what_it_could_not_remove_and_not_what_was_already_gone`, which
  expected a container named `app1` and was told of `sv-1-1-test-app`; the re-run passed, and `main`'s runs of the
  same hour passed. Filed as an item of its own.
