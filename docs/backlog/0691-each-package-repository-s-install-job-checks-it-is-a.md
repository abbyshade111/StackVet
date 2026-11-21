# Each package repository's install job checks it is a required check

**Status:** open

Filed 21 November 2026, at the owner's word, beside 0690. The "Protect main" rulesets in
`abbyshade111/homebrew-stackvet` and `abbyshade111/scoop-stack-vet` require their install jobs by name, matrix values
included (`install (macos-latest, head)`). A job renamed, or a system added to the formula's matrix, is then not
required, and nothing says so: a formula or manifest that fails there could still merge.

- **The step.** In each repository's `test.yml`, every `install` job reads the rules for `main`
  (`GET /repos/{owner}/{repo}/rules/branches/main`) and fails, saying which, when its own name is not among the required
  checks. It runs on pull requests and on the weekly schedule.
- **What it misses, said in the item's note:** a job removed from the workflow stays required and never reports, so
  pull requests wait on it. That is visible ("Expected — waiting"), not silent, so it is left.
- **The witness.** Rename the job on a branch and watch the step fail.
- The two repositories are outside this one, so their pull requests carry the work; this item records them and is
  marked done here.
