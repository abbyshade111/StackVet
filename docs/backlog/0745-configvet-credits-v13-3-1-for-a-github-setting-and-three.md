# ConfigVet credits V13.3.1 for a GitHub setting, and three more citations to read again

**Status:** open

Found by the ninth weekly review of the decision records (0742, 23 November 2026), reading ADR-157's citations
against the requirement texts in `data/frameworks`. A citation is a claim (CLAUDE.md), so each of these wants reading
again by whoever takes this; none was changed by the review.

1. **V13.3.1 is credited, in part, when GitHub's secret scanning or push protection is on**
   (`crates/cv-live/src/judge.rs`, `SECRET_SCANNING` and `PUSH_PROTECTION`). The requirement asks that "a secrets
   management solution, such as a key vault, is used"; that secrets are not in source code is its second half.
   ADR-157's own part 3 says of the outside tools that a clean result "never credits it", since finding no secret
   does not show a vault is used. Part 4 credits it for a setting being on. The two parts disagree. A finding against
   V13.3.1 when the setting is off fits; a credit, even in part, probably does not.
   **Part status:** open
2. **V15.2.4 is cited for `sha_pinning_required`.** The requirement is about components coming "from the expected
   repository" and dependency confusion. Pinning an action to a commit is about the code not changing under its name,
   which is nearer than nothing but is not where it comes from; the `allowed_actions` setting fits better.
   **Part status:** open
3. **AC.8.3 is credited, in part, when a pull request is required, even when the ruleset's bypass list names
   someone;** the credit's words then say who may bypass. ADR-157's entry of 21 November says what a hidden list
   counts as (not assessed) but not what a list with someone on it counts as, and no test holds it. The requirement is
   that agents "cannot bypass branch protection". Also, where classic branch protection is what requires the review
   and there are no rulesets, the credit says no one may bypass the rulesets, which is true of nothing, and whether
   the classic rule binds administrators is not read.
   **Part status:** open
4. **AC.7.2 is cited for an Environment's required reviewers.** The requirement asks for a person's review of
   AI-written infrastructure and pipeline configuration before it runs; an Environment's reviewer approves a job, not
   the configuration. The record says "in part"; it is a loose fit, and may be right to keep with plainer words.
   **Part status:** open
