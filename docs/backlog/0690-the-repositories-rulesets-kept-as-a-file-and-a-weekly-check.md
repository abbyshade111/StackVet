# The repositories' rulesets kept as a file, and a weekly check that they still match

**Status:** open

Filed 21 November 2026, at the owner's word, after `abbyshade111/homebrew-stackvet` and `abbyshade111/scoop-stack-vet`
were given a "Protect main" ruleset (backlog 0120's note of that day, ADR-108's Later entry). `tools/adr_check.py`
reads a pull request's diff, so a change made in a repository's settings, here or in the two package repositories,
owes no record line: nothing in this repository changed. ADR-051 records this repository's "Main - ADR Check" ruleset
as prose, "read on 12 October 2026", and nothing compares that prose with GitHub.

- **The file.** The expected rules for `main` in the three repositories, written as a file here (for example
  `.github/rulesets.json`): each ruleset's rules, its required checks with the app that must report them, its bypass
  list, and each repository's auto-merge setting. ADR-051 (this repository) and ADR-108 (the package repositories) add
  it to their Governs lists, so changing a repository's protection means changing the file in a pull request, and
  the "Decision records" check asks for the record's line as it does for any governed file. `tools/adr_check.py` is
  not changed.
- **The weekly check.** A scheduled workflow reads each repository's rules for `main` through GitHub's API and fails,
  naming the difference, when they are not the file's. A rule changed in the settings page without the file then
  turns red within a week. It is CI, not `sv`, so `sv`'s fence is unchanged; it needs no secret beyond the workflow's
  own read-only token if the public endpoints answer it, and the item finds out which do before relying on them.
- **The witness.** Change one value in the file on a branch and watch the check go red; put it back.
- **Later, not here:** once ConfigVet's GitHub live provider exists (ADR-157, build step 4), this check can become a
  `cv` run with drift (step 5), with these three repositories as its first real account.
